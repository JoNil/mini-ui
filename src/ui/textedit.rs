use crate::{
    cairo::Context,
    math::{vec2, vec4, Vec2},
    ui::{
        color::{held_color, hover_color},
        frame, Ui,
    },
    window::{Key, MouseButton, Window},
};
use std::{ffi::CString, mem, time::Instant};

use super::Align;

const TEXTEDIT_K_SHIFT: i32 = 0x4000_0000;
const TEXTEDIT_K_CONTROL: i32 = 0x2000_0000;

const TEXTEDIT_K_LEFT: i32 = Key::Left as i32;
const TEXTEDIT_K_RIGHT: i32 = Key::Right as i32;
const TEXTEDIT_K_UP: i32 = Key::Up as i32;
const TEXTEDIT_K_DOWN: i32 = Key::Down as i32;
const TEXTEDIT_K_LINESTART: i32 = Key::Home as i32;
const TEXTEDIT_K_LINEEND: i32 = Key::End as i32;
const TEXTEDIT_K_TEXTSTART: i32 = Key::Home as i32 | TEXTEDIT_K_CONTROL;
const TEXTEDIT_K_TEXTEND: i32 = Key::End as i32 | TEXTEDIT_K_CONTROL;
const TEXTEDIT_K_DELETE: i32 = Key::Delete as i32;
const TEXTEDIT_K_BACKSPACE: i32 = Key::Backspace as i32;
const TEXTEDIT_K_UNDO: i32 = Key::Z as i32 | TEXTEDIT_K_CONTROL;
const TEXTEDIT_K_REDO: i32 = Key::Y as i32 | TEXTEDIT_K_CONTROL;
const TEXTEDIT_K_INSERT: i32 = Key::Insert as i32;
const TEXTEDIT_K_WORDLEFT: i32 = Key::Left as i32 | TEXTEDIT_K_CONTROL;
const TEXTEDIT_K_WORDRIGHT: i32 = Key::Right as i32 | TEXTEDIT_K_CONTROL;
const TEXTEDIT_K_PGUP: i32 = Key::PageUp as i32;
const TEXTEDIT_K_PGDOWN: i32 = Key::PageDown as i32;

const TEXTEDIT_K_SHIFT_LEFT: i32 = TEXTEDIT_K_LEFT | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_RIGHT: i32 = TEXTEDIT_K_RIGHT | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_UP: i32 = TEXTEDIT_K_UP | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_DOWN: i32 = TEXTEDIT_K_DOWN | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_LINESTART: i32 = TEXTEDIT_K_LINESTART | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_LINEEND: i32 = TEXTEDIT_K_LINEEND | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_TEXTSTART: i32 = TEXTEDIT_K_TEXTSTART | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_TEXTEND: i32 = TEXTEDIT_K_TEXTEND | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_DELETE: i32 = TEXTEDIT_K_DELETE | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_BACKSPACE: i32 = TEXTEDIT_K_BACKSPACE | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_UNDO: i32 = TEXTEDIT_K_UNDO | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_REDO: i32 = TEXTEDIT_K_REDO | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_INSERT: i32 = TEXTEDIT_K_INSERT | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_WORDLEFT: i32 = TEXTEDIT_K_WORDLEFT | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_WORDRIGHT: i32 = TEXTEDIT_K_WORDRIGHT | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_PGUP: i32 = TEXTEDIT_K_PGUP | TEXTEDIT_K_SHIFT;
const TEXTEDIT_K_SHIFT_PGDOWN: i32 = TEXTEDIT_K_PGDOWN | TEXTEDIT_K_SHIFT;

const KEYS_TO_PASS: &[i32] = &[
    TEXTEDIT_K_LEFT,
    TEXTEDIT_K_RIGHT,
    TEXTEDIT_K_UP,
    TEXTEDIT_K_DOWN,
    TEXTEDIT_K_LINESTART,
    TEXTEDIT_K_LINEEND,
    TEXTEDIT_K_TEXTSTART,
    TEXTEDIT_K_TEXTEND,
    TEXTEDIT_K_DELETE,
    TEXTEDIT_K_BACKSPACE,
    TEXTEDIT_K_UNDO,
    TEXTEDIT_K_REDO,
    TEXTEDIT_K_INSERT,
    TEXTEDIT_K_WORDLEFT,
    TEXTEDIT_K_WORDRIGHT,
    TEXTEDIT_K_PGUP,
    TEXTEDIT_K_PGDOWN,
    TEXTEDIT_K_SHIFT_LEFT,
    TEXTEDIT_K_SHIFT_RIGHT,
    TEXTEDIT_K_SHIFT_UP,
    TEXTEDIT_K_SHIFT_DOWN,
    TEXTEDIT_K_SHIFT_LINESTART,
    TEXTEDIT_K_SHIFT_LINEEND,
    TEXTEDIT_K_SHIFT_TEXTSTART,
    TEXTEDIT_K_SHIFT_TEXTEND,
    TEXTEDIT_K_SHIFT_DELETE,
    TEXTEDIT_K_SHIFT_BACKSPACE,
    TEXTEDIT_K_SHIFT_UNDO,
    TEXTEDIT_K_SHIFT_REDO,
    TEXTEDIT_K_SHIFT_INSERT,
    TEXTEDIT_K_SHIFT_WORDLEFT,
    TEXTEDIT_K_SHIFT_WORDRIGHT,
    TEXTEDIT_K_SHIFT_PGUP,
    TEXTEDIT_K_SHIFT_PGDOWN,
];

#[derive(Copy, Clone, Default)]
struct UndoRecord {
    where_0: i32,
    insert_length: i32,
    delete_length: i32,
    char_storage: i32,
}

#[derive(Copy, Clone)]
struct UndoState {
    undo_rec: [UndoRecord; 99],
    undo_char: [char; 999],
    undo_point: i32,
    redo_point: i32,
    undo_char_point: i32,
    redo_char_point: i32,
}

impl UndoState {
    fn flush_redo(&mut self) {
        self.redo_point = 99;
        self.redo_char_point = 999;
    }

    fn discard_undo(&mut self) {
        if self.undo_point > 0 {
            if self.undo_rec[0].char_storage >= 0 {
                let n = self.undo_rec[0].insert_length;

                self.undo_char
                    .copy_within(n as usize..self.undo_char_point as usize, 0);
                self.undo_char_point -= n;

                for i in 0..self.undo_point {
                    if self.undo_rec[i as usize].char_storage >= 0 {
                        self.undo_rec[i as usize].char_storage -= n;
                    }
                }
            }

            self.undo_rec.copy_within(1..self.undo_point as usize, 0);
            self.undo_point -= 1;
        }
    }

    fn discard_redo(&mut self) {
        let k = 99 - 1;
        if self.redo_point <= k {
            if self.undo_rec[k as usize].char_storage >= 0 {
                let n = self.undo_rec[k as usize].insert_length;

                self.redo_char_point += n;
                self.undo_char.copy_within(
                    (self.redo_char_point as usize - n as usize)..(999 - n as usize),
                    self.redo_char_point as usize,
                );
                for i in self.redo_point..k {
                    if self.undo_rec[i as usize].char_storage >= 0 {
                        self.undo_rec[i as usize].char_storage += n;
                    }
                }
            }

            self.redo_point += 1;
            self.undo_rec.copy_within(
                self.redo_char_point as usize..99,
                self.redo_point as usize + 1,
            );
        }
    }

    fn create_undo_record(&mut self, numchars: i32) -> Option<&mut UndoRecord> {
        self.flush_redo();
        if self.undo_point == 99 {
            self.discard_undo();
        }
        if numchars > 999 {
            self.undo_point = 0;
            self.undo_char_point = 0;
            return None;
        }
        while self.undo_char_point + numchars > 999 {
            self.discard_undo();
        }
        let prev_undo_point = self.undo_point;
        self.undo_point += 1;

        Some(&mut self.undo_rec[prev_undo_point as usize])
    }

    fn create_undo(&mut self, pos: i32, insert_len: i32, delete_len: i32) -> Option<&mut [char]> {
        let undo_char_point = self.undo_char_point;

        let r = self.create_undo_record(insert_len)?;
        r.where_0 = pos;
        r.insert_length = insert_len;
        r.delete_length = delete_len;

        if insert_len == 0 {
            r.char_storage = -1;
            None
        } else {
            r.char_storage = undo_char_point;
            self.undo_char_point += insert_len;

            Some(
                &mut self.undo_char
                    [undo_char_point as usize..(undo_char_point + insert_len) as usize],
            )
        }
    }
}

#[derive(Copy, Clone, Default)]
struct TexteditRow {
    x0: f32,
    x1: f32,
    baseline_y_delta: f32,
    ymin: f32,
    ymax: f32,
    num_chars: i32,
}

impl TexteditRow {
    fn layout(&mut self, calc_text_width: &impl Fn(&str) -> f32, str: &mut TextEdit, start_i: i32) {
        let width = calc_text_width(&str.string);
        let remaining_chars = str.string.chars().count() as i32 - start_i;

        self.num_chars = remaining_chars;
        self.x0 = 0.0;
        self.x1 = width;
        self.baseline_y_delta = 1.25;
        self.ymin = -1.0;
        self.ymax = 0.0;
    }
}

#[derive(Copy, Clone, Default)]
struct FindState {
    x: f32,
    y: f32,
    height: f32,
    first_char: i32,
    length: i32,
    prev_first: i32,
}

impl FindState {
    fn find_charpos(
        &mut self,
        calc_text_width: &impl Fn(&str) -> f32,
        str: &mut TextEdit,
        n: i32,
        single_line: bool,
    ) {
        let mut r = TexteditRow::default();
        let mut prev_start = 0;
        let z = string_len(&str.string);
        let mut i = 0;

        if n == z {
            if single_line {
                r.layout(calc_text_width, str, 0);
                self.y = 0.0;
                self.first_char = 0;
                self.length = z;
                self.height = r.ymax - r.ymin;
                self.x = r.x1;
            } else {
                self.y = 0.0;
                self.x = 0.0;
                self.height = 1.0;
                while i < z {
                    r.layout(calc_text_width, str, i);
                    prev_start = i;
                    i += r.num_chars;
                }
                self.first_char = i;
                self.length = 0;
                self.prev_first = prev_start;
            }
            return;
        }

        self.y = 0.0;
        loop {
            r.layout(calc_text_width, str, i);
            if n < i + r.num_chars {
                break;
            }
            prev_start = i;
            i += r.num_chars;
            self.y += r.baseline_y_delta;
        }

        let first = i;
        self.first_char = first;
        self.length = r.num_chars;
        self.height = r.ymax - r.ymin;
        self.prev_first = prev_start;
        self.x = r.x0;
        i = 0;

        while first + i < n {
            self.x += string_width(calc_text_width, str, first, i);
            i += 1;
        }
    }
}

fn delete_chars(string: &mut String, pos: i32, num: i32) {
    let char_pos = pos as usize;
    let num_chars = num as usize;

    let start_byte_pos = string
        .char_indices()
        .nth(char_pos)
        .map_or(string.len(), |(idx, _)| idx);

    let end_byte_pos = string
        .char_indices()
        .nth(char_pos + num_chars)
        .map_or(string.len(), |(idx, _)| idx);

    string.replace_range(start_byte_pos..end_byte_pos, "");
}

fn insert_chars(string: &mut String, pos: i32, newtext: &str) {
    let byte_pos = string
        .char_indices()
        .nth(pos as usize)
        .map(|(idx, _)| idx)
        .unwrap_or(string.len());

    string.insert_str(byte_pos, newtext);
}

fn string_len(string: &str) -> i32 {
    string.chars().count() as _
}

fn string_width(
    calc_text_width: impl Fn(&str) -> f32,
    str: &mut TextEdit,
    _line_start_idx: i32,
    char_idx: i32,
) -> f32 {
    let begin = str
        .string
        .char_indices()
        .nth(char_idx as usize)
        .map(|(i, _)| i)
        .unwrap_or(str.string.len());

    let end = str
        .string
        .char_indices()
        .nth(char_idx as usize + 1)
        .map(|(i, _)| i)
        .unwrap_or(str.string.len());

    let s = &str.string[begin..end];

    calc_text_width(s)
}

fn key_to_text(key: i32) -> Option<char> {
    let key = key as u32 & !(TEXTEDIT_K_SHIFT as u32) & !(TEXTEDIT_K_CONTROL as u32);
    char::from_u32(key)
}

fn get_char(string: &str, pos: i32) -> char {
    string.chars().nth(pos as usize).unwrap_or_default()
}

fn is_space(c: char) -> bool {
    c.is_whitespace()
}

#[derive(Clone)]
pub struct TextEdit {
    pub string: String,
    limit: i32,
    active: bool,
    no_drag: bool,
    blink_timer: Instant,

    cursor: i32,
    select_start: i32,
    select_end: i32,
    insert_mode: bool,
    row_count_per_page: i32,
    has_preferred_x: bool,
    single_line: bool,
    preferred_x: f32,
    undostate: UndoState,
}

impl TextEdit {
    pub fn new(string: impl Into<String>, limit: i32) -> TextEdit {
        let string = string.into();
        let start_width = string_len(&string);

        TextEdit {
            string,
            limit,
            active: false,
            no_drag: false,
            blink_timer: Instant::now(),
            cursor: start_width,
            select_start: 0,
            select_end: start_width,
            has_preferred_x: false,
            preferred_x: 0.0,
            single_line: true,
            insert_mode: false,
            row_count_per_page: 0,
            undostate: UndoState {
                undo_rec: [UndoRecord {
                    where_0: 0,
                    insert_length: 0,
                    delete_length: 0,
                    char_storage: 0,
                }; 99],
                undo_char: ['\0'; 999],
                undo_point: 0,
                undo_char_point: 0,
                redo_point: 99,
                redo_char_point: 999,
            },
        }
    }

    pub fn show(&mut self, size: Vec2, window: &Window, context: &Context, ui: &mut Ui) {
        let response = ui.response();
        let style = ui.style;

        let style = if response.held && !self.active {
            style.frame_color(held_color(style.frame_color))
        } else if response.hovered && !self.active {
            style.frame_color(hover_color(style.frame_color))
        } else {
            style
        };

        let calc_text_width = {
            let text_height = style.text_height;
            let font_id = style.font.unwrap_or_default().id;

            move |s: &str| {
                context.set_font_size(text_height as _);
                let extent = context.text_extents(s).unwrap();
                extent.width() as f32
            }
        };

        let pressed = window.get_mouse_down(MouseButton::Left);

        if response.double_clicked && self.active {
            self.select_start = 0;
            self.select_end = string_len(&self.string);
            self.no_drag = true;
        } else if response.pressed && self.active {
            self.click(
                &calc_text_width,
                response.relative_mouse_pos.x - style.padding.left - style.margin.left,
                response.relative_mouse_pos.y,
            );
        } else if response.held && self.active && !self.no_drag {
            self.drag(
                &calc_text_width,
                response.relative_mouse_pos.x - style.padding.left - style.margin.left,
                response.relative_mouse_pos.y,
            );
        } else if response.released {
            self.active = true;
            self.no_drag = false;
        } else if pressed {
            self.active = false;
        }

        if self.active {
            /* TODO
            for event in api.window_events() {
                match event {
                    WindowEvent::KeyEvent(k) => {
                        if k.actions as u32 == PLUGIN_KEY_ACTION_PRESS
                            || k.actions as u32 == PLUGIN_KEY_ACTION_REPEAT
                        {
                            let key = k.key
                                | (k.mods as u32 & PLUGIN_MOD_SHIFT > 0)
                                    .then_some(TEXTEDIT_K_SHIFT)
                                    .unwrap_or_default()
                                | (k.mods as u32 & PLUGIN_MOD_CONTROL > 0)
                                    .then_some(TEXTEDIT_K_CONTROL)
                                    .unwrap_or_default();

                            if KEYS_TO_PASS.contains(&key) {
                                self.key(&calc_text_width, key);

                                if string_len(&self.string) > self.limit {
                                    self.undo()
                                }

                                self.blink_timer = Instant::now();
                            }

                            match (k.key as u32, k.mods as u32) {
                                (PLUGIN_KEY_C | PLUGIN_KEY_X, PLUGIN_MOD_CONTROL) => {
                                    if self.select_start != self.select_end {
                                        if let Ok(mut ctx) = ClipboardContext::new() {
                                            let start_char_idx =
                                                self.select_start.min(self.select_end);
                                            let end_char_idx =
                                                self.select_start.max(self.select_end);

                                            let start_byte_idx = self
                                                .string
                                                .char_indices()
                                                .nth(start_char_idx as _)
                                                .map(|(i, _)| i)
                                                .unwrap_or(0);
                                            let end_byte_idx = self
                                                .string
                                                .char_indices()
                                                .nth(end_char_idx as _)
                                                .map(|(i, _)| i)
                                                .unwrap_or(self.string.len());

                                            let selected =
                                                &self.string[start_byte_idx..end_byte_idx];

                                            ctx.set_contents(selected.to_owned()).ok();
                                        }

                                        if k.key as u32 == PLUGIN_KEY_X {
                                            self.cut();
                                        }
                                    }
                                }
                                (PLUGIN_KEY_V, PLUGIN_MOD_CONTROL) => {
                                    if let Ok(mut ctx) = ClipboardContext::new() {
                                        if let Ok(content) = ctx.get_contents() {
                                            self.paste(&content);

                                            if string_len(&self.string) > self.limit {
                                                self.undo()
                                            }
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    WindowEvent::CharEvent(char) => {
                        self.key(&calc_text_width, char.c as _);

                        if string_len(&self.string) > self.limit {
                            self.undo()
                        }

                        self.blink_timer = Instant::now();
                    }
                    _ => (),
                }
            }*/
        }

        let text = self.string.clone(); // TODO(JoNil) Lifetime so we con't have to clone!
        let active = self.active;
        let cursor = self.cursor;

        let selection = if self.select_start != self.select_end {
            let start_char_idx = self.select_start.min(self.select_end);
            let end_char_idx = self.select_start.max(self.select_end);

            let start_byte_idx = text
                .char_indices()
                .nth(start_char_idx as _)
                .map(|(i, _)| i)
                .unwrap_or(0);
            let end_byte_idx = text
                .char_indices()
                .nth(end_char_idx as _)
                .map(|(i, _)| i)
                .unwrap_or(text.len());

            let before_selection = &text[..start_byte_idx];
            let selected = &text[start_byte_idx..end_byte_idx];

            let before_width = ui.draw.calc_text_size(
                before_selection,
                style.text_height,
                10000.0,
                style.font.unwrap_or_default(),
            );
            let selected_width = ui.draw.calc_text_size(
                selected,
                style.text_height,
                10000.0,
                style.font.unwrap_or_default(),
            );

            Some((before_width.x, selected_width.x))
        } else {
            None
        };

        let cursor_byte_idx = text
            .char_indices()
            .nth(cursor as _)
            .map(|(i, _)| i)
            .unwrap_or(text.len());

        let before_cursor = &text[..cursor_byte_idx];

        let before_cursor_width = ui.draw.calc_text_size(
            before_cursor,
            style.text_height,
            10000.0,
            style.font.unwrap_or_default(),
        );

        let blink_time = self.blink_timer.elapsed();

        frame::show(ui, false, style, None, Some(ui.current_id()), true, |ui| {
            ui.canvas(size, move |draw, cursor, size| {
                draw.text(
                    &text,
                    cursor
                        + vec2(
                            0.0,
                            -size.y / 2.0 + style.text_height / 2.0 - 0.1 * style.text_height / 2.0,
                        ),
                    vec2(size.x, style.text_height + 0.001),
                    style.text_height,
                    Align::Left,
                    style.text_color,
                    style.font.unwrap_or_default(),
                );

                if active && blink_time.as_millis() % 1000 < 500 {
                    draw.rectangle(
                        cursor
                            + vec2(
                                before_cursor_width.x,
                                -size.y / 2.0
                                    + style.text_height / 2.0
                                    + 0.1 * style.text_height / 2.0,
                            ),
                        vec2(2.0, style.text_height),
                        style.text_color,
                    );
                }

                if active {
                    if let Some((before_width, selected_width)) = selection {
                        draw.rectangle(
                            cursor
                                + vec2(
                                    before_width,
                                    -size.y / 2.0
                                        + style.text_height / 2.0
                                        + 0.1 * style.text_height / 2.0,
                                ),
                            vec2(selected_width, style.text_height),
                            vec4(0.0, 0.0, 1.0, 0.2),
                        );
                    }
                }
            });
        });
    }

    fn locate_coord(&mut self, calc_text_width: &impl Fn(&str) -> f32, x: f32, y: f32) -> i32 {
        let mut r = TexteditRow::default();
        let n = string_len(&self.string);
        let mut base_y = 0.0;
        let mut i = 0;

        while i < n {
            r.layout(calc_text_width, self, i);
            if r.num_chars <= 0 {
                return n;
            }
            if i == 0 && y < base_y + r.ymin {
                return 0;
            }
            if y < base_y + r.ymax {
                break;
            }
            i += r.num_chars;
            base_y += r.baseline_y_delta;
        }
        if i >= n {
            return n;
        }
        if x < r.x0 {
            return i;
        }
        if x < r.x1 {
            let mut prev_x = r.x0;

            for k in 0..r.num_chars {
                let w = string_width(calc_text_width, self, i, k);
                if x < prev_x + w {
                    if x < prev_x + w / 2.0 {
                        return k + i;
                    } else {
                        return k + i + 1;
                    }
                }
                prev_x += w;
            }
        }

        if get_char(&self.string, i + r.num_chars - 1) == '\n' {
            i + r.num_chars - 1
        } else {
            i + r.num_chars
        }
    }

    fn click(&mut self, calc_text_width: &impl Fn(&str) -> f32, x: f32, mut y: f32) {
        if self.single_line {
            let mut r = TexteditRow::default();
            r.layout(calc_text_width, self, 0);
            y = r.ymin;
        }
        self.cursor = self.locate_coord(calc_text_width, x, y);
        self.select_start = self.cursor;
        self.select_end = self.cursor;
        self.has_preferred_x = false;
    }

    fn drag(&mut self, calc_text_width: &impl Fn(&str) -> f32, x: f32, mut y: f32) {
        if self.single_line {
            let mut r = TexteditRow::default();
            r.layout(calc_text_width, self, 0);
            y = r.ymin;
        }

        if self.select_start == self.select_end {
            self.select_start = self.cursor;
        }

        let p = self.locate_coord(calc_text_width, x, y);
        self.select_end = p;
        self.cursor = p;
    }

    fn clamp(&mut self) {
        let n = string_len(&self.string);
        if self.select_start != self.select_end {
            if self.select_start > n {
                self.select_start = n;
            }
            if self.select_end > n {
                self.select_end = n;
            }
            if self.select_start == self.select_end {
                self.cursor = self.select_start;
            }
        }
        if self.cursor > n {
            self.cursor = n;
        }
    }

    fn delete(&mut self, where_0: i32, len: i32) {
        self.makeundo_delete(where_0, len);
        delete_chars(&mut self.string, where_0, len);
        self.has_preferred_x = false;
    }

    fn delete_selection(&mut self) {
        self.clamp();
        if self.select_start != self.select_end {
            if self.select_start < self.select_end {
                self.delete(self.select_start, self.select_end - self.select_start);
                self.cursor = self.select_start;
                self.select_end = self.select_start;
            } else {
                self.delete(self.select_end, self.select_start - self.select_end);
                self.cursor = self.select_end;
                self.select_start = self.select_end;
            }
            self.has_preferred_x = false;
        }
    }

    fn sortselection(&mut self) {
        if self.select_end < self.select_start {
            mem::swap(&mut self.select_end, &mut self.select_start);
        }
    }

    fn move_to_first(&mut self) {
        if self.select_start != self.select_end {
            self.sortselection();
            self.cursor = self.select_start;
            self.select_end = self.select_start;
            self.has_preferred_x = false;
        }
    }

    fn move_to_last(&mut self) {
        if self.select_start != self.select_end {
            self.sortselection();
            self.clamp();
            self.cursor = self.select_end;
            self.select_start = self.select_end;
            self.has_preferred_x = false;
        }
    }

    fn is_word_boundary(&mut self, idx: i32) -> bool {
        if idx > 0 {
            is_space(get_char(&self.string, idx - 1)) && !is_space(get_char(&self.string, idx))
        } else {
            true
        }
    }

    fn move_to_word_previous(&mut self, mut c: i32) -> i32 {
        c -= 1;

        while c >= 0 && !self.is_word_boundary(c) {
            c -= 1;
        }

        if c < 0 {
            c = 0;
        }

        c
    }

    fn move_to_word_next(&mut self, mut c: i32) -> i32 {
        let len = string_len(&self.string);

        c += 1;

        while c < len && !self.is_word_boundary(c) {
            c += 1;
        }

        if c > len {
            c = len;
        }

        c
    }

    fn prep_selection_at_cursor(&mut self) {
        if self.select_start == self.select_end {
            self.select_end = self.cursor;
            self.select_start = self.cursor;
        } else {
            self.cursor = self.select_end;
        };
    }

    fn cut(&mut self) -> bool {
        if self.select_start != self.select_end {
            self.delete_selection();
            self.has_preferred_x = false;
            return true;
        }
        false
    }

    fn paste(&mut self, text: &str) {
        let len = text.len() as _;
        self.clamp();
        self.delete_selection();
        insert_chars(&mut self.string, self.cursor, text);
        let cursor = self.cursor;
        self.makeundo_insert(cursor, len);
        self.cursor += len;
        self.has_preferred_x = false;
    }

    fn key(&mut self, calc_text_width: &impl Fn(&str) -> f32, mut key: i32) {
        loop {
            match key {
                TEXTEDIT_K_INSERT => {
                    self.insert_mode = !self.insert_mode;
                    break;
                }
                TEXTEDIT_K_UNDO => {
                    self.undo();
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_REDO => {
                    self.redo();
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_LEFT => {
                    if self.select_start != self.select_end {
                        self.move_to_first();
                    } else if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_RIGHT => {
                    if self.select_start != self.select_end {
                        self.move_to_last();
                    } else {
                        self.cursor += 1;
                    }
                    self.clamp();
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_SHIFT_LEFT => {
                    self.clamp();
                    self.prep_selection_at_cursor();
                    if self.select_end > 0 {
                        self.select_end -= 1;
                    }
                    self.cursor = self.select_end;
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_WORDLEFT => {
                    if self.select_start != self.select_end {
                        self.move_to_first();
                    } else {
                        self.cursor = self.move_to_word_previous(self.cursor);
                        self.clamp();
                    }
                    break;
                }
                TEXTEDIT_K_SHIFT_WORDLEFT => {
                    if self.select_start == self.select_end {
                        self.prep_selection_at_cursor();
                    }
                    self.cursor = self.move_to_word_previous(self.cursor);
                    self.select_end = self.cursor;
                    self.clamp();
                    break;
                }
                TEXTEDIT_K_WORDRIGHT => {
                    if self.select_start != self.select_end {
                        self.move_to_last();
                    } else {
                        self.cursor = self.move_to_word_next(self.cursor);
                        self.clamp();
                    }
                    break;
                }
                TEXTEDIT_K_SHIFT_WORDRIGHT => {
                    if self.select_start == self.select_end {
                        self.prep_selection_at_cursor();
                    }
                    self.cursor = self.move_to_word_next(self.cursor);
                    self.select_end = self.cursor;
                    self.clamp();
                    break;
                }
                TEXTEDIT_K_SHIFT_RIGHT => {
                    self.prep_selection_at_cursor();
                    self.select_end += 1;
                    self.clamp();
                    self.cursor = self.select_end;
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_DOWN
                | TEXTEDIT_K_SHIFT_DOWN
                | TEXTEDIT_K_PGDOWN
                | TEXTEDIT_K_SHIFT_PGDOWN => {
                    let mut find = FindState::default();
                    let mut row = TexteditRow::default();
                    let sel = key & TEXTEDIT_K_SHIFT != 0;
                    let is_page = key & !TEXTEDIT_K_SHIFT == TEXTEDIT_K_PGDOWN;
                    let row_count = if is_page { self.row_count_per_page } else { 1 };
                    if !is_page && self.single_line {
                        key = TEXTEDIT_K_RIGHT | key & TEXTEDIT_K_SHIFT;
                    } else {
                        if sel {
                            self.prep_selection_at_cursor();
                        } else if self.select_start != self.select_end {
                            self.move_to_last();
                        }
                        self.clamp();
                        find.find_charpos(calc_text_width, self, self.cursor, self.single_line);

                        for _ in 0..row_count {
                            let goal_x = if self.has_preferred_x {
                                self.preferred_x
                            } else {
                                find.x
                            };
                            let start = find.first_char + find.length;
                            if find.length == 0 {
                                break;
                            }
                            self.cursor = start;
                            row.layout(calc_text_width, self, self.cursor);
                            let mut x = row.x0;
                            for i in 0..row.num_chars {
                                let dx = string_width(calc_text_width, self, start, i);
                                x += dx;
                                if x > goal_x {
                                    break;
                                }
                                self.cursor += 1;
                            }
                            self.clamp();
                            self.has_preferred_x = true;
                            self.preferred_x = goal_x;
                            if sel {
                                self.select_end = self.cursor;
                            }
                            find.first_char += find.length;
                            find.length = row.num_chars;
                        }
                        break;
                    }
                }
                TEXTEDIT_K_UP | TEXTEDIT_K_SHIFT_UP | TEXTEDIT_K_PGUP | TEXTEDIT_K_SHIFT_PGUP => {
                    let mut find: FindState = FindState::default();
                    let mut row: TexteditRow = TexteditRow::default();
                    let sel = key & TEXTEDIT_K_SHIFT != 0;
                    let is_page = key & !(TEXTEDIT_K_SHIFT) == TEXTEDIT_K_PGUP;
                    let row_count = if is_page { self.row_count_per_page } else { 1 };
                    if !is_page && self.single_line {
                        key = TEXTEDIT_K_LEFT | key & TEXTEDIT_K_SHIFT;
                    } else {
                        if sel {
                            self.prep_selection_at_cursor();
                        } else if self.select_start != self.select_end {
                            self.move_to_first();
                        }
                        self.clamp();
                        find.find_charpos(calc_text_width, self, self.cursor, self.single_line);

                        for _ in 0..row_count {
                            let goal_x = if self.has_preferred_x {
                                self.preferred_x
                            } else {
                                find.x
                            };
                            if find.prev_first == find.first_char {
                                break;
                            }
                            self.cursor = find.prev_first;
                            row.layout(calc_text_width, self, self.cursor);
                            let mut x = row.x0;
                            for i in 0..row.num_chars {
                                let dx = string_width(calc_text_width, self, find.prev_first, i);
                                x += dx;
                                if x > goal_x {
                                    break;
                                }
                                self.cursor += 1;
                            }
                            self.clamp();
                            self.has_preferred_x = true;
                            self.preferred_x = goal_x;
                            if sel {
                                self.select_end = self.cursor;
                            }
                            let mut prev_scan = if find.prev_first > 0 {
                                find.prev_first - 1
                            } else {
                                0
                            };
                            while prev_scan > 0 && get_char(&self.string, prev_scan - 1) != '\n' {
                                prev_scan -= 1;
                            }
                            find.first_char = find.prev_first;
                            find.prev_first = prev_scan;
                        }
                        break;
                    }
                }
                TEXTEDIT_K_DELETE | TEXTEDIT_K_SHIFT_DELETE => {
                    if self.select_start != self.select_end {
                        self.delete_selection();
                    } else {
                        let n = string_len(&self.string);
                        if self.cursor < n {
                            self.delete(self.cursor, 1);
                        }
                    }
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_BACKSPACE | TEXTEDIT_K_SHIFT_BACKSPACE => {
                    if self.select_start != self.select_end {
                        self.delete_selection();
                    } else {
                        self.clamp();
                        if self.cursor > 0 {
                            self.delete(self.cursor - 1, 1);
                            self.cursor -= 1;
                        }
                    }
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_TEXTSTART => {
                    self.select_end = 0;
                    self.select_start = 0;
                    self.cursor = 0;
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_TEXTEND => {
                    self.cursor = string_len(&self.string);
                    self.select_end = 0;
                    self.select_start = 0;
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_SHIFT_TEXTSTART => {
                    self.prep_selection_at_cursor();
                    self.select_end = 0;
                    self.cursor = 0;
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_SHIFT_TEXTEND => {
                    self.prep_selection_at_cursor();
                    self.select_end = string_len(&self.string);
                    self.cursor = self.select_end;
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_LINESTART => {
                    self.clamp();
                    self.move_to_first();
                    if self.single_line {
                        self.cursor = 0;
                    } else {
                        while self.cursor > 0 && get_char(&self.string, self.cursor - 1) != '\n' {
                            self.cursor -= 1;
                        }
                    }
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_LINEEND => {
                    let n = string_len(&self.string);
                    self.clamp();
                    self.move_to_first();
                    if self.single_line {
                        self.cursor = n;
                    } else {
                        while self.cursor < n && get_char(&self.string, self.cursor) != '\n' {
                            self.cursor += 1;
                        }
                    }
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_SHIFT_LINESTART => {
                    self.clamp();
                    self.prep_selection_at_cursor();
                    if self.single_line {
                        self.cursor = 0;
                    } else {
                        while self.cursor > 0 && get_char(&self.string, self.cursor - 1) != '\n' {
                            self.cursor -= 1;
                        }
                    }
                    self.select_end = self.cursor;
                    self.has_preferred_x = false;
                    break;
                }
                TEXTEDIT_K_SHIFT_LINEEND => {
                    let n = string_len(&self.string);
                    self.clamp();
                    self.prep_selection_at_cursor();
                    if self.single_line {
                        self.cursor = n;
                    } else {
                        while self.cursor < n && get_char(&self.string, self.cursor) != '\n' {
                            self.cursor += 1;
                        }
                    }
                    self.select_end = self.cursor;
                    self.has_preferred_x = false;
                    break;
                }
                _ => {
                    if let Some(c) = key_to_text(key) {
                        let mut ch_buf = [0; 4];
                        let ch = &*(c).encode_utf8(&mut ch_buf);

                        if !(c == '\n' && self.single_line) {
                            if self.insert_mode
                                && self.select_start == self.select_end
                                && self.cursor < string_len(&self.string)
                            {
                                self.makeundo_replace(self.cursor, 1, 1);
                                delete_chars(&mut self.string, self.cursor, 1);
                                insert_chars(&mut self.string, self.cursor, ch);
                                self.cursor += 1;

                                self.has_preferred_x = false;
                            } else {
                                self.delete_selection();
                                insert_chars(&mut self.string, self.cursor, ch);
                                let cursor = self.cursor;
                                self.makeundo_insert(cursor, 1);
                                self.cursor += 1;
                                self.has_preferred_x = false;
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    fn undo(&mut self) {
        let s: &mut UndoState = &mut self.undostate;
        if s.undo_point == 0 {
            return;
        }

        let u = s.undo_rec[(s.undo_point - 1) as usize];
        let r = &mut s.undo_rec[(s.redo_point - 1) as usize];

        r.char_storage = -1;
        r.insert_length = u.delete_length;
        r.delete_length = u.insert_length;
        r.where_0 = u.where_0;

        if u.delete_length != 0 {
            if s.undo_char_point + u.delete_length >= 999 {
                r.insert_length = 0;
            } else {
                while s.undo_char_point + u.delete_length > s.redo_char_point {
                    if s.redo_point == 99 {
                        return;
                    }
                    s.discard_redo();
                }
                let r = &mut s.undo_rec[(s.redo_point - 1) as usize];
                r.char_storage = s.redo_char_point - u.delete_length;
                s.redo_char_point -= u.delete_length;
                for i in 0..u.delete_length {
                    s.undo_char[(r.char_storage + i) as usize] =
                        get_char(&self.string, u.where_0 + i);
                }
            }
            delete_chars(&mut self.string, u.where_0, u.delete_length);
        }

        if u.insert_length != 0 {
            for i in (0..u.insert_length).rev() {
                let mut buf = [0; 4];
                let ch = s.undo_char[(u.char_storage + i) as usize];
                let s = ch.encode_utf8(&mut buf);

                insert_chars(&mut self.string, u.where_0, s);
            }
            s.undo_char_point -= u.insert_length;
        }

        self.cursor = u.where_0 + u.insert_length;
        s.undo_point -= 1;
        s.redo_point -= 1;
    }

    fn redo(&mut self) {
        let s: &mut UndoState = &mut self.undostate;
        if s.redo_point == 99 {
            return;
        }

        let r = s.undo_rec[s.redo_point as usize];
        let u = &mut s.undo_rec[s.undo_point as usize];

        u.delete_length = r.insert_length;
        u.insert_length = r.delete_length;
        u.where_0 = r.where_0;
        u.char_storage = -1;

        if r.delete_length != 0 {
            if s.undo_char_point + u.insert_length > s.redo_char_point {
                u.insert_length = 0;
                u.delete_length = 0;
            } else {
                u.char_storage = s.undo_char_point;
                s.undo_char_point += u.insert_length;

                for i in 0..u.insert_length {
                    s.undo_char[(u.char_storage + i) as usize] =
                        get_char(&self.string, u.where_0 + i);
                }
            }
            delete_chars(&mut self.string, r.where_0, r.delete_length);
        }

        if r.insert_length != 0 {
            for i in (0..r.insert_length).rev() {
                let mut buf = [0; 4];
                let ch = s.undo_char[(r.char_storage + i) as usize];
                let s = &*ch.encode_utf8(&mut buf);

                insert_chars(&mut self.string, r.where_0, s);
            }
            s.redo_char_point += r.insert_length;
        }
        self.cursor = r.where_0 + r.insert_length;
        s.undo_point += 1;
        s.redo_point += 1;
    }

    fn makeundo_insert(&mut self, where_0: i32, length: i32) {
        self.undostate.create_undo(where_0, 0, length);
    }

    fn makeundo_delete(&mut self, where_0: i32, length: i32) {
        if let Some(p) = self.undostate.create_undo(where_0, length, 0) {
            for i in 0..length {
                p[i as usize] = get_char(&self.string, where_0 + i);
            }
        }
    }

    fn makeundo_replace(&mut self, where_0: i32, old_length: i32, new_length: i32) {
        if let Some(p) = self.undostate.create_undo(where_0, old_length, new_length) {
            for i in 0..old_length {
                p[i as usize] = get_char(&self.string, where_0 + i);
            }
        }
    }
}
