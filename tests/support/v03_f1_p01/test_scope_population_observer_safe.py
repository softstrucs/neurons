"""Arquivos temporários e clientes Python curtos; nenhum scope ou systemd real."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock

import scope_population_observer as observer


class SafePopulationObserver(unittest.TestCase):
    def make(self, path, populated='1'):
        path.mkdir()
        (path / 'cgroup.events').write_text('populated ' + populated + '\nfrozen 0\n')

    def test_readings_one_then_zero_and_fast_removal(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'scope'
            self.make(path)
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start()
            monitor.bind(path.stat().st_ino)
            (path / 'cgroup.events').write_text('populated 0\nfrozen 0\n')
            monitor.poll()
            (path / 'cgroup.events').unlink()
            path.rmdir()
            result = monitor.finish()
        self.assertEqual([x['events']['populated'] for x in result['samples'][:2]], ['1', '0'])
        self.assertTrue(result['readings_1_then_0'])
        self.assertFalse(result['transition_instant_observed'])
        self.assertTrue(result['descriptors_closed'])
        self.assertTrue(result['instance_bound'])
        report = {'invocation_id': 'a'*32, 'concurrent_population': result,
                  'completion': {'status': 'NAO_COMPROVADA'}, 'known_processes': [{'classification': 'AUSENTE'}]}
        classes = observer.classifications(report)
        self.assertTrue(classes['POPULACAO_VAZIA_OBSERVADA'])
        self.assertFalse(classes['PARADA_CONFIRMADA'])
        self.assertTrue(classes['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        self.assertTrue(classes['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_no_zero_when_transition_is_missed(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'scope'
            self.make(path)
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start()
            monitor.bind(path.stat().st_ino)
            # Simula kernfs removido: descritor não fornece leitura final.
            with mock.patch.object(observer.os, 'read', side_effect=OSError('kernfs removido')):
                result = monitor.finish()
        self.assertFalse(result['readings_1_then_0'])
        self.assertTrue(result['errors'])
        classes = observer.classifications({'invocation_id': 'a'*32, 'concurrent_population': result, 'unit_removed': True})
        self.assertFalse(classes['POPULACAO_VAZIA_OBSERVADA'])
        self.assertTrue(classes['UNIDADE_REMOVIDA_OBSERVADA'])
        self.assertTrue(result['descriptors_closed'])

    def test_absent_replaced_and_wrong_binding_never_prove_empty(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'scope'
            result = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start().finish()
            self.assertFalse(result['samples'])
            self.make(path, '0')
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start()
            monitor.bind(-1)
            result = monitor.finish()
            self.assertFalse(observer.classifications({'invocation_id': 'a'*32, 'concurrent_population': result})['POPULACAO_VAZIA_OBSERVADA'])
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1).start()
            monitor.bind(path.stat().st_ino)
            with mock.patch.object(Path, 'stat', return_value=mock.Mock(st_ino=-1)):
                monitor.poll()
            result = monitor.finish()
            self.assertTrue(any('substituído' in x['reason'] for x in result['errors']))
            self.assertFalse(observer.classifications({'invocation_id': 'b'*32, 'concurrent_population': result})['POPULACAO_VAZIA_OBSERVADA'])

    def test_volume_deadline_and_finish_are_bounded(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'scope'
            self.make(path)
            monitor = observer.PopulationObserver(path, 'a'*32, time.monotonic() + 1)
            with mock.patch.object(observer, 'MAX_SAMPLES', 2), mock.patch.object(observer, 'LOG_INTERVAL_NS', 0):
                monitor.start()
                for _ in range(10):
                    monitor.poll()
                result = monitor.finish()
            self.assertEqual(len(result['samples']), 2)
            self.assertGreater(result['dropped_samples'], 0)
            self.assertEqual(monitor.finish(), result)
            result = observer.PopulationObserver(path, 'a'*32, time.monotonic() - 1).start().finish()
            self.assertFalse(result['samples'])
            self.assertTrue(result['descriptors_closed'])

    def test_short_client_is_observed_while_running_and_reaped(self):
        samples = []
        result = observer.run_observed([sys.executable, '-B', '-c',
                  "import os,time; print(os.getpid(),flush=True); time.sleep(0.03)"], 1,
                  lambda: samples.append(time.monotonic_ns()))
        self.assertEqual(result.returncode, 0)
        self.assertGreater(len(samples), 1)
        self.assertFalse(Path('/proc/' + result.stdout.strip()).exists())

    def test_timeout_and_callback_failure_leave_no_client(self):
        with tempfile.TemporaryDirectory() as directory:
            pidfile = Path(directory) / 'pid'
            program = "import os,time; open(" + repr(str(pidfile)) + ",'w').write(str(os.getpid())); time.sleep(2)"
            with self.assertRaises(subprocess.TimeoutExpired):
                observer.run_observed([sys.executable, '-B', '-c', program], 0.15, lambda: None)
            self.assertFalse(Path('/proc/' + pidfile.read_text()).exists())
        result = observer.run_observed([sys.executable, '-B', '-c', "print('ok')"], 1,
                                      mock.Mock(side_effect=RuntimeError('observador falhou')))
        self.assertEqual(result.returncode, 0)
        self.assertTrue(result.observer_errors)

    def test_journal_by_name_or_invocation_is_only_corroborative(self):
        proof = mock.Mock(unit='scope', invocation_id='a'*32)
        for record, linked in [({'MESSAGE': 'Stopped scope', 'USER_UNIT': 'scope'}, False),
                               ({'USER_UNIT': 'scope', 'INVOCATION_ID': 'b'*32}, False),
                               ({'USER_UNIT': 'scope', 'INVOCATION_ID': 'a'*32}, True)]:
            result = observer.journal_evidence(record, proof)
            self.assertEqual(result['instance_linked'], linked)
            self.assertFalse(result['confirms_stop'])

    def test_independent_outcomes_do_not_promote_known_absence_to_empty(self):
        report = {'unit': 'scope', 'invocation_id': 'a'*32, 'completion': {'status': 'CONCLUIDA',
                  'properties': {'Id': 'scope', 'InvocationID': 'a'*32}},
                  'known_processes': [{'classification': 'AUSENTE'}]}
        result = observer.classifications(report)
        self.assertTrue(result['PARADA_CONFIRMADA'])
        report['completion']['properties']['InvocationID'] = 'b'*32
        self.assertFalse(observer.classifications(report)['PARADA_CONFIRMADA'])
        report['completion']['properties']['InvocationID'] = 'a'*32
        self.assertTrue(result['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        self.assertFalse(result['POPULACAO_VAZIA_OBSERVADA'])
        for classification in ('SOBREVIVENTE', 'ZUMBI', 'PID_REUTILIZADO', 'NAO_IDENTIFICAVEL'):
            report['known_processes'] = [{'classification': classification}]
            self.assertFalse(observer.classifications(report)['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        report['request'] = {'status': 'FALHOU'}
        self.assertTrue(observer.classifications(report)['FALHA_DE_PARADA'])
        report['request'] = {'status': 'ACEITA'}
        report['completion'] = {'status': 'NAO_COMPROVADA'}
        self.assertFalse(observer.classifications(report)['PARADA_CONFIRMADA'])


if __name__ == '__main__':
    unittest.main()
