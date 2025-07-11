use crate::allstruct::{Proposal,vote};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH,Duration};
use uuid::Uuid;
use ed25519_dalek::{Signature};
use rand::Rng;

#[derive(Debug,Clone)]
pub struct Vote {
    pub vote:vote,
    pub id: Uuid,
    pub proposalId:Uuid,
    pub timestamp: SystemTime,
    pub weight: f64,
    pub signature: Signature,
    pub voteescalation:vote_decay_type,
}

#[derive(Debug,Clone)]
pub enum vote_decay_type {
    linear,
    exponential,
}

impl vote_decay_type {
    pub fn random() -> vote_decay_type {
        let mut rng = rand::rng();

        let count = 3;

        let index = rng.random_range(0..count);

        match index {
            0 => vote_decay_type::linear,
            1 => vote_decay_type::exponential,
            _ => vote_decay_type::linear,
        }
    }
}

impl Vote {
    pub fn calculate_weight(&mut self,s_time:SystemTime)->f64{
        println!("inside weight");
        let now = SystemTime::now();
        let elapsed = now
            .duration_since(s_time)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64();

        if self.weight < 0.1 * self.weight {
			self.weight*=0.1;
        }

        let new_weight=match self.voteescalation {
            vote_decay_type::linear => {
                let rate = 0.01; 
                self.weight -= rate * elapsed;
				self.weight
            }
            vote_decay_type::exponential => {
				let lambda = 0.01;
                self.weight *= (-lambda * elapsed).exp();
				self.weight
            }
        };
        
        return new_weight
    }

    pub fn submit_vote(proposal: &mut Proposal, vote: Vote) {
        println!("created vote {:?}",vote);
        proposal.getVote(vote.vote);
        if vote.vote == vote::Yes {
            proposal.result.for_votes += 1;
        } else {
            proposal.result.against += 1;
        }
        
    }


    pub fn hash_data(&mut self) -> Vec<u8> {
        println!("inside hash data");
        let choice=match self.vote{
            vote::Yes=>"Yes",
            vote::No=>"No",
            _=>"Yes"
        };
        let timestamp = self
            .timestamp
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut hasher = Sha256::new();
        hasher.update(timestamp.to_be_bytes());
        hasher.update(choice.as_bytes());
        hasher.update(self.id.as_bytes());

        hasher.finalize().to_vec()
    }
}
