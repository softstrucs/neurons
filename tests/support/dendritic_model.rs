#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DendriteError {
    InvalidConfiguration,
    UnknownCompartment,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dendrite {
    potential: i32,
    threshold: i32,
    limit: i32,
}

impl Dendrite {
    pub fn new(threshold: i32, limit: i32) -> Result<Self, DendriteError> {
        if threshold <= 0 || limit < threshold {
            return Err(DendriteError::InvalidConfiguration);
        }
        Ok(Self {
            potential: 0,
            threshold,
            limit,
        })
    }

    pub fn potential(&self) -> i32 {
        self.potential
    }

    pub fn receive(&mut self, signal: i32) -> bool {
        let sum = i64::from(self.potential) + i64::from(signal);
        let limit = i64::from(self.limit);

        self.potential = sum.clamp(-limit, limit) as i32;

        if self.potential >= self.threshold {
            self.potential = 0;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DendriticNeuron {
    dendrites: [Dendrite; 2],
    soma_potential: i32,
    soma_threshold: i32,
    soma_limit: i32,
    activations: u64,
}

impl DendriticNeuron {
    pub fn new(
        dendrite_threshold: i32,
        dendrite_limit: i32,
        soma_threshold: i32,
        soma_limit: i32,
    ) -> Result<Self, DendriteError> {
        if soma_threshold <= 0 || soma_limit < soma_threshold {
            return Err(DendriteError::InvalidConfiguration);
        }

        let dendrite = Dendrite::new(dendrite_threshold, dendrite_limit)?;

        Ok(Self {
            dendrites: [dendrite; 2],
            soma_potential: 0,
            soma_threshold,
            soma_limit,
            activations: 0,
        })
    }

    pub fn receive(&mut self, compartment: usize, signal: i32) -> Result<bool, DendriteError> {
        let dendrite = self
            .dendrites
            .get_mut(compartment)
            .ok_or(DendriteError::UnknownCompartment)?;

        if !dendrite.receive(signal) {
            return Ok(false);
        }

        let sum = i64::from(self.soma_potential) + 1;
        self.soma_potential = sum.min(i64::from(self.soma_limit)) as i32;

        if self.soma_potential >= self.soma_threshold {
            self.soma_potential = 0;
            self.activations = self.activations.saturating_add(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn potentials(&self) -> [i32; 2] {
        [self.dendrites[0].potential(), self.dendrites[1].potential()]
    }

    pub fn soma_potential(&self) -> i32 {
        self.soma_potential
    }

    pub fn activations(&self) -> u64 {
        self.activations
    }
}
