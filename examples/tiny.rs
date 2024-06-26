use mini_ui::math::{vec2, vec4, Vec2, Vec4};
use mini_ui::{
    cairo::{self, Context, Format, ImageSurface},
    ui::{
        Align, Font, FrameStyle, Image, Oui, OuiContext, Spacing, Style, TextEdit, Ui, VertAlign,
    },
    window::{Key, Window, WindowOptions},
};
use std::{f32::consts::TAU, slice};

const WIDTH: usize = 1900;
const HEIGHT: usize = 1000;

fn main() {
    let mut window = Window::new(
        "Tiny - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let mut state = State::new();

    let mut surface = ImageSurface::create(Format::ARgb32, WIDTH as _, HEIGHT as _)
        .expect("Can't create surface");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        {
            let context = Context::new(&surface).unwrap();

            context.select_font_face(
                "Helvetica",
                cairo::FontSlant::Normal,
                cairo::FontWeight::Normal,
            );

            state.update(&window, &context, vec2(WIDTH as f32, HEIGHT as f32));
        }

        let data = surface.data().unwrap();
        window
            .update_with_buffer(
                unsafe { slice::from_raw_parts(data.as_ptr() as *const _, data.len() / 4) },
                WIDTH,
                HEIGHT,
            )
            .unwrap();
    }
}

const GRAY: Vec4 = vec4(82.0 / 100.0, 82.0 / 100.0, 82.0 / 100.0, 1.0);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ConnectionState {
    Available,
    Occupied,
    Unavailable,
}

impl ConnectionState {
    fn color(&self) -> Vec4 {
        match self {
            ConnectionState::Available => vec4(52.2 / 100.0, 88.6 / 100.0, 56.9 / 100.0, 1.0),
            ConnectionState::Occupied => vec4(99.6 / 100.0, 0.0, 0.8 / 100.0, 1.0),
            ConnectionState::Unavailable => GRAY,
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    name: String,
    id: String,
    connection_state: ConnectionState,
    latency: f32,
    warning: bool,
}

struct Styles {
    background: Style,
    back_frame: Style,
    frame: Style,
    label: Style,
    label_sub: Style,
    icon: Style,
    button: Style,
    filter_frame: Style,
    filter_elements: Style,
    count: Style,
    top_frame: Style,
    search_button: Style,
}

impl Styles {
    fn new() -> Styles {
        let base = Style::default()
            .border_color(GRAY)
            .frame_color(GRAY)
            .frame_style(FrameStyle::RoundedRectangle(10.0))
            .debug(false);

        Styles {
            background: base
                .frame_color(vec4(91.4 / 100.0, 94.5 / 100.0, 95.3 / 100.0, 1.0))
                .frame_style(FrameStyle::Rectangle),
            back_frame: base
                .margin(Spacing::symmetrical(10.0))
                .shadow_color(vec4(0.0, 0.0, 0.0, 0.05))
                .shadow_dir(vec2(2.0, -2.0))
                .text_color(vec4(1.0, 1.0, 1.0, 1.0))
                .text_height(60.0),
            frame: base
                .frame_color(vec4(1.0, 1.0, 1.0, 1.0))
                .text_color(vec4(0.0, 0.0, 0.0, 1.0))
                .text_height(70.0),
            label: base
                .margin(Spacing::symmetrical(5.0))
                .text_color(vec4(0.0, 0.0, 0.0, 1.0))
                .text_height(30.0),
            label_sub: base
                .margin(Spacing::symmetrical(5.0).with_top(0.0))
                .text_color(vec4(0.4, 0.4, 0.4, 1.0))
                .text_height(15.0),
            icon: base
                .margin(Spacing::symmetrical(5.0))
                .text_color(vec4(1.0, 1.0, 1.0, 1.0))
                .text_height(30.0)
                .vert_align(VertAlign::Center),
            button: base
                .align(Align::Center)
                .margin(Spacing::symmetrical(5.0))
                .text_color(vec4(1.0, 1.0, 1.0, 1.0))
                .text_height(15.0)
                .vert_align(VertAlign::Center),
            filter_frame: base
                .frame_color(vec4(1.0, 1.0, 1.0, 1.0))
                .text_color(vec4(0.0, 0.0, 0.0, 1.0))
                .text_height(20.0)
                .shadow_color(vec4(0.0, 0.0, 0.0, 0.05))
                .shadow_dir(vec2(2.0, -2.0))
                .margin(Spacing::symmetrical(10.0))
                .padding(Spacing::symmetrical(10.0)),
            filter_elements: base
                .frame_color(vec4(1.0, 1.0, 1.0, 1.0))
                .text_color(vec4(0.0, 0.0, 0.0, 1.0))
                .text_height(26.0)
                .margin(Spacing::symmetrical(5.0)),
            count: base
                .text_color(vec4(5.9 / 100.0, 61.2 / 100.0, 83.1 / 100.0, 1.0))
                .text_height(90.0)
                .margin(Spacing::symmetrical(5.0)),
            top_frame: base
                .frame_style(FrameStyle::Rectangle)
                .padding(Spacing::y(10.0))
                .shadow_color(vec4(0.0, 0.0, 0.0, 0.05))
                .shadow_dir(vec2(2.0, -2.0))
                .text_color(vec4(0.0, 0.0, 0.0, 1.0))
                .text_height(35.0)
                .frame_color(vec4(1.0, 1.0, 1.0, 1.0)),
            search_button: base
                .text_color(vec4(0.0, 0.0, 0.0, 1.0))
                .text_height(20.0)
                .frame_color(vec4(91.4 / 100.0, 94.5 / 100.0, 95.3 / 100.0, 1.0))
                .vert_align(VertAlign::Center)
                .margin(Spacing::x(16.0)),
        }
    }
}

struct State {
    ctx: OuiContext,

    style: Styles,

    camera: Image,
    compass: Image,
    person: Image,
    robot: Image,
    warning: Image,
    no_warning: Image,
    on: Image,
    off: Image,

    show_pushers: bool,
    show_packers: bool,
    show_bagers: bool,
    show_platforms: bool,

    search: TextEdit,
}

impl State {
    fn new() -> Self {
        State {
            ctx: OuiContext::new(),
            style: Styles::new(),
            camera: Image::load_png(include_bytes!("assets/camera.png")),
            compass: Image::load_png(include_bytes!("assets/compass.png")),
            person: Image::load_png(include_bytes!("assets/person.png")),
            robot: Image::load_png(include_bytes!("assets/robot.png")),
            warning: Image::load_png(include_bytes!("assets/warning.png")),
            no_warning: Image::load_png(include_bytes!("assets/no_warning.png")),
            on: Image::load_png(include_bytes!("assets/on.png")),
            off: Image::load_png(include_bytes!("assets/off.png")),
            show_pushers: true,
            show_packers: true,
            show_bagers: true,
            show_platforms: true,
            search: TextEdit::new("Search", 30),
        }
    }

    fn update(&mut self, window: &Window, context: &Context, size: Vec2) {
        let robots = vec![
            Robot {
                name: "Pusher".into(),
                id: "x7d61".into(),
                connection_state: ConnectionState::Occupied,
                latency: 60.0,
                warning: false,
            },
            Robot {
                name: "Packer".into(),
                id: "x7d62".into(),
                connection_state: ConnectionState::Unavailable,
                latency: 5.0,
                warning: true,
            },
            Robot {
                name: "Platform".into(),
                id: "x7d63".into(),
                connection_state: ConnectionState::Available,
                latency: 60.0,
                warning: false,
            },
            Robot {
                name: "Bager".into(),
                id: "x7d64".into(),
                connection_state: ConnectionState::Available,
                latency: 60.0,
                warning: false,
            },
            Robot {
                name: "Pusher".into(),
                id: "x7d65".into(),
                connection_state: ConnectionState::Available,
                latency: 60.0,
                warning: false,
            },
        ];

        let all_count = robots.len();
        let pusher_count = robots.iter().filter(|r| r.name == "Pusher").count();
        let packer_count = robots.iter().filter(|r| r.name == "Packer").count();
        let bager_count = robots.iter().filter(|r| r.name == "Bager").count();
        let platform_count = robots.iter().filter(|r| r.name == "Platform").count();
        let unavailable_count = robots
            .iter()
            .filter(|r| r.connection_state == ConnectionState::Unavailable)
            .count();

        let filtered_robots = robots
            .iter()
            .filter(|r| {
                (r.name == "Pusher" && self.show_pushers)
                    || (r.name == "Packer" && self.show_packers)
                    || (r.name == "Bager" && self.show_bagers)
                    || (r.name == "Platform" && self.show_platforms)
            })
            .cloned()
            .collect::<Vec<Robot>>();

        struct Filter<'a> {
            name: String,
            state: &'a mut bool,
            count: usize,
        }

        let mut filters = [
            Filter {
                name: "Pusher".to_string(),
                state: &mut self.show_pushers,
                count: pusher_count,
            },
            Filter {
                name: "Packer".to_string(),
                state: &mut self.show_packers,
                count: packer_count,
            },
            Filter {
                name: "Bager".to_string(),
                state: &mut self.show_bagers,
                count: bager_count,
            },
            Filter {
                name: "Platform".to_string(),
                state: &mut self.show_platforms,
                count: platform_count,
            },
        ];

        Oui::new(&mut self.ctx)
            .style(self.style.background.align(Align::Center))
            .fill(true)
            .show(window, context, size, |ui: &mut Ui| {
                ui.frame(self.style.top_frame, |ui| {
                    ui.with_style(self.style.search_button, |ui| {
                        self.search.show(vec2(376.0, 44.0), context, ui);
                    });

                    ui.horizontal_spring();
                });

                ui.next_line();

                ui.no_expand_area(ui.style(), |ui| {
                    ui.sized_frame(vec2(186.0, 180.0), self.style.filter_frame, |ui| {
                        ui.with_style(self.style.filter_elements, |ui| {
                            ui.text("All Robots");
                            ui.next_line();

                            ui.with_style(self.style.count, |ui| ui.text(format!("{all_count}")));
                            ui.next_line();

                            let all_on = filters.iter().all(|f| *f.state);

                            if all_on {
                                if ui.image_button(self.on).pressed {
                                    filters.iter_mut().for_each(|f| *f.state = false);
                                }
                            } else if ui.image_button(self.off).pressed {
                                filters.iter_mut().for_each(|f| *f.state = true);
                            }
                        });
                    });

                    for filter in filters {
                        ui.sized_frame(vec2(186.0, 180.0), self.style.filter_frame, |ui| {
                            ui.with_style(self.style.filter_elements, |ui| {
                                ui.image(self.robot);
                                ui.text(filter.name);
                                ui.next_line();

                                ui.with_style(self.style.count, |ui| {
                                    ui.text(format!("{}", filter.count))
                                });
                                ui.next_line();

                                if *filter.state {
                                    if ui.image_button(self.on).pressed {
                                        *filter.state = false;
                                    }
                                } else if ui.image_button(self.off).pressed {
                                    *filter.state = true;
                                }
                            });
                        });
                    }

                    ui.sized_frame(vec2(186.0, 180.0), self.style.filter_frame, |ui| {
                        ui.with_style(self.style.filter_elements, |ui| {
                            ui.image(self.no_warning);
                            ui.text("Unavalible");
                            ui.next_line();

                            ui.with_style(self.style.count, |ui| {
                                ui.text(format!("{unavailable_count}"))
                            });
                        });
                    });

                    ui.next_line();

                    for robot in filtered_robots {
                        ui.frame(self.style.back_frame, |ui| {
                            ui.frame(self.style.frame, |ui| {
                                ui.with_style(
                                    self.style.frame.margin(Spacing::symmetrical(5.0)),
                                    |ui| {
                                        ui.rounded_rectangle(
                                            vec2(
                                                self.style.frame.text_height / 10.0,
                                                self.style.frame.text_height,
                                            ),
                                            self.style.frame.text_height / 20.0,
                                            robot.connection_state.color(),
                                        );
                                        ui.image(self.robot);

                                        ui.sized_area(vec2(360.0, 0.0), self.style.label, |ui| {
                                            ui.text(robot.name.clone());
                                            ui.next_line();
                                            ui.with_style(self.style.label_sub, |ui| {
                                                ui.text(format!("ID: {}", robot.id));
                                            });
                                        });

                                        ui.area(self.style.icon, |ui| {
                                            if ui.image_button(self.camera).pressed {
                                                println!("Camera");
                                            }
                                            if ui.image_button(self.compass).pressed {
                                                println!("Compass");
                                            }
                                            if ui.image_button(self.person).pressed {
                                                println!("Person");
                                            }
                                        });

                                        ui.horizontal_spring();

                                        ui.with_style(self.style.button, |ui| {
                                            if ui.sized_button(vec2(120.0, 30.0), "Logs").pressed {
                                                println!("Show Logs");
                                            }
                                        });

                                        ui.with_style(
                                            self.style
                                                .button
                                                .frame_color(robot.connection_state.color()),
                                            |ui| {
                                                if ui
                                                    .sized_button(
                                                        vec2(120.0, 30.0),
                                                        match robot.connection_state {
                                                            ConnectionState::Available => "Connect",
                                                            ConnectionState::Occupied => {
                                                                "Disconnect"
                                                            }
                                                            ConnectionState::Unavailable => {
                                                                "Unavailable"
                                                            }
                                                        },
                                                    )
                                                    .pressed
                                                {
                                                    println!("State: {:?}", robot.connection_state);
                                                }
                                            },
                                        );

                                        ui.with_style(self.style.frame, |ui| {
                                            ui.separator();
                                        });

                                        ui.with_style(self.style.icon, |ui| {
                                            latency_widget(ui, robot.latency);

                                            if robot.warning {
                                                ui.image(self.warning);
                                            } else {
                                                ui.image(self.no_warning);
                                            }
                                        });

                                        ui.empty_area(vec2(5.0, 0.0));
                                    },
                                );
                            });

                            ui.with_style(
                                self.style
                                    .back_frame
                                    .margin(Spacing::x(5.0).with_top(5.0))
                                    .no_shadow(),
                                |ui| {
                                    ui.text(":");
                                },
                            );
                        });

                        ui.next_line();
                    }
                });
            });
    }
}

fn latency_widget(ui: &mut Ui, latency: f32) {
    let style = ui.style();

    let size = style.text_height * 1.2;

    ui.canvas(Vec2::splat(size), move |draw, cursor, size| {
        let size = size.x;

        let latency_ratio = (latency / 100.0).clamp(0.0, 1.0);

        let width = 5.0;
        let radius = size / 2.0 - width / 2.0;
        let text_size = size / 3.0 * 1.2;
        let text_offset = text_size * 0.1;

        if style.debug {
            draw.rectangle(
                cursor + vec2(0.0, -text_offset - size / 2.0 + text_size / 2.0),
                vec2(size, text_size),
                vec4(1.0, 1.0, 1.0, 0.3),
            );
        }

        draw.text(
            &format!("{latency:.0}"),
            cursor + vec2(0.0, -text_offset - size / 2.0 + text_size / 2.0),
            vec2(size, text_size),
            text_size,
            Align::Center,
            vec4(0.0, 0.0, 0.0, 1.0),
            Font::default(),
        );

        draw.circle_segment(
            cursor,
            radius,
            0.0,
            -latency_ratio * TAU,
            width,
            vec4(96.6 / 100.0, 1.2 / 100.0, 2.0 / 100.0, 1.0),
            16,
        );

        draw.circle_segment(
            cursor,
            radius,
            -latency_ratio * TAU,
            -TAU,
            width,
            vec4(31.0 / 100.0, 65.5 / 100.0, 19.2 / 100.0, 1.0),
            16,
        )
    });
}
