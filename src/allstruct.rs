use crate::validator::hash_vote_for_timestamp_proof;
use clap::ValueEnum;
use ed25519_dalek::{Verifier, VerifyingKey};
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use crate::votestruct::Vote;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Proposal {
    pub statement: String,
    pub id: Uuid,
    pub votes: HashMap<vote, f64>,
    pub s_time: SystemTime,
    pub duration: Duration,
    pub threshold_config: ThresholdConfig,
    pub result: ProposalResult,
}

#[derive(Debug, Clone, Copy,Serialize,Deserialize)]
pub struct ProposalResult {
    pub for_votes: f64,
    pub against: f64,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy,Serialize,Deserialize)]
pub enum vote {
    Yes,
    No,
}

impl vote {
    pub fn random() -> vote {
        let mut rng = rand::rng();

        let count = 3;

        let index = rng.random_range(0..count);

        match index {
            0 => vote::No,
            1 => vote::Yes,
            _ => vote::Yes,
        }
    }
}


#[derive(Debug, Clone, Copy,Serialize,Deserialize)]
pub struct ThresholdConfig {
    profile: ProgressionProfile,
    pub base: f64,
    rate: f64,
    pub max: f64,
    pub escalation_type: proposal_escalation,
}



#[derive(Debug, Copy, Clone,Serialize,Deserialize,PartialEq)]
pub enum proposal_escalation {
    linear,
    exponential,
}

impl proposal_escalation {
    pub fn random() -> Self {
        let mut rng = rand::rng();

        let count = 3;

        let index = rng.random_range(0..count);

        match index {
            0 => proposal_escalation::exponential,
            1 => proposal_escalation::linear,
            _ => proposal_escalation::linear,
        }
    }
}

#[derive(Debug, Clone, PartialEq, ValueEnum,Serialize,Deserialize)]
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

#[derive(Debug, Clone, Copy,Serialize,Deserialize)]
pub enum ProgressionProfile {
    Conservative,
    aggressive,
    adaptive,
}

impl ProgressionProfile {
    pub fn random() -> Self {
        let mut rng = rand::rng();

        let count = 3;

        let index = rng.random_range(0..count);

        match index {
            0 => ProgressionProfile::Conservative,
            1 => ProgressionProfile::adaptive,
            2 => ProgressionProfile::aggressive,
            _ => panic!("Invalid variant index"),
        }
    }
}

impl Proposal {
    pub fn getVote(&mut self, choice: &Vote) {
        *self.votes.entry(choice.vote).or_insert(0.0) += choice.weight;
    }

    pub fn getTimeBasedThreshold(proposal: &mut Proposal) ->(){
        let Proposal {
            threshold_config,
            s_time,
            ..
        } = proposal;
        if threshold_config.base == threshold_config.max {
            return ();
        }
        

        let elapsed_secs = match SystemTime::now().duration_since(*s_time) {
            Ok(duration) => duration.as_secs_f64(),
            Err(_) => 0.0, 
        };
    
        match threshold_config.escalation_type {
            proposal_escalation::linear => {
                let increment = threshold_config.rate * elapsed_secs * 0.01;
                threshold_config.base = (threshold_config.base + increment).min(threshold_config.max);
                threshold_config.base
            }
            proposal_escalation::exponential => {
                
                threshold_config.base *= (1.0 + 0.03 * threshold_config.rate).powf(elapsed_secs / 60.0); // per minute
                threshold_config.base = threshold_config.base.min(threshold_config.max);
                threshold_config.base
            }
        };
    }

    pub fn verify_vote_signature(
        choice: &vote,
        vote: &Vote,
        validator_pubkey: &VerifyingKey,
    ) -> bool {
        let message =
            hash_vote_for_timestamp_proof(vote.id, vote.proposalId, choice, vote.timestamp);
        validator_pubkey.verify(&message, &vote.signature).is_ok()
    }

    pub fn submit_proposal(
        statement: String,
        duration: Window,
        progression_profile: ProgressionProfile,
        escalation_type: proposal_escalation,
    ) -> Self {
        let time = match duration {
            Window::short => 2,
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
            votes: HashMap::new(),
            result: ProposalResult {
                for_votes: 0.0f64,
                against: 0.0f64,
                passed: false,
            },
        };
    }

    pub fn is_active(&self) -> bool {
        match SystemTime::now().duration_since(self.s_time) {
            Ok(elapsed) => elapsed < self.duration,
            Err(_) => false,
        }
    }

    pub fn check_status(proposal: &mut Proposal) -> bool {
        if proposal.result.for_votes > proposal.result.against {
            proposal.result.passed = true;
            true
        } else {
            proposal.result.passed = false;
            false
        }
    }

    pub fn extend_window(&mut self) {
        if Self::is_active(&self) {
            self.duration += Duration::from_secs(120);
        }
    }
}
