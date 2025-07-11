use ed25519_dalek::{SigningKey, Signature, Signer};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

use crate::allstruct::vote;

pub struct Validator {
    pub signing_key: SigningKey,
}

impl Validator {
    
    pub fn sign_vote_timestamp(
        &self,
        voter_id: Uuid,
        proposal_id: Uuid,
        vote_choice: &vote,
    ) -> (SystemTime, Signature) {
        let timestamp = SystemTime::now(); 
        let message = hash_vote_for_timestamp_proof(
            voter_id,
            proposal_id,
            vote_choice,
            timestamp,
        );
        let signature = self.signing_key.sign(&message);
        (timestamp, signature)
    }

}

pub fn hash_vote_for_timestamp_proof(
	voter_id: Uuid,
	proposal_id: Uuid,
	vote_choice: &vote,
	timestamp: SystemTime,
) -> Vec<u8> {

	let choice=match vote_choice{
		vote::Yes=>"Yes",
		vote::No=>"No",
		_=>"Yes"
	};
	let timestamp_secs = timestamp
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	let mut hasher = Sha256::new();
	hasher.update(voter_id.as_bytes());
	hasher.update(proposal_id.as_bytes());
	hasher.update(choice.as_bytes());
	hasher.update(timestamp_secs.to_be_bytes());

	hasher.finalize().to_vec()
}