"""C2-B T01–T50: contratos puros, todas as evidências explicitamente sintéticas."""
from dataclasses import FrozenInstanceError, replace
from pathlib import Path
import unittest
from unittest.mock import patch

import systemd_observation_contracts as b
import systemd_job_evidence as c1
import systemd_job_transport as t
import scope_population_observer as population


class SafeContracts(unittest.TestCase):
    def setUp(self):
        self.origin = b.Origin(b.Provenance.SIMULADA, 'fixture-c2b-sem-operacao-real')
        self.target = t.ManagerTarget(t.Manager.USER, 77)
        self.manager = b.ManagerContract(self.target, 'fixture-connection', ':1.10', 5, origin=self.origin)
        self.adapter = b.ObservationalContracts(self.manager)
        self.unit = 'fixture.scope'
        self.path = '/org/freedesktop/systemd1/unit/fixture_2escope'
        self.before = b.UnitSnapshot(self.manager, self.unit, self.path, 'a'*32,
                                     '/user.slice/fixture.scope', 99, 10, origin=self.origin)
        self.after = replace(self.before, monotonic_ns=50, active_state='inactive')
        self.emission = b.EmissionContract(self.manager, self.unit, self.path, 7, 20, 100, self.origin)
        self.reply = b.ReplyEvidence(self.manager,
                                    c1.StopReply('/org/freedesktop/systemd1/job/42', 30, 7,
                                                 self.manager.connection_id, self.manager.owner), self.origin)
        self.event = b.EventEvidence(self.manager,
                                    c1.JobRemoved(42, '/org/freedesktop/systemd1/job/42', self.unit, 'done',
                                                  40, self.manager.connection_id, self.manager.owner), self.origin)
        self.directory = b.ObjectFixture('/run/user/77', '/run/user/77', 'directory', 77, 0o700, 1, 2, 2, origin=self.origin)
        self.socket = b.ObjectFixture('/run/user/77/bus', '/run/user/77/bus', 'socket', 77, 0o600, 1, 3, 3, origin=self.origin)
        self.runtime = b.RuntimeContract(self.target, socket_mode=0o600)  # Política da fixture, não do host.

    def finish(self, *, early=False):
        self.adapter.record_snapshot('before', self.before)
        self.adapter.declare_emission(self.emission)
        if early:
            self.adapter.receive_job_removed(replace(self.event, event=replace(self.event.event, signal_monotonic_ns=25)))
            self.adapter.receive_reply(self.reply)
        else:
            self.adapter.receive_reply(self.reply)
            self.adapter.receive_job_removed(self.event)
        self.adapter.record_snapshot('after', self.after)
        return self.adapter.result(80)

    def no_done(self, report=None):
        report = self.adapter.result(80) if report is None else report
        if report['simulated_c1'] is not None:
            self.assertFalse(report['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertFalse(report['operational']['PARADA_CONFIRMADA'])

    def test_T01_user_contract(self):
        self.assertEqual(self.manager.target.manager, t.Manager.USER)
        self.assertEqual(b.ManagerContract.experimental(77).target, self.target)
        self.assertEqual(self.manager.service, t.SERVICE)

    def test_T02_system_contract(self):
        manager = b.ManagerContract(t.ManagerTarget(t.Manager.SYSTEM))
        self.assertEqual(manager.target.address, t.SYSTEM_BUS_ADDRESS)
        self.assertNotEqual(manager.target, self.target)

    def test_T03_uid_mismatch(self):
        result = self.runtime.evaluate(replace(self.directory, uid=78), self.socket)
        self.assertIn('PROPRIEDADE_DIVERGENTE', result['errors'])

    def test_T04_runtime_mismatch(self):
        result = self.runtime.evaluate(replace(self.directory, path='/run/user/78'), self.socket)
        self.assertIn('CAMINHO_DIVERGENTE', result['errors'])
        self.assertIn('RUNTIME_AUSENTE', self.runtime.evaluate(None, self.socket)['errors'])

    def test_T05_socket_mismatch(self):
        result = self.runtime.evaluate(self.directory, replace(self.socket, path='/tmp/socket'))
        self.assertIn('CAMINHO_DIVERGENTE', result['errors'])

    def test_T06_relative_path(self):
        with self.assertRaises(ValueError):
            t.ManagerTarget(t.Manager.USER, 77, 'run/user/77')
        result = self.runtime.evaluate(self.directory, replace(self.socket, path='relative'))
        self.assertFalse(result['simulated_socket_verified'])

    def test_T07_symlink(self):
        result = self.runtime.evaluate(self.directory, replace(self.socket, symlink=True))
        self.assertIn('SYMLINK_RECUSADO', result['errors'])

    def test_T08_object_replacement(self):
        previous = replace(self.socket, monotonic_ns=2)
        self.assertTrue(self.runtime.evaluate(self.directory, self.socket, previous_socket=previous)['simulated_socket_verified'])
        result = self.runtime.evaluate(self.directory, replace(self.socket, inode=4), previous_socket=previous)
        self.assertIn('SOCKET_SUBSTITUIDO', result['errors'])
        result = self.runtime.evaluate(replace(self.directory, inode=8, monotonic_ns=4), self.socket,
                                       previous_runtime=self.directory)
        self.assertIn('RUNTIME_SUBSTITUIDO', result['errors'])

    def test_T09_permissions(self):
        for bad in (0o777, 0o660, True):
            result = self.runtime.evaluate(self.directory, replace(self.socket, mode=bad))
            self.assertIn('PERMISSAO_INADEQUADA', result['errors'])

    def test_T10_owner_mismatch(self):
        result = self.runtime.evaluate(self.directory, replace(self.socket, uid=0))
        self.assertIn('PROPRIEDADE_DIVERGENTE', result['errors'])
        result = self.runtime.evaluate(self.directory, self.socket, environment={'XDG_RUNTIME_DIR': '/tmp/wrong'})
        self.assertIn('AMBIENTE_INCONSISTENTE', result['errors'])

    def test_T11_no_authentication(self):
        result = self.adapter.record_runtime(self.directory, self.socket, socket_mode=0o600)
        self.assertTrue(result['simulated_socket_verified'])
        self.assertFalse(result['connection_authenticated'])
        self.assertEqual(self.manager.authentication, b.Verification.NAO_VERIFICADO)
        self.assertEqual(self.adapter.result(4)['runtime']['state'], b.Verification.OBSERVADO_SIMULADAMENTE)
        unspecified = b.RuntimeContract(self.target).evaluate(self.directory, self.socket)
        self.assertIn('POLITICA_PERMISSAO_SOCKET_NAO_DEFINIDA', unspecified['errors'])
        self.assertFalse(unspecified['operational_object_verified'])

    def test_T12_unique_owner(self):
        result = self.adapter.record_manager('owner', self.manager)
        self.assertTrue(result['consistent'])
        self.assertEqual(result['authentication'], b.Verification.NAO_VERIFICADO)
        with self.assertRaises(ValueError):
            replace(self.manager, owner=t.SERVICE)

    def test_T13_owner_change(self):
        self.adapter.record_manager('owner', replace(self.manager, owner=':1.11'))
        self.assertTrue(self.adapter.broken)
        self.no_done(self.finish())

    def test_T14_connection_mismatch(self):
        result = self.adapter.record_manager('connection', replace(self.manager, connection_id='other'))
        self.assertFalse(result['consistent'])
        self.no_done(self.finish())

    def test_T15_missing_subscription(self):
        self.manager = replace(self.manager, subscription_ns=None)
        self.adapter = b.ObservationalContracts(self.manager)
        self.adapter.record_manager('subscription', self.manager)
        self.no_done(self.finish())

    def test_T16_simulated_subscription(self):
        report = self.adapter.record_manager('subscription', self.manager)
        self.assertTrue(report['consistent'])
        self.assertEqual(report['provenance'], b.Provenance.SIMULADA)
        self.assertNotEqual(self.manager.state, b.Verification.AUTENTICADO_OPERACIONALMENTE)
        late = replace(self.manager, subscription_ns=25)
        self.assertFalse(self.adapter.record_manager('subscription', late)['consistent'])

    def test_T17_unit_identified_simulated(self):
        report = self.finish()
        self.assertTrue(report['simulated_c1']['unit_identity_before_verified'])
        self.assertFalse(report['operational']['PARADA_CONFIRMADA'])

    def test_T18_reused_name(self):
        self.after = replace(self.after, invocation_id='b'*32)
        report = self.finish()
        self.assertTrue(any('NOME_REUTILIZADO' in error['code'] for error in report['errors']))
        self.no_done(report)

    def test_T19_invocation_changed(self):
        self.after = replace(self.after, invocation_id=None, load_state='not-found', cgroup_inode=None)
        report = self.finish()
        self.no_done(report)
        self.assertFalse(report['operational']['POPULACAO_VAZIA_OBSERVADA'])

    def test_T20_before_snapshot(self):
        self.adapter.record_snapshot('before', self.before)
        self.assertEqual(self.adapter.before.monotonic_ns, 10)
        self.assertEqual(self.adapter.before.origin.provenance, b.Provenance.SIMULADA)

    def test_T21_after_snapshot(self):
        report = self.finish()
        self.assertEqual(report['snapshots']['after']['monotonic_ns'], 50)
        self.assertTrue(report['simulated_c1']['unit_identity_after_corroborated'])

    def test_T22_missing_snapshot(self):
        self.adapter.declare_emission(self.emission)
        report = self.adapter.result(80)
        self.assertIn('SNAPSHOT_ANTERIOR_AUSENTE', [e['code'] for e in report['errors']])
        self.no_done(report)
        # Um relatório intermediário incompleto não fabrica falha de transporte
        # nem impede a chegada posterior dos snapshots.
        self.adapter = b.ObservationalContracts(self.manager)
        self.adapter.result(5)
        self.assertFalse(self.adapter.broken)
        self.assertTrue(self.finish()['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T23_other_manager_snapshot(self):
        other = replace(self.manager, target=t.ManagerTarget(t.Manager.SYSTEM))
        self.before = replace(self.before, manager=other)
        self.no_done(self.finish())
        self.assertIsNone(self.adapter.before)

    def test_T24_other_connection_snapshot(self):
        self.before = replace(self.before, manager=replace(self.manager, connection_id='old'))
        self.no_done(self.finish())

    def test_T25_invalid_timestamp(self):
        for value in (None, 0, True, 51):
            with self.subTest(value=value):
                self.adapter = b.ObservationalContracts(self.manager)
                self.before = replace(self.before, monotonic_ns=value)
                self.no_done(self.finish())

    def test_T26_declared_emission(self):
        declared = replace(self.emission, origin=b.Origin())
        self.assertEqual(declared.state, b.EmissionState.EMISSAO_DECLARADA)
        self.emission = declared
        self.no_done(self.finish())

    def test_T27_serial_not_authenticated(self):
        self.assertEqual(self.emission.serial, 7)
        self.assertNotEqual(self.emission.state, b.EmissionState.EMISSAO_NATIVA_COMPROVADA)
        for serial in (0, True, -1, 4294967296):
            with self.assertRaises(ValueError):
                replace(self.emission, serial=serial)

    def test_T28_reply_serial_matches(self):
        self.assertTrue(self.finish()['simulated_c1']['classifications']['STOP_ACEITO'])

    def test_T29_reply_serial_mismatch(self):
        self.reply = replace(self.reply, reply=replace(self.reply.reply, reply_serial=8))
        self.no_done(self.finish())

    def test_T30_matching_job(self):
        report = self.finish()
        self.assertTrue(report['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(report['simulated_c1']['job_id'], 42)

    def test_T31_other_job(self):
        self.event = replace(self.event, event=replace(self.event.event, job_id=43))
        self.no_done(self.finish())

    def test_T32_other_unit(self):
        self.event = replace(self.event, event=replace(self.event.event, unit_name='other.scope'))
        self.no_done(self.finish())

    def test_T33_event_before_reply(self):
        report = self.finish(early=True)
        self.assertTrue(report['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(report['simulated_c1']['signal_monotonic_ns'], 25)

    def test_T34_duplicate_event(self):
        self.adapter.record_snapshot('before', self.before)
        self.adapter.declare_emission(self.emission)
        self.adapter.receive_reply(self.reply)
        self.adapter.receive_job_removed(self.event)
        self.adapter.receive_job_removed(replace(self.event, event=replace(self.event.event, signal_monotonic_ns=41)))
        self.adapter.record_snapshot('after', self.after)
        report = self.adapter.result(80)
        self.assertEqual(report['simulated_c1']['duplicate_events'], 1)
        self.assertTrue(report['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T35_conflicting_event(self):
        self.adapter.record_snapshot('before', self.before)
        self.adapter.declare_emission(self.emission)
        self.adapter.receive_reply(self.reply)
        self.adapter.receive_job_removed(self.event)
        self.adapter.receive_job_removed(replace(self.event, event=replace(self.event.event, signal_monotonic_ns=41, job_result='failed')))
        self.adapter.record_snapshot('after', self.after)
        report = self.adapter.result(80)
        self.assertTrue(report['simulated_c1']['conflicting_events'])
        self.no_done(report)

    def test_T36_overflow(self):
        self.adapter.record_snapshot('before', self.before)
        self.adapter.declare_emission(self.emission)
        first = self.adapter.history[0]
        for n in range(200):
            self.adapter.receive_job_removed(replace(self.event, event=replace(self.event.event, signal_monotonic_ns=40+n)))
        self.assertEqual(len(self.adapter.history), t.MAX_EVENTS)
        self.assertEqual(self.adapter.history[0], first)
        self.assertLessEqual(len(self.adapter.errors), t.MAX_ERRORS)
        self.assertLessEqual(self.adapter.buffer_bytes, t.MAX_BUFFER_BYTES)
        self.no_done(self.adapter.result(300))

    def test_T37_continuity_loss(self):
        self.adapter.record_snapshot('before', self.before)
        self.adapter.fail('EVENTO_PERDIDO', 15)
        self.adapter.declare_emission(self.emission)
        self.adapter.receive_reply(self.reply)
        self.adapter.receive_job_removed(self.event)
        self.adapter.record_snapshot('after', self.after)
        self.no_done()
        self.assertEqual(self.adapter.history[0], self.before)
        for code in ('GLIB_INDISPONIVEL', 'TIMEOUT_NAO_COMPROVADO', 'CONEXAO_ENCERRADA'):
            self.adapter.fail(code, None)
        self.assertTrue(self.adapter.broken)
        self.adapter = b.ObservationalContracts(self.manager)
        self.before = replace(self.before, continuous=False)
        self.no_done(self.finish())

    def test_T38_close_failure(self):
        self.finish()
        history = list(self.adapter.history)
        self.adapter.close(failure='fixture', timestamp=60)
        self.adapter.close()
        self.assertTrue(self.adapter.closed)
        self.assertEqual(self.adapter.history, history)
        self.no_done()

    def test_T39_no_operational_authentication(self):
        for provenance in (b.Provenance.AUTENTICADA, b.Provenance.OBSERVADA_NATIVAMENTE, 'AUTENTICADA'):
            with self.assertRaises(ValueError):
                b.Origin(provenance)
        with self.assertRaises(TypeError):
            b.ManagerContract(self.target, authentication=b.Verification.AUTENTICADO_OPERACIONALMENTE)
        with self.assertRaises((FrozenInstanceError, AttributeError)):
            self.manager.authentication = b.Verification.AUTENTICADO_OPERACIONALMENTE
        disguised = replace(self.manager, origin=b.Origin(b.Provenance.DECLARADA, 'AUTENTICADA'))
        self.assertEqual(disguised.authentication, b.Verification.NAO_VERIFICADO)
        self.assertFalse(self.finish()['operational']['AUTENTICADO_OPERACIONALMENTE'])

    def test_T40_no_native_emission(self):
        with self.assertRaises(TypeError):
            b.EmissionContract(self.manager, self.unit, self.path, 7, 20, 100,
                               state=b.EmissionState.EMISSAO_NATIVA_COMPROVADA)
        self.assertFalse(self.finish()['operational']['EMISSAO_NATIVA_COMPROVADA'])

    def test_T41_no_stop_promotion(self):
        self.assertFalse(self.finish()['operational']['PARADA_CONFIRMADA'])

    def test_T42_no_population_promotion(self):
        self.assertFalse(self.finish()['operational']['POPULACAO_VAZIA_OBSERVADA'])

    def test_T43_no_cleanup_promotion(self):
        report = self.finish()
        self.assertFalse(report['operational']['LIMPEZA_INTEGRAL_COMPROVADA'])
        self.assertTrue(report['operational']['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_T44_no_real_connection_or_runtime_stat(self):
        with patch.object(t.NativeTransport, 'open', side_effect=AssertionError('real bus')), \
                patch('dbus.bus.BusConnection', side_effect=AssertionError('real bus')), \
                patch.object(Path, 'stat', side_effect=AssertionError('filesystem')):
            self.runtime.evaluate(self.directory, self.socket)
            self.finish()
        self.assertFalse(hasattr(b, 'dbus'))

    def test_T45_no_stop(self):
        self.finish()
        self.assertFalse(hasattr(self.adapter, 'stop'))
        self.assertFalse(hasattr(self.adapter, 'StopUnit'))
        self.assertEqual(self.emission.method, 'StopUnit')

    def test_T46_no_scope(self):
        self.assertFalse(hasattr(b, 'subprocess'))
        self.assertFalse(hasattr(self.adapter, 'create_scope'))
        self.assertFalse(hasattr(self.adapter, 'open'))

    def test_T47_executor_preserved(self):
        root = Path(__file__).parent
        for name in ('dv_cgroup_executor.py', 'synthetic_scope_cleanup.py', 'scope_cleanup_evidence.py'):
            self.assertNotIn('systemd_observation_contracts', (root / name).read_text())

    def test_T48_c1_preserved(self):
        report = self.finish()
        self.assertEqual(report['simulated_c1']['model'], 'C1_SIMULADO_SEM_TRANSPORTE')
        self.assertFalse(hasattr(c1, 'Origin'))

    def test_T49_b2_preserved(self):
        self.finish()
        self.assertFalse(population.classifications({})['POPULACAO_VAZIA_OBSERVADA'])
        self.assertNotIn('systemd_observation_contracts', Path(population.__file__).read_text())

    def test_T50_r2_compatible(self):
        report = self.finish()
        backend = t.SimulatedTransport(target=self.target)
        backend.connection_id = self.manager.connection_id  # Epoch de fixture explícita.
        now = [5]
        observation = t.Observation(backend, clock=lambda: now[0])
        self.addCleanup(observation.close)
        observation.start()
        now[0] = 20
        observation.observe_request(c1.StopRequest('compat-fixture', self.unit, self.path, 'a'*32,
                                                  99, 20, 100, 7, self.manager.connection_id, self.manager.owner))
        now[0] = 30
        observation.receive(t.Envelope('reply', self.manager.owner, 'o', (self.reply.reply.job_object_path,),
                                       reply_serial=7, target=self.target, connection_id=self.manager.connection_id))
        now[0] = 40
        observation.receive(t.Envelope('signal', self.manager.owner, 'uoss',
                                       (42, self.event.event.job_object_path, self.unit, 'done'),
                                       target=self.target, connection_id=self.manager.connection_id))
        now[0] = 80
        before = c1.UnitIdentity(self.unit, self.path, 'a'*32, 99, 10, self.manager.connection_id, self.manager.owner)
        after = replace(before, observed_monotonic_ns=50, active_state='inactive')
        original = observation.result(before=t.ScopedEvidence(self.target, self.manager.connection_id, before),
                                      after=t.ScopedEvidence(self.target, self.manager.connection_id, after))
        self.assertEqual(report['simulated_c1']['classifications'], original['classifications'])
        self.assertEqual(self.runtime.expected_socket, self.target.address.removeprefix('unix:path='))


if __name__ == '__main__':
    unittest.main()
