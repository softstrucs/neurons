"""C2-A T01–T40: transporte simulado; nenhum acesso ao barramento."""
from dataclasses import replace
from pathlib import Path
import unittest
from unittest.mock import patch

import systemd_job_evidence as c1
import systemd_job_transport as t
import scope_population_observer as population


class SafeTransport(unittest.TestCase):
    def setUp(self):
        self.now = 5
        self.backend = t.SimulatedTransport()
        self.obs = t.Observation(self.backend, clock=lambda: self.now)
        self.obs.start()
        self.request = c1.StopRequest('c2a-001', 'test.scope', '/org/freedesktop/systemd1/unit/test_2escope',
                                     'a'*32, 99, 20, 100, 7, self.backend.connection_id, ':1.10')
        self.before = c1.UnitIdentity(self.request.unit_name, self.request.unit_object_path,
                                     'a'*32, 99, 10, self.backend.connection_id, ':1.10')
        self.event = t.Envelope('signal', ':1.10', 'uoss',
                                (42, '/org/freedesktop/systemd1/job/42', 'test.scope', 'done'))
        self.reply = t.Envelope('reply', ':1.10', 'o', ('/org/freedesktop/systemd1/job/42',), reply_serial=7)
        self.addCleanup(self.obs.close)

    def request_at(self, timestamp=20):
        self.now = timestamp
        self.obs.observe_request(self.request)

    def receive(self, envelope, at):
        self.now = at
        self.obs.receive(envelope)

    def complete(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(self.event, 40)

    def result(self, **kwargs):
        self.now = max(self.now, 80)
        return self.obs.result(before=kwargs.pop('before', self.before), **kwargs)

    def flags(self):
        return self.result()['classifications']

    def assert_no_completion(self):
        self.assertFalse(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T01_simulated_identity(self):
        self.complete()
        report = self.result()
        self.assertIn('SIMULADO', report['transport']['provenance'])
        self.assertFalse(report['transport']['real_authentication_proven'])

    def test_T02_expected_sender(self):
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T03_unexpected_sender(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, sender='org.freedesktop.systemd1'), 40)
        self.assert_no_completion()
        self.assertIn('REMETENTE_DIVERGENTE', self.result()['transport']['errors'])

    def test_T04_owner_changed(self):
        self.complete()
        self.receive(t.Envelope('owner-change', '', '', ()), 45)
        self.assert_no_completion()

    def test_T05_disconnect_before_request(self):
        self.receive(t.Envelope('disconnect', '', '', ()), 10)
        self.complete()
        self.assert_no_completion()

    def test_T06_disconnect_during_window(self):
        self.complete()
        self.receive(t.Envelope('disconnect', '', '', ()), 50)
        self.assert_no_completion()

    def test_T07_subscription_before_request(self):
        self.complete()
        self.assertLess(self.result()['transport']['subscription_ready_ns'], 20)
        self.assertTrue(self.flags()['JOB_IDENTIFICADO'])

    def test_T08_late_subscription(self):
        self.obs.ready = 21
        self.complete()
        self.assert_no_completion()

    def test_T09_subscription_refused(self):
        self.obs.close()
        self.backend = t.SimulatedTransport(subscribe_ok=False)
        self.obs = t.Observation(self.backend, clock=lambda: self.now)
        self.obs.start()
        self.complete()
        self.assert_no_completion()

    def test_T10_event_before_reply(self):
        self.request_at()
        self.receive(self.event, 25)
        self.receive(self.reply, 30)
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T11_event_after_reply(self):
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T12_duplicate(self):
        self.complete()
        self.receive(self.event, 41)
        self.assertEqual(self.result()['duplicate_events'], 1)
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T13_conflict(self):
        self.complete()
        self.receive(replace(self.event, body=(*self.event.body[:3], 'failed')), 41)
        self.assert_no_completion()
        self.assertTrue(self.result()['conflicting_events'])

    def test_T14_job_id_divergent(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(43, *self.event.body[1:])), 40)
        self.assert_no_completion()

    def test_T15_job_path_divergent(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(42, '/org/freedesktop/systemd1/job/43', *self.event.body[2:])), 40)
        self.assert_no_completion()

    def test_T16_unit_divergent(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(*self.event.body[:2], 'external.scope', 'done')), 40)
        self.assert_no_completion()

    def test_T17_overflow(self):
        self.obs.max_events = 1
        self.complete()
        self.receive(self.event, 41)
        self.assert_no_completion()
        self.assertIn('OVERFLOW', self.result()['transport']['errors'])

    def test_T18_message_limit(self):
        self.complete()
        self.receive(replace(self.event, body=(*self.event.body[:3], 'x'*65536)), 41)
        self.assert_no_completion()
        self.assertLessEqual(self.obs.buffer_bytes, t.MAX_BUFFER_BYTES)

    def test_T19_timeout(self):
        self.request_at()
        self.now = 100
        self.assertTrue(self.result()['observation_timeout'])
        self.assert_no_completion()

    def test_T20_reply_absent(self):
        self.request_at()
        self.receive(self.event, 40)
        self.assertFalse(self.flags()['STOP_ACEITO'])

    def test_T21_done(self):
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T22_failed(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(*self.event.body[:3], 'failed')), 40)
        self.assertTrue(self.flags()['JOB_FALHOU'])

    def test_T23_canceled(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(*self.event.body[:3], 'canceled')), 40)
        self.assertTrue(self.flags()['JOB_CANCELADO'])

    def test_T24_unknown_result(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(*self.event.body[:3], 'future')), 40)
        self.assertTrue(self.flags()['RESULTADO_JOB_DESCONHECIDO'])

    def test_T25_unit_removed(self):
        self.complete()
        after = replace(self.before, observed_monotonic_ns=50, load_state='not-found', invocation_id=None)
        self.assertTrue(self.result(after=after)['unit_removed_corroborative'])
        self.assertFalse(self.result(after=after)['classifications']['PARADA_CONFIRMADA'])

    def test_T26_invocation_divergent(self):
        self.complete()
        report = self.result(before=replace(self.before, invocation_id='b'*32))
        self.assertFalse(report['unit_identity_before_verified'])
        self.assertFalse(report['classifications']['PARADA_CONFIRMADA'])

    def test_T27_missing_snapshot(self):
        self.complete()
        self.assertFalse(self.result(before=None)['unit_identity_before_verified'])
        self.assertFalse(self.flags()['PARADA_CONFIRMADA'])

    def test_T28_old_instance(self):
        self.receive(self.event, 15)
        self.request_at()
        self.receive(self.reply, 30)
        self.assert_no_completion()
        self.assertFalse(self.flags()['PARADA_CONFIRMADA'])

    def test_T29_continuity_loss(self):
        self.complete()
        self.receive(t.Envelope('lost', '', '', ()), 45)
        self.assert_no_completion()

    def test_T30_close_resources(self):
        self.complete()
        self.obs.close()
        self.obs.close()
        self.assertTrue(self.backend.closed)
        self.assertEqual(self.backend.operations.count('close-dedicated-connection'), 1)
        self.assertNotIn('Unsubscribe', self.backend.operations)
        self.receive(self.event, 45)
        self.assert_no_completion()

    def test_T31_concurrent_jobs(self):
        self.request_at()
        self.receive(self.reply, 30)
        other = replace(self.event, body=(43, '/org/freedesktop/systemd1/job/43', 'other.scope', 'done'))
        self.receive(other, 35)
        self.receive(self.event, 40)
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(len(self.result()['uncorrelated_events']), 1)

    def test_T32_out_of_order(self):
        self.complete()
        self.receive(self.event, 39)
        self.assert_no_completion()

    def test_T33_buffer_preserves_prior(self):
        self.obs.max_events = 1
        self.complete()
        first = list(self.obs.events)
        for index in range(100):
            self.receive(self.event, 41 + index)
        self.assertEqual(self.obs.events, first)
        self.assertEqual(len(self.obs.errors), t.MAX_ERRORS)
        self.assertEqual(self.obs.error_count, 100)
        self.assert_no_completion()

    def test_T34_decode_failure(self):
        self.complete()
        self.receive(replace(self.event, signature='ssss'), 45)
        self.assert_no_completion()

    def test_T35_unavailable(self):
        backend = t.SimulatedTransport(available=False)
        observation = t.Observation(backend, clock=lambda: 5)
        observation.start()
        self.assertTrue(observation.broken)
        self.assertIsNone(observation.ready)
        observation.close()

    def test_T36_no_artificial_zero(self):
        self.complete()
        self.assertEqual(self.result()['independent_evidence']['POPULACAO_VAZIA_OBSERVADA'], 'NAO_AVALIADA_C1')
        self.assertFalse(population.classifications({})['POPULACAO_VAZIA_OBSERVADA'])

    def test_T37_done_without_atomic_binding(self):
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        self.assertFalse(self.flags()['PARADA_CONFIRMADA'])
        self.assertTrue(self.flags()['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_T38_known_absence_independent(self):
        self.request_at()
        report = population.classifications({'known_processes': [{'classification': 'AUSENTE'}]})
        self.assertTrue(report['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        self.assert_no_completion()

    def test_T39_no_real_stop(self):
        with patch.object(t.NativeTransport, 'open', side_effect=AssertionError('real bus')):
            self.complete()
            self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(self.backend.operations, ['open', 'Subscribe'])
        with self.assertRaises(RuntimeError):
            t.NativeTransport()
        native = t.NativeTransport(enabled=True)
        with self.assertRaises(ValueError):
            native._call(':1.10', t.PATH, t.INTERFACE, 'StopUnit')
        native.close()

    def test_T40_no_scope_or_executor_integration(self):
        self.complete()
        self.assertFalse(hasattr(t, 'subprocess'))
        root = Path(__file__).parent
        for filename in ('synthetic_scope_cleanup.py', 'dv_cgroup_executor.py', 'scope_cleanup_evidence.py'):
            self.assertNotIn('systemd_job_transport', (root / filename).read_text())
        self.assertFalse(hasattr(self.backend, 'stop'))


class NativeAdapterWithoutBus(unittest.TestCase):
    """Exercita decoder/ciclo nativo com mensagens e conexão falsas."""
    def message(self, **overrides):
        from unittest.mock import Mock
        from dbus.lowlevel import MESSAGE_TYPE_SIGNAL
        values = dict(get_type=MESSAGE_TYPE_SIGNAL, get_serial=99, get_sender=':1.10', get_path=t.PATH,
                      get_interface=t.INTERFACE, get_member='JobRemoved', get_signature='uoss',
                      get_args_list=[42, '/org/freedesktop/systemd1/job/42', 'test.scope', 'done'])
        values.update(overrides)
        message = Mock()
        for method, value in values.items():
            getattr(message, method).return_value = value
        return message

    def native(self):
        backend = t.NativeTransport(enabled=True, clock=lambda: 40)
        backend.owner = ':1.10'
        self.addCleanup(backend.close)
        return backend

    def test_native_headers_and_local_timestamp(self):
        backend = self.native()
        backend._filter(None, self.message())
        timestamp, frame = backend.queue[0]
        self.assertEqual(timestamp, 40)
        self.assertEqual(frame.sender, ':1.10')
        self.assertEqual(frame.body[0], 42)
        self.assertFalse(backend.loss)

    def test_native_owner_change_from_bus_daemon_only(self):
        backend = self.native()
        fields = dict(get_path='/org/freedesktop/DBus', get_interface='org.freedesktop.DBus',
                      get_member='NameOwnerChanged', get_signature='sss',
                      get_args_list=[t.SERVICE, ':1.10', ':1.11'])
        backend._filter(None, self.message(get_sender=':1.99', **fields))
        self.assertFalse(backend.loss)
        backend._filter(None, self.message(get_sender='org.freedesktop.DBus', **fields))
        self.assertTrue(backend.loss)

    def test_native_overflow_preserves_frames(self):
        backend = self.native()
        for _ in range(129):
            backend._filter(None, self.message())
        self.assertTrue(backend.loss)
        self.assertEqual(len(backend.queue), 128)
        self.assertLessEqual(backend.queue_bytes, t.MAX_BUFFER_BYTES)

    def test_native_decode_error(self):
        backend = self.native()
        backend._filter(None, self.message(get_signature='ssss'))
        self.assertTrue(backend.loss)
        self.assertEqual(len(backend.queue), 0)

    def test_native_reply_uses_actual_header_serial(self):
        from dbus.lowlevel import MESSAGE_TYPE_METHOD_RETURN
        backend = self.native()
        backend.expected_reply_serial = 7
        message = self.message(get_type=MESSAGE_TYPE_METHOD_RETURN, get_reply_serial=8,
                               get_signature='o', get_args_list=['/org/freedesktop/systemd1/job/42'])
        backend._filter(None, message)
        self.assertFalse(backend.queue)
        message.get_reply_serial.return_value = 7
        backend._filter(None, message)
        self.assertEqual(backend.queue[0][1].reply_serial, 7)
        self.assertEqual(backend.queue[0][1].sender, ':1.10')

    def test_native_reply_error_invalidates_continuity(self):
        from dbus.lowlevel import MESSAGE_TYPE_ERROR
        backend = self.native()
        backend.expected_reply_serial = 7
        backend._filter(None, self.message(get_type=MESSAGE_TYPE_ERROR, get_reply_serial=7))
        self.assertTrue(backend.loss)
        self.assertFalse(backend.queue)

    def test_context_closes_after_body_exception(self):
        backend = t.SimulatedTransport()
        with self.assertRaises(OSError):
            with t.Observation(backend, clock=lambda: 5):
                raise OSError('body')
        self.assertTrue(backend.closed)

    def test_native_close_even_when_remove_filter_fails(self):
        from unittest.mock import Mock
        backend = self.native()
        bus = Mock()
        backend.bus = bus
        bus.remove_message_filter.side_effect = OSError('remove failed')
        with self.assertRaises(OSError):
            backend.close()
        bus.close.assert_called_once()
        self.assertTrue(backend.closed)

    def test_native_drain_preserves_receipt_time(self):
        backend = self.native()
        observation = t.Observation(backend, clock=lambda: 80)
        observation.connection_id = backend.connection_id
        observation.owner = ':1.10'
        backend._filter(None, self.message())
        with patch.object(backend, 'drain', return_value=(list(backend.queue), False)):
            observation.poll_native()
        self.assertEqual(observation.events[0].signal_monotonic_ns, 40)

    def test_reply_serial_cannot_be_overridden_by_body(self):
        backend = t.SimulatedTransport()
        now = [5]
        observation = t.Observation(backend, clock=lambda: now[0])
        self.addCleanup(observation.close)
        observation.start()
        request = c1.StopRequest('serial', 'test.scope', '/org/freedesktop/systemd1/unit/test',
                                 'a'*32, 99, 20, 100, 7, backend.connection_id, ':1.10')
        now[0] = 20
        observation.observe_request(request)
        now[0] = 30
        observation.receive(t.Envelope('reply', ':1.10', 'o', ('/org/freedesktop/systemd1/job/42',), reply_serial=8))
        now[0] = 40
        observation.receive(t.Envelope('signal', ':1.10', 'uoss',
                                       (42, '/org/freedesktop/systemd1/job/42', 'test.scope', 'done')))
        self.assertFalse(observation.result()['classifications']['STOP_ACEITO'])

    def test_close_failure_recorded_with_original_fault(self):
        backend = t.SimulatedTransport()
        observation = t.Observation(backend)
        observation.fault('ORIGINAL')
        with patch.object(backend, 'close', side_effect=OSError('close')):
            observation.close()
        self.assertEqual(observation.errors, ['ORIGINAL', 'ENCERRAMENTO:OSError'])
        self.assertTrue(observation.closed)


if __name__ == '__main__':
    unittest.main()
