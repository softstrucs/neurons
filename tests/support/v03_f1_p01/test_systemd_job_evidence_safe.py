"""C1: T01–T24 com DTOs simulados; nenhuma conexão D-Bus ou unidade real."""
from dataclasses import replace
import unittest

import systemd_job_evidence as job
import scope_population_observer as population


class SafeJobEvidence(unittest.TestCase):
    def setUp(self):
        self.unit = 'neurons-v03-f1-cleanup-0123456789abcdef.scope'
        self.path = '/org/freedesktop/systemd1/unit/neurons_2dv03_2df1_2dcleanup_2d0123456789abcdef_2escope'
        self.request = job.StopRequest('c1-001', self.unit, self.path, 'a'*32, 99, 20, 100, 7, 'connection-1', ':1.10')
        self.subscription = job.Subscription(5, 'connection-1', ':1.10')
        self.reply = job.StopReply('/org/freedesktop/systemd1/job/42', 30, 7, 'connection-1', ':1.10')
        self.event = job.JobRemoved(42, self.reply.job_object_path, self.unit, 'done', 40, 'connection-1', ':1.10')
        self.before = job.UnitIdentity(self.unit, self.path, 'a'*32, 99, 10, 'connection-1', ':1.10')
        self.after = replace(self.before, observed_monotonic_ns=50, active_state='inactive', cgroup_inode=None)

    def correlate(self, events=None, **kwargs):
        defaults = dict(request=self.request, subscription=self.subscription, reply=self.reply,
                        events=[self.event] if events is None else events,
                        observed_until_ns=80, before=self.before, after=self.after)
        defaults.update(kwargs)
        return job.correlate_stop_job(**defaults)

    def test_T01_successful_job(self):
        result = self.correlate()
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(result['job_id'], 42)
        self.assertEqual(result['identity_binding_status'], 'PRE_E_POS_CORROBORADAS')
        self.assertFalse(result['classifications']['PARADA_CONFIRMADA'])
        self.assertTrue(result['classifications']['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_T02_failed_job(self):
        result = self.correlate([replace(self.event, job_result='failed')])
        self.assertTrue(result['classifications']['JOB_FALHOU'])
        self.assertFalse(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T03_canceled_job(self):
        result = self.correlate([replace(self.event, job_result='canceled')])
        self.assertTrue(result['classifications']['JOB_CANCELADO'])
        self.assertFalse(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T04_unknown_result(self):
        result = self.correlate([replace(self.event, job_result='future-result')])
        self.assertEqual(result['job_result'], 'future-result')
        self.assertTrue(result['classifications']['RESULTADO_JOB_DESCONHECIDO'])
        self.assertFalse(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T05_wrong_job_id(self):
        result = self.correlate([replace(self.event, job_id=43)])
        self.assertFalse(result['classifications']['RESULTADO_JOB_RECEBIDO'])
        self.assertTrue(result['classifications']['EVENTO_JOB_NAO_CORRELACIONADO'])
        self.assertFalse(self.correlate(reply=replace(self.reply, reply_serial=8))['classifications']['JOB_IDENTIFICADO'])

    def test_T06_wrong_job_path(self):
        result = self.correlate([replace(self.event, job_object_path='/org/freedesktop/systemd1/job/43')])
        self.assertFalse(result['classifications']['RESULTADO_JOB_RECEBIDO'])

    def test_T07_wrong_unit(self):
        result = self.correlate([replace(self.event, unit_name='external.scope')])
        self.assertFalse(result['classifications']['RESULTADO_JOB_RECEBIDO'])

    def test_T08_invocation_unavailable(self):
        result = self.correlate(request=replace(self.request, invocation_id_before_stop=None))
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertFalse(result['unit_identity_before_verified'])
        self.assertFalse(result['classifications']['PARADA_CONFIRMADA'])

    def test_T09_invocation_divergent(self):
        result = self.correlate(before=replace(self.before, invocation_id='b'*32))
        self.assertFalse(result['unit_identity_before_verified'])
        self.assertTrue(result['classifications']['IDENTIDADE_INSTANCIA_INCONCLUSIVA'])
        self.assertFalse(result['classifications']['PARADA_CONFIRMADA'])

    def test_T10_duplicate_job_removed(self):
        result = self.correlate([self.event, replace(self.event, signal_monotonic_ns=41)])
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(result['duplicate_events'], 1)
        conflict = self.correlate([self.event, replace(self.event, signal_monotonic_ns=41, job_result='failed')])
        self.assertTrue(conflict['conflicting_events'])
        self.assertFalse(conflict['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T11_missing_job_removed(self):
        result = self.correlate([])
        self.assertTrue(result['classifications']['JOB_IDENTIFICADO'])
        self.assertFalse(result['classifications']['RESULTADO_JOB_RECEBIDO'])

    def test_T12_event_before_reply(self):
        result = self.correlate([replace(self.event, signal_monotonic_ns=25)])
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertLess(result['signal_monotonic_ns'], result['response_monotonic_ns'])

    def test_T13_event_after_reply(self):
        result = self.correlate()
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertGreater(result['signal_monotonic_ns'], result['response_monotonic_ns'])

    def test_T14_invalid_event_time(self):
        for timestamp in (None, 0, 19, 101):
            result = self.correlate([replace(self.event, signal_monotonic_ns=timestamp)])
            self.assertFalse(result['classifications']['RESULTADO_JOB_RECEBIDO'])
        result = self.correlate([self.event, replace(self.event, signal_monotonic_ns=35)])
        self.assertTrue(result['conflicting_events'])
        self.assertFalse(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T15_observation_timeout(self):
        result = self.correlate([], observed_until_ns=100)
        self.assertTrue(result['observation_timeout'])
        self.assertTrue(result['classifications']['RESULTADO_JOB_DESCONHECIDO'])

    def test_T16_bus_unavailable(self):
        result = self.correlate([], subscription=replace(self.subscription, available=False), reply=None)
        self.assertFalse(result['classifications']['STOP_ACEITO'])
        self.assertFalse(result['classifications']['JOB_IDENTIFICADO'])

    def test_T17_unit_removed_before_post_query(self):
        result = self.correlate(after=replace(self.after, load_state='not-found', invocation_id=None))
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertTrue(result['unit_removed_corroborative'])
        self.assertFalse(result['unit_identity_after_corroborated'])
        self.assertFalse(result['classifications']['PARADA_CONFIRMADA'])

    def test_T18_journal_without_instance_binding(self):
        evidence = population.journal_evidence({'USER_UNIT': self.unit, 'MESSAGE': 'Stopped'},
                                               type('Proof', (), {'unit': self.unit, 'invocation_id': 'a'*32})())
        self.assertFalse(evidence['instance_linked'])
        self.assertFalse(evidence['confirms_stop'])
        self.assertFalse(self.correlate([])['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T19_completed_job_without_zero(self):
        result = self.correlate()
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(result['independent_evidence']['POPULACAO_VAZIA_OBSERVADA'], 'NAO_AVALIADA_C1')
        self.assertTrue(result['classifications']['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_T20_zero_without_job_completion(self):
        report = {'invocation_id': 'a'*32, 'request': {'status': 'ACEITA'},
                  'request_started': {'monotonic_ns': 20}, 'request_finished': {'monotonic_ns': 25},
                  'concurrent_population': {'instance_bound': True, 'inode': 99, 'invocation_id': 'a'*32,
                   'samples': [{'monotonic_ns': t, 'inode': 99, 'invocation_id': 'a'*32,
                                'events': {'populated': value}} for t, value in [(10, '1'), (40, '0')]]}}
        self.assertTrue(population.classifications(report)['POPULACAO_VAZIA_OBSERVADA'])
        self.assertFalse(self.correlate([])['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T21_known_absent_without_job_completion(self):
        self.assertTrue(population.classifications({'known_processes': [{'classification': 'AUSENTE'}]})['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        self.assertFalse(self.correlate([])['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T22_concurrent_jobs(self):
        other = replace(self.event, job_id=43, job_object_path='/org/freedesktop/systemd1/job/43', unit_name='other.scope')
        result = self.correlate([replace(other, signal_monotonic_ns=35), self.event, replace(other, signal_monotonic_ns=45)])
        self.assertTrue(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(len(result['uncorrelated_events']), 2)
        for bad in (replace(self.event, member='UnitRemoved'), replace(self.event, connection_id='other-connection')):
            self.assertFalse(self.correlate([bad])['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T23_previous_instance_event(self):
        # O sinal real não contém InvocationID. Evento antigo é recusado por janela/conexão/job.
        result = self.correlate([replace(self.event, signal_monotonic_ns=15)])
        self.assertFalse(result['classifications']['RESULTADO_JOB_RECEBIDO'])
        result = self.correlate([replace(self.event, sender=':1.9')])
        self.assertFalse(result['classifications']['RESULTADO_JOB_RECEBIDO'])
        result = self.correlate(after=replace(self.after, invocation_id='b'*32))
        self.assertFalse(result['unit_identity_after_corroborated'])
        self.assertFalse(result['classifications']['PARADA_CONFIRMADA'])

    def test_T24_subscription_lost(self):
        for subscription in (replace(self.subscription, confirmed=False),
                             replace(self.subscription, ready_monotonic_ns=21),
                             replace(self.subscription, lost_monotonic_ns=35)):
            result = self.correlate(subscription=subscription)
            self.assertFalse(result['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        # Perda posterior não apaga resultado já recebido na janela íntegra.
        self.assertTrue(self.correlate(subscription=replace(self.subscription, lost_monotonic_ns=60))['classifications']['JOB_CONCLUIDO_OBSERVADO'])


if __name__ == '__main__':
    unittest.main()
