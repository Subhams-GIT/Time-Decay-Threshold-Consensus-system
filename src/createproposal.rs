use crate::allstruct::Proposal;
use crate::allstruct::Window;
use crate::allstruct::{ProgressionProfile,proposal_escalation};

pub fn createproposal(statement:String,duration:Window){
	let progression=ProgressionProfile::random();
	let esclation=proposal_escalation::random();
	let submittedProposal=Proposal::submit_proposal(statement,duration,progression,esclation);
	while  Proposal::is_active(&submittedProposal){
		println!("proposal submitted!{:?}",submittedProposal);
	}
}