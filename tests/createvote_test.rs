#[test]
fn test_create_vote_adds_votes() {
    use tempoVote::allstruct::{Proposal, ProgressionProfile, proposal_escalation, Window};
    use tempoVote::createvote::create_vote; 
	use tempoVote::allstruct::vote;
    let mut proposal = Proposal::submit_proposal(
        "Voting Test".to_string(),
        Window::short,
        ProgressionProfile::adaptive,
        proposal_escalation::linear,
    );

    assert!(proposal.votes.is_empty());

    create_vote(&mut proposal); 

    
    let yes_votes = proposal.votes.get(&vote::Yes).cloned().unwrap_or(0.0);
    let no_votes = proposal.votes.get(&vote::No).cloned().unwrap_or(0.0);
    let total_votes = yes_votes + no_votes;

    assert!(total_votes > 0.0, "Votes not recorded");
    assert_eq!(proposal.votes.len(), 1usize.max(proposal.votes.len())); 
    assert!((total_votes >= 50.0) && (total_votes <= 100.0), "Expected vote weights in reasonable range");
}

