use crate::{
    commands::{Command, Rectangle, Text},
    Context,
};

pub struct Ui<'ctx, 'cmd> {
    ctx: &'ctx Context,
    commands: &'cmd mut Vec<Command>,
    cursor_x: i32,
    cursor_y: i32,
}

impl<'ctx, 'cmd> Ui<'ctx, 'cmd> {
    pub(crate) fn new(ctx: &'ctx Context, commands: &'cmd mut Vec<Command>) -> Ui<'ctx, 'cmd> {
        Ui {
            ctx,
            commands,
            cursor_x: 0,
            cursor_y: 0,
        }
    }
}

impl<'ctx, 'cmd> Ui<'ctx, 'cmd> {
    pub fn button(&mut self) -> bool {
        self.commands.push(Command::Rectangle(Rectangle {
            x: self.cursor_x as f32,
            y: self.cursor_y as f32,
            width: 40.0,
            height: 18.0,
        }));
        self.cursor_y += 20;

        false
    }

    pub fn label(&mut self, text: &str) {
        self.commands.push(Command::Text(Text {
            x: self.cursor_x as f32,
            y: self.cursor_y as f32,
            text: text.to_owned(),
        }));
        self.cursor_y += 20;
    }
}
