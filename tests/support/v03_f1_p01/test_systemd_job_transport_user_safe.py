"""C2-A-R2 T01–T45: seleção/namespace com factories e mensagens falsas."""
from dataclasses import replace
from pathlib import Path
import unittest
from unittest.mock import patch

import systemd_job_transport as t
import systemd_job_evidence as c1
import scope_population_observer as b2
import test_systemd_job_transport_safe as original
import test_systemd_job_transport_revision_safe as revision


class SafeUserTarget(unittest.TestCase):
    request_at = original.SafeTransport.request_at
    receive = original.SafeTransport.receive
    complete = original.SafeTransport.complete
    result = original.SafeTransport.result
    flags = original.SafeTransport.flags
    assert_no_completion = original.SafeTransport.assert_no_completion
    fake_context = revision.SafeRevision.fake_context

    def setUp(self):
        self.enterContext(patch.object(t.os, 'geteuid', return_value=77))
        original.SafeTransport.setUp(self)
        self.user = t.ManagerTarget(t.Manager.USER, 77)
        self.system = t.ManagerTarget(t.Manager.SYSTEM)

    def native(self, target=None):
        backend, bus, factory, loop = revision.SafeRevision.native(self)
        backend.target = self.user if target is None else target
        self.enterContext(patch.dict('os.environ', {'XDG_RUNTIME_DIR': '/run/user/77',
                                                  'DBUS_SESSION_BUS_ADDRESS': self.user.address}))
        return backend, bus, factory, loop

    def test_T01_user_selected(self):
        backend, _, factory, _ = self.native()
        backend.open()
        self.assertEqual(factory.call_args.args, ('unix:path=/run/user/77/bus',))
        self.assertEqual(backend.target.manager, t.Manager.USER)

    def test_T02_system_selected(self):
        backend, _, factory, _ = self.native(self.system)
        backend.open()
        self.assertEqual(factory.call_args.args, (t.SYSTEM_BUS_ADDRESS,))

    def test_T03_experimental_default_user(self):
        self.assertEqual(t.ManagerTarget.experimental(), self.user)
        native = t.NativeTransport(enabled=True)
        self.addCleanup(native.close)
        self.assertEqual(native.target, self.user)
        self.assertEqual(self.backend.target, self.user)
        self.complete()
        self.assertEqual(self.result()['transport']['bus_target'], 'USER')
        backend = t.SimulatedTransport(target=self.user)
        observation = t.Observation(backend)
        self.addCleanup(observation.close)
        backend.target = self.system
        observation.start()
        self.assertTrue(observation.broken)
        self.assertEqual(backend.operations, [])

    def test_T04_no_user_system_fallback(self):
        backend, _, factory, _ = self.native()
        factory.side_effect = OSError('USER unavailable')
        with self.assertRaises(OSError):
            backend.open()
        factory.assert_called_once()
        self.assertEqual(factory.call_args.args[0], self.user.address)

    def test_T05_no_system_user_fallback(self):
        backend, _, factory, _ = self.native(self.system)
        factory.side_effect = OSError('SYSTEM unavailable')
        with self.assertRaises(OSError):
            backend.open()
        factory.assert_called_once()
        self.assertEqual(factory.call_args.args[0], self.system.address)

    def test_T06_valid_uid(self):
        for uid in (0, 77, 4294967294):
            target = t.ManagerTarget(t.Manager.USER, uid)
            self.assertEqual(target.address, 'unix:path=/run/user/' + str(uid) + '/bus')
        # Declarar outro UID não autoriza conectar como ele.
        backend, _, factory, _ = self.native(t.ManagerTarget(t.Manager.USER, 78))
        with self.assertRaises(ValueError):
            backend.open()
        factory.assert_not_called()

    def test_T07_invalid_uid(self):
        for uid in (None, -1, True, '77', 1.5, 4294967295):
            with self.subTest(uid=uid), self.assertRaises(ValueError):
                t.ManagerTarget(t.Manager.USER, uid)
        with self.assertRaises(ValueError):
            t.ManagerTarget('USER', 77)

    def test_T08_absolute_path(self):
        target = t.ManagerTarget(t.Manager.USER, 77, '/run/user/77', 'unix:path=/run/user/77/bus')
        self.assertEqual(target, self.user)
        self.assertEqual(target.report()['address_status'], 'VALIDADO_SINTATICAMENTE')

    def test_T09_relative_path_rejected(self):
        for runtime in ('run/user/77', '../run/user/77', '/run/user/77/../77', '/run/user/77/'):
            with self.subTest(runtime=runtime), self.assertRaises(ValueError):
                t.ManagerTarget(t.Manager.USER, 77, runtime)

    def test_T10_arbitrary_address_rejected(self):
        for address in ('autolaunch:', 'tcp:host=example', 'unix:path=/tmp/bus',
                        'unix:path=/run/user/77/bus;unix:path=/run/dbus/system_bus_socket',
                        'unix:abstract=bus', 'unix:path=/run/user/78/bus'):
            with self.subTest(address=address), self.assertRaises(ValueError):
                t.ManagerTarget(t.Manager.USER, 77, address=address)
        with self.assertRaises(ValueError):
            t.ManagerTarget(t.Manager.SYSTEM, address=self.user.address)

    def test_T11_inconsistent_runtime_environment(self):
        with self.assertRaises(ValueError):
            self.user.validate_environment({'XDG_RUNTIME_DIR': '/run/user/78'})
        with self.assertRaises(ValueError):
            self.user.validate_environment({'XDG_RUNTIME_DIR': 'relative'})
        self.user.validate_environment({'XDG_RUNTIME_DIR': '/run/user/77'})

    def test_T12_divergent_environment_never_selects_address(self):
        with self.assertRaises(ValueError):
            self.user.validate_environment({'DBUS_SESSION_BUS_ADDRESS': self.system.address})
        self.user.validate_environment({})  # Sem sessão interativa: seleção sintática apenas.
        backend, _, factory, _ = self.native()
        with patch.dict('os.environ', {'DBUS_SESSION_BUS_ADDRESS': 'autolaunch:',
                                       'XDG_RUNTIME_DIR': '/tmp/wrong'}):
            with self.assertRaises(ValueError):
                backend.open()
        factory.assert_not_called()
        self.assertFalse(backend.target.report()['socket_verified'])

    def test_T13_address_not_authentication(self):
        report = self.user.report()
        self.assertTrue(report['address_configured'])
        self.assertEqual(report['address_status'], 'VALIDADO_SINTATICAMENTE')
        self.assertFalse(report['runtime_ownership_verified'])
        self.assertFalse(report['socket_verified'])
        self.assertFalse(report['real_authentication_proven'])
        self.assertFalse(report['manager_identity_proven'])
        self.assertFalse(report['unit_correlated'])

    def test_T14_simulation_not_real_authentication(self):
        self.complete()
        report = self.result()['transport']
        self.assertIn('SIMULADO', report['provenance'])
        self.assertFalse(report['real_authentication_proven'])

    def test_T15_valid_unique_owner(self):
        backend, _, _, _ = self.native()
        connection, owner = backend.open()
        self.assertEqual(owner, ':1.10')
        self.assertEqual(connection, backend.connection_id)
        self.assertEqual(backend.target, self.user)

    def test_T16_missing_owner(self):
        self.obs.owner = None
        self.complete()
        self.assert_no_completion()

    def test_T17_wrong_owner(self):
        self.request_at()
        self.receive(replace(self.reply, sender=':1.99'), 30)
        self.receive(self.event, 40)
        self.assert_no_completion()

    def test_T18_owner_changed(self):
        self.complete()
        self.receive(t.Envelope('owner-change', '', '', (), target=self.user), 45)
        self.assert_no_completion()

    def test_T19_disconnect(self):
        self.complete()
        self.receive(t.Envelope('disconnect', '', '', (), target=self.user), 45)
        self.assert_no_completion()

    def test_T20_subscription_confirmed(self):
        backend, _, _, _ = self.native()
        backend.open()
        backend.subscribe()
        self.assertTrue(backend.subscribed)
        self.assertEqual(self.calls[-1][0], 'Subscribe')

    def test_T21_subscription_refused(self):
        backend, bus, _, _ = self.native()
        backend.open()
        bus.send_message_with_reply_and_block.side_effect = OSError('Subscribe')
        with self.assertRaises(OSError):
            backend.subscribe()
        self.assertTrue(backend.loss)

    def test_T22_late_subscription(self):
        self.obs.ready = 21
        self.complete()
        self.assert_no_completion()

    def test_T23_event_before_reply(self):
        self.request_at()
        self.receive(self.event, 25)
        self.receive(self.reply, 30)
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T24_event_after_reply(self):
        self.complete()
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])

    def test_T25_matching_serial(self):
        self.reply = replace(self.reply, message_serial=99)
        self.complete()
        self.assertTrue(self.flags()['STOP_ACEITO'])
        self.assertEqual(self.obs.reply.reply_serial, 7)

    def test_T26_wrong_reply_serial(self):
        self.request_at()
        self.receive(replace(self.reply, reply_serial=8), 30)
        self.receive(self.event, 40)
        self.assert_no_completion()

    def test_T27_cross_manager_jobs_both_directions(self):
        for target, other in ((self.user, self.system), (self.system, self.user)):
            with self.subTest(target=target.manager):
                backend = t.SimulatedTransport(target=target)
                observation = t.Observation(backend, clock=lambda: self.now)
                self.addCleanup(observation.close)
                self.now = 5
                observation.start()
                self.now = 20
                observation.observe_request(replace(self.request, connection_id=backend.connection_id))
                self.now = 30
                observation.receive(replace(self.reply, target=target, connection_id=backend.connection_id))
                self.now = 40
                # Mesmo nome/serial/job/remetente não torna os gerenciadores equivalentes.
                observation.receive(replace(self.event, target=other, connection_id=backend.connection_id))
                report = observation.result()
                self.assertFalse(report['classifications']['JOB_CONCLUIDO_OBSERVADO'])
                self.assertTrue(observation.broken)

        backend, _, _, _ = self.native()
        backend.open()
        with self.assertRaises(ValueError):
            backend._enqueue(replace(self.event, target=self.system))
        backend.target = self.system
        with self.assertRaises(ValueError):
            backend._enqueue(self.event)
        self.assertTrue(backend.loss)

    def test_T28_unit_or_request_wrong_manager(self):
        self.complete()
        wrong = t.ScopedEvidence(self.system, self.backend.connection_id, self.before)
        report = self.result(before=wrong)
        self.assertFalse(report['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertFalse(report['unit_identity_before_verified'])
        backend = t.SimulatedTransport(target=self.user)
        observation = t.Observation(backend, clock=lambda: self.now)
        self.addCleanup(observation.close)
        self.now = 5
        observation.start()
        self.now = 20
        observation.observe_request(t.ScopedEvidence(self.system, backend.connection_id,
                                                     replace(self.request, connection_id=backend.connection_id)))
        self.assertTrue(observation.broken)
        self.assertFalse(observation.result()['classifications']['STOP_ACEITO'])

    def test_T29_old_connection(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, target=self.user, connection_id='old-epoch'), 40)
        self.assert_no_completion()

        backend, _, _, _ = self.native()
        backend.open()
        message = original.NativeAdapterWithoutBus.message(self)
        backend._filter(object(), message)
        self.assertTrue(backend.loss)
        self.assertFalse(backend.queue)

    def test_T30_concurrent_jobs(self):
        self.request_at()
        self.receive(self.reply, 30)
        self.receive(replace(self.event, body=(43, '/org/freedesktop/systemd1/job/43', 'other.scope', 'done')), 35)
        self.receive(self.event, 40)
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(len(self.result()['uncorrelated_events']), 1)

    def test_T31_duplicate(self):
        self.complete()
        self.receive(self.event, 41)
        self.assertTrue(self.flags()['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(self.result()['duplicate_events'], 1)

    def test_T32_conflict(self):
        self.complete()
        self.receive(replace(self.event, body=(*self.event.body[:3], 'failed')), 41)
        self.assert_no_completion()

    def test_T33_overflow(self):
        self.obs.max_events = 1
        self.complete()
        first = list(self.obs.events)
        self.receive(self.event, 41)
        self.assertEqual(first, self.obs.events)
        self.assert_no_completion()

    def test_T34_decode_error(self):
        self.complete()
        self.receive(replace(self.event, signature='ssss'), 41)
        self.assert_no_completion()

    def test_T35_dispatch_failure(self):
        backend, _, _, _ = self.native()
        backend.open()
        context = self.fake_context()
        context.pending.side_effect = OSError('dispatch')
        with self.assertRaises(OSError):
            backend.drain()
        self.assertTrue(backend.loss)

    def test_T36_idempotent_close(self):
        backend, bus, _, _ = self.native()
        backend.open()
        backend.close()
        backend.close()
        bus.close.assert_called_once()

    def test_T37_no_real_connection(self):
        backend, bus, factory, _ = self.native()
        backend.open()
        factory.assert_called_once()
        self.assertIs(backend.bus, bus)
        with self.assertRaises(RuntimeError):
            t.NativeTransport()

    def test_T38_subscribe_fake_only(self):
        backend, _, _, _ = self.native()
        backend.open()
        backend.subscribe()
        self.assertEqual([name for name, _ in self.calls], ['AddMatch', 'AddMatch', 'GetNameOwner', 'Subscribe'])

    def test_T39_no_stop_operation(self):
        backend, _, _, _ = self.native()
        backend.open()
        before = list(self.calls)
        with self.assertRaises(ValueError):
            backend._call(backend.owner, t.PATH, t.INTERFACE, 'StopUnit')
        self.assertEqual(self.calls, before)

    def test_T40_no_scope(self):
        self.complete()
        self.assertFalse(hasattr(t, 'subprocess'))
        self.assertEqual(self.backend.operations, ['open', 'Subscribe'])

    def test_T41_executor_unchanged_integration(self):
        root = Path(__file__).parent
        for name in ('dv_cgroup_executor.py', 'synthetic_scope_cleanup.py', 'scope_cleanup_evidence.py'):
            source = (root / name).read_text()
            self.assertIn("'--user'", source)
            self.assertNotIn('systemd_job_transport', source)

    def test_T42_c1_preserved(self):
        self.complete()
        direct = c1.correlate_stop_job(self.obs.request,
                                      c1.Subscription(5, self.backend.connection_id, ':1.10'),
                                      self.obs.reply, self.obs.events, 80, before=self.before)
        self.assertEqual(self.result()['classifications'], direct['classifications'])
        self.assertFalse(hasattr(c1, 'ManagerTarget'))

    def test_T43_b2_preserved(self):
        self.assertTrue(b2.classifications({'known_processes': [{'classification': 'AUSENTE'}]})['AUSENCIA_DOS_PROCESSOS_CONHECIDOS'])
        self.assertNotIn('systemd_job_transport', Path(b2.__file__).read_text())

    def test_T44_no_stop_promotion(self):
        self.complete()
        self.assertFalse(self.flags()['PARADA_CONFIRMADA'])
        self.assertTrue(self.flags()['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_T45_no_population_promotion(self):
        self.complete()
        self.assertFalse(b2.classifications({})['POPULACAO_VAZIA_OBSERVADA'])
        self.assertEqual(self.result()['independent_evidence']['POPULACAO_VAZIA_OBSERVADA'], 'NAO_AVALIADA_C1')


if __name__ == '__main__':
    unittest.main()
