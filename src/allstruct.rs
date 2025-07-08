use clap::ValueEnum;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct Proposal {
    statement: String,
    id: Uuid,
    votes: Option<HashMap<String, Vote>>,
    s_time: SystemTime,
    duration: Duration,
    threshold_config: ThresholdConfig,
}

pub struct ProposalResult {
    for_votes: usize,
    against: usize,
    passed: bool,
}
#[derive(Debug)]
pub struct Vote {
    id: String,
    timestamp: SystemTime,
    weight: u64,
    signature: String,
}
#[derive(Debug)]
pub struct ThresholdConfig {
    profile: ProgressionProfile,
    base: f64,
    rate: f64,
    max: f64,
    escalation_type: proposal_escalation,
}

pub enum escalation_type {
    linear,
    exponential,
    stepped,
}

#[derive(Debug, Copy, Clone)]
pub enum proposal_escalation {
    linear,
    exponential,
}

impl proposal_escalation {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let count = 3;

        let index = rng.gen_range(0..count);

        match index {
            0 => proposal_escalation::exponential,
            1 => proposal_escalation::linear,
            _ => panic!("Invalid variant index"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Window {
    short,
    medium,
    long,
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Window::short => "short",
            Window::medium => "Medium",
            Window::long => "long",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug)]
pub enum ProgressionProfile {
    Conservative,
    aggressive,
    adaptive,
}

impl ProgressionProfile {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let count = 3;

        let index = rng.gen_range(0..count);

        match index {
            0 => ProgressionProfile::Conservative,
            1 => ProgressionProfile::adaptive,
            2 => ProgressionProfile::aggressive,
            _ => panic!("Invalid variant index"),
        }
    }
}

impl Proposal {
    pub fn getTimeBasedThreshold(proposal: &Proposal)->f64 {
        let Proposal {
            threshold_config,..
        } = proposal;
        let e_type = threshold_config.escalation_type;
        let secs = SystemTime::now()
            .duration_since(proposal.s_time)
            .map(|d| d.as_secs_f64()) // Convert to f64 if successful
            .unwrap_or(0.0); 
		
        let new_threshold: (f64)=match e_type {
            proposal_escalation::linear => {
                return threshold_config.base + (secs * threshold_config.rate);
            },
            proposal_escalation::exponential => {
				return threshold_config.base*(1.0+0.03*secs).powf(2.0);
			}
        };

    }

    pub fn getProposalTypeThreshold() {}

    pub fn calculate_current_threshold() {}

    pub fn submit_proposal(
        statement: String,
        duration: Window,
        progression_profile: ProgressionProfile,
        escalation_type: proposal_escalation,
    ) -> Self {
        let time = match duration {
            Window::short => 5,
            Window::medium => 30,
            Window::long => 60,
        };
        let base = 0.51;
        let profile = progression_profile;
        let etype = escalation_type;

        let total_rate = match escalation_type {
            proposal_escalation::linear => 0.1 * 1.0,
            proposal_escalation::exponential => (1.0f64 + 0.03f64 * 1.0f64).powf(2.0),
            _ => 0.1,
        };

        return Self {
            statement,
            duration: Duration::from_secs(time * 60),
            s_time: SystemTime::now(),
            id: Uuid::new_v4(),
            threshold_config: ThresholdConfig {
                profile,
                base,
                rate: total_rate,
                max: 0.68,
                escalation_type: etype,
            },
            votes: Some(HashMap::new()),
        };
    }

    pub fn is_active(&self) -> bool {
        match SystemTime::now().duration_since(self.s_time) {
            Ok(elapsed) => elapsed < self.duration,
            Err(_) => false,
        }
    }

    pub fn check_status() {}

    pub fn extend_window() {}
}
