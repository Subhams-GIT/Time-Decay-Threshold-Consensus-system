use clap::ValueEnum;
use ed25519_dalek::{VerifyingKey,Verifier,Signature};
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use crate::validator::hash_vote_for_timestamp_proof;
#[derive(Debug,Clone)]
pub struct Proposal {
    pub statement: String,
    pub id: Uuid,
    votes: Option<HashMap<String, Vote>>,
    pub s_time: SystemTime,
    duration: Duration,
    threshold_config: ThresholdConfig,
    pub result:ProposalResult
}

#[derive(Debug,Clone)]
pub struct ProposalResult {
    for_votes: usize,
    against: usize,
    passed: bool,
}
#[derive(Debug,Clone)]
pub enum vote{
    Yes,
    No
}

impl vote {
    pub fn random() -> String {
        let mut rng = rand::thread_rng();

        let count = 3;

        let index = rng.gen_range(0..count);

        let index=match index {
            0 => vote::No,
            1 => vote::Yes,
            _ => vote::Yes,
        };

       let choice= match index{
            vote::No=>"No",
            vote::Yes=>"Yes",
            _=>"Yes"
        };
        choice.to_string()
    }
}

#[derive(Debug,Clone)]
pub struct Vote {
    pub vote:String,
    pub id: Uuid,
    pub proposalId:Uuid,
    pub timestamp: SystemTime,
    pub weight: f64,
    pub signature: Signature,
    pub voteescalation:vote_decay_type,
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
pub enum vote_decay_type {
    linear,
    exponential,
}

impl vote_decay_type {
    pub fn random() -> vote_decay_type {
        let mut rng = rand::thread_rng();

        let count = 3;

        let index = rng.gen_range(0..count);

        match index {
            0 => vote_decay_type::linear,
            1 => vote_decay_type::exponential,
            _ => vote_decay_type::linear,
        }
    }
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

    pub fn verify_vote_signature(vote: &Vote, validator_pubkey: &VerifyingKey) -> bool {
        let message = hash_vote_for_timestamp_proof(
            vote.id,
            vote.proposalId,
            &vote.vote,
            vote.timestamp,
        );
        validator_pubkey.verify(&message, &vote.signature).is_ok()
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
