use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::sync::mpsc;

use eframe::egui;
use egui::*;

use fluid_sim::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(Clone)]
enum MyEguiApp {
    Setup {
        particle_num: usize,
        spacing: f32,
        positions: Vec<Pos2>,
        offset: egui::Vec2,
    },
    Simulate {
        simulation: Arc<Mutex<Simulation>>,
        stop_tx: mpsc::Sender<bool>,
    },
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        MyEguiApp::Setup {
            particle_num: 200,
            spacing: 7.,
            positions: vec![],
            offset: egui::Vec2 { x: 17., y: 17. },
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("settings").show(ctx, |ui| {
            use MyEguiApp::*;

            match self {
                Setup {
                    ref mut particle_num,
                    ref mut spacing,
                    ref mut positions,
                    ref mut offset,
                } => {
                    ui.add(Slider::new(particle_num, 0..=800).text("particle num"));
                    ui.add(Slider::new(spacing, 0.0..=100.).text("spacing"));
                    ui.add(Slider::new(&mut offset.x, 0.0..=100.).text("x offset"));
                    ui.add(Slider::new(&mut offset.y, 0.0..=100.).text("y offset"));

                    if ui.button("run").clicked() {
                        let arc_simulation = Arc::new(Mutex::new(Simulation::with_particles(
                            positions
                                .into_iter()
                                .map(|pos| Particle {
                                    pos: fluid_sim::Vec2 { x: pos.x, y: pos.y },
                                    vel: fluid_sim::Vec2 { x: 0., y: 0. },
                                    prev_pos: fluid_sim::Vec2 { x: 0., y: 0. },
                                })
                                .collect(),
                        )));

                        let (tx, rx) = mpsc::channel::<bool>();

                        *self = Simulate {
                            simulation: Arc::clone(&arc_simulation),
                            stop_tx: tx,
                        };

                        thread::spawn(move || {
                            println!("starting simulation thread");
                            
                            let dt = 1. / 60.; // seconds
                            let mut now = Instant::now();

                            let mut accum = 0.;
    
                            'blk: loop {
                                accum += now.elapsed().as_secs_f32();
                                now = Instant::now();

                                while accum >= dt {
                                    let mut simulation = arc_simulation.lock().unwrap();
                                    simulation.step(dt);

                                    accum -= dt;
                                }

                                match rx.try_recv() {
                                    Ok(stop) if stop => break 'blk,
                                    _ => (),
                                }
                            }

                            // TODO: stop simulation thread after stopping simulation
                            println!("stopping simulation thread");
                        });

                        return;
                    }

                    *positions = vec![Pos2::ZERO; *particle_num];

                    let columns = f32::sqrt(*particle_num as f32) as u32;

                    let mut x = 0;
                    let mut y = 0;
                    for i in 0..*particle_num {
                        if x >= columns {
                            x = 0;
                            y += 1;
                        }

                        positions[i] = pos2(
                            x as f32 * *spacing + offset.x,
                            y as f32 * *spacing + offset.y,
                        );

                        x += 1;
                    }
                }
                Simulate { simulation, stop_tx } => {
                    if ui.button("stop").clicked() {
                        stop_tx.send(true).unwrap();

                        *self = Setup {
                            particle_num: 200,
                            spacing: 7.,
                            positions: vec![],
                            offset: egui::Vec2 { x: 17., y: 17. },
                        };

                        return;
                    }
                },
            }
        });

        let mut central_panel_rect = Rect::ZERO;

        egui::CentralPanel::default().show(ctx, |ui| {
            central_panel_rect = ui.max_rect();
        });

        egui::Window::new("simulation").constrain_to(central_panel_rect).show(ctx, |ui| {
            use MyEguiApp::*;
            
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                Sense::empty(),
            );

            let painter_pos =
                |pos: Pos2| pos2(pos.x + response.rect.min.x, pos.y + response.rect.min.y);

            painter.debug_rect(
                Rect {
                    min: painter_pos(pos2(0., 0.)),
                    max: painter_pos(pos2(response.rect.width(), response.rect.height())),
                },
                Color32::GREEN,
                "text",
            );

            match self {
                Setup { positions, .. } => {
                    for position in positions.iter().map(|pos| painter_pos(*pos)) {
                        painter.circle_filled(position, 3., Color32::BLUE);
                    }
                }
                Simulate { simulation, .. } => {
                    let simulation = simulation.lock().unwrap();

                    for particle in simulation.particles() {
                        let pos = painter_pos(pos2(particle.pos.x, particle.pos.y));
                        painter.circle_filled(pos, 3., Color32::BLUE);
                    }

                    ctx.request_repaint();
                }
            }
        });
    }
}
