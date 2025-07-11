use std::thread;
use std::time::Duration;

use crate::validator::Validator;
use uuid::Uuid;
use crate::allstruct::{Proposal, vote};
use crate::votestruct::Vote;
use crate::votestruct::vote_decay_type;
use ed25519_dalek::SigningKey;

pub fn create_vote(proposal:&mut Proposal) {

    for _ in 0..10 {
        thread::sleep(Duration::from_secs(10));
        let voter_id = Uuid::new_v4();
        let votedecay = vote_decay_type::random();

        let seed = [1u8; 32];
        let signing_key = SigningKey::from_bytes(&seed);
        let validator = Validator { signing_key };

        let choice = vote::random();
        let (timestamp, signature) = validator.sign_vote_timestamp(voter_id, proposal.id, &choice);

        let mut myvote = Vote {
            vote: choice,
            id: voter_id,
            proposalId: proposal.id,
            timestamp,
            weight: 1.0,
            signature,
            voteescalation: votedecay,
        };


        myvote.calculate_weight(proposal.s_time);
		Vote::submit_vote(proposal, myvote.clone());

    }
}

