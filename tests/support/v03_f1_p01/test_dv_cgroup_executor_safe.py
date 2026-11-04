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
    def test_close_failures_preserve_cleanup_and_both_diagnostics(self):
        proof = object()
        for error in (BrokenPipeError('pipe'), OSError('close')):
            channel = mock.Mock(closed=False)
            channel.close.side_effect = error
            with mock.patch.object(executor, 'stop_owned_scope', return_value={'status': 'INCONCLUSIVO', 'reason': 'stop falhou'}) as stop:
                result = executor.finish_owned_scope('unit', proof, channel)
            stop.assert_called_once_with('unit', proof)
            self.assertEqual(result['channel_close']['status'], 'INCONCLUSIVO')
            self.assertEqual(result['cleanup']['reason'], 'stop falhou')
        channel = mock.Mock(closed=True)
        with mock.patch.object(executor, 'stop_owned_scope', side_effect=OSError('bus ausente')) as stop:
            result = executor.finish_owned_scope('unit', proof, channel)
        channel.close.assert_not_called()
        stop.assert_called_once_with('unit', proof)
        self.assertEqual(result['cleanup']['status'], 'INCONCLUSIVO')

    def test_host_close_failure_does_not_suppress_owned_stop(self):
        token = '0123456789abcdef'
        unit = 'neurons-v03-f1-cleanup-' + token + '.scope'
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, '/user.slice/' + unit, 'a'*32, 42, 123, os.getpid())
        for error in (BrokenPipeError('pipe'), OSError('close')):
            channel = mock.Mock(closed=False)
            channel.close.side_effect = error
            proc = mock.Mock(pid=42, stdin=channel)
            proc.poll.return_value = 77
            with mock.patch.object(synthetic.subprocess, 'Popen', return_value=proc), \
                 mock.patch.object(synthetic.secrets, 'token_hex', return_value=token), \
                 mock.patch.object(executor, 'attest_scope', return_value=proof), \
                 mock.patch.object(executor, 'stop_owned_scope', return_value={'status': 'INCONCLUSIVO', 'reason': 'stop falhou'}) as stop, \
                 mock.patch('sys.stdout', new_callable=io.StringIO) as output:
                self.assertEqual(synthetic.host_test(), 1)
            stop.assert_called_once_with(unit, proof)
            self.assertIn('stop falhou', output.getvalue())
            self.assertIn(type(error).__name__, output.getvalue())

    def test_dv_supervisor_close_failure_still_attempts_owned_cleanup(self):
        unit = 'neurons-v03-f1-dv-0123456789abcdef.scope'
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, '/user.slice/' + unit, 'a'*32, 42, 123, os.getpid())
        ready = b'DV_READY={"pid":42,"path":"/fake.scope","before":{"max":"268435456"}}\n'
        channel = mock.Mock(closed=False)
        channel.close.side_effect = BrokenPipeError('close falhou')
        proc = mock.Mock(pid=42, stdin=channel, returncode=1)
        selector = mock.MagicMock()
        selector.__enter__.return_value = selector
        selector.get_map.side_effect = [True, False]
        selector.select.return_value = [(mock.Mock(fd=88), None)]
        shown = mock.Mock(returncode=0, stdout='ActiveState=active\nControlGroup=/fake.scope\n', stderr='')
        with mock.patch.object(executor, 'free_home', return_value=executor.MIN_FREE), \
             mock.patch.object(executor.secrets, 'token_hex', return_value='0123456789abcdef'), \
             mock.patch.object(executor.subprocess, 'Popen', return_value=proc), \
             mock.patch.object(executor.subprocess, 'run', return_value=shown), \
             mock.patch.object(executor.selectors, 'DefaultSelector', return_value=selector), \
             mock.patch.object(executor.os, 'read', return_value=ready), \
             mock.patch.object(executor, 'attest_scope', return_value=proof), \
             mock.patch.object(executor, 'validate_scope', return_value=True), \
             mock.patch.object(executor, 'stop_owned_scope', return_value={'status': 'INCONCLUSIVO', 'reason': 'stop falhou'}) as stop, \
             mock.patch('sys.stdout', new_callable=io.StringIO) as output:
            self.assertEqual(executor.supervisor(), 1)
        stop.assert_called_once_with(unit, proof)
        self.assertIn('BrokenPipeError', output.getvalue())
        self.assertIn('stop falhou', output.getvalue())

    def test_limited_line_partial_open_writer_timeout_and_mode_restore(self):
        r, w = os.pipe()
        try:
            with os.fdopen(r, 'rb') as stream:
                os.write(w, b'partial')
                started = time.monotonic()
                with self.assertRaises(TimeoutError):
                    executor.read_limited_line(stream, started + 0.05)
                self.assertLess(time.monotonic() - started, 1)
                self.assertTrue(os.get_blocking(stream.fileno()))
        finally:
            os.close(w)

    def test_limited_line_eof_partial_invalid_and_size(self):
        for payload, error in [(b'', EOFError), (b'par', EOFError),
                               (b'abcde\n', ValueError), (b'\xff\n', UnicodeDecodeError)]:
            r, w = os.pipe()
            os.write(w, payload)
            os.close(w)
            with os.fdopen(r, 'rb') as stream:
                with self.assertRaises(error):
                    executor.read_limited_line(stream, time.monotonic() + 1, 4)
        r, w = os.pipe()
        os.write(w, b'ok\nnext\n')
        os.close(w)
        with os.fdopen(r, 'rb') as stream:
            self.assertEqual(executor.read_limited_line(stream, time.monotonic() + 1), 'ok\n')
            self.assertEqual(executor.read_limited_line(stream, time.monotonic() + 1), 'next\n')
            with self.assertRaises(EOFError):
                executor.read_limited_line(stream, time.monotonic() + 1)

    def test_limited_line_short_processes_are_reaped(self):
        for program, error in [("import os,time; os.write(1,b'part'); time.sleep(2)", TimeoutError),
                               ("import os; os.write(1,b'part')", EOFError),
                               ("print('ok',flush=True)", None)]:
            proc = subprocess.Popen([sys.executable, '-c', program], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
            try:
                if error:
                    with self.assertRaises(error):
                        executor.read_limited_line(proc.stdout, time.monotonic() + 0.3)
                else:
                    self.assertEqual(executor.read_limited_line(proc.stdout, time.monotonic() + 1), 'ok\n')
            finally:
                if proc.poll() is None:
                    proc.kill()
                proc.wait(timeout=2)
                proc.stdout.close()
            self.assertFalse(Path(f'/proc/{proc.pid}').exists())

    def test_invalid_release_content_cannot_spawn(self):
        for content in ('{broken\n', '[]\n', '{"release":false}\n'):
            with mock.patch.object(executor.common, 'cgroup_path', return_value=Path('/fake.scope')), \
                 mock.patch.object(Path, 'read_text', return_value=str(64*1024*1024)), \
                 mock.patch.object(executor, 'read_limited_line', return_value=content), \
                 mock.patch.object(synthetic, 'process_identity', return_value={'pid': 42, 'ticks': 123}), \
                 mock.patch.object(synthetic.subprocess, 'Popen') as spawn:
                with self.assertRaises((ValueError, RuntimeError)):
                    synthetic.coordinator()
                spawn.assert_not_called()

    def test_live_membership_canonical_paths_and_identity(self):
        group = '/user.slice/test.scope'
        absolute = str(executor.ROOT / group.lstrip('/'))
        for live_group in (group, absolute):
            for marker_group in (group, absolute):
                line = f'V03TEST_READY|run|42|7|123|{marker_group}\n'
                with mock.patch.object(executor, 'live_process', return_value=(7, 123, live_group)):
                    self.assertEqual(executor.verify_live_test_ready(line, 'run', 42, 7, group), (7, 123, group))
        for live, line in [((7, 123, '/outside'), f'V03TEST_READY|run|42|7|123|/outside\n'),
                           ((8, 123, group), f'V03TEST_READY|run|42|8|123|{group}\n'),
                           ((7, 124, group), f'V03TEST_READY|run|42|7|123|{group}\n'),
                           ((7, 123, group), f'V03TEST_READY|run|99|7|123|{group}\n'),
                           ((7, 123, group), 'V03TEST_READY|run|42|7|123|/wrong\n')]:
            with mock.patch.object(executor, 'live_process', return_value=live):
                with self.assertRaises(RuntimeError):
                    executor.verify_live_test_ready(line, 'run', 42, 7, group)
        for invalid in ('relative/path', '/scope/../other', '/scope/./child'):
            with self.assertRaises(ValueError):
                executor.cgroup_relative(invalid)

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
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, group, 'a' * 32, 42, 123, os.getpid())
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
             mock.patch.object(executor, 'discover_coordinator', return_value=(42, 123, os.getpid())), \
             mock.patch.object(executor, 'live_process', side_effect=[(os.getpid(), 123, path)]*3):
            proof = executor.attest_scope(unit, 42)
        self.assertEqual(proof.launcher_ticks, 123)
        with mock.patch.object(executor.subprocess, 'run', return_value=shown), \
             mock.patch.object(executor, 'discover_coordinator', return_value=(42, 123, os.getpid())), \
             mock.patch.object(executor, 'live_process', side_effect=[(os.getpid(), 123, path), (os.getpid(), 124, path), (os.getpid(), 123, path)]):
            with self.assertRaises(ValueError):
                executor.attest_scope(unit, 42)
        with mock.patch.object(executor, 'live_process', side_effect=FileNotFoundError('coordenador encerrado')):
            with self.assertRaises(FileNotFoundError):
                executor.attest_scope(unit, 42)

    def test_distinct_launcher_and_coordinator(self):
        unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        group = '/user.slice/' + unit
        path = str(executor.ROOT / group.lstrip('/'))
        shown = mock.Mock(returncode=0, stdout='LoadState=loaded\nActiveState=active\nControlGroup=' + group + '\nInvocationID=' + 'a'*32 + '\n')
        with mock.patch.object(executor.subprocess, 'run', return_value=shown), \
             mock.patch.object(executor, 'discover_coordinator', return_value=(43, 124, 42)), \
             mock.patch.object(executor, 'live_process', side_effect=[(os.getpid(), 123, '/outside')]*2 + [(42, 124, path)]):
            proof = executor.attest_scope(unit, 42)
        self.assertEqual((proof.launcher_pid, proof.coordinator_pid), (42, 43))
        members = [{'pid': 43, 'ticks': 124, 'ppid': 42, 'cgroup': path},
                   {'pid': 44, 'ticks': 125, 'ppid': 43, 'cgroup': path},
                   {'pid': 45, 'ticks': 126, 'ppid': 44, 'cgroup': path}]
        self.assertTrue(synthetic.validate_marker({'members': members, 'scope_cgroup': path,
                                                  'memory_max': str(64*1024*1024)}, proof))

    def test_coordinator_exits_during_attestation(self):
        unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        group = '/user.slice/' + unit
        shown = mock.Mock(returncode=0, stdout='LoadState=loaded\nActiveState=active\nControlGroup=' + group + '\nInvocationID=' + 'a'*32 + '\n')
        with mock.patch.object(executor.subprocess, 'run', return_value=shown), \
             mock.patch.object(executor, 'discover_coordinator', return_value=(43, 124, 42)), \
             mock.patch.object(executor, 'live_process', side_effect=[(os.getpid(), 123, '/outside')]*2 + [FileNotFoundError('coordenador saiu')]):
            with self.assertRaises(FileNotFoundError):
                executor.attest_scope(unit, 42)

    def test_discovery_kernel_evidence_and_ambiguity(self):
        unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        group = '/user.slice/' + unit
        path = str(executor.ROOT / group.lstrip('/'))
        argv = b'\0'.join(os.fsencode(x) for x in [sys.executable,
                    str(Path(executor.__file__).resolve().with_name('synthetic_scope_cleanup.py')), '--coordinator']) + b'\0'
        with mock.patch.object(Path, 'read_text', return_value='43'), \
             mock.patch.object(Path, 'read_bytes', return_value=argv), \
             mock.patch.object(executor, 'live_process', return_value=(42, 124, path)):
            self.assertEqual(executor.discover_coordinator(unit, 42, group), (43, 124, 42))
        for identity, command in [((99, 124, path), argv), ((42, 124, '/wrong'), argv),
                                  ((42, 124, path), b'forged\0')]:
            with mock.patch.object(Path, 'read_text', return_value='43'), \
                 mock.patch.object(Path, 'read_bytes', return_value=command), \
                 mock.patch.object(executor, 'live_process', return_value=identity):
                with self.assertRaises(ValueError):
                    executor.discover_coordinator(unit, 42, group)
        with mock.patch.object(Path, 'read_text', return_value='43 44'), \
             mock.patch.object(Path, 'read_bytes', return_value=argv), \
             mock.patch.object(executor, 'live_process', return_value=(42, 124, path)):
            with self.assertRaises(ValueError):
                executor.discover_coordinator(unit, 42, group)

    def test_partial_or_divergent_marker_does_not_change_cleanup_proof(self):
        unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        group = '/user.slice/' + unit
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, group, 'a'*32, 42, 123, os.getpid())
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
        proof = executor.ScopeOwnership(unit, os.getpid(), 42, 123, '/user.slice/' + unit, 'a'*32, 42, 123, os.getpid())
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
             mock.patch.object(executor, 'read_limited_line', return_value='{"release":false}\n'), \
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
