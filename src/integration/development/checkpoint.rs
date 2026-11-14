//! Envelopes PIFI v1/v2/v3: F1/MP05 intactos; v3 fixa o vínculo da hipótese.
//! Checksum detecta corrupção acidental; não autentica origem ou autoridade.
use super::*;

const MAGIC: &[u8; 8] = b"NRNPIFI1";
const MAGIC_ACTIONS: &[u8; 8] = b"NRNPIFI2";
const MAX_BYTES: usize = 128 * 1024;
const MAGIC_CONSEQUENCES: &[u8; 8] = b"NRNPIFI3";

// Camada operacional aditiva: significado da hipótese retido junto às evidências.
// Bases v1/v2 não mudam um byte; F1/MP05 não recebem campos novos.
pub(super) fn encode(runtime: &Runtime, knowledge: &Session) -> Result<Vec<u8>, String> {
    use crate::curriculum::PedagogyHost;
    let base = encode_base(runtime, knowledge)?;
    let Some(c) = runtime.consequence_contract else {
        return Ok(base);
    };
    if knowledge.pedagogy_identity()? != (c.concept, c.scope)
        || c.world == 0
        || c.revision == 0
        || c.action == 0
    {
        return Err("significado conceitual incompatível com checkpoint".into());
    }
    let mut out = MAGIC_CONSEQUENCES.to_vec();
    section(&mut out, &base);
    for value in [
        c.concept.get(),
        c.world,
        c.revision,
        c.source.get(),
        c.action,
    ] {
        put(&mut out, u64::from(value));
    }
    signed(&mut out, c.target);
    let sum = checksum(&out);
    put(&mut out, sum);
    if out.len() > MAX_BYTES {
        return Err("envelope de consequências excedido".into());
    }
    Ok(out)
}
pub(super) fn decode(
    bytes: &[u8],
    sensors: &[Sensor],
    access: Access,
) -> Result<(Runtime, Session), String> {
    use crate::curriculum::PedagogyHost;
    if bytes.get(..8) != Some(MAGIC_CONSEQUENCES.as_slice()) {
        return decode_base(bytes, sensors, access);
    }
    if bytes.len() < 72 || bytes.len() > MAX_BYTES {
        return Err("envelope de consequências inválido".into());
    }
    let body = &bytes[..bytes.len() - 8];
    if checksum(body) != u64::from_le_bytes(bytes[bytes.len() - 8..].try_into().unwrap()) {
        return Err("checksum de consequências inválido".into());
    }
    let mut r = Reader { bytes: body, at: 8 };
    let base = r.section(MAX_BYTES)?;
    let (mut runtime, session) = decode_base(base, sensors, access)?;
    let c = ConsequenceContract {
        concept: NodeId::new(r.id()?).map_err(|e| format!("{e:?}"))?,
        scope: session.pedagogy_identity()?.1,
        world: r.id()?,
        revision: r.id()?,
        source: SourceId::new(r.id()?).map_err(|e| format!("{e:?}"))?,
        action: r.id()?,
        target: r.signed()?,
    };
    runtime.consequence_contract = Some(c);
    if r.at != body.len() || encode(&runtime, &session)? != bytes {
        return Err("envelope de consequências não canônico".into());
    }
    Ok((runtime, session))
}
fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    })
}
fn put(out: &mut Vec<u8>, value: u64) {
    out.extend(value.to_le_bytes());
}
fn section(out: &mut Vec<u8>, bytes: &[u8]) {
    put(out, bytes.len() as u64);
    out.extend(bytes);
}
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}
impl<'a> Reader<'a> {
    fn integer(&mut self) -> Result<u64, String> {
        let end = self.at.checked_add(8).ok_or("offset inválido")?;
        let value = self.bytes.get(self.at..end).ok_or("checkpoint truncado")?;
        self.at = end;
        Ok(u64::from_le_bytes(value.try_into().unwrap()))
    }
    fn count(&mut self, max: usize) -> Result<usize, String> {
        let n = self.integer()?;
        if n > max as u64 {
            return Err("quantidade excedida".into());
        }
        Ok(n as usize)
    }
    fn id(&mut self) -> Result<u32, String> {
        let n = self.integer()?;
        u32::try_from(n).map_err(|_| "ID excedido".into())
    }
    fn signed(&mut self) -> Result<i32, String> {
        let bits = self.id()?;
        Ok(bits as i32)
    }
    fn flag(&mut self) -> Result<bool, String> {
        match self.integer()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("flag inválida".into()),
        }
    }
    fn section(&mut self, max: usize) -> Result<&'a [u8], String> {
        let size = self.count(max)?;
        let end = self.at.checked_add(size).ok_or("tamanho inválido")?;
        let bytes = self.bytes.get(self.at..end).ok_or("seção truncada")?;
        self.at = end;
        Ok(bytes)
    }
}
fn signed(out: &mut Vec<u8>, value: i32) {
    put(out, u64::from(value as u32));
}
fn encoding(s: &Sensor) -> u64 {
    match s.encoding {
        Encoding::Scalar => 0,
        Encoding::BooleanAttributes => 1,
        Encoding::Inclusion => 2,
    }
}

fn encode_base(runtime: &Runtime, knowledge: &Session) -> Result<Vec<u8>, String> {
    let neural = runtime.neural.encode()?;
    let logical = knowledge.encode();
    if neural.len() > super::super::synaptic::MAX_FILE
        || logical.len() > crate::session::f1::MAX_CHECKPOINT_BYTES
    {
        return Err("estado excede codec existente".into());
    }
    let actions = !runtime.action_permits.is_empty() || runtime.planning_queries != 0;
    let mut out = if actions {
        MAGIC_ACTIONS.to_vec()
    } else {
        MAGIC.to_vec()
    };
    section(&mut out, &neural);
    section(&mut out, &logical);
    for n in [
        runtime.limits.records,
        runtime.limits.observations,
        runtime.limits.discoveries,
        runtime.limits.neural_events,
        runtime.limits.attention_scans,
        runtime.discoveries_used,
        runtime.attention_scans,
    ] {
        put(&mut out, n as u64);
    }
    put(&mut out, runtime.at);
    for flag in [
        runtime.conceptual_attempted,
        runtime.relational_attempted,
        runtime.verification_attempted,
    ] {
        put(&mut out, u64::from(flag));
    }
    let c = runtime.cognitive_resources;
    for n in [
        c.reserved_conceptual_queries,
        c.reserved_relational_queries,
        c.reserved_neural_events,
        c.conceptual_queries,
        c.relational_queries,
        c.neural_events,
        c.conceptual_events,
        c.operations_without_report,
    ] {
        put(&mut out, n as u64);
    }
    put(&mut out, runtime.sensors.len() as u64);
    for s in &runtime.sensors {
        for n in [s.id, s.source.get(), s.context, s.input.get()] {
            put(&mut out, u64::from(n));
        }
        put(&mut out, encoding(s));
    }
    put(&mut out, runtime.memory.len() as u64);
    for r in runtime.memory.iter() {
        for n in [r.id().get(), r.cue().context(), r.cue().source().get()] {
            put(&mut out, u64::from(n));
        }
        signed(&mut out, r.cue().pattern());
        signed(&mut out, r.value());
        put(&mut out, r.observed_at());
        put(&mut out, u64::from(r.evidence().map_or(0, EvidenceId::get)));
        if r.status() != RecordStatus::Observed || r.origin().is_some() {
            return Err("estado epistemológico operacional incompatível".into());
        }
    }
    put(&mut out, runtime.experiences.len() as u64);
    for e in &runtime.experiences {
        let r = e.reading;
        for n in [
            r.id,
            r.item.get(),
            r.sensor,
            r.source.get(),
            r.evidence.get(),
            r.event.destination().get(),
        ] {
            put(&mut out, u64::from(n));
        }
        signed(&mut out, r.event.payload());
        put(&mut out, r.event.logical_time());
        put(&mut out, u64::from(e.record.get()));
        put(&mut out, e.neural_events as u64);
    }
    if actions {
        put(&mut out, runtime.action_permits.len() as u64);
        for id in &runtime.action_permits {
            put(&mut out, u64::from(*id));
        }
        for value in [
            runtime.action_reserved_events,
            runtime.planning_queries,
            runtime.planning_scans,
            runtime.action_trials.len(),
        ] {
            put(&mut out, value as u64);
        }
        for trial in &runtime.action_trials {
            for value in [
                trial.id,
                trial.permit,
                trial.world,
                trial.revision,
                trial.context,
                trial.source.get(),
                trial.action,
            ] {
                put(&mut out, u64::from(value));
            }
            signed(&mut out, trial.from);
            signed(&mut out, trial.to);
            put(&mut out, u64::from(trial.before.get()));
            put(&mut out, u64::from(trial.after.get()));
        }
    }
    let sum = checksum(&out);
    put(&mut out, sum);
    if out.len() > MAX_BYTES {
        return Err("checkpoint operacional excedido".into());
    }
    Ok(out)
}

fn decode_base(
    bytes: &[u8],
    sensors: &[Sensor],
    access: Access,
) -> Result<(Runtime, Session), String> {
    let actions = bytes.get(..8) == Some(MAGIC_ACTIONS.as_slice());
    if bytes.len() < 16
        || bytes.len() > MAX_BYTES
        || (!actions && bytes.get(..8) != Some(MAGIC.as_slice()))
    {
        return Err("formato PIFI inválido".into());
    }
    let body = &bytes[..bytes.len() - 8];
    if checksum(body) != u64::from_le_bytes(bytes[bytes.len() - 8..].try_into().unwrap()) {
        return Err("checksum PIFI inválido".into());
    }
    let mut r = Reader { bytes: body, at: 8 };
    let neural = SynapticEpisode::decode(r.section(super::super::synaptic::MAX_FILE)?)?;
    let knowledge = Session::decode(r.section(crate::session::f1::MAX_CHECKPOINT_BYTES)?)?;
    let limits = Limits {
        records: r.count(32)?,
        observations: r.count(8)?,
        discoveries: r.count(4)?,
        neural_events: r.count(128)?,
        attention_scans: r.count(32)?,
    };
    let discoveries_used = r.count(limits.discoveries)?;
    let scans = r.count(limits.attention_scans)?;
    let at = r.integer()?;
    let conceptual = r.flag()?;
    let relational = r.flag()?;
    let verification = r.flag()?;
    let resources = CognitiveResources {
        reserved_conceptual_queries: r.count(8)?,
        reserved_relational_queries: r.count(9)?,
        reserved_neural_events: r.count(288)?,
        conceptual_queries: r.count(8)?,
        relational_queries: r.count(9)?,
        neural_events: r.count(288)?,
        conceptual_events: r.count(15)?,
        operations_without_report: r.count(3)?,
    };
    // As reservas são determinadas pelos episódios consumidos, nunca reabertas.
    let reserved = if conceptual {
        (
            resources
                .reserved_conceptual_queries
                .saturating_sub(usize::from(verification) * 2),
            resources
                .reserved_relational_queries
                .saturating_sub(usize::from(relational) * 3 + usize::from(verification) * 2),
            resources
                .reserved_neural_events
                .saturating_sub(usize::from(relational) * 96 + usize::from(verification) * 64),
        )
    } else {
        (0, 0, 0)
    };
    let concept_reserve = if conceptual && reserved == (6, 4, 128) {
        (6, 4, 128)
    } else if conceptual && reserved == (5, 0, 0) {
        (5, 0, 0)
    } else if !conceptual {
        (0, 0, 0)
    } else {
        return Err("reserva conceitual incompatível".into());
    };
    if resources.reserved_conceptual_queries != concept_reserve.0 + usize::from(verification) * 2
        || resources.reserved_relational_queries
            != concept_reserve.1 + usize::from(relational) * 3 + usize::from(verification) * 2
        || resources.reserved_neural_events
            != concept_reserve.2 + usize::from(relational) * 96 + usize::from(verification) * 64
        || resources.conceptual_queries > resources.reserved_conceptual_queries
        || resources.relational_queries > resources.reserved_relational_queries
        || resources.neural_events > resources.reserved_neural_events
        || resources.operations_without_report
            > usize::from(conceptual) + usize::from(relational) + usize::from(verification)
    {
        return Err("contadores ou reservas inconsistentes".into());
    }
    let sensor_count = r.count(8)?;
    if sensor_count != sensors.len() {
        return Err("interfaces devem ser reinstaladas explicitamente".into());
    }
    let mut saved_sensors = vec![];
    for _ in 0..sensor_count {
        let id = r.id()?;
        let source = SourceId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let context = r.id()?;
        let input = NodeId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let encoding = match r.integer()? {
            0 => Encoding::Scalar,
            1 => Encoding::BooleanAttributes,
            2 => Encoding::Inclusion,
            _ => return Err("codificação desconhecida".into()),
        };
        let sensor = Sensor {
            id,
            source,
            context,
            input,
            encoding,
        };
        if !sensors.contains(&sensor) {
            return Err("interface não reinstalada ou semântica alterada".into());
        }
        saved_sensors.push(sensor);
    }
    let mut runtime = Runtime::new(neural, &saved_sensors, access, limits)?;
    let count = r.count(limits.records)?;
    for index in 0..count {
        let id = EngramId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let context = r.id()?;
        let source = NodeId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let pattern = r.signed()?;
        let value = r.signed()?;
        let observed_at = r.integer()?;
        let evidence = r.id()?;
        if id.get() != index as u32 + 1 || observed_at > at {
            return Err("memória não canônica ou futura".into());
        }
        let evidence = if evidence == 0 {
            None
        } else {
            Some(EvidenceId::new(evidence).map_err(|e| format!("{e:?}"))?)
        };
        let record = Engram::new(
            id,
            Cue::new(context, source, pattern).map_err(|e| format!("{e:?}"))?,
            value,
            None,
            observed_at,
            evidence,
            RecordStatus::Observed,
        )
        .map_err(|e| format!("{e:?}"))?;
        runtime
            .memory
            .record(record)
            .map_err(|e| format!("{e:?}"))?;
    }
    let experiences = r.count(limits.observations)?;
    for _ in 0..experiences {
        let id = r.id()?;
        let item = NodeId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let sensor_id = r.id()?;
        let source = SourceId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let evidence = EvidenceId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let destination = NodeId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let payload = r.signed()?;
        let time = r.integer()?;
        let record = EngramId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
        let neural_events = r.count(16)?;
        let sensor = runtime
            .sensors
            .iter()
            .find(|s| s.id == sensor_id)
            .ok_or("sensor histórico ausente")?;
        let neural = runtime
            .neural
            .experiences()
            .iter()
            .find(|e| e.id == id)
            .ok_or("experiência MP05 ausente")?;
        if id == 0
            || runtime
                .experiences
                .iter()
                .any(|e| e.reading.id == id || e.record == record)
            || sensor.source != source
            || sensor.input != destination
            || (sensor.encoding == Encoding::BooleanAttributes && !(1..=64).contains(&payload))
            || (sensor.encoding == Encoding::Inclusion && (payload == 0 || payload == i32::MIN))
            || time > at
            || neural.context != sensor.context
            || neural.source != destination
            || neural.at != time
            || neural.payload != payload
            || !neural.complete
            || !neural.valid
            || runtime
                .experiences
                .last()
                .is_some_and(|e| e.reading.event.logical_time() > time)
        {
            return Err("experiência histórica incompatível".into());
        }
        let expected = Engram::new(
            record,
            Cue::new(sensor.context, sensor.input, 1).map_err(|e| format!("{e:?}"))?,
            payload,
            None,
            time,
            Some(evidence),
            RecordStatus::Observed,
        )
        .map_err(|e| format!("{e:?}"))?;
        if runtime.memory.get(record).map_err(|e| format!("{e:?}"))? != expected {
            return Err("proveniência operacional divergente".into());
        }
        let after = neural.after.as_ref().ok_or("experiência incompleta")?;
        let before = crate::persistence::decode(&neural.before.engine)
            .map_err(|e| format!("{e:?}"))?
            .0
            .metrics()
            .dispatched;
        let after = crate::persistence::decode(&after.engine)
            .map_err(|e| format!("{e:?}"))?
            .0
            .metrics()
            .dispatched;
        if after.checked_sub(before) != Some(neural_events as u64) {
            return Err("custo neural divergente".into());
        }
        let previous = runtime
            .experiences
            .iter()
            .rev()
            .find(|e| e.reading.sensor == sensor_id && e.reading.item == item);
        let change = match previous.map(|e| e.reading.event.payload()) {
            None => Change::First,
            Some(p) if p == payload => Change::Stable,
            Some(previous) => Change::Changed { previous },
        };
        runtime.experiences.push(Experience {
            reading: Reading {
                id,
                item,
                sensor: sensor_id,
                source,
                evidence,
                event: Event::new(None, destination, EventKind::Stimulus, payload, time),
            },
            record,
            change,
            neural_events,
            nature: RecordStatus::Observed,
        });
    }
    if actions {
        let permits = r.count(4)?;
        for _ in 0..permits {
            let id = r.id()?;
            if id == 0 || runtime.action_permits.contains(&id) {
                return Err("concessão de ação inválida/repetida".into());
            }
            runtime.action_permits.push(id);
        }
        runtime.action_reserved_events = r.count(128)?;
        runtime.planning_queries = r.count(4)?;
        runtime.planning_scans = r.count(32)?;
        let trials = r.count(permits)?;
        let mut used_records = vec![];
        let mut used_permits = vec![];
        let mut last_after = 0;
        for _ in 0..trials {
            let id = r.id()?;
            let permit = r.id()?;
            let world = r.id()?;
            let revision = r.id()?;
            let context = r.id()?;
            let source = SourceId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
            let action = r.id()?;
            let from = r.signed()?;
            let to = r.signed()?;
            let before = EngramId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
            let after = EngramId::new(r.id()?).map_err(|e| format!("{e:?}"))?;
            let bi = runtime
                .experiences
                .iter()
                .position(|e| e.record == before)
                .ok_or("observação anterior ausente")?;
            let ai = runtime
                .experiences
                .iter()
                .position(|e| e.record == after)
                .ok_or("consequência ausente")?;
            let b = runtime.experiences[bi].reading;
            let a = runtime.experiences[ai].reading;
            let sensor = runtime
                .sensors
                .iter()
                .find(|s| s.id == b.sensor)
                .ok_or("sensor de ação ausente")?;
            let permit_index = runtime
                .action_permits
                .iter()
                .position(|p| *p == permit)
                .ok_or("concessão não consumida")?;
            if id == 0
                || world == 0
                || revision == 0
                || action == 0
                || runtime.action_trials.iter().any(|t| t.id == id)
                || used_permits.last().is_some_and(|p| *p >= permit_index)
                || used_records.contains(&before)
                || used_records.contains(&after)
                || ai != bi + 1
                || (!runtime.action_trials.is_empty() && bi <= last_after)
                || sensor.encoding != Encoding::Scalar
                || context != sensor.context
                || source != sensor.source
                || b.sensor != a.sensor
                || b.item.get() != world
                || a.item.get() != world
                || b.source != source
                || a.source != source
                || b.evidence == a.evidence
                || runtime.experiences.iter().enumerate().any(|(index, e)| {
                    index != bi
                        && index != ai
                        && [b.evidence, a.evidence].contains(&e.reading.evidence)
                })
                || b.event.payload() != from
                || a.event.payload() != to
                || b.event.logical_time() >= a.event.logical_time()
            {
                return Err("transição divergente das observações".into());
            }
            // Concessões e evidências não podem servir a duas intervenções.
            if runtime.action_trials.iter().any(|t| {
                [t.before, t.after].iter().any(|record| {
                    runtime
                        .experiences
                        .iter()
                        .find(|e| e.record == *record)
                        .is_some_and(|e| [b.evidence, a.evidence].contains(&e.reading.evidence))
                })
            }) {
                return Err("evidência compartilhada entre intervenções".into());
            }
            used_permits.push(permit_index);
            used_records.extend([before, after]);
            last_after = ai;
            runtime.action_trials.push(ActionTrial {
                id,
                permit,
                world,
                revision,
                context,
                source,
                action,
                from,
                to,
                before,
                after,
            });
        }
        if runtime.action_reserved_events != (permits - trials) * 32
            || (runtime.planning_queries == 0 && runtime.planning_scans != 0)
        {
            return Err("reservas de ação/planejamento inconsistentes".into());
        }
    }
    let diagnostics = runtime
        .memory
        .iter()
        .filter(|m| m.cue().context() == u32::MAX)
        .collect::<Vec<_>>();
    if diagnostics.len() != 4 * discoveries_used
        || count != diagnostics.len() + experiences
        || diagnostics.chunks(4).any(|chunk| {
            chunk.iter().enumerate().any(|(i, m)| {
                m.cue().source().get() != 1
                    || m.cue().pattern() != i as i32 + 1
                    || m.evidence().is_some()
            })
        })
        || runtime.neural.circuit().is_some_and(|c| {
            c.engine_metrics().dispatched + runtime.action_reserved_events as u64
                > limits.neural_events as u64
                || c.logical_time().is_some_and(|t| t > at)
        })
        || r.at != body.len()
    {
        return Err("estado operacional inconsistente ou conteúdo extra".into());
    }
    runtime.discoveries_used = discoveries_used;
    runtime.attention_scans = scans;
    runtime.at = at;
    runtime.conceptual_attempted = conceptual;
    runtime.relational_attempted = relational;
    runtime.verification_attempted = verification;
    runtime.cognitive_resources = resources;
    if encode_base(&runtime, &knowledge)? != bytes {
        return Err("checkpoint operacional não canônico".into());
    }
    Ok((runtime, knowledge))
}
