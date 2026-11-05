"""C2-C T01–T60 e casos adicionais: dados adversariais pequenos, nenhum backend."""
from dataclasses import FrozenInstanceError, replace
import json
from pathlib import Path
import unittest
from unittest.mock import patch

import systemd_integration_boundary as c
import systemd_observation_contracts as b
import systemd_job_transport as t
import scope_population_observer as population
import test_systemd_observation_contracts_safe as fixtures


class SafeBoundary(unittest.TestCase):
    def setUp(self):
        fixtures.SafeContracts.setUp(self)
        self.boundary = c.Boundary(self.manager)

    def ready(self):
        for state in (c.State.CONFIGURED, c.State.PREVALIDATED, c.State.SIMULATING):
            self.assertTrue(self.boundary.transition(state))

    def report(self, timestamp=80):
        return json.loads(self.boundary.report(timestamp))

    def finish(self, *, early=False):
        self.ready()
        self.boundary.ingest('before', self.before)
        self.boundary.ingest('emission', self.emission)
        if early:
            self.boundary.ingest('event', replace(self.event, event=replace(self.event.event, signal_monotonic_ns=25)))
            self.boundary.ingest('reply', self.reply)
        else:
            self.boundary.ingest('reply', self.reply)
            self.boundary.ingest('event', self.event)
        self.boundary.ingest('after', self.after)
        return self.report()

    def no_done(self, report):
        model = report['model']
        if model['simulated_c1']:
            self.assertFalse(model['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertFalse(report['operational_guarantees_proven'])
        self.assertTrue(model['operational']['LIMPEZA_INTEGRAL_INCONCLUSIVA'])

    def test_T01_user_manager(self):
        self.assertEqual(self.boundary.manager.target.manager, t.Manager.USER)
        self.assertTrue(self.finish()['model']['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T02_wrong_system_manager(self):
        self.before = replace(self.before, manager=replace(self.manager, target=t.ManagerTarget(t.Manager.SYSTEM)))
        self.no_done(self.finish())
        self.assertEqual(self.boundary.state, c.State.INVALID)

    def test_T03_wrong_uid(self):
        self.before = replace(self.before, manager=replace(self.manager, target=t.ManagerTarget(t.Manager.USER, 78)))
        self.no_done(self.finish())

    def test_T04_runtime_replaced(self):
        self.ready()
        self.assertFalse(self.boundary.ingest('runtime', dict(runtime=replace(self.directory, inode=9, monotonic_ns=4),
                                socket=self.socket, previous_runtime=self.directory, socket_mode=0o600)))
        self.assertEqual(self.boundary.state, c.State.INVALID)

    def test_T05_socket_replaced(self):
        self.ready()
        self.assertFalse(self.boundary.ingest('runtime', dict(runtime=self.directory,
                          socket=replace(self.socket, inode=8, monotonic_ns=4), previous_socket=self.socket, socket_mode=0o600)))

    def test_T06_symlink(self):
        self.ready()
        self.assertFalse(self.boundary.ingest('runtime', dict(runtime=self.directory,
                          socket=replace(self.socket, symlink=True), socket_mode=0o600)))

    def test_T07_relative_runtime(self):
        with self.assertRaises(ValueError):
            t.ManagerTarget(t.Manager.USER, 77, 'run/user/77')

    def test_T08_wrong_owner(self):
        self.ready()
        self.assertFalse(self.boundary.ingest('runtime', dict(runtime=self.directory,
                          socket=replace(self.socket, uid=78), socket_mode=0o600)))

    def test_T09_wrong_mode(self):
        self.ready()
        self.assertFalse(self.boundary.ingest('runtime', dict(runtime=replace(self.directory, mode=0o777),
                          socket=self.socket, socket_mode=0o600)))

    def test_T10_toctou_not_eliminated(self):
        self.ready()
        self.assertTrue(self.boundary.ingest('runtime', dict(runtime=self.directory, socket=self.socket,
                         previous_socket=replace(self.socket, monotonic_ns=2), socket_mode=0o600)))
        result = self.report(4)['model']['runtime']
        self.assertTrue(result['simulated_socket_verified'])
        self.assertFalse(result['connection_authenticated'])
        self.assertIn('substituição', result['limitation'])

    def test_T11_transport_authentication_claim(self):
        with self.assertRaises(ValueError):
            self.boundary.assessment('G06', c.GuaranteeStatus.OPERATIONALLY_PROVEN)
        self.assertEqual(self.boundary.state, c.State.INVALID)

    def test_T12_path_not_peer(self):
        self.ready()
        result = self.report(5)
        self.assertFalse(result['model']['manager']['target']['socket_verified'])
        self.assertFalse(result['model']['manager']['target']['manager_identity_proven'])

    def test_T13_unique_name_not_authentication(self):
        report = self.finish()
        self.assertEqual(report['model']['manager']['unique_owner'], ':1.10')
        self.assertEqual(report['model']['manager']['authentication'], 'NAO_VERIFICADO')

    def test_T14_owner_change(self):
        self.before = replace(self.before, manager=replace(self.manager, owner=':1.11'))
        self.no_done(self.finish())

    def test_T15_reused_connection(self):
        self.event = replace(self.event, manager=replace(self.manager, connection_id='previous-epoch'))
        self.no_done(self.finish())

    def test_T16_connection_lost(self):
        self.ready()
        self.boundary.invalidate('CONEXAO_PERDIDA')
        self.assertFalse(self.boundary.ingest('before', self.before))
        self.no_done(self.report())

    def test_T17_valid_syntax_is_not_native(self):
        report = self.finish()
        self.assertFalse(report['native_producer_available'])
        self.assertFalse(report['model']['operational']['AUTENTICADO_OPERACIONALMENTE'])

    def test_T18_native_producer_missing(self):
        self.ready()
        self.assertFalse(self.boundary.ingest('before', self.before, category=c.Trust.NATIVE_UNAUTHENTICATED))

    def test_T19_authentication_field_forged(self):
        with self.assertRaises(TypeError):
            b.ManagerContract(self.target, authentication=b.Verification.AUTENTICADO_OPERACIONALMENTE)
        with self.assertRaises((FrozenInstanceError, AttributeError)):
            self.manager.authentication = b.Verification.AUTENTICADO_OPERACIONALMENTE

    def test_T20_service_resolution_pending(self):
        g = next(g for g in c.GUARANTEES if g.identifier == 'G07')
        self.assertEqual(g.current_status, c.GuaranteeStatus.BLOCKED)
        self.assertEqual(g.required_origin, c.Trust.NATIVE_AUTHENTICATED)

    def test_T21_missing_subscription(self):
        self.emission = replace(self.emission, manager=replace(self.manager, subscription_ns=None))
        self.no_done(self.finish())

    def test_T22_late_subscription(self):
        self.emission = replace(self.emission, manager=replace(self.manager, subscription_ns=25))
        self.no_done(self.finish())

    def test_T23_declared_serial(self):
        self.emission = replace(self.emission, origin=b.Origin())
        report = self.finish()
        self.no_done(report)
        self.assertEqual(report['model']['emission_state'], 'EMISSAO_DECLARADA')

    def test_T24_fabricated_native_serial(self):
        with self.assertRaises(TypeError):
            b.EmissionContract(self.manager, self.unit, self.path, 7, 20, 100,
                               state=b.EmissionState.EMISSAO_NATIVA_COMPROVADA)
        self.ready()
        self.assertFalse(self.boundary.ingest('emission', self.emission, category=c.Trust.NATIVE_AUTHENTICATED))

    def test_T25_matching_reply(self):
        report = self.finish()
        self.assertTrue(report['model']['simulated_c1']['classifications']['STOP_ACEITO'])
        self.assertFalse(report['model']['operational']['EMISSAO_NATIVA_COMPROVADA'])

    def test_T26_wrong_reply_serial(self):
        self.reply = replace(self.reply, reply=replace(self.reply.reply, reply_serial=8))
        self.no_done(self.finish())

    def test_T27_wrong_response_owner(self):
        self.reply = replace(self.reply, reply=replace(self.reply.reply, sender=':1.99'))
        self.no_done(self.finish())

    def test_T28_response_outside_window(self):
        self.reply = replace(self.reply, reply=replace(self.reply.reply, response_monotonic_ns=101))
        self.no_done(self.finish())

    def test_T29_missing_reply(self):
        self.ready()
        for kind, data in (('before', self.before), ('emission', self.emission), ('event', self.event), ('after', self.after)):
            self.boundary.ingest(kind, data)
        self.no_done(self.report())

    def test_T30_early_event(self):
        report = self.finish(early=True)
        self.assertTrue(report['model']['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])

    def test_T31_matching_job(self):
        report = self.finish()
        self.assertEqual(report['model']['simulated_c1']['job_id'], 42)
        self.assertFalse(report['model']['operational']['PARADA_CONFIRMADA'])

    def test_T32_other_unit_job(self):
        self.event = replace(self.event, event=replace(self.event.event, unit_name='other.scope'))
        self.no_done(self.finish())

    def test_T33_other_connection_job(self):
        self.event = replace(self.event, event=replace(self.event.event, connection_id='other-connection'))
        self.no_done(self.finish())

    def test_T34_reused_name(self):
        self.after = replace(self.after, invocation_id='b'*32)
        self.no_done(self.finish())

    def test_T35_missing_invocation(self):
        self.after = replace(self.after, invocation_id=None)
        self.no_done(self.finish())

    def test_T36_cgroup_changed(self):
        self.after = replace(self.after, cgroup_inode=100)
        self.no_done(self.finish())

    def test_T37_snapshot_order(self):
        self.after = replace(self.after, monotonic_ns=15)
        self.no_done(self.finish())

    def test_T38_snapshot_absent(self):
        self.ready()
        self.boundary.ingest('emission', self.emission)
        result = self.report()
        self.no_done(result)
        self.assertEqual(result['disposition'], 'INCONCLUSIVO')

    def test_T39_unit_removed_without_zero(self):
        self.after = replace(self.after, load_state='not-found', invocation_id=None, cgroup_inode=None)
        result = self.finish()
        self.assertFalse(result['model']['operational']['POPULACAO_VAZIA_OBSERVADA'])
        self.no_done(result)

    def test_T40_duplicate_and_conflict(self):
        for outcome in ('done', 'failed'):
            with self.subTest(outcome=outcome):
                self.boundary = c.Boundary(self.manager)
                self.ready()
                for kind, data in (('before', self.before), ('emission', self.emission), ('reply', self.reply), ('event', self.event)):
                    self.boundary.ingest(kind, data)
                self.boundary.ingest('event', replace(self.event, event=replace(self.event.event, signal_monotonic_ns=41, job_result=outcome)))
                self.boundary.ingest('after', self.after)
                result = self.report()
                if outcome == 'done':
                    self.assertEqual(result['model']['simulated_c1']['duplicate_events'], 1)
                else:
                    self.no_done(result)
                    self.assertEqual(result['disposition'], 'INVALIDADO')

    def test_T41_reserved_trust_categories(self):
        for category in (c.Trust.NATIVE_AUTHENTICATED, c.Trust.VERIFIED_CORRELATION, c.Trust.EXPERIMENTAL, 'DADOS_SIMULADOS'):
            self.boundary = c.Boundary(self.manager)
            self.ready()
            self.assertFalse(self.boundary.ingest('before', self.before, category=category))

    def test_T42_forged_provenance(self):
        # Simula dado que contornou construtor; gateway revalida antes de asdict.
        forged = object.__new__(b.Origin)
        object.__setattr__(forged, 'provenance', b.Provenance.AUTENTICADA)
        object.__setattr__(forged, 'source', 'fixture-forjada-sem-autenticacao-real')
        self.before = replace(self.before, origin=forged)
        self.no_done(self.finish())

    def test_T43_descriptive_fsm(self):
        self.ready()
        self.assertEqual(self.boundary.state, c.State.SIMULATING)
        self.assertNotIn(self.boundary.state, c.RESERVED_STATES)

    def test_T44_operational_states_unreachable(self):
        for state in c.RESERVED_STATES:
            self.boundary = c.Boundary(self.manager)
            self.ready()
            self.assertFalse(self.boundary.transition(state))
            self.assertEqual(self.boundary.state, c.State.INVALID)
        with self.assertRaises(TypeError):
            c.Boundary(self.manager, initial_state=c.State.OBSERVING)

    def test_T45_forged_internal_state(self):
        with self.assertRaises(AttributeError):
            self.boundary.state = c.State.OBSERVING
        self.boundary._state = c.State.OBSERVING  # Corrupção sintética detectada ao relatar.
        self.assertEqual(self.report()['state'], 'INVALIDADA')

    def test_T46_circular_graph(self):
        with self.assertRaises(ValueError):
            c.validate_graph({'G01': ('G02',), 'G02': ('G01',)})

    def test_T47_missing_dependency(self):
        with self.assertRaises(ValueError):
            c.validate_graph({'G01': ('G02',)})
        order = c.validate_graph(dict(c.GRAPH))
        self.assertEqual(len(order), 21)
        for node, dependencies in c.GRAPH.items():
            self.assertTrue(all(order.index(dep) < order.index(node) for dep in dependencies))

    def test_T48_loss_cannot_restore_trust(self):
        self.ready()
        self.boundary.invalidate('EVENTO_PERDIDO')
        self.assertFalse(self.boundary.transition(c.State.CONFIGURED))
        self.assertEqual(self.boundary.state, c.State.INVALID)
        self.boundary.close()
        self.assertFalse(self.boundary.transition(c.State.CONFIGURED))
        self.assertEqual(self.boundary.state, c.State.CLOSED)

    def test_T49_declared_category_cannot_upgrade(self):
        self.ready()
        self.boundary.ingest('before', self.before, category=c.Trust.DECLARED)
        for kind, data in (('emission', self.emission), ('reply', self.reply), ('event', self.event), ('after', self.after)):
            self.boundary.ingest(kind, data)
        self.no_done(self.report())

    def test_T50_no_experimental_promotion(self):
        result = self.finish()
        self.assertTrue(result['model']['operational']['LIMPEZA_INTEGRAL_INCONCLUSIVA'])
        for g in c.GUARANTEES:
            self.assertNotEqual(g.current_status, c.GuaranteeStatus.OPERATIONALLY_PROVEN)
            self.assertFalse(self.boundary.assessment(g.identifier).operationally_proven)
        self.assertFalse(population.classifications({})['POPULACAO_VAZIA_OBSERVADA'])

    def test_T51_excessive_input(self):
        self.ready()
        self.assertFalse(self.boundary.ingest('runtime', {'environment': {'label': 'x'*65537}}))
        self.assertEqual(self.boundary.state, c.State.INVALID)

    def test_T52_nested_numeric_field_before_asdict(self):
        # Defeito de tipagem permissiva C2-B demonstrado com objeto pequeno.
        nested = replace(self.before, cgroup_inode=[1]*16)
        self.ready()
        with patch.object(b, 'asdict', side_effect=AssertionError('cópia antes de validar')):
            self.assertFalse(self.boundary.ingest('before', nested))
        self.assertEqual(len(self.boundary._contracts.history), 0)

    def test_T53_accumulated_budget_preserves_prefix(self):
        self.boundary = c.Boundary(self.manager, c.Budget(max_history_bytes=4096))
        self.ready()
        self.boundary.ingest('before', self.before)
        first = self.boundary._contracts.history[0]
        for n in range(20):
            if not self.boundary.ingest('event', replace(self.event, event=replace(self.event.event, signal_monotonic_ns=40+n))):
                break
        self.assertEqual(self.boundary._contracts.history[0], first)
        self.assertEqual(self.boundary.state, c.State.INVALID)
        self.assertLessEqual(self.boundary._history_bytes, 4096)

    def test_T54_recursive_budgets(self):
        for item, budget in (([0]*17, c.Budget(max_nodes=16)), ([[[0]]], c.Budget(max_depth=2))):
            with self.assertRaises(c.BudgetExceeded):
                c.bounded_tree(item, budget)
        cyclic = []
        cyclic.append(cyclic)
        with self.assertRaises(c.BudgetExceeded):
            c.bounded_tree(cyclic)

    def test_T55_repeated_reports_limited(self):
        self.finish()
        for _ in range(3):
            self.boundary.report(80)
        with patch.object(b, 'asdict', side_effect=AssertionError('cópia após quota')):
            with self.assertRaises(c.BudgetExceeded):
                self.boundary.report(80)
        self.assertLessEqual(self.boundary._serialized_bytes, 4 * self.boundary.budget.max_report_bytes)

    def test_T56_glib_isolation_not_equivalent(self):
        self.assertFalse(c.GLIB_POLICY['dedicated_connection_is_dedicated_context'])
        self.assertFalse(c.GLIB_POLICY['dedicated_context_is_dedicated_thread'])
        self.assertFalse(c.GLIB_POLICY['dedicated_thread_is_process_isolation'])
        self.assertFalse(c.GLIB_POLICY['dbus_python_1_3_2_nondefault_context_supported'])

    def test_T57_no_absolute_native_deadline(self):
        self.assertFalse(c.GLIB_POLICY['absolute_native_deadline_proven'])
        self.ready()
        self.boundary.invalidate('OPERACAO_NATIVA_BLOQUEANTE_SIMULADA')
        self.no_done(self.report())

    def test_T58_no_total_native_memory_claim(self):
        self.assertFalse(c.GLIB_POLICY['total_native_memory_bound_proven'])
        with self.assertRaises(ValueError):
            c.Budget(max_history_bytes=262145)
        with self.assertRaises(c.BudgetExceeded):
            c.bounded_tree(2**65)

    def test_T59_simultaneous_operation_close_failure(self):
        self.finish()
        history = list(self.boundary._contracts.history)
        self.boundary.invalidate('FALHA_OPERACAO')
        self.boundary.close(failure='fixture')
        result = self.report()
        self.assertEqual(result['errors'][:2], ['FALHA_OPERACAO', 'FALHA_FECHAMENTO'])
        self.assertEqual(self.boundary._contracts.history, history)
        self.no_done(result)

    def test_T60_no_backend_or_experiment(self):
        with patch.object(t.NativeTransport, 'open', side_effect=AssertionError('real bus')), \
                patch('dbus.bus.BusConnection', side_effect=AssertionError('real bus')), \
                patch.object(Path, 'stat', side_effect=AssertionError('runtime real')):
            result = self.finish()
        self.assertFalse(result['native_producer_available'])
        self.assertFalse(hasattr(c, 'subprocess'))
        self.assertFalse(hasattr(self.boundary, 'StopUnit'))
        root = Path(__file__).parent
        for name in ('dv_cgroup_executor.py', 'synthetic_scope_cleanup.py', 'scope_cleanup_evidence.py'):
            self.assertNotIn('systemd_integration_boundary', (root / name).read_text())

    def test_T61_history_overflow(self):
        self.ready()
        for n in range(140):
            if not self.boundary.ingest('event', replace(self.event, event=replace(self.event.event, signal_monotonic_ns=40+n))):
                break
        self.assertLessEqual(len(self.boundary._contracts.history), 128)
        self.assertEqual(self.boundary.state, c.State.INVALID)

    def test_T62_report_serialization_budget(self):
        self.boundary = c.Boundary(self.manager, c.Budget(max_report_bytes=1024))
        self.ready()
        with self.assertRaises(c.BudgetExceeded):
            self.boundary.report(80)
        self.assertEqual(self.boundary.state, c.State.INVALID)
        self.assertEqual(self.boundary._serialized_bytes, 0)

    def test_T63_unknown_objects_no_hooks(self):
        class Hostile:
            def __deepcopy__(self, memo):
                raise AssertionError('hook')
        with self.assertRaises(c.BudgetExceeded):
            c.bounded_tree(Hostile())

    def test_T64_catalog_completeness(self):
        self.assertEqual([g.identifier for g in c.GUARANTEES], ['G%02d' % n for n in range(1, 22)])
        self.assertEqual([a.identifier for a in c.THREATS], ['A%02d' % n for n in range(1, 28)])
        for g in c.GUARANTEES:
            self.assertTrue(g.evidence and g.verifier and g.acceptance and g.failures)
        for a in c.THREATS:
            self.assertTrue(a.existing_control and a.missing_control and a.residual_risk and a.operational_block)

    def test_T65_concurrent_jobs(self):
        self.ready()
        for kind, data in (('before', self.before), ('emission', self.emission), ('reply', self.reply)):
            self.boundary.ingest(kind, data)
        other = replace(self.event, event=replace(self.event.event, job_id=43,
                         job_object_path='/org/freedesktop/systemd1/job/43', unit_name='other.scope', signal_monotonic_ns=35))
        self.boundary.ingest('event', other)
        self.boundary.ingest('event', self.event)
        self.boundary.ingest('after', self.after)
        result = self.report()
        self.assertTrue(result['model']['simulated_c1']['classifications']['JOB_CONCLUIDO_OBSERVADO'])
        self.assertEqual(len(result['model']['simulated_c1']['uncorrelated_events']), 1)

    def test_T66_no_untrusted_string_hooks(self):
        calls = []
        class HostileText(str):
            def __len__(self):
                calls.append('len')
                return 5
        forged = object.__new__(b.Origin)
        object.__setattr__(forged, 'provenance', b.Provenance.SIMULADA)
        object.__setattr__(forged, 'source', HostileText('forged'))
        with self.assertRaises(c.BudgetExceeded):
            c.bounded_tree(forged)
        self.assertEqual(calls, [])

    def test_T67_unexpected_exception_fails_closed(self):
        self.ready()
        with patch.object(self.boundary._contracts, 'record_snapshot', side_effect=RuntimeError('fixture')):
            self.assertFalse(self.boundary.ingest('before', self.before))
        self.assertEqual(self.boundary.state, c.State.INVALID)
        self.assertGreater(self.boundary._history_bytes, 0)

    def test_T68_budget_forgery_rejected(self):
        forged = object.__new__(c.Budget)
        for field in c.fields(c.Budget):
            object.__setattr__(forged, field.name, getattr(c.Budget(), field.name))
        object.__setattr__(forged, 'max_nodes', 999999)
        with self.assertRaises(ValueError):
            c.Boundary(self.manager, forged)


if __name__ == '__main__':
    unittest.main()
