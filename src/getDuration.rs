use crate::allstruct::Window;
use inquire::{Select};

pub fn getduration()->Window{
	return Select::new("Choose Duration of proposal:", vec![
            Window::short,
            Window::medium,
            Window::long
        ])
        .prompt()
        .expect("Prompt failed");
}
