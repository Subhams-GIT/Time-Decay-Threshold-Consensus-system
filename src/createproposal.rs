
use crate::allstruct::{Proposal, Window, ProgressionProfile, proposal_escalation};
use crate::createvote;
pub fn createproposal(statement: String, duration: Window) -> Proposal {
    let progression = ProgressionProfile::random();
    let escalation = proposal_escalation::random();

    let mut proposal = Proposal::submit_proposal(statement, duration, progression, escalation);
    println!("proposal created");
    createvote::create_vote(&mut proposal);

    while proposal.is_active() {
        Proposal::getTimeBasedThreshold(&mut proposal);
    }
    Proposal::check_status(&mut proposal);
    println!("{:?}",proposal.result);
    proposal
}

