#[path = "local_plasticity.rs"]
#[allow(dead_code)]
pub mod neural;
use neural::{Local, Observation};
use neurons::learning::{Feedback, PlasticityRule};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    pub action: usize,
    pub eligibility: [bool; 4],
    pub reward: i8,
    pub before: [i32; 4],
    pub after: [i32; 4],
    pub observation: Observation,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Learner {
    pub net: Local,
    pub tick: usize,
}
impl Learner {
    pub fn new() -> Self {
        Self {
            net: Local::new(),
            tick: 0,
        }
    }
    // Exploração fixa 0,1,0,1, independente da atividade e do alvo.
    pub fn act(&mut self, input: [bool; 2]) -> (usize, [bool; 4], Observation) {
        let observation = self.net.query(input, false);
        let action = self.tick % 2;
        self.tick += 1;
        let mut eligible = [false; 4];
        for i in 0..2 {
            for j in 0..2 {
                eligible[2 * i + j] = input[i] && j == action;
            }
        }
        (action, eligible, observation)
    }
    pub fn reinforce(
        &mut self,
        action: usize,
        eligibility: [bool; 4],
        observation: Observation,
        reward: i8,
    ) -> Step {
        let before = self.net.weights;
        assert!((-1..=1).contains(&reward));
        let rule = PlasticityRule::new(0, 2, 1).unwrap();
        for (i, e) in eligibility.into_iter().enumerate() {
            if e {
                let signal = match reward {
                    1 => Some(Feedback::Positive),
                    -1 => Some(Feedback::Negative),
                    _ => None,
                };
                self.net.weights[i] = rule.adjust(self.net.weights[i], signal).unwrap().after;
            }
        }
        Step {
            action,
            eligibility,
            reward,
            before,
            after: self.net.weights,
            observation,
        }
    }
}
// Controle simbólico implementa política e atualizações independentemente.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbolic {
    pub weights: [i32; 4],
    pub tick: usize,
}
impl Symbolic {
    pub fn new() -> Self {
        Self {
            weights: [0; 4],
            tick: 0,
        }
    }
    pub fn act(&mut self, input: [bool; 2]) -> (usize, [bool; 4]) {
        let action = self.tick % 2;
        self.tick += 1;
        let mut marks = [false; 4];
        for i in 0..2 {
            for j in 0..2 {
                marks[2 * i + j] = input[i] && j == action;
            }
        }
        (action, marks)
    }
    pub fn update(&mut self, marks: [bool; 4], reward: i8) -> [i32; 4] {
        for k in 0..4 {
            if marks[k] {
                self.weights[k] = (self.weights[k] + i32::from(reward)).clamp(0, 2);
            }
        }
        self.weights
    }
}
