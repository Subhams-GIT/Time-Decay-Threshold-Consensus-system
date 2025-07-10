use crate::allstruct::{vote_decay_type, Vote};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH,Duration};

impl Vote {
    pub fn calculate_weight(&mut self,s_time:SystemTime)->f64{
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
        
        return new_weight;
    }

    pub fn submit_vote(
        &self,decay:vote_decay_type
    ) -> Self {
		
        Self {
            id: self.id,
            vote:self.vote.clone(),
            proposalId: self.proposalId,
            timestamp: SystemTime::now(),
            signature:self.signature,
            weight:self.weight,
            voteescalation: decay,
        }
    }


    pub fn hash_data(&mut self) -> Vec<u8> {
        let timestamp = self
            .timestamp
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut hasher = Sha256::new();
        hasher.update(timestamp.to_be_bytes());
        hasher.update(self.vote.as_bytes());
        hasher.update(self.id.as_bytes());

        hasher.finalize().to_vec()
    }
}
