mod allstruct;
mod vote;
mod createproposal;
use inquire::{Select};
use allstruct::Window;
fn main() {
    let duration=Select::new("Choose Duration of proposal:", vec![
            Window::short,
            Window::medium,
            Window::long
        ])
        .prompt()
        .expect("Prompt failed");

    createproposal::createproposal(String::from("english or spanish?"), duration);
}
