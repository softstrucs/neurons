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

    def test_live_policy_confirmed_and_refuses_missing_or_wrong(self):
        unit = 'neurons-v03-f1-012345'
        coordinator = {'cgroup': '/sys/fs/cgroup/user.slice/' + unit + '.scope', 'coordinator_pid': 42}
        output = ('LoadState=loaded\nActiveState=active\nOOMPolicy=continue\n'
                  'ControlGroup=/user.slice/' + unit + '.scope\n')
        shown = mock.Mock(returncode=0, stdout=output)
        with mock.patch.object(instrument.subprocess, 'run', return_value=shown), \
             mock.patch.object(instrument, 'cgroup_path', return_value=Path(coordinator['cgroup'])):
            policy = instrument.verify_live_policy(unit, coordinator)
        self.assertTrue(instrument.confirmed_policy_evidence(policy, unit))
        for key, value in [('OOMPolicy', ''), ('OOMPolicy', 'stop'), ('ActiveState', 'inactive'),
                           ('LoadState', 'not-found'), ('ControlGroup', '/user.slice/wrong.scope')]:
            changed = copy.deepcopy(policy)
            changed['properties'][key] = value
            self.assertFalse(instrument.confirmed_policy_evidence(changed, unit))
        self.assertFalse(instrument.confirmed_policy_evidence({}, unit))
        policy['pid_in_scope'] = False
        self.assertFalse(instrument.confirmed_policy_evidence(policy, unit))

    def test_disappeared_scope_cleanup_and_incomplete_cleanup(self):
        self.assertTrue(instrument.scope_cleanup_verified({'LoadState': 'not-found', 'ActiveState': 'inactive', 'ControlGroup': ''}, 0))
        self.assertFalse(instrument.scope_cleanup_verified({'ActiveState': 'active', 'ControlGroup': ''}, 0))
        self.assertFalse(instrument.scope_cleanup_verified({'ActiveState': 'inactive'}, 0))
        self.assertFalse(instrument.scope_cleanup_verified({'ActiveState': 'inactive', 'ControlGroup': ''}, 1))

    def test_pressure_refuses_absent_policy_before_allocation(self):
        with mock.patch.object(instrument, 'cgroup_path', return_value=Path('/sys/fs/cgroup/fake.scope')), \
             mock.patch.object(instrument, 'require_synthetic_limits'), \
             mock.patch.dict(instrument.os.environ, {'NEURONS_CGROUP_LIVE_POLICY': ''}), \
             mock.patch.object(instrument.mmap, 'mmap') as allocate:
            with self.assertRaises(RuntimeError):
                instrument.worker('pressure')
            allocate.assert_not_called()

    def test_stage_is_flushed_to_external_stream(self):
        stream = io.StringIO()
        with contextlib.redirect_stdout(stream):
            instrument.emit_stage('before_pressure', coordinator_pid=42)
        self.assertIn('NEURONS_CGROUP_STAGE=', stream.getvalue())
        self.assertIn('"coordinator_pid": 42', stream.getvalue())


if __name__ == '__main__':
    unittest.main()
