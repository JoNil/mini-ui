use crate::commands::Command;

pub fn render(commands: &[Command]) {
    for command in commands {
        println!("Render! {command:?}");
    }
}
