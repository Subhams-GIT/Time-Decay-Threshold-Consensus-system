use ed25519_dalek::{ SigningKey};
use tempoVote::allstruct::{Proposal,ProgressionProfile,proposal_escalation,Window,vote};
use std::time::{Duration};
use tempoVote::votestruct::Vote;
use tempoVote::validator::Validator;
use uuid::Uuid;
use std::thread::sleep;
use tempoVote::validator;
    #[test]
    fn test_submit_proposal() {
        let proposal = Proposal::submit_proposal(
            "Test Statement".to_string(),
            Window::short,
            ProgressionProfile::Conservative,
            proposal_escalation::linear,
        );

        assert_eq!(proposal.statement, "Test Statement");
        assert!(proposal.duration <= Duration::from_secs(2 * 60));
        assert_eq!(proposal.threshold_config.base, 0.51);
        assert_eq!(proposal.result.for_votes, 0.0);
        assert_eq!(proposal.result.against, 0.0);
        assert!(proposal.votes.is_empty());
    }

    #[test]
    fn test_is_active_true() {
        let proposal = Proposal::submit_proposal(
            "Still active".to_string(),
            Window::short,
            ProgressionProfile::adaptive,
            proposal_escalation::linear,
        );
        assert!(proposal.is_active());
    }

    #[test]
    fn test_is_active_false() {
        let mut proposal = Proposal::submit_proposal(
            "Not active".to_string(),
            Window::short,
            ProgressionProfile::aggressive,
            proposal_escalation::linear,
        );
        
        proposal.s_time -= Duration::from_secs(3 * 60);
        assert!(!proposal.is_active());
    }

    #[test]
    fn test_get_vote() {
        let mut proposal = Proposal::submit_proposal(
            "Vote".to_string(),
            Window::short,
            ProgressionProfile::adaptive,
            proposal_escalation::linear,
        );
		let id=Uuid::new_v4();
		let seed = [1u8; 32];
        let signing_key = SigningKey::from_bytes(&seed);
        let validator = Validator { signing_key };
		let (timestamp,signature)=validator::Validator::sign_vote_timestamp(&validator,id, proposal.id, &vote::Yes);
        let vote = Vote {
            vote: vote::Yes,
            weight: 2.5,
            id: Uuid::new_v4(),
            timestamp,
            proposalId: proposal.id,
            signature,
			voteescalation:tempoVote::votestruct::vote_decay_type::exponential 
        };

        proposal.getVote(&vote);
        assert_eq!(proposal.votes.get(&vote::Yes).cloned().unwrap_or(0.0), 2.5);
    }

    #[test]
    fn test_check_status_passed() {
        let mut proposal = Proposal::submit_proposal(
            "Status".to_string(),
            Window::short,
            ProgressionProfile::adaptive,
            proposal_escalation::linear,
        );
        proposal.result.for_votes = 5.0;
        proposal.result.against = 3.0;

        let result = Proposal::check_status(&mut proposal);
        assert!(result);
        assert!(proposal.result.passed);
    }

    #[test]
    fn test_check_status_failed() {
        let mut proposal = Proposal::submit_proposal(
            "Status fail".to_string(),
            Window::short,
            ProgressionProfile::adaptive,
            proposal_escalation::linear,
        );
        proposal.result.for_votes = 2.0;
        proposal.result.against = 5.0;

        let result = Proposal::check_status(&mut proposal);
        assert!(!result);
        assert!(!proposal.result.passed);
    }

    #[test]
    fn test_get_time_based_threshold_linear() {
        let mut proposal = Proposal::submit_proposal(
            "Linear".to_string(),
            Window::short,
            ProgressionProfile::adaptive,
            proposal_escalation::linear,
        );

        let base_before = proposal.threshold_config.base;
        sleep(Duration::from_secs(1));
        Proposal::getTimeBasedThreshold(&mut proposal);
        let base_after = proposal.threshold_config.base;

        assert!(base_after >= base_before);
        assert!(base_after <= proposal.threshold_config.max);
    }

    #[test]
    fn test_get_time_based_threshold_exponential() {
        let mut proposal = Proposal::submit_proposal(
            "Exp".to_string(),
            Window::short,
            ProgressionProfile::adaptive,
            proposal_escalation::exponential,
        );

        let base_before = proposal.threshold_config.base;
        sleep(Duration::from_secs(1));
        Proposal::getTimeBasedThreshold(&mut proposal);
        let base_after = proposal.threshold_config.base;

        assert!(base_after >= base_before);
        assert!(base_after <= proposal.threshold_config.max);
    }

    #[test]
    fn test_window_display_trait() {
        assert_eq!(Window::short.to_string(), "short");
        assert_eq!(Window::medium.to_string(), "Medium");
        assert_eq!(Window::long.to_string(), "long");
    }

    #[test]
    fn test_progression_profile_random() {
        let profile = ProgressionProfile::random();
        assert!(matches!(
            profile,
            ProgressionProfile::adaptive
                | ProgressionProfile::aggressive
                | ProgressionProfile::Conservative
        ));
    }

    #[test]
    fn test_escalation_random() {
        let e = proposal_escalation::random();
        assert!(matches!(e, proposal_escalation::linear | proposal_escalation::exponential));
    }

    #[test]
    fn test_vote_random() {
        let v = vote::random();
        assert!(matches!(v, vote::Yes | vote::No));
    }
