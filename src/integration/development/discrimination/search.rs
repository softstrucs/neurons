//! Cursor operacional separado de autoridade e dos codecs científicos congelados.
use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeSearch {
    knowledge: Vec<u8>,
    integrity_evaluations: usize,
    contract: Option<ConsequenceContract>,
    offered: Vec<Attributes>,
    retired: Vec<Attributes>,
    selection: ProbeSelection,
}
impl ProbeSearch {
    pub fn new(
        session: &Session,
        contract: ConsequenceContract,
        offered: &[Attributes],
    ) -> Result<Self, String> {
        Self::make(session, Some(contract), offered)
    }
    pub(super) fn make(
        session: &Session,
        contract: Option<ConsequenceContract>,
        offered: &[Attributes],
    ) -> Result<Self, String> {
        if offered.len() > 4 {
            return Err("máximo de quatro ofertas".into());
        }
        let identity = session.pedagogy_identity()?;
        if !identity.1.validity.contains(session.logical_time())
            || contract.is_some_and(|c| {
                (c.concept, c.scope) != identity || c.world == 0 || c.revision == 0 || c.action == 0
            })
        {
            return Err("vínculo de busca inválido".into());
        }
        let mut offered = offered.to_vec();
        offered.sort_by_key(|a| a.encode());
        offered.dedup();
        Ok(Self {
            knowledge: session.encode(),
            integrity_evaluations: 0,
            contract,
            offered,
            retired: vec![],
            selection: ProbeSelection {
                version: session.pedagogy_version()?,
                status: ProbeStatus::ResourceLimited,
                patterns: vec![],
                inspected: 0,
                evaluations: 0,
                excluded: 0,
                space_size: 64,
                exhaustive: false,
                round_inspected: 0,
                round_evaluations: 0,
            },
        })
    }
    /// Verificação do envelope local, separada da busca: até 64×23 avaliações,
    /// sem eventos/ação/autoridade e sem descontar o histórico já consumido.
    pub fn integrity_evaluations(&self) -> usize {
        self.integrity_evaluations
    }
    pub fn selection(&self) -> &ProbeSelection {
        &self.selection
    }
    pub fn retired(&self) -> &[Attributes] {
        &self.retired
    }
    /// Descarta somente propostas, nunca evidências. Não executa/admite nada.
    pub fn dismiss_batch(&mut self) -> Result<(), String> {
        if self.selection.status != ProbeStatus::Ready {
            return Err("lote não pronto".into());
        }
        self.retired.append(&mut self.selection.patterns);
        self.selection.status = if self.selection.exhaustive {
            ProbeStatus::Exhausted
        } else {
            ProbeStatus::ResourceLimited
        };
        Ok(())
    }
    fn validate(
        &self,
        session: &Session,
        contract: ConsequenceContract,
        offered: &[Attributes],
    ) -> Result<(), String> {
        let mut offered = offered.to_vec();
        if offered.len() > 4 {
            return Err("ofertas excedidas".into());
        }
        offered.sort_by_key(|a| a.encode());
        offered.dedup();
        if self.contract != Some(contract)
            || self.knowledge != session.encode()
            || self.offered != offered
        {
            return Err(
                "busca obsoleta: identidade/contexto/versão/evidência/ofertas mudaram".into(),
            );
        }
        Ok(())
    }
    /// Envelope local próprio: não restaura Session, permissões ou orçamento.
    /// Checksum detecta corrupção acidental, não autentica arquivos hostis.
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        let c = self.contract.ok_or("busca sem vínculo persistível")?;
        let mut out = b"NRNPRB01".to_vec();
        out.extend((self.knowledge.len() as u32).to_le_bytes());
        out.extend(&self.knowledge);
        for n in [
            c.world,
            c.revision,
            c.source.get(),
            c.action,
            c.target as u32,
        ] {
            out.extend(n.to_le_bytes());
        }
        out.push(self.selection.inspected as u8);
        out.extend((self.selection.evaluations as u32).to_le_bytes());
        out.push(self.selection.excluded as u8);
        for list in [&self.offered, &self.retired, &self.selection.patterns] {
            out.push(list.len() as u8);
            out.extend(list.iter().map(|a| (a.encode() - 1) as u8));
        }
        let sum = super::super::checkpoint::checksum(&out);
        out.extend(sum.to_le_bytes());
        Ok(out)
    }
    pub fn restore(
        bytes: &[u8],
        session: &Session,
        contract: ConsequenceContract,
    ) -> Result<Self, String> {
        if bytes.len() < 49 || bytes.len() > 41_024 || &bytes[..8] != b"NRNPRB01" {
            return Err("envelope de busca inválido".into());
        }
        let end = bytes.len() - 8;
        if super::super::checkpoint::checksum(&bytes[..end])
            != u64::from_le_bytes(bytes[end..].try_into().unwrap())
        {
            return Err("checksum de busca inválido".into());
        }
        struct Reader<'a> {
            b: &'a [u8],
            at: usize,
        }
        impl Reader<'_> {
            fn take(&mut self, n: usize) -> Result<&[u8], String> {
                let end = self.at.checked_add(n).ok_or("offset inválido")?;
                let v = self.b.get(self.at..end).ok_or("busca truncada")?;
                self.at = end;
                Ok(v)
            }
            fn integer(&mut self) -> Result<usize, String> {
                Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()) as usize)
            }
            fn byte(&mut self) -> Result<usize, String> {
                Ok(self.take(1)?[0] as usize)
            }
            fn list(&mut self, max: usize) -> Result<Vec<Attributes>, String> {
                let n = self.byte()?;
                if n > max {
                    return Err("lista excedida".into());
                }
                self.take(n)?
                    .iter()
                    .map(|m| Attributes::decode(i32::from(*m) + 1).map_err(|e| format!("{e:?}")))
                    .collect()
            }
        }
        let mut r = Reader {
            b: &bytes[..end],
            at: 8,
        };
        let size = r.integer()?;
        if size > crate::session::f1::MAX_CHECKPOINT_BYTES || r.take(size)? != session.encode() {
            return Err("conhecimento da busca obsoleto".into());
        }
        for n in [
            contract.world,
            contract.revision,
            contract.source.get(),
            contract.action,
            contract.target as u32,
        ] {
            if r.integer()? != n as usize {
                return Err("identidade de busca incompatível".into());
            }
        }
        let inspected = r.byte()?;
        let evaluations = r.integer()?;
        let excluded = r.byte()?;
        let offered = r.list(4)?;
        let retired = r.list(64)?;
        let patterns = r.list(2)?;
        if r.at != end || inspected > 64 || excluded > inspected || retired.len() % 2 != 0 {
            return Err("estado de busca inválido".into());
        }
        let mut search = Self::new(session, contract, &offered)?;
        let known = session.pedagogy_existing()?;
        let count_excluded = (64 - inspected..64)
            .filter(|m| {
                let a = Attributes::decode(*m as i32 + 1).unwrap();
                known.contains(&a) || offered.contains(&a)
            })
            .count();
        if excluded != count_excluded
            || evaluations != (inspected - excluded) * search.selection.version.criteria.len()
        {
            return Err("contabilidade da busca inválida".into());
        }
        let mut previous = 65;
        for a in retired.iter().chain(&patterns) {
            let mask = a.encode() as usize - 1;
            let positive = search
                .selection
                .version
                .criteria
                .iter()
                .filter(|h| h.evaluate(*a) == Label::Positive)
                .count();
            if mask >= previous
                || mask < 64 - inspected
                || known.contains(a)
                || offered.contains(a)
                || positive == 0
                || positive == search.selection.version.criteria.len()
            {
                return Err("candidato da busca inválido".into());
            }
            previous = mask;
        }
        search.integrity_evaluations =
            (retired.len() + patterns.len()) * search.selection.version.criteria.len();
        search.retired = retired;
        search.selection.patterns = patterns;
        search.selection.inspected = inspected;
        search.selection.evaluations = evaluations;
        search.selection.excluded = excluded;
        search.selection.exhaustive = inspected == 64;
        search.selection.status = if search.selection.version.criteria.is_empty() {
            ProbeStatus::RepresentationLimited
        } else if search.selection.version.criteria.len() == 1 {
            ProbeStatus::NoDisagreement
        } else if search.selection.patterns.len() == 2 {
            ProbeStatus::Ready
        } else if inspected == 64 {
            ProbeStatus::Exhausted
        } else {
            ProbeStatus::ResourceLimited
        };
        if search.encode()? != bytes {
            return Err("envelope de busca não canônico".into());
        }
        Ok(search)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeCorrespondence {
    pub pattern: Attributes,
    pub exact_offers: Vec<u32>,
    /// Mesma separação prevista, NÃO equivalência física das experiências.
    pub informational_alternatives: Vec<u32>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeFeasibility {
    pub options: Vec<DiscriminatingOption>,
    pub correspondence: Vec<ProbeCorrespondence>,
    pub complete: bool,
    pub evaluations: usize,
}
impl ProbeFeasibility {
    fn link(&mut self, probes: &ProbeSelection) -> Result<(), String> {
        self.correspondence.clear();
        for pattern in &probes.patterns {
            let predicted: Vec<_> = probes
                .version
                .criteria
                .iter()
                .map(|h| h.evaluate(*pattern))
                .collect();
            self.evaluations += predicted.len();
            let mut c = ProbeCorrespondence {
                pattern: *pattern,
                exact_offers: vec![],
                informational_alternatives: vec![],
            };
            for option in &self.options {
                if Attributes::decode(option.reading.event.payload())
                    .map_err(|e| format!("{e:?}"))?
                    == *pattern
                {
                    c.exact_offers.push(option.reading.id);
                }
                if option.separation > 0
                    && option
                        .predictions
                        .iter()
                        .map(|(_, l)| *l)
                        .eq(predicted.iter().copied())
                {
                    c.informational_alternatives.push(option.reading.id);
                }
            }
            self.correspondence.push(c);
        }
        Ok(())
    }
}
pub struct ContinuedInvestigation {
    pub feasibility: Option<ProbeFeasibility>,
    pub outcome: OperationalInvestigation,
}

impl Runtime {
    pub(super) fn advance_probe_search(
        &mut self,
        session: &Session,
        search: &mut ProbeSearch,
        budget: ProbeBudget,
    ) -> Result<ProbeSelection, String> {
        if !self.access.read_knowledge
            || budget.patterns > 32
            || budget.evaluations > 736
            || search.knowledge != session.encode()
        {
            return Err("continuação não autorizada, obsoleta ou orçamento inválido".into());
        }
        let known = session.pedagogy_existing()?;
        let out = &mut search.selection;
        out.round_inspected = 0;
        out.round_evaluations = 0;
        if out.status == ProbeStatus::Ready || out.status == ProbeStatus::Exhausted {
            return Ok(out.clone());
        }
        if out.version.criteria.is_empty() {
            out.status = ProbeStatus::RepresentationLimited;
            return Ok(out.clone());
        }
        if out.version.criteria.len() == 1 {
            out.status = ProbeStatus::NoDisagreement;
            return Ok(out.clone());
        }
        while out.inspected < 64 {
            // Reserve integralmente antes de avançar o cursor: orçamento de
            // avaliações insuficiente não pode pular um padrão não avaliado.
            if out.round_inspected >= budget.patterns
                || self.attention_scans >= self.limits.attention_scans
                || out.round_evaluations + out.version.criteria.len() > budget.evaluations
            {
                return Ok(out.clone());
            }
            let mask = 63 - out.inspected;
            out.inspected += 1;
            out.round_inspected += 1;
            self.attention_scans += 1;
            let a = Attributes::decode(mask as i32 + 1).map_err(|e| format!("{e:?}"))?;
            if known.contains(&a) || search.offered.contains(&a) {
                out.excluded += 1;
                continue;
            }
            let mut positive = 0;
            for h in &out.version.criteria {
                out.evaluations += 1;
                out.round_evaluations += 1;
                positive += usize::from(h.evaluate(a) == Label::Positive);
            }
            if positive > 0 && positive < out.version.criteria.len() {
                out.patterns.push(a);
                if out.patterns.len() == 2 {
                    out.status = ProbeStatus::Ready;
                    out.exhaustive = out.inspected == 64;
                    return Ok(out.clone());
                }
            }
        }
        out.exhaustive = true;
        out.status = ProbeStatus::Exhausted;
        Ok(out.clone())
    }
    pub fn continue_probe_search(
        &mut self,
        session: &Session,
        contract: ConsequenceContract,
        offered: &[Attributes],
        search: &mut ProbeSearch,
        budget: ProbeBudget,
    ) -> Result<ProbeSelection, String> {
        search.validate(session, contract, offered)?;
        self.advance_probe_search(session, search, budget)
    }
    pub fn correlate_probe_offers(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &Session,
        contract: ConsequenceContract,
        probes: &ProbeSelection,
        offers: &[InvestigationOffer<'_>],
    ) -> Result<ProbeFeasibility, String> {
        if !self.access.read_knowledge
            || offers.len() > 4
            || session.pedagogy_version()? != probes.version
            || session.pedagogy_identity()? != (contract.concept, contract.scope)
        {
            return Err("correspondência não autorizada ou obsoleta".into());
        }
        validate_offer_identities(offers)?;
        let mut out = ProbeFeasibility {
            options: vec![],
            correspondence: vec![],
            complete: false,
            evaluations: 0,
        };
        for offer in offers {
            if self.attention_scans >= self.limits.attention_scans {
                return Ok(out);
            }
            self.attention_scans += 1;
            let option =
                self.assess_investigation_offer(memory, session, contract, offer, &probes.version)?;
            out.evaluations += option.predictions.len();
            out.options.push(option);
        }
        out.link(probes)?;
        out.complete = true;
        Ok(out)
    }
    pub fn investigate_continued_probes(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &mut Session,
        contract: ConsequenceContract,
        question: Attributes,
        offers: &mut [InvestigationOffer<'_>],
        teaching: Option<ProbeTeaching>,
        search: &mut ProbeSearch,
        budget: ProbeBudget,
    ) -> Result<ContinuedInvestigation, String> {
        let offered = offers
            .iter()
            .map(|o| Attributes::decode(o.attributes.event.payload()).map_err(|e| format!("{e:?}")))
            .collect::<Result<Vec<_>, _>>()?;
        search.validate(session, contract, &offered)?;
        if budget.patterns > 32 || budget.evaluations > 736 {
            return Err("orçamento de continuação inválido".into());
        }
        // Revalida os observáveis/identidades antes de mover o cursor. As
        // permissões são recalculadas; nunca persistidas no estado da busca.
        let mut preview = search.selection.clone();
        preview.patterns.clear();
        let mut feasibility =
            self.correlate_probe_offers(memory, session, contract, &preview, offers)?;
        let probes = if feasibility.complete {
            let probes = self.continue_probe_search(session, contract, &offered, search, budget)?;
            feasibility.link(&probes)?;
            probes
        } else {
            let mut probes = search.selection.clone();
            probes.status = ProbeStatus::ResourceLimited;
            probes.round_inspected = 0;
            probes.round_evaluations = 0;
            probes
        };
        let feasibility = Some(feasibility);
        let outcome = self.finish_probe_investigation(
            memory, session, contract, question, offers, teaching, probes,
        )?;
        Ok(ContinuedInvestigation {
            feasibility,
            outcome,
        })
    }
}
