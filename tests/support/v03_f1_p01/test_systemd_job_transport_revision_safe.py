"""C2-A-R1 T01–T45: factories, barramento e contexto exclusivamente falsos."""
from dataclasses import replace
from pathlib import Path
import unittest
from unittest.mock import Mock, patch

import systemd_job_evidence as c1
import systemd_job_transport as t
import scope_population_observer as population
import test_systemd_job_transport_safe as original


class SafeRevision(unittest.TestCase):
    setUp = original.SafeTransport.setUp
    request_at = original.SafeTransport.request_at
    receive = original.SafeTransport.receive
    complete = original.SafeTransport.complete
    result = original.SafeTransport.result
    flags = original.SafeTransport.flags
    assert_no_completion = original.SafeTransport.assert_no_completion

    def native(self, *, authenticated=True):
        # Toda abertura deve passar por esta factory falsa. Nenhuma conexão real.
        bus = Mock()
        bus.get_is_authenticated.return_value = authenticated
        bus.get_is_connected.return_value = True
        self.calls = []

        def call(message, *, timeout_s):
            member = message.get_member()
            self.calls.append((member, timeout_s))
            response = Mock()
            response.get_type.return_value = 2
            response.get_sender.return_value = ':1.10' if member == 'Subscribe' else 'org.freedesktop.DBus'
            response.get_signature.return_value = 's' if member == 'GetNameOwner' else ''
            response.get_args_list.return_value = [':1.10'] if member == 'GetNameOwner' else []
            return response

        bus.send_message_with_reply_and_block.side_effect = call
        factory = self.enterContext(patch('dbus.bus.BusConnection', return_value=bus))
        loop = self.enterContext(patch('dbus.mainloop.glib.DBusGMainLoop', return_value=object()))
        backend = t.NativeTransport(enabled=True, clock=lambda: self.now,
                                    target=t.ManagerTarget(t.Manager.SYSTEM))
        self.addCleanup(backend.close)
        return backend, bus, factory, loop

    def fake_context(self):
        context = Mock()
        context.acquire.return_value = True
        context.pending.return_value = False
        self.enterContext(patch('gi.repository.GLib.MainContext.default', return_value=context))
        return context

    def test_T01_explicit_system_bus(self):
        backend, bus, factory, loop = self.native()
        backend.open()
        self.assertEqual(factory.call_args.args, ('unix:path=/run/dbus/system_bus_socket',))
        self.assertEqual(loop.call_args.kwargs, {'set_as_default': False})

    def test_T02_no_session_or_environment_fallback(self):
        backend, bus, factory, _ = self.native()
        factory.side_effect = OSError('unavailable')
        with patch.dict('os.environ', {'DBUS_SYSTEM_BUS_ADDRESS': 'unix:path=/tmp/wrong',
                                       'DBUS_SESSION_BUS_ADDRESS': 'unix:path=/tmp/session'}):
            with self.assertRaises(OSError):
                backend.open()
        factory.assert_called_once()
        self.assertEqual(factory.call_args.args[0], t.SYSTEM_BUS_ADDRESS)
        self.assertTrue(backend.closed)

    def test_T03_dedicated_connection(self):
        first, first_bus, factory, _ = self.native()
        second_bus = Mock()
        second_bus.get_is_authenticated.return_value = True
        second_bus.get_is_connected.return_value = True
        second_bus.send_message_with_reply_and_block.side_effect = first_bus.send_message_with_reply_and_block.side_effect
        factory.side_effect = [first_bus, second_bus]
        second = t.NativeTransport(enabled=True, target=t.ManagerTarget(t.Manager.SYSTEM))
        self.addCleanup(second.close)
        first.open()
        second.open()
        self.assertEqual(factory.call_count, 2)
        self.assertIsNot(first.bus, second.bus)
        self.assertNotEqual(first.connection_id, second.connection_id)
        self.assertEqual(factory.call_args.kwargs.keys(), {'mainloop'})

    def test_T04_open_failure(self):
        backend, _, factory, _ = self.native()
        factory.side_effect = OSError('open')
        with self.assertRaises(OSError):
            backend.open()
        self.assertTrue(backend.loss)
        self.assertTrue(backend.closed)
        self.assertIsNone(backend.bus)

    def test_T05_authentication_failure(self):
        backend, bus, _, _ = self.native(authenticated=False)
        with self.assertRaises(OSError):
            backend.open()
        bus.close.assert_called_once()
        bus.send_message_with_reply_and_block.assert_not_called()

    def test_T06_unique_owner(self):
        backend, _, _, _ = self.native()
        connection, owner = backend.open()
        self.assertEqual(owner, ':1.10')
        self.assertEqual(connection, backend.connection_id)
        self.assertEqual([name for name, _ in self.calls], ['AddMatch', 'AddMatch', 'GetNameOwner'])

    def test_T07_owner_absent(self):
        backend, bus, _, _ = self.native()
        response = Mock()
        response.get_sender.return_value = 'org.freedesktop.DBus'
        response.get_signature.return_value = 's'
        response.get_args_list.return_value = ['']
        bus.send_message_with_reply_and_block.return_value = response
        bus.send_message_with_reply_and_block.side_effect = None
        with self.assertRaises(ValueError):
            backend.open()
        self.assertTrue(backend.loss)

    def test_T08_owner_reply_wrong_sender(self):
        backend, bus, _, _ = self.native()
        response = Mock()
        response.get_sender.return_value = ':1.99'
        response.get_signature.return_value = 's'
        response.get_args_list.return_value = [':1.10']
        bus.send_message_with_reply_and_block.side_effect = None
        bus.send_message_with_reply_and_block.return_value = response
        with self.assertRaises(ValueError):
            backend.open()

    def test_T09_owner_change(self):
        self.complete()
        self.receive(t.Envelope('owner-change', '', '', ()), 45)
        self.assert_no_completion()

    def test_T10_disconnect(self):
        backend, _, _, _ = self.native()
        backend.open()
        context = self.fake_context()
        backend.bus.get_is_connected.return_value = False
        _, lost = backend.drain()
        self.assertTrue(lost)
        context.release.assert_called_once()

    def test_T11_declared_serial_is_not_emission(self):
        backend, _, _, _ = self.native()
        backend.open()
        backend.subscribe()
        observation = t.Observation(backend, clock=lambda: self.now)
        observation.connection_id, observation.owner, observation.ready = backend.connection_id, backend.owner, 5
        self.now = 20
        observation.observe_request(replace(self.request, connection_id=backend.connection_id))
        self.now = 80
        report = observation.result()
        self.assertEqual(backend.expected_reply_serial, 7)
        self.assertFalse(report['classifications']['STOP_SOLICITADO'])
        self.assertEqual(report['transport']['call_binding_status'], 'DECLARADO_SEM_EMISSAO_COMPROVADA')
        for bad in (0, -1, True, 4294967296):
            other = t.NativeTransport(enabled=True)
            other.expect_declared_reply(bad)
            self.assertTrue(other.loss)
            other.close()

    def test_T12_reply_serial_matches_call(self):
        self.reply = replace(self.reply, message_serial=99)
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        # Serial da própria resposta difere: a correlação usa ReplySerial=7.
        self.assertEqual(self.obs.reply.reply_serial, 7)

    def test_T13_reply_serial_divergent(self):
        self.request_at()
        self.receive(replace(self.reply, reply_serial=99, message_serial=7), 30)
        self.receive(self.event, 40)
        self.assert_no_completion()
        self.assertTrue(self.obs.broken)

    def test_T14_reply_wrong_sender(self):
        self.request_at()
        self.receive(replace(self.reply, sender=':1.99'), 30)
        self.receive(self.event, 40)
        self.assert_no_completion()

    def test_T15_duplicate_reply(self):
        self.complete()
        first = self.obs.reply
        self.receive(self.reply, 45)
        self.assertEqual(self.obs.reply, first)
        self.assert_no_completion()

    def test_T16_missing_reply(self):
        self.request_at()
        self.receive(self.event, 40)
        self.assert_no_completion()
        self.assertFalse(self.flags()['STOP_ACEITO'])

    def test_T17_early_event(self):
        self.request_at()
        self.receive(self.event, 25)
        self.receive(self.reply, 30)
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T18_late_event(self):
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        # Resposta fora da janela é recusada, sem inferir aceitação.
        self.obs.reply = None
        self.receive(self.reply, 101)
        self.assert_no_completion()

    def test_T19_other_job(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(43, '/org/freedesktop/systemd1/job/43', 'test.scope', 'done')), 40)
        self.assert_no_completion()

    def test_T20_other_unit(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(*self.event.body[:2], 'other.scope', 'done')), 40)
        self.assert_no_completion()

    def test_T21_out_of_order(self):
        self.complete()
        self.receive(self.event, 39)
        self.assert_no_completion()

    def test_T22_old_instance_or_connection(self):
        self.request_at()
        self.obs.request = replace(self.obs.request, connection_id='previous-connection')
        self.receive(self.reply, 30)
        self.receive(self.event, 40)
        self.assert_no_completion()
        self.assertFalse(self.flags()['PARADA_CONFIRMADA'])

    def test_T23_subscription_ack(self):
        backend, _, _, _ = self.native()
        backend.open()
        backend.subscribe()
        self.assertTrue(backend.subscribed)
        self.assertEqual(self.calls[-1][0], 'Subscribe')

    def test_T24_subscription_unconfirmed(self):
        self.obs.ready = None
        self.complete()
        self.assert_no_completion()

    def test_T25_add_match_failure(self):
        backend, bus, _, _ = self.native()
        bus.send_message_with_reply_and_block.side_effect = OSError('AddMatch')
        with self.assertRaises(OSError):
            backend.open()
        self.assertTrue(backend.closed)
        self.assertTrue(backend.loss)

    def test_T26_subscribe_failure(self):
        backend, bus, _, _ = self.native()
        backend.open()
        bus.send_message_with_reply_and_block.side_effect = OSError('Subscribe')
        observation = t.Observation(backend, clock=lambda: self.now)
        with patch.object(backend, 'open', return_value=(backend.connection_id, backend.owner)):
            observation.start()
        self.assertTrue(observation.broken)
        self.assertTrue(backend.closed)

    def test_T27_overflow(self):
        self.obs.max_events = 1
        self.complete()
        self.receive(self.event, 41)
        self.assertIn('OVERFLOW', self.obs.errors)
        self.assert_no_completion()

    def test_T28_message_limit(self):
        self.complete()
        self.receive(replace(self.event, body=(*self.event.body[:3], 'x'*65537)), 45)
        self.assert_no_completion()

    def test_T29_decode_failure(self):
        self.complete()
        self.receive(replace(self.event, body=(object(),)), 45)
        self.assert_no_completion()

    def test_T30_dispatch_empty(self):
        backend, _, _, _ = self.native()
        backend.open()
        context = self.fake_context()
        self.assertEqual(backend.drain(), ([], False))
        context.iteration.assert_not_called()
        context.release.assert_called_once()

    def test_T31_dispatch_pending_bounded(self):
        backend, _, _, _ = self.native()
        backend.open()
        context = self.fake_context()
        context.pending.return_value = True
        context.iteration.side_effect = lambda blocking: backend._enqueue(self.event)
        with patch.object(t.time, 'monotonic', return_value=1.0):
            frames, lost = backend.drain()
        self.assertEqual(len(frames), t.MAX_DISPATCH)
        self.assertFalse(lost)
        self.assertEqual(context.iteration.call_count, t.MAX_DISPATCH)
        context.iteration.assert_called_with(False)

    def test_T32_context_failure_preserves_queue(self):
        backend, _, _, _ = self.native()
        backend.open()
        backend._enqueue(self.event)
        first = list(backend.queue)
        context = self.fake_context()
        context.pending.side_effect = OSError('context')
        with self.assertRaises(OSError):
            backend.drain()
        self.assertTrue(backend.loss)
        self.assertEqual(list(backend.queue), first)
        context.release.assert_called_once()
        context.acquire.return_value = False
        context.release.reset_mock()
        with self.assertRaises(RuntimeError):
            backend.drain()
        context.release.assert_not_called()
        context.acquire.return_value = True
        context.release.side_effect = OSError('release')
        with self.assertRaisesRegex(OSError, 'context'):
            backend.drain()
        self.assertTrue(any('LIBERACAO_CONTEXTO' in gap['reason'] for gap in backend.gaps))

    def test_T33_simulated_call_timeout(self):
        backend, bus, _, _ = self.native()
        backend.open()
        self.assertTrue(all(timeout == t.WAIT_SECONDS for _, timeout in self.calls))
        bus.send_message_with_reply_and_block.side_effect = TimeoutError('timeout')
        with self.assertRaises(TimeoutError):
            backend.subscribe()
        self.assertFalse(backend.subscribed)

    def test_T34_no_absolute_time_claim(self):
        self.complete()
        self.assertFalse(self.result()['transport']['absolute_deadline_guaranteed'])
        self.assertFalse(self.result()['transport']['total_memory_bound_guaranteed'])
        backend, _, _, _ = self.native()
        backend.open()
        context = self.fake_context()
        context.pending.return_value = True
        with patch.object(t.time, 'monotonic', side_effect=[0.0, 0.0, 10.0, 10.0]):
            backend.drain()
        self.assertEqual(context.iteration.call_count, 1)
        self.assertGreater(backend.last_dispatch['elapsed_seconds'], t.DISPATCH_SECONDS)
        self.assertFalse(backend.last_dispatch['absolute_deadline_guaranteed'])

    def test_T35_close_idempotent(self):
        backend, bus, _, _ = self.native()
        backend.open()
        backend.close()
        backend.close()
        bus.close.assert_called_once()
        self.assertIsNone(backend.bus)

    def test_T36_close_failure_preserves_original(self):
        backend, bus, _, _ = self.native()
        bus.get_is_authenticated.return_value = False
        bus.close.side_effect = OSError('close')
        bus.remove_message_filter.side_effect = OSError('remove')
        with self.assertRaisesRegex(OSError, 'autenticada'):
            backend.open()
        self.assertTrue(backend.closed)
        self.assertIsNone(backend.bus)
        self.assertTrue(any('ENCERRAMENTO' in gap['reason'] for gap in backend.gaps))
        self.assertTrue(any('REMOCAO_FILTRO' in gap['reason'] for gap in backend.gaps))
        self.assertTrue(any('FECHAMENTO_CONEXAO' in gap['reason'] for gap in backend.gaps))

    def test_T37_continuity_loss(self):
        self.complete()
        self.receive(t.Envelope('lost', '', '', ()), 45)
        self.assert_no_completion()

    def test_T38_prior_events_preserved(self):
        self.complete()
        events = list(self.obs.events)
        self.receive(t.Envelope('lost', '', '', ()), 45)
        self.assertEqual(self.obs.events, events)
        self.assertIsNotNone(self.result()['transport']['reply_envelope'])

    def test_T39_no_stop_promotion(self):
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        self.assertFalse(self.flags()['PARADA_CONFIRMADA'])

    def test_T40_no_zero_promotion(self):
        self.complete()
        self.assertEqual(self.result()['independent_evidence']['POPULACAO_VAZIA_OBSERVADA'], 'NAO_AVALIADA_C1')
        self.assertFalse(population.classifications({})['POPULACAO_VAZIA_OBSERVADA'])

    def test_T41_stop_not_in_native_allowlist(self):
        backend, bus, _, _ = self.native()
        backend.open()
        previous_calls = len(self.calls)
        with self.assertRaises(ValueError):
            backend._call(backend.owner, t.PATH, t.INTERFACE, 'StopUnit')
        self.assertEqual(len(self.calls), previous_calls)

    def test_T42_no_scope_creation(self):
        self.complete()
        self.assertFalse(hasattr(t, 'subprocess'))
        self.assertEqual(self.backend.operations, ['open', 'Subscribe'])

    def test_T43_no_operational_import(self):
        root = Path(__file__).parent
        for name in ('dv_cgroup_executor.py', 'synthetic_scope_cleanup.py', 'scope_cleanup_evidence.py'):
            self.assertNotIn('systemd_job_transport', (root / name).read_text())

    def test_T44_b2_independent(self):
        self.complete()
        report = population.classifications({'known_processes': [{'classification': 'AUSENTE'}]})
        self.assertTrue(report['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        self.assertFalse(report['POPULACAO_VAZIA_OBSERVADA'])

    def test_T45_c1_compatible(self):
        self.complete()
        direct = c1.correlate_stop_job(self.obs.request,
                                      c1.Subscription(5, self.backend.connection_id, ':1.10'),
                                      self.obs.reply, self.obs.events, 80, before=self.before)
        self.assertEqual(self.result()['classifications'], direct['classifications'])


if __name__ == '__main__':
    unittest.main()
