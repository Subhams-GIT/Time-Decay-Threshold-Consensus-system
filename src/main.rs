mod allstruct;
mod vote;
mod createproposal;
mod getDuration;
mod createvote;
mod validator;
fn main() {
    let duration=getDuration::getduration();
    createproposal::createproposal(String::from("english or spanish?"), duration);
}
