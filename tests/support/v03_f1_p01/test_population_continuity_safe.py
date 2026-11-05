"""B2 T01–T20: dados e arquivos locais, nunca systemd ou scope real."""
from pathlib import Path
import tempfile
import time
import unittest
from unittest import mock

import scope_population_observer as observer


class SafeContinuity(unittest.TestCase):
    def report(self, readings=((10, '1'), (30, '0'))):
        return {'unit': 'scope', 'invocation_id': 'a'*32,
                'request': {'status': 'ACEITA'}, 'request_started': {'monotonic_ns': 20},
                'request_finished': {'monotonic_ns': 25},
                'concurrent_population': {'instance_bound': True, 'inode': 99, 'invocation_id': 'a'*32,
                    'errors': [], 'error_count': 0, 'gap_events': [], 'gap_events_lost': 0,
                    'dropped_samples': 0, 'samples': [self.sample(t, value) for t, value in readings]}}

    def sample(self, timestamp, value):
        return {'monotonic_ns': timestamp, 'inode': 99, 'invocation_id': 'a'*32,
                'events': {'populated': value}}

    def result(self, report):
        return observer.classifications(report)

    def interrupted(self, report, diagnostic=None):
        result = self.result(report)
        self.assertFalse(result['POPULACAO_VAZIA_OBSERVADA'])
        continuity = result['population_timing']['continuity']
        self.assertTrue(continuity['SEQUENCIA_INTERROMPIDA'])
        if diagnostic:
            self.assertTrue(continuity[diagnostic])
        self.assertTrue(result['LIMPEZA_INTEGRAL_INCONCLUSIVA'])
        return result

    def middle(self, sample):
        report = self.report()
        report['concurrent_population']['samples'].insert(1, sample)
        return report

    def test_T01_valid_one_zero_without_gaps(self):
        result = self.result(self.report())
        self.assertTrue(result['POPULACAO_VAZIA_OBSERVADA'])
        self.assertTrue(result['population_timing']['continuity']['SEQUENCIA_CONTINUA_OBSERVADA'])
        self.assertFalse(result['population_timing']['exact_transition_instant_observed'])

    def test_T02_invalid_intermediate_sample(self):
        self.interrupted(self.middle(None), 'AMOSTRA_INTERMEDIARIA_REJEITADA')

    def test_T03_invalid_intermediate_timestamp(self):
        self.interrupted(self.middle(self.sample(None, '1')), 'ORDEM_TEMPORAL_INVALIDA')

    def test_T04_out_of_order_timestamp(self):
        self.interrupted(self.middle(self.sample(5, '1')), 'ORDEM_TEMPORAL_INVALIDA')

    def test_T05_wrong_intermediate_invocation(self):
        sample = self.sample(15, '1')
        sample['invocation_id'] = 'b'*32
        self.interrupted(self.middle(sample), 'AMOSTRA_INTERMEDIARIA_REJEITADA')

    def test_T06_wrong_intermediate_inode(self):
        sample = self.sample(15, '1')
        sample['inode'] = 100
        self.interrupted(self.middle(sample), 'AMOSTRA_INTERMEDIARIA_REJEITADA')

    def test_T07_invalid_intermediate_populated(self):
        self.interrupted(self.middle(self.sample(15, 'invalid')), 'AMOSTRA_INTERMEDIARIA_REJEITADA')

    def test_T08_positioned_drop_between_one_and_zero(self):
        report = self.report()
        report['concurrent_population'].update(dropped_samples=1,
            gap_events=[{'kind': 'sample_limit', 'monotonic_ns': 15}])
        result = self.interrupted(report)
        self.assertIn('LACUNA_REGISTRADA_NO_INTERVALO', result['population_timing']['interrupted_pairs'][0]['reasons'])

    def test_T09_drop_with_unknown_position(self):
        report = self.report()
        report['concurrent_population']['dropped_samples'] = 1
        self.interrupted(report, 'DESCARTE_COM_POSICAO_DESCONHECIDA')
        report = self.report()
        report['concurrent_population']['errors'] = [{'monotonic_ns': 15, 'reason': 'contador inconsistente'}]
        self.interrupted(report, 'DESCARTE_COM_POSICAO_DESCONHECIDA')
        report = self.report()
        report['concurrent_population']['gap_events_lost'] = 1
        self.interrupted(report, 'DESCARTE_COM_POSICAO_DESCONHECIDA')

    def test_T10_valid_deduplication(self):
        report = self.report()
        first, last = report['concurrent_population']['samples']
        first.update(read_index=1, gap_epoch=0, deduplicated_since_previous=0)
        last.update(read_index=3, gap_epoch=0, deduplicated_since_previous=1)
        report['concurrent_population']['deduplicated_samples'] = 1
        self.assertTrue(self.result(report)['POPULACAO_VAZIA_OBSERVADA'])
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            events = path / 'cgroup.events'
            events.write_text('populated 1\n')
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start()
            monitor.bind(path.stat().st_ino)
            with mock.patch.object(observer, 'LOG_INTERVAL_NS', 10**10):
                monitor.poll()
                events.write_text('populated 0\n')
                monitor.poll()
                data = monitor.finish()
            self.assertGreater(data['deduplicated_samples'], 0)
            self.assertEqual(data['dropped_samples'], 0)
            self.assertTrue(data['readings_1_then_0'])
            self.assertTrue(data['descriptors_closed'])

    def test_T11_isolated_zero_after_request(self):
        result = self.result(self.report(((30, '0'),)))
        self.assertFalse(result['POPULACAO_VAZIA_OBSERVADA'])
        self.assertEqual(result['population_timing']['zero_after_request_start_ns'], [30])

    def test_T12_zero_only_before_request(self):
        result = self.result(self.report(((10, '0'),)))
        self.assertFalse(result['POPULACAO_VAZIA_OBSERVADA'])
        self.assertEqual(result['population_timing']['zero_before_stop_ns'], [10])

    def test_T13_stop_refused(self):
        report = self.report()
        report['request']['status'] = 'FALHOU'
        self.assertFalse(self.result(report)['POPULACAO_VAZIA_OBSERVADA'])

    def test_T14_stop_accepted(self):
        result = self.result(self.report())
        self.assertTrue(result['POPULACAO_VAZIA_OBSERVADA'])
        self.assertFalse(result['PARADA_CONFIRMADA'])
        self.assertFalse(result['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        self.assertTrue(result['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_T15_removed_without_zero(self):
        report = self.report(((10, '1'),))
        report['unit_removed'] = True
        result = self.result(report)
        self.assertFalse(result['POPULACAO_VAZIA_OBSERVADA'])
        self.assertTrue(result['population_timing']['removed_without_zero'])

    def test_T16_read_error_between_samples(self):
        report = self.report()
        report['concurrent_population'].update(errors=[{'monotonic_ns': 15, 'reason': 'read'}], error_count=1,
            gap_events=[{'kind': 'read_error', 'monotonic_ns': 15}])
        self.interrupted(report)
        # Mesmo erro antigo, mas sem posição, nunca permite reconstrução.
        report['concurrent_population'].update(errors=[{'reason': 'read'}], gap_events=[])
        self.interrupted(report, 'DESCARTE_COM_POSICAO_DESCONHECIDA')
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            events = path / 'cgroup.events'
            events.write_text('populated 1\n')
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start()
            monitor.bind(path.stat().st_ino)
            with mock.patch.object(observer.os, 'read', side_effect=OSError('erro entre leituras')):
                monitor.poll()
            events.write_text('populated 0\n')
            monitor.poll()
            data = monitor.finish()
        self.assertFalse(data['readings_1_then_0'])
        self.assertTrue(data['continuity']['SEQUENCIA_INTERROMPIDA'])
        self.assertTrue(data['descriptors_closed'])

    def test_T17_error_outside_qualifying_interval(self):
        for timestamp in (5, 40):
            report = self.report()
            report['concurrent_population'].update(errors=[{'monotonic_ns': timestamp, 'reason': 'read'}], error_count=1,
                gap_events=[{'kind': 'read_error', 'monotonic_ns': timestamp}])
            result = self.result(report)
            self.assertTrue(result['POPULACAO_VAZIA_OBSERVADA'])
            self.assertTrue(result['population_timing']['limitations'])

    def test_T18_sample_limit_reached(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            events = path / 'cgroup.events'
            events.write_text('populated 1\n')
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start()
            monitor.bind(path.stat().st_ino)
            with mock.patch.object(observer, 'MAX_SAMPLES', 1):
                events.write_text('populated 0\n')
                monitor.poll()
                data = monitor.finish()
            self.assertGreater(data['dropped_samples'], 0)
            self.assertTrue(data['gap_events'])
            self.assertFalse(data['readings_1_then_0'])
            self.assertTrue(data['descriptors_closed'])
        report = self.report()
        report['concurrent_population'].update(dropped_samples=1,
            gap_events=[{'kind': 'sample_limit', 'monotonic_ns': 40}])
        self.assertTrue(self.result(report)['POPULACAO_VAZIA_OBSERVADA'])

    def test_T19_multiple_valid_and_interrupted_sequences(self):
        report = self.report(((10, '1'), (15, 'bad'), (30, '0'), (35, '1'), (50, '0')))
        report['request_started']['monotonic_ns'] = 40
        report['request_finished']['monotonic_ns'] = 45
        result = self.result(report)
        self.assertTrue(result['POPULACAO_VAZIA_OBSERVADA'])
        self.assertEqual(len(result['population_timing']['readings_1_then_0']), 1)
        self.assertEqual(result['population_timing']['readings_1_then_0'][0]['one_ns'], 35)
        self.assertTrue(result['population_timing']['continuity']['SEQUENCIA_INTERROMPIDA'])
        report['concurrent_population']['samples'][1]['monotonic_ns'] = 5
        result = self.result(report)
        self.assertTrue(result['POPULACAO_VAZIA_OBSERVADA'])
        self.assertTrue(result['population_timing']['continuity']['ORDEM_TEMPORAL_INVALIDA'])
        self.assertEqual(result['population_timing']['readings_1_before_0_after'][0]['one_ns'], 35)

    def test_T20_missing_request_timestamps(self):
        for key in ('request_started', 'request_finished'):
            report = self.report()
            report.pop(key)
            self.assertFalse(self.result(report)['POPULACAO_VAZIA_OBSERVADA'])


if __name__ == '__main__':
    unittest.main()
