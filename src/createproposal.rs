use uuid::Uuid;
use std::sync::{Arc, Mutex};
use crate::allstruct::{vote_decay_type, Proposal};
use crate::allstruct::Window;
use crate::allstruct::{ProgressionProfile, proposal_escalation};
use std::thread;
use std::time::{Duration};
use crate::createvote::createVote;

// pid:Uuid,voterid:Uuid,votedecay:vote_decay_type,s_time:SystemTime

pub fn createproposal(statement: String, duration: Window) {
    let interval = Duration::from_millis(5000);
    let progression = ProgressionProfile::random();
    let esclation = proposal_escalation::random();
    let vote_decay=vote_decay_type::random();
    let mut submittedProposal = Arc::new(Mutex::new(Proposal::submit_proposal(statement, duration, progression, esclation)));

    let num_voters=5;

    for _ in 0..num_voters{

        let proposal_ref=Arc::clone(&submittedProposal);
        let decay_clone=vote_decay.clone();

        thread::spawn(move || {
            let delay = Duration::from_millis(rand::random::<u64>() % 8000); 
            thread::sleep(delay);

            let proposal = proposal_ref.lock().unwrap();

            if Proposal::is_active(&proposal) {
                let voter_id = Uuid::new_v4();
                
                createVote(proposal.id, voter_id, &decay_clone, proposal.s_time);
                println!("✅ Voter {:?} voted after {:?} ms", voter_id, delay.as_millis());
            }

        });
    }
    while Proposal::is_active(&submittedProposal.lock().unwrap()) {
        thread::sleep(interval);
    }
    

}
