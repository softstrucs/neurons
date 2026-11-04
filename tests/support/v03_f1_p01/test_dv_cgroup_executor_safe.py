"""Verificação pura do preflight; nunca executa partição nem cria scope."""
import io
import os
from pathlib import Path
import signal
import select
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock

import dv_cgroup_executor as executor
import synthetic_scope_cleanup as synthetic


class SafeExecutor(unittest.TestCase):
    def test_ready_pipe_deadline_and_closed(self):
        read_fd, write_fd = os.pipe()
        try:
            with os.fdopen(read_fd, 'rb') as stream:
                start = time.monotonic()
                with self.assertRaises(TimeoutError):
                    executor.read_test_ready(stream, start + 0.05)
                self.assertLess(time.monotonic() - start, 1.0)
                os.close(write_fd)
                write_fd = None
                with self.assertRaises(RuntimeError):
                    executor.read_test_ready(stream, time.monotonic() + 0.5)
        finally:
            if write_fd is not None:
                os.close(write_fd)

    def test_ready_pipe_valid(self):
        read_fd, write_fd = os.pipe()
        try:
            os.write(write_fd, b'prefix\nV03TEST_READY|synthetic\n')
            with os.fdopen(read_fd, 'rb') as stream:
                self.assertIn('V03TEST_READY|synthetic', executor.read_test_ready(stream, time.monotonic() + 1))
        finally:
            os.close(write_fd)

    def test_scope_stop_failure_and_failed_state(self):
        unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        group = '/user.slice/' + unit
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, group, 'a' * 32)
        prior = mock.Mock(returncode=0, stdout='InvocationID=' + 'a' * 32 + '\nControlGroup=' + group + '\n', stderr='')
        changed = mock.Mock(returncode=0, stdout='InvocationID=' + 'b' * 32 + '\nControlGroup=' + group + '\n', stderr='')
        ok = mock.Mock(returncode=0, stdout='ActiveState=inactive\nControlGroup=\n', stderr='')
        failed = mock.Mock(returncode=0, stdout='ActiveState=failed\nControlGroup=\n', stderr='')
        denied = mock.Mock(returncode=1, stdout='', stderr='denied')
        with mock.patch.object(executor.subprocess, 'run', side_effect=[prior, ok, ok]):
            self.assertEqual(executor.stop_owned_scope(unit, proof)['status'], 'LIMPEZA_OBSERVADA')
        with mock.patch.object(executor.subprocess, 'run', side_effect=[prior, denied, ok]):
            self.assertEqual(executor.stop_owned_scope(unit, proof)['status'], 'INCONCLUSIVO')
        with mock.patch.object(executor.subprocess, 'run', side_effect=[prior, ok, failed]):
            self.assertEqual(executor.stop_owned_scope(unit, proof)['status'], 'INCONCLUSIVO')
        with mock.patch.object(executor.subprocess, 'run', return_value=changed) as run:
            self.assertEqual(executor.stop_owned_scope(unit, proof)['status'], 'INCONCLUSIVO')
            self.assertEqual(run.call_count, 1)
        with mock.patch.object(executor.subprocess, 'run', side_effect=OSError('bus indisponível')):
            self.assertEqual(executor.stop_owned_scope(unit, proof)['status'], 'INCONCLUSIVO')
        with mock.patch.object(executor.subprocess, 'run') as run:
            self.assertEqual(executor.stop_owned_scope('user.slice', proof)['status'], 'INCONCLUSIVO')
            self.assertEqual(executor.stop_owned_scope(unit, None)['status'], 'INCONCLUSIVO')
            self.assertEqual(executor.stop_owned_scope(unit, True)['status'], 'INCONCLUSIVO')
            run.assert_not_called()

    def test_scope_ownership_independent_of_child_markers(self):
        unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        group = '/user.slice/' + unit
        path = str(executor.ROOT / group.lstrip('/'))
        shown = mock.Mock(returncode=0, stdout='LoadState=loaded\nActiveState=active\nControlGroup=' + group + '\nInvocationID=' + 'a'*32 + '\n')
        with mock.patch.object(executor.subprocess, 'run', return_value=shown), \
             mock.patch.object(executor, 'live_process', side_effect=[(os.getpid(), 123, path)]*2):
            proof = executor.attest_scope(unit, 42)
        self.assertEqual(proof.launcher_ticks, 123)
        with mock.patch.object(executor.subprocess, 'run', return_value=shown), \
             mock.patch.object(executor, 'live_process', side_effect=[(os.getpid(), 123, path), (os.getpid(), 124, path)]):
            with self.assertRaises(ValueError):
                executor.attest_scope(unit, 42)
        with mock.patch.object(executor, 'live_process', side_effect=FileNotFoundError('coordenador encerrado')):
            with self.assertRaises(FileNotFoundError):
                executor.attest_scope(unit, 42)

    def test_partial_or_divergent_marker_does_not_change_cleanup_proof(self):
        unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        group = '/user.slice/' + unit
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, group, 'a'*32)
        path = str(executor.ROOT / group.lstrip('/'))
        members = [{'pid': 42, 'ppid': os.getpid(), 'ticks': 123, 'cgroup': path},
                   {'pid': 43, 'ppid': 42, 'ticks': 124, 'cgroup': path},
                   {'pid': 44, 'ppid': 43, 'ticks': 125, 'cgroup': path}]
        record = {'scope_cgroup': path, 'memory_max': str(64*1024*1024), 'members': members}
        self.assertTrue(synthetic.validate_marker(record, proof))
        self.assertFalse(synthetic.validate_marker({}, proof))
        bad = dict(record, members=members[:2])
        self.assertFalse(synthetic.validate_marker(bad, proof))
        bad = dict(record, scope_cgroup='/wrong')
        self.assertFalse(synthetic.validate_marker(bad, proof))
        self.assertEqual(proof.launcher_ticks, 123)

    def test_missing_or_divergent_marker_still_requests_owned_cleanup(self):
        token = '0123456789abcdef'
        unit = 'neurons-v03-f1-cleanup-' + token + '.scope'
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, '/user.slice/' + unit, 'a'*32)
        for payload in (b'', b'SYNTH_SCOPE_READY={"scope_cgroup":"/wrong","members":[]}\n'):
            process = mock.Mock(pid=42, stdin=io.BytesIO())
            process.poll.return_value = 77
            with mock.patch.object(synthetic.subprocess, 'Popen', return_value=process), \
                 mock.patch.object(synthetic.secrets, 'token_hex', return_value=token), \
                 mock.patch.object(executor, 'attest_scope', return_value=proof), \
                 mock.patch.object(executor, 'stop_owned_scope', return_value={'status': 'INCONCLUSIVO'}) as stop, \
                 mock.patch.object(synthetic.os, 'pread', return_value=payload), \
                 mock.patch.object(synthetic.time, 'monotonic', side_effect=[0, 0, 0 if payload else 6, 6, 7, 8]), \
                 mock.patch('sys.stdout', new_callable=io.StringIO):
                self.assertEqual(synthetic.host_test(), 1)
            stop.assert_called_once_with(unit, proof)

    def test_failed_ownership_never_releases_or_stops_scope(self):
        token = '0123456789abcdef'
        unit = 'neurons-v03-f1-cleanup-' + token + '.scope'
        channel = mock.Mock(closed=False)
        process = mock.Mock(pid=42, stdin=channel)
        process.poll.return_value = 77
        with mock.patch.object(synthetic.subprocess, 'Popen', return_value=process), \
             mock.patch.object(synthetic.secrets, 'token_hex', return_value=token), \
             mock.patch.object(executor, 'attest_scope', side_effect=FileNotFoundError('processo terminou')), \
             mock.patch.object(executor, 'stop_owned_scope', return_value={'status': 'INCONCLUSIVO'}) as stop, \
             mock.patch('sys.stdout', new_callable=io.StringIO):
            self.assertEqual(synthetic.host_test(), 1)
        channel.write.assert_not_called()
        stop.assert_called_once_with(unit, None)

    def test_survivor_identity_distinguishes_pid_reuse(self):
        expected = {'pid': 42, 'ticks': 100}
        self.assertEqual(synthetic.classify_pid(expected, {'pid': 42, 'ticks': 100}), 'original')
        self.assertEqual(synthetic.classify_pid(expected, {'pid': 42, 'ticks': 101}), 'pid_reutilizado')
        with self.assertRaises(ValueError):
            synthetic.classify_pid(expected, {'pid': 43, 'ticks': 100})

    def test_real_active_descendant_and_cleanup(self):
        script = ("import subprocess,sys; c=subprocess.Popen([sys.executable,'-c','import time; time.sleep(8)']); "
                  "print(c.pid,flush=True); sys.stdin.readline(); c.terminate(); c.wait(timeout=2)")
        parent = subprocess.Popen([sys.executable, '-u', '-c', script], stdin=subprocess.PIPE,
                                  stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                                  start_new_session=True)
        descendant = None
        try:
            self.assertTrue(select.select([parent.stdout], [], [], 3)[0], 'timeout de inicialização')
            descendant = int(parent.stdout.readline())
            self.assertTrue(Path(f'/proc/{descendant}').exists())
            self.assertFalse(executor.only_coordinator([parent.pid, descendant], parent.pid))
            self.assertTrue(executor.only_coordinator([parent.pid], parent.pid))
            parent.communicate(input='stop\n', timeout=3)
            self.assertEqual(parent.returncode, 0)
            self.assertFalse(Path(f'/proc/{descendant}').exists())
        finally:
            if parent.poll() is None:
                os.killpg(parent.pid, signal.SIGKILL)
                parent.wait(timeout=3)

    def test_harness_identity_gate_rejects_missing_or_wrong(self):
        args = ('run', 12, 7, 99, '/scope')
        self.assertTrue(executor.verify_test_ready('test development ... V03TEST_READY|run|12|7|99|/scope\n', *args))
        for line in ('', 'V03TEST_READY|run|12|7|99',
                     'V03TEST_READY|run|12|8|99|/scope',
                     'V03TEST_READY|run|12|7|98|/scope'):
            self.assertFalse(executor.verify_test_ready(line, *args))

    def trace_lines(self):
        return [f'V03TRACE_RECORD|run|D-{rep}-{case}|restore|{100 + rep * 6 + case}|42|{1000 + rep * 6 + case}|/scope|10|11|12|2|0|0'
                for rep in range(3) for case in range(1, 7)]

    def test_restore_records_complete(self):
        records = executor.verify_restore_traces('\n'.join(self.trace_lines()), 'run', 'D', 42, '/scope')
        self.assertEqual(len(records), 18)
        self.assertEqual(records[0]['parent_pid'], 42)
        with self.assertRaises(ValueError):
            executor.verify_restore_traces('', 'run', 'F', 42, '/scope')

    def test_restore_records_missing_duplicated_or_divergent(self):
        lines = self.trace_lines()
        variants = [lines[:-1], lines + [lines[0]],
                    [lines[0].replace('|42|', '|41|')] + lines[1:],
                    [lines[0].replace('|/scope|', '|/wrong|')] + lines[1:],
                    [lines[0].rsplit('|', 1)[0]] + lines[1:],
                    [lines[0].replace('|10|11|12|', '|10|9|12|')] + lines[1:]]
        for value in variants:
            with self.assertRaises(ValueError):
                executor.verify_restore_traces('\n'.join(value), 'run', 'D', 42, '/scope')

    def test_only_explicit_partitions(self):
        self.assertEqual(executor.allowed_test('development'), 'development')
        self.assertEqual(executor.allowed_test('validation'), 'validation')
        for value in ('F', 'final', '', 'child_process'):
            with self.assertRaises(ValueError):
                executor.allowed_test(value)

    def test_scope_validation_fails_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / 'neurons-v03-f1-dv-test.scope'
            path.mkdir()
            (path / 'memory.max').write_text(str(executor.LIMIT))
            props = {'LoadState': 'loaded', 'ActiveState': 'active',
                     'ControlGroup': '/neurons-v03-f1-dv-test.scope'}
            with mock.patch.object(executor, 'ROOT', root), \
                 mock.patch.object(executor.common, 'cgroup_path', return_value=path):
                self.assertTrue(executor.validate_scope(path, 'neurons-v03-f1-dv-test', 42, props))
                for key in props:
                    bad = dict(props)
                    del bad[key]
                    self.assertFalse(executor.validate_scope(path, 'neurons-v03-f1-dv-test', 42, bad))
                (path / 'memory.max').write_text('max')
                self.assertFalse(executor.validate_scope(path, 'neurons-v03-f1-dv-test', 42, props))

    def test_worker_without_release_cannot_launch(self):
        with mock.patch.object(executor.common, 'cgroup_path', return_value=Path('/fake.scope')), \
             mock.patch.object(executor, 'snapshot', return_value={'max': str(executor.LIMIT)}), \
             mock.patch('sys.stdin', io.StringIO('{"release":false}\n')), \
             mock.patch.object(executor.subprocess, 'Popen') as spawn, \
             mock.patch('sys.stdout', new_callable=io.StringIO):
            with self.assertRaises(RuntimeError):
                executor.worker()
            spawn.assert_not_called()

    def test_low_space_cannot_launch_scope(self):
        with mock.patch.object(executor, 'free_home', return_value=executor.MIN_FREE - 1), \
             mock.patch.object(executor.subprocess, 'Popen') as spawn:
            with self.assertRaises(RuntimeError):
                executor.supervisor()
            spawn.assert_not_called()


if __name__ == '__main__':
    unittest.main()
