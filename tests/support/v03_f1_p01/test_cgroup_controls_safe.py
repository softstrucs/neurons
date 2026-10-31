"""Testes puros: sem scope, sem subprocessos e sem pressão de memória."""
import copy
import contextlib
import io
from pathlib import Path
import tempfile
import unittest
from unittest import mock
import cgroup_controls as instrument


class SafeAssessment(unittest.TestCase):
    def test_pressure_touches_each_page_safely(self):
        sample = bytearray(19)
        instrument.touch_pages(sample, len(sample), 4)
        self.assertEqual(list(sample), [1 if i % 4 == 0 else 0 for i in range(19)])
        for size, page in ((0, 4), (4, 0), (-1, 4)):
            with self.assertRaises(ValueError):
                instrument.touch_pages(sample, size, page)

    def record(self):
        return {'before': {'max': str(instrument.LIMIT), 'swap_max': '0', 'swap_current': 0,
                           'swap_events': None, 'events': {'max': 0, 'oom': 0, 'oom_kill': 0}},
                'after': {'max': str(instrument.LIMIT), 'swap_max': '0', 'swap_current': 0,
                          'swap_events': None, 'events': {'max': 1, 'oom': 1, 'oom_kill': 1}},
                'below_exit': 0, 'below_inside': True, 'grandchild_inside': True,
                'pressure_inside': True, 'timeout_inside': True,
                'pressure_exit': -9, 'timeout_occurred': True, 'timeout_exit': -9,
                'children_reaped': True, 'remaining_pids': [42], 'coordinator_pid': 42, 'elapsed_s': 2}

    def test_complete_synthetic_record(self):
        self.assertEqual(instrument.evaluate(self.record())['status'], 'CONTROLES_SINTETICOS_APROVADOS')

    def test_missing_or_ambiguous_never_passes(self):
        original = self.record()
        for key in original:
            r = copy.deepcopy(original)
            del r[key]
            self.assertEqual(instrument.evaluate(r)['status'], 'INCONCLUSIVO', key)
        for change in ({'pressure_exit': -9, 'after': {'max': str(instrument.LIMIT), 'swap_max': '0',
                        'swap_current': 0, 'swap_events': None, 'events': {'max': 1, 'oom': 0, 'oom_kill': 0}}},
                       {'remaining_pids': [42, 43]}, {'grandchild_inside': False}, {'pressure_inside': False},
                       {'timeout_occurred': False}, {'elapsed_s': instrument.DEADLINE}):
            r = self.record()
            r.update(change)
            self.assertEqual(instrument.evaluate(r)['status'], 'INCONCLUSIVO')

    def test_swap_missing_or_inconsistent_is_inconclusive(self):
        for phase in ('before', 'after'):
            for value in ('max', '1', ''):
                r = self.record()
                r[phase]['swap_max'] = value
                self.assertEqual(instrument.evaluate(r)['status'], 'INCONCLUSIVO')
            for key in ('swap_max', 'swap_current', 'swap_events'):
                r = self.record()
                del r[phase][key]
                self.assertEqual(instrument.evaluate(r)['status'], 'INCONCLUSIVO')
            for value in (-1, '0', None):
                r = self.record()
                r[phase]['swap_current'] = value
                self.assertEqual(instrument.evaluate(r)['status'], 'INCONCLUSIVO')
            for value in ({}, {'max': -1}, {'max': '1'}):
                r = self.record()
                r[phase]['swap_events'] = value
                self.assertEqual(instrument.evaluate(r)['status'], 'INCONCLUSIVO')

    def test_limit_guard_fails_closed_before_allocation(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            (path / 'memory.max').write_text(str(instrument.LIMIT))
            with self.assertRaises(RuntimeError):
                instrument.require_synthetic_limits(path)
            (path / 'memory.swap.max').write_text('max')
            with self.assertRaises(RuntimeError):
                instrument.require_synthetic_limits(path)
            (path / 'memory.swap.max').write_text('0')
            instrument.require_synthetic_limits(path)
            (path / 'memory.max').write_text('max')
            with mock.patch.object(instrument, 'cgroup_path', return_value=path), \
                 mock.patch.object(instrument.mmap, 'mmap') as allocate:
                with self.assertRaises(RuntimeError):
                    instrument.worker('pressure')
                allocate.assert_not_called()
            with mock.patch.object(instrument, 'cgroup_path', return_value=path), \
                 mock.patch.object(instrument, 'snapshot', return_value={}), \
                 mock.patch.object(instrument.subprocess, 'Popen') as launch:
                with self.assertRaises(RuntimeError):
                    instrument.coordinate()
                launch.assert_not_called()

    def test_snapshot_optional_swap_events(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            for name, value in {'memory.max': str(instrument.LIMIT), 'memory.current': '10',
                                'memory.swap.max': '0', 'memory.swap.current': '0',
                                'memory.events': 'max 0\noom 0\noom_kill 0\n',
                                'cgroup.procs': '42\n'}.items():
                (path / name).write_text(value)
            self.assertIsNone(instrument.snapshot(path)['swap_events'])
            (path / 'memory.swap.events').write_text('max 3\n')
            self.assertEqual(instrument.snapshot(path)['swap_events'], {'max': 3})

    def test_systemd_oom_without_full_evidence_is_inconclusive(self):
        process = mock.Mock()
        process.communicate.return_value = ('NEURONS_CGROUP_STAGE={"stage":"before_pressure"}\n', '')
        process.returncode = -15
        shown = mock.Mock(returncode=0, stdout='ActiveState=failed\nControlGroup=\nResult=oom-kill\nOOMPolicy=continue\n')
        output = io.StringIO()
        with mock.patch.object(instrument.subprocess, 'Popen', return_value=process), \
             mock.patch.object(instrument.subprocess, 'run', return_value=shown), \
             contextlib.redirect_stdout(output):
            self.assertEqual(instrument.run_host(), 1)
        text = output.getvalue()
        self.assertIn('"status": "INCONCLUSIVO"', text)
        self.assertIn('"full_evidence_received": false', text)
        self.assertIn('"Result": "oom-kill"', text)
        self.assertIn('--property=OOMPolicy=continue', text)

    def test_stage_is_flushed_to_external_stream(self):
        stream = io.StringIO()
        with contextlib.redirect_stdout(stream):
            instrument.emit_stage('before_pressure', coordinator_pid=42)
        self.assertIn('NEURONS_CGROUP_STAGE=', stream.getvalue())
        self.assertIn('"coordinator_pid": 42', stream.getvalue())


if __name__ == '__main__':
    unittest.main()
