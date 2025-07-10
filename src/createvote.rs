use std::time::SystemTime;
use crate::allstruct::{vote, vote_decay_type,Vote};
use ed25519_dalek;
use ed25519_dalek::SigningKey;
use uuid::Uuid;
use crate::validator;

pub fn createVote(pid:Uuid,voterid:Uuid,votedecay:&vote_decay_type,s_time:SystemTime) ->Vote{
	let seed = [1u8; 32];
    let signing_key = SigningKey::from_bytes(&seed);
    let choice = vote::random();
	let validator = validator::Validator { signing_key };
	let (timestamp, sign) = validator.sign_vote_timestamp(voterid, pid, &choice);
	
	let mut vote=Vote{
		id:voterid,
		proposalId:pid,
		vote:choice,
		timestamp,
		voteescalation:votedecay.clone(),
		weight:0.0,
		signature:sign,
	};

	let weight=vote.calculate_weight(s_time);
	vote.weight=weight;
	
	vote.submit_vote(votedecay.clone())
	
}
