mod allstruct;
mod vote;
mod createproposal;
mod getDuration;
fn main() {
    let duration=getDuration::getduration();
    createproposal::createproposal(String::from("english or spanish?"), duration);
}
