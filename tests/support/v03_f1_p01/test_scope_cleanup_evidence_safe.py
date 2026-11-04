"""Somente arquivos temporários, mocks e leitura /proc; nenhuma unidade real."""
import io
import os
from pathlib import Path
import subprocess
import tempfile
import time
import unittest
from unittest import mock

import dv_cgroup_executor as executor
import scope_cleanup_evidence as evidence


class SafeEvidence(unittest.TestCase):
    def setUp(self):
        self.unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        self.group = '/user.slice/' + self.unit
        self.proof = executor.ScopeOwnership(self.unit, os.getpid(), 42, 123, self.group, 'a'*32, 42, 123, os.getpid())
        self.active = {'exit': 0, 'properties': dict(evidence.POLICY, Id=self.unit, InvocationID='a'*32,
                       LoadState='loaded', ActiveState='active', ControlGroup=self.group)}
        self.inactive = {'exit': 0, 'properties': {'Id': self.unit, 'InvocationID': 'a'*32,
                         'LoadState': 'loaded', 'ActiveState': 'inactive', 'ControlGroup': ''}}
        self.empty = {'status': 'COLETA_COMPLETA', 'empty': True, 'identities': [], 'inode': 99,
                      'nodes': [{'events': {'populated': '0'}, 'pids': [], 'tids': []}]}

    def run_stop(self, queries, sample=None, request=None):
        with mock.patch.object(evidence, 'query', side_effect=queries), \
             mock.patch.object(evidence, 'collect_population', return_value=sample or self.empty), \
             mock.patch.object(evidence, 'run_request', side_effect=request) as run:
            if request is None:
                run.return_value = mock.Mock(returncode=0, stderr='')
            result = executor.stop_owned_scope(self.unit, self.proof)
        return result, run

    def test_collection_exceptions_before_stop_do_not_suppress_request(self):
        for error in (OSError('leitura original'), RuntimeError('inesperada original'), KeyError('campo original')):
            with self.subTest(error=type(error).__name__), \
                 mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
                 mock.patch.object(evidence, 'collect_population', side_effect=[error, self.empty, self.empty]), \
                 mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')) as request:
                result = executor.stop_owned_scope(self.unit, self.proof)
            request.assert_called_once()
            self.assertEqual(result['request']['status'], 'ACEITA')
            self.assertEqual(result['completion']['status'], 'CONCLUIDA')
            self.assertEqual(result['status'], 'INCONCLUSIVO')
            self.assertEqual(result['cleanup_observation']['status'], 'INCONCLUSIVO')
            self.assertIn(str(error), result['reason'])
            self.assertEqual(result['errors'][0]['stage'], 'population_before_stop')

    def test_incomplete_or_malformed_collection_still_allows_revalidated_stop(self):
        for sample in ({'status': 'INDISPONIVEL', 'empty': False, 'errors': ['permissão recusada']},
                       None, {'status': 'COLETA_COMPLETA', 'empty': True},
                       dict(self.empty, identities=[{'pid': 'wrong', 'ticks': 123}])):
            with mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
                 mock.patch.object(evidence, 'collect_population', side_effect=[sample, self.empty, self.empty]), \
                 mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')) as request:
                result = executor.stop_owned_scope(self.unit, self.proof)
            request.assert_called_once()
            self.assertEqual(result['status'], 'INCONCLUSIVO')
            self.assertEqual(result['observation_status'], 'INCOMPLETA')

    def test_final_collection_exception_preserves_stop_and_completion(self):
        with mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
             mock.patch.object(evidence, 'collect_population', side_effect=[self.empty, self.empty, AttributeError('falha final')]), \
             mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')):
            result = executor.stop_owned_scope(self.unit, self.proof)
        self.assertEqual(result['request']['status'], 'ACEITA')
        self.assertEqual(result['completion']['status'], 'CONCLUIDA')
        self.assertEqual(result['errors'][-1]['stage'], 'population_final')
        self.assertIn('falha final', result['reason'])
        self.assertEqual(result['status'], 'INCONCLUSIVO')

    def test_simultaneous_observation_request_and_final_failures_are_retained(self):
        with mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
             mock.patch.object(evidence, 'collect_population', side_effect=[RuntimeError('primeira causa'), self.empty, OSError('última coleta')]), \
             mock.patch.object(evidence, 'inspect_known', side_effect=TypeError('identidades finais')), \
             mock.patch.object(evidence, 'run_request', side_effect=OSError('pedido recusado')):
            result = executor.stop_owned_scope(self.unit, self.proof)
        self.assertIn('primeira causa', result['reason'])
        errors = str(result['errors'])
        for message in ('primeira causa', 'última coleta', 'pedido recusado', 'identidades finais'):
            self.assertIn(message, errors)
        self.assertEqual(result['request']['status'], 'FALHOU')
        self.assertEqual(result['completion']['status'], 'CONCLUIDA')
        self.assertEqual(result['status'], 'INCONCLUSIVO')

    def test_observer_failure_never_overrides_ownership_or_policy_refusal(self):
        for shown in ({'exit': 0, 'properties': dict(self.active['properties'], InvocationID='b'*32)},
                      {'exit': 0, 'properties': dict(self.active['properties'], KillMode='process')},
                      {'exit': 0, 'properties': {}}, None):
            with mock.patch.object(evidence, 'query', return_value=shown), \
                 mock.patch.object(evidence, 'collect_population', side_effect=RuntimeError('observador')), \
                 mock.patch.object(evidence, 'run_request') as request:
                result = executor.stop_owned_scope(self.unit, self.proof)
            request.assert_not_called()
            self.assertEqual(result['request']['status'], 'NAO_SOLICITADO')
            self.assertTrue(any(x['stage'] == 'revalidation' for x in result['errors']))

    def test_late_collection_and_expired_deadline_are_inconclusive(self):
        with mock.patch.object(evidence, 'collect_population') as collect:
            result = evidence.observe_population(Path('/fake'), time.monotonic() - 1)
        collect.assert_not_called()
        self.assertEqual(result['status'], 'INDISPONIVEL')
        with mock.patch.object(evidence, 'collect_population', side_effect=lambda *args: (time.sleep(0.02), self.empty)[1]):
            result = evidence.observe_population(Path('/fake'), time.monotonic() + 0.005)
        self.assertEqual(result['status'], 'INDISPONIVEL')
        self.assertIn('ultrapassado', result['errors'][0])

    def test_final_inspection_and_classification_errors_do_not_erase_original(self):
        with mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
             mock.patch.object(evidence, 'collect_population', return_value=self.empty), \
             mock.patch.object(evidence, 'inspect_known', side_effect=RuntimeError('inspeção')), \
             mock.patch.object(evidence, 'classify_cleanup', side_effect=ValueError('classificação')), \
             mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')):
            result = executor.stop_owned_scope(self.unit, self.proof)
        self.assertIn('inspeção', result['reason'])
        self.assertIn('classificação', str(result['errors']))
        self.assertEqual(result['completion']['status'], 'CONCLUIDA')
        self.assertEqual(result['status'], 'INCONCLUSIVO')

    def test_previous_incomplete_evidence_and_unexpected_channel_error_are_retained(self):
        previous = {'status': 'INDISPONIVEL', 'empty': False, 'identities': [], 'errors': ['causa antes da finalização']}
        channel = mock.Mock(closed=False)
        channel.close.side_effect = RuntimeError('canal inesperado')
        with mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
             mock.patch.object(evidence, 'collect_population', return_value=self.empty), \
             mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')) as request:
            result = executor.finish_owned_scope(self.unit, self.proof, channel, observations=[previous])
        request.assert_called_once()
        self.assertIn('canal inesperado', result['channel_close']['reason'])
        self.assertIn('causa antes da finalização', result['cleanup']['reason'])
        self.assertEqual(result['cleanup']['status'], 'INCONCLUSIVO')
        self.assertEqual(result['cleanup']['completion']['status'], 'CONCLUIDA')

    def test_concurrent_monitor_starts_before_request_and_retains_zero_after_removal(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / self.group.lstrip('/')
            path.mkdir(parents=True)
            events = path / 'cgroup.events'
            events.write_text('populated 1\nfrozen 0\n')
            sample = dict(self.empty, inode=path.stat().st_ino)
            removed = {'exit': 0, 'properties': {'Id': self.unit, 'LoadState': 'not-found',
                       'ActiveState': 'inactive', 'InvocationID': ''}}
            def accepted(unit, timeout, poll):
                monitor = poll.__self__
                self.assertTrue(monitor.samples)
                self.assertTrue(monitor.instance_bound)
                events.write_text('populated 0\nfrozen 0\n')
                poll()
                events.unlink()
                path.rmdir()
                return subprocess.CompletedProcess([], 0, '', '')
            with mock.patch.object(executor, 'ROOT', root), \
                 mock.patch.object(evidence, 'query', side_effect=[self.active, removed]), \
                 mock.patch.object(evidence, 'collect_population', return_value=sample), \
                 mock.patch.object(evidence, 'run_request', side_effect=accepted):
                result = executor.stop_owned_scope(self.unit, self.proof)
        self.assertTrue(result['concurrent_population']['descriptors_closed'])
        self.assertTrue(result['classifications']['POPULACAO_VAZIA_OBSERVADA'])
        self.assertFalse(result['classifications']['PARADA_CONFIRMADA'])
        self.assertTrue(result['classifications']['UNIDADE_REMOVIDA_OBSERVADA'])
        self.assertTrue(result['classifications']['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_monitor_finalization_error_does_not_hide_request_or_completion(self):
        with mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
             mock.patch.object(evidence, 'collect_population', return_value=self.empty), \
             mock.patch.object(evidence.concurrent.PopulationObserver, 'finish', side_effect=RuntimeError('monitor final falhou')), \
             mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')):
            result = executor.stop_owned_scope(self.unit, self.proof)
        self.assertEqual(result['request']['status'], 'ACEITA')
        self.assertEqual(result['completion']['status'], 'CONCLUIDA')
        self.assertIn('monitor final falhou', str(result['errors']))
        self.assertTrue(result['classifications']['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_release_pipe_deadline_and_no_reader(self):
        r, w = os.pipe()
        try:
            with os.fdopen(w, 'wb', buffering=0) as stream:
                evidence.send_release(stream, b'ok\n', time.monotonic() + 1)
                self.assertEqual(os.read(r, 3), b'ok\n')
                os.set_blocking(stream.fileno(), False)
                try:
                    while True:
                        os.write(stream.fileno(), b'x'*4096)
                except BlockingIOError:
                    pass
                os.set_blocking(stream.fileno(), True)
                with self.assertRaises(TimeoutError):
                    evidence.send_release(stream, b'ok\n', time.monotonic() + 0.02)
                self.assertTrue(os.get_blocking(stream.fileno()))
        finally:
            os.close(r)
        r, w = os.pipe()
        os.close(r)
        with os.fdopen(w, 'wb', buffering=0) as stream:
            with self.assertRaises(BrokenPipeError):
                evidence.send_release(stream, b'ok\n', time.monotonic() + 1)

    def test_population_changes_and_read_failure_are_incomplete(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / self.group.lstrip('/')
            self.make_node(path, '42\n', '42\n', '1')
            for identities in ([{'pid': 42, 'ppid': 7, 'ticks': 123, 'state': 'S', 'cgroup': self.group},
                                {'pid': 42, 'ppid': 7, 'ticks': 124, 'state': 'S', 'cgroup': self.group}],
                               [PermissionError('proc recusado')]):
                with mock.patch.object(evidence, 'CGROOT', root), \
                     mock.patch.object(evidence, 'process_identity', side_effect=identities):
                    result = evidence.collect_population(path, time.monotonic() + 1)
                self.assertEqual(result['status'], 'INDISPONIVEL')
                self.assertFalse(result['empty'])

    def test_disappearance_does_not_replace_final_empty_evidence(self):
        missing = {'status': 'INDISPONIVEL', 'empty': False, 'errors': ['FileNotFoundError']}
        with mock.patch.object(evidence, 'query', side_effect=[self.active, self.inactive]), \
             mock.patch.object(evidence, 'collect_population', side_effect=[self.empty, missing, missing]), \
             mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')):
            result = executor.stop_owned_scope(self.unit, self.proof)
        self.assertEqual(result['status'], 'INCONCLUSIVO')
        self.assertIn('completion', result)

    def test_policy_configured_values_and_preflight(self):
        self.assertTrue(evidence.valid_policy(self.active['properties']))
        self.assertIn('--property=TimeoutStopSec=2s', evidence.launch_properties())
        self.assertNotIn('--property=OOMPolicy=continue', evidence.launch_properties())
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / self.group.lstrip('/')
            path.mkdir(parents=True)
            (path / 'memory.max').write_text(str(evidence.MEMORY_MAX))
            with mock.patch.object(evidence, 'query', return_value=self.active), \
                 mock.patch.object(evidence, 'process_identity', return_value={'cgroup': '/outside'}):
                self.assertIn('supervisor', evidence.verify_preflight(self.unit, self.proof, root))
            for props in ({}, dict(self.active['properties'], KillMode='process')):
                with mock.patch.object(evidence, 'query', return_value={'exit': 0, 'properties': props}):
                    with self.assertRaises(ValueError):
                        evidence.verify_preflight(self.unit, self.proof, root)
            with mock.patch.object(evidence, 'query', return_value=self.active), \
                 mock.patch.object(evidence, 'process_identity', return_value={'cgroup': self.group}):
                with self.assertRaises(ValueError):
                    evidence.verify_preflight(self.unit, self.proof, root)

    def test_policy_absent_or_divergent_refuses_stop(self):
        for key in evidence.POLICY:
            for value in (None, 'wrong'):
                props = dict(self.active['properties'])
                if value is None:
                    props.pop(key)
                else:
                    props[key] = value
                result, run = self.run_stop([{'exit': 0, 'properties': props}])
                self.assertEqual(result['status'], 'INCONCLUSIVO')
                run.assert_not_called()

    def test_completed_stop_requires_confirmed_empty_population(self):
        result, run = self.run_stop([self.active, self.inactive])
        self.assertEqual(result['status'], 'VAZIO_DO_CGROUP_OBSERVADO')
        self.assertEqual(run.call_args.args[0], self.unit)
        self.assertIn('completion', result)
        self.assertIn('scope_of_claim', result)

    def test_request_timeout_does_not_imply_completion(self):
        result, _ = self.run_stop([self.active, self.inactive], request=subprocess.TimeoutExpired('stop', 1))
        self.assertTrue(result['request']['timeout'])
        self.assertIn('completion', result)
        self.assertEqual(result['status'], 'INCONCLUSIVO')

    def test_request_failure_and_failed_unit_are_inconclusive(self):
        for response in (mock.Mock(returncode=1, stderr='denied'), mock.Mock(returncode=0, stderr='')):
            final = dict(self.inactive, properties=dict(self.inactive['properties'], ActiveState='failed'))
            result, _ = self.run_stop([self.active, final], request=[response])
            self.assertEqual(result['status'], 'INCONCLUSIVO')

    def test_completion_timeout_is_bounded(self):
        with mock.patch.object(evidence, 'COMPLETION_S', 0.03), \
             mock.patch.object(evidence, 'query', return_value=self.active), \
             mock.patch.object(evidence, 'collect_population', return_value=self.empty), \
             mock.patch.object(evidence, 'run_request', return_value=mock.Mock(returncode=0, stderr='')):
            start = time.monotonic()
            result = executor.stop_owned_scope(self.unit, self.proof)
        self.assertLess(time.monotonic() - start, 1)
        self.assertEqual(result['status'], 'INCONCLUSIVO')
        self.assertIn('deadline', result['reason'])

    def test_missing_replaced_or_unavailable_unit(self):
        for shown in ({'exit': 1, 'properties': {}},
                      {'exit': 0, 'properties': dict(self.active['properties'], InvocationID='b'*32)},
                      {'exit': 0, 'properties': dict(self.active['properties'], ControlGroup='/other')}):
            result, run = self.run_stop([shown])
            run.assert_not_called()
            self.assertEqual(result['status'], 'INCONCLUSIVO')
        result, _ = self.run_stop([self.active, {'exit': 1, 'properties': {}}])
        self.assertEqual(result['status'], 'INCONCLUSIVO')
        with mock.patch.object(evidence, 'query', side_effect=OSError('bus indisponível')), \
             mock.patch.object(evidence, 'collect_population', return_value=self.empty):
            self.assertEqual(executor.stop_owned_scope(self.unit, self.proof)['status'], 'INCONCLUSIVO')

    def test_replacement_after_request_and_query_timeout(self):
        replaced = {'exit': 0, 'properties': dict(self.inactive['properties'], InvocationID='b'*32)}
        result, run = self.run_stop([self.active, replaced])
        run.assert_called_once()
        self.assertEqual(result['status'], 'INCONCLUSIVO')
        result, run = self.run_stop([self.active, subprocess.TimeoutExpired('show', 1)])
        run.assert_called_once()
        self.assertEqual(result['status'], 'INCONCLUSIVO')
        self.assertIn('known_processes', result)

    def test_identity_budget_overflow_is_not_empty_evidence(self):
        observations = [{'identities': [{'pid': 42, 'ticks': 123}, {'pid': 43, 'ticks': 124}]}]
        with mock.patch.object(evidence, 'MAX_KNOWN', 1), \
             mock.patch.object(evidence, 'process_identity', side_effect=FileNotFoundError()):
            result = evidence.inspect_known(observations, time.monotonic() + 1)
        self.assertEqual(result[-1]['classification'], 'NAO_IDENTIFICAVEL')

    def test_missing_population_and_residual_population_never_pass(self):
        for sample in ({'status': 'INDISPONIVEL', 'empty': False},
                       dict(self.empty, empty=False), dict(self.empty, identities=[{'pid': 42, 'ticks': 123}])):
            with mock.patch.object(evidence, 'inspect_known', return_value=[{'classification': 'SOBREVIVENTE'}]):
                result, _ = self.run_stop([self.active, self.inactive], sample=sample)
            self.assertEqual(result['status'], 'INCONCLUSIVO')

    def make_node(self, path, pids='', tids='', populated='0'):
        path.mkdir(parents=True, exist_ok=True)
        for name, text in [('cgroup.procs', pids), ('cgroup.threads', tids), ('cgroup.events', 'populated ' + populated + '\nfrozen 0\n')]:
            (path / name).write_text(text)

    def test_empty_population_recursive_and_missing_files(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / self.group.lstrip('/')
            self.make_node(path)
            self.make_node(path / 'child')
            with mock.patch.object(evidence, 'CGROOT', root):
                result = evidence.collect_population(path, time.monotonic() + 1)
                self.assertTrue(result['empty'])
                self.assertEqual(len(result['nodes']), 2)
                (path / 'child' / 'cgroup.threads').unlink()
                result = evidence.collect_population(path, time.monotonic() + 1)
                self.assertEqual(result['status'], 'INDISPONIVEL')
                self.assertFalse(result['empty'])
            self.assertEqual(evidence.collect_population(path / 'missing', time.monotonic() + 1)['status'], 'INDISPONIVEL')

    def test_subgroup_process_and_thread_without_marker(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / self.group.lstrip('/')
            self.make_node(path, populated='1')
            self.make_node(path / 'child', '42\n', '42\n43\n', '1')
            with mock.patch.object(evidence, 'CGROOT', root), \
                 mock.patch.object(evidence, 'process_identity', side_effect=lambda pid: {'pid': pid, 'ticks': 123, 'ppid': 7, 'state': 'S', 'cgroup': self.group + '/child'}):
                result = evidence.collect_population(path, time.monotonic() + 1)
            self.assertEqual(result['status'], 'COLETA_COMPLETA')
            self.assertFalse(result['empty'])
            self.assertEqual({x['pid'] for x in result['identities']}, {42, 43})

    def test_reuse_zombie_survivor_absent_and_unreadable(self):
        observations = [{'identities': [{'pid': 42, 'ticks': 123}]}]
        for response, classification in [({'ticks': 124, 'state': 'S'}, 'PID_REUTILIZADO'),
                                         ({'ticks': 123, 'state': 'Z'}, 'ZUMBI'),
                                         ({'ticks': 123, 'state': 'S'}, 'SOBREVIVENTE')]:
            with mock.patch.object(evidence, 'process_identity', return_value=response):
                self.assertEqual(evidence.inspect_known(observations, time.monotonic() + 1)[0]['classification'], classification)
        for exc, classification in [(FileNotFoundError(), 'AUSENTE'), (PermissionError(), 'NAO_IDENTIFICAVEL')]:
            with mock.patch.object(evidence, 'process_identity', side_effect=exc):
                self.assertEqual(evidence.inspect_known(observations, time.monotonic() + 1)[0]['classification'], classification)

    def test_real_local_identity_read_only(self):
        result = evidence.process_identity(os.getpid())
        self.assertEqual(result['pid'], os.getpid())
        self.assertGreater(result['ticks'], 0)

    def test_deadline_and_invalid_events_are_inconclusive(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            self.make_node(path)
            result = evidence.collect_population(path, time.monotonic() - 1)
            self.assertEqual(result['status'], 'INDISPONIVEL')
            (path / 'cgroup.events').write_text('frozen 0\n')
            self.assertEqual(evidence.collect_population(path, time.monotonic() + 1)['status'], 'INDISPONIVEL')

    def test_external_unit_and_channel_failure(self):
        channel = mock.Mock(closed=False)
        channel.close.side_effect = BrokenPipeError('canal')
        with mock.patch.object(evidence, 'query') as query:
            result = executor.finish_owned_scope('external.scope', self.proof, channel)
        query.assert_not_called()
        self.assertEqual(result['channel_close']['status'], 'INCONCLUSIVO')
        self.assertEqual(result['cleanup']['status'], 'INCONCLUSIVO')
        with mock.patch.object(evidence, 'query', side_effect=OSError('stop indisponível')), \
             mock.patch.object(evidence, 'collect_population', return_value=self.empty):
            result = executor.finish_owned_scope(self.unit, self.proof, channel)
        self.assertIn('canal', result['channel_close']['reason'])
        self.assertIn('stop indisponível', result['cleanup']['reason'])


if __name__ == '__main__':
    unittest.main()
