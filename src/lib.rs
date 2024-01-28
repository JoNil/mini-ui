mod arena;
mod commands;
mod render;
mod ui;

use commands::Command;
use ui::Ui;

pub use render::render;

pub struct Context {}

pub fn new() -> Context {
    Context {}
}

impl Context {
    pub fn run(&mut self, ui_fn: impl Fn(&mut Ui)) -> Vec<Command> {
        let mut commands = Vec::new();

        let mut ui = Ui::new(self, &mut commands);

        ui_fn(&mut ui);

        commands
    }
}
