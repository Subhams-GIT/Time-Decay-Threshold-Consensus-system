mod allstruct;
mod createproposal;
mod getDuration;
mod validator;
mod votestruct;
mod createvote;
use std::{thread};

fn main() {
    let duration = getDuration::getduration();

    let proposal_thread = thread::spawn(move || {
        createproposal::createproposal(String::from("english or spanish?"), duration);
    });

  
    proposal_thread.join().expect("Proposal thread panicked");
    // vote_thread.join().expect("Vote thread panicked");
}
