use clap::ValueEnum;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct Proposal {
    statement: String,
    id: Uuid,
    votes: Option<HashMap<String, Vote>>,
    s_time: SystemTime,
    duration: Duration,
    threshold_config: ThresholdConfig,
    result:ProposalResult
}

#[derive(Debug,Clone)]
pub struct ProposalResult {
    for_votes: usize,
    against: usize,
    passed: bool,
}

#[derive(Debug,Clone)]
pub struct Vote {
    vote:bool,
    id: String,
    proposalId:Uuid,
    timestamp: SystemTime,
    weight: u64,
    signature: String,
    voteescalation:vote_escalation_type,
}

#[derive(Debug,Clone,Copy)]
pub struct ThresholdConfig {
    profile: ProgressionProfile,
    base: f64,
    rate: f64,
    max: f64,
    escalation_type: proposal_escalation,
}

#[derive(Debug,Clone)]
pub enum vote_escalation_type {
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
            _ => proposal_escalation::linear,
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

#[derive(Debug,Clone,Copy)]
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
    pub fn getTimeBasedThreshold(proposal: &mut Proposal)->(f64,&mut Proposal) {
        let Proposal {
            threshold_config,s_time,..
        } = proposal;
        let e_type = threshold_config.escalation_type;
        if threshold_config.base==threshold_config.max{
            return (threshold_config.base,proposal)
        }
        
        let new_threshold:f64=match e_type {
            proposal_escalation::linear => {
                threshold_config.base+=0.01f64 * threshold_config.rate;
                threshold_config.base
            },
            proposal_escalation::exponential => {
				threshold_config.base*=(1.0+0.03*0.01f64).powf(2.0);
                threshold_config.base
			}
        };
        (new_threshold,proposal)
    }

    pub fn getProposalTypeThreshold() {

    }


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
            result:ProposalResult{
                for_votes:0,
                against:0,
                passed:false
            }
        };
    }

    pub fn is_active(&self) -> bool {
        match SystemTime::now().duration_since(self.s_time) {
            Ok(elapsed) => elapsed < self.duration,
            Err(_) => false,
        }
    }

    pub fn check_status(proposal: &mut Proposal)->bool {
        
            if  proposal.result.for_votes>proposal.result.against{
                proposal.result.passed=true;
                true
            }
            else {
                proposal.result.passed=false;
                false
            }
      
    }

    pub fn extend_window(&mut self) {
        if Self::is_active(&self) {
            self.duration+=Duration::from_secs(120);
        }
    }
}
