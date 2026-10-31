"""Testes puros: sem scope, sem subprocessos e sem pressão de memória."""
import copy
import unittest
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
        return {'before': {'max': str(instrument.LIMIT), 'events': {'max': 0, 'oom': 0, 'oom_kill': 0}},
                'after': {'max': str(instrument.LIMIT), 'events': {'max': 1, 'oom': 1, 'oom_kill': 1}},
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
        for change in ({'pressure_exit': -9, 'after': {'max': str(instrument.LIMIT), 'events': {'max': 1, 'oom': 0, 'oom_kill': 0}}},
                       {'remaining_pids': [42, 43]}, {'grandchild_inside': False}, {'pressure_inside': False},
                       {'timeout_occurred': False}, {'elapsed_s': instrument.DEADLINE}):
            r = self.record()
            r.update(change)
            self.assertEqual(instrument.evaluate(r)['status'], 'INCONCLUSIVO')


if __name__ == '__main__':
    unittest.main()
