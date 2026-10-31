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
        ok = mock.Mock(returncode=0, stdout='ActiveState=inactive\nControlGroup=\n', stderr='')
        failed = mock.Mock(returncode=0, stdout='ActiveState=failed\nControlGroup=\n', stderr='')
        denied = mock.Mock(returncode=1, stdout='', stderr='denied')
        with mock.patch.object(executor.subprocess, 'run', side_effect=[ok, ok]):
            self.assertEqual(executor.stop_owned_scope(unit, True)['status'], 'LIMPEZA_OBSERVADA')
        with mock.patch.object(executor.subprocess, 'run', side_effect=[denied, ok]):
            self.assertEqual(executor.stop_owned_scope(unit, True)['status'], 'INCONCLUSIVO')
        with mock.patch.object(executor.subprocess, 'run', side_effect=[ok, failed]):
            self.assertEqual(executor.stop_owned_scope(unit, True)['status'], 'INCONCLUSIVO')
        with mock.patch.object(executor.subprocess, 'run') as run:
            self.assertEqual(executor.stop_owned_scope('user.slice', True)['status'], 'INCONCLUSIVO')
            self.assertEqual(executor.stop_owned_scope(unit, False)['status'], 'INCONCLUSIVO')
            run.assert_not_called()

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
