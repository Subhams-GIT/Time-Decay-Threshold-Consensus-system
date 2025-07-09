use crate::allstruct::Proposal;
use crate::allstruct::Window;
use crate::allstruct::{ProgressionProfile, proposal_escalation};
use std::thread;
use std::time::{Duration};

pub fn createproposal(statement: String, duration: Window) {
	let mut count=0;
    let interval = Duration::from_millis(5000);
    let progression = ProgressionProfile::random();
    let esclation = proposal_escalation::random();
    let mut submittedProposal = Proposal::submit_proposal(statement, duration, progression, esclation);
    while Proposal::is_active(&submittedProposal) {
		println!("{}",count);
        let (threshold,new_proposal)=Proposal::getTimeBasedThreshold(&mut submittedProposal);
		println!("new threshold {}",threshold);
		println!("{:?}",submittedProposal);
		count+=1;
		thread::sleep(interval);
    }

	print!("{}",Proposal::check_status(&mut submittedProposal));
}
