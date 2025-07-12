use std::thread;
use std::time::Duration;
use uuid::Uuid;
use crate::allstruct::{Proposal, vote};
use crate::votestruct::Vote;
use crate::votestruct::vote_decay_type;
use crate::allstruct::proposal_escalation;
use crate::validator::Validator;
use ed25519_dalek::SigningKey;
pub fn create_vote(proposal:&mut Proposal) {

    for _ in 0..10 {
        thread::sleep(Duration::from_secs(10));
        let voter_id = Uuid::new_v4();
        let mut votedecay = vote_decay_type::random();
        if proposal.threshold_config.escalation_type==proposal_escalation::linear{
            votedecay=vote_decay_type::linear;
        } else{
            votedecay=vote_decay_type::exponential;
        }
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
            weight: 10.0,
            signature,
            voteescalation: votedecay,
        };


        myvote.calculate_weight(proposal.s_time);
		Vote::submit_vote(proposal, myvote.clone());
        println!("submitted vote with vote id {:?} and weight {}",myvote.id,myvote.weight);
    }
}

