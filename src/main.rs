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
        builder: SimulationBuilder,
    },
    Simulate {
        simulation: Arc<Mutex<Simulation>>,
        // sender to stop simulation thread
        stop_tx: mpsc::Sender<bool>,
        // what should the app revert to when simulation is stopped?
        revert_state: Box<MyEguiApp>,
    },
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        MyEguiApp::Setup {
            particle_num: 280,
            spacing: 7.,
            positions: vec![],
            offset: egui::Vec2 { x: 17., y: 17. },
            builder: SimulationBuilder::default(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("settings").show(ctx, |ui| {
            use MyEguiApp::*;

            match self {
                Setup {
                    particle_num,
                    spacing,
                    positions,
                    offset,
                    builder,
                } => {
                    ui.add(Slider::new(particle_num, 0..=800).text("particle num"));
                    ui.add(Slider::new(spacing, 0.0..=100.).text("spacing"));
                    ui.add(Slider::new(&mut offset.x, 0.0..=100.).text("x offset"));
                    ui.add(Slider::new(&mut offset.y, 0.0..=100.).text("y offset"));

                    ui.add(egui::Slider::new(&mut builder.interaction_radius, 0.0..=200.).text("interaction radius"));
                    ui.add(egui::Slider::new(&mut builder.pressure_multiplier, 0.0..=50.).text("pressure multiplier"));
                    ui.add(egui::Slider::new(&mut builder.near_pressure_multiplier, 0.0..=50.).text("near pressure multiplier"));
                    ui.add(egui::Slider::new(&mut builder.rest_density, 0.0..=50.).text("rest density"));
                    ui.add(egui::Slider::new(&mut builder.gravity, 0.0..=1000.).text("gravity"));

                    if ui.button("run").clicked() {
                        *builder = builder.with_particles(
                            positions
                                .into_iter()
                                .map(|pos| Particle {
                                    pos: fluid_sim::Vec2 { x: pos.x, y: pos.y },
                                    vel: fluid_sim::Vec2 { x: 0., y: 0. },
                                    prev_pos: fluid_sim::Vec2 { x: 0., y: 0. },
                                })
                                .collect(),
                        );

                        let arc_simulation = Arc::new(Mutex::new(builder.build()));

                        let (tx, rx) = mpsc::channel::<bool>();
                        let revert_state = self.clone();

                        *self = Simulate {
                            simulation: Arc::clone(&arc_simulation),
                            stop_tx: tx,
                            revert_state: Box::new(revert_state),
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
                Simulate { simulation, stop_tx, revert_state } => {
                    if ui.button("stop").clicked() {
                        stop_tx.send(true).unwrap();
                        *self = *revert_state.clone();
                        return;
                    }

                    let mut simulation = simulation.lock().unwrap();

                    ui.add(egui::Slider::new(&mut simulation.interaction_radius, 0.0..=200.).text("interaction radius"));
                    ui.add(egui::Slider::new(&mut simulation.pressure_multiplier, 0.0..=50.).text("pressure multiplier"));
                    ui.add(egui::Slider::new(&mut simulation.near_pressure_multiplier, 0.0..=50.).text("near pressure multiplier"));
                    ui.add(egui::Slider::new(&mut simulation.rest_density, 0.0..=50.).text("rest density"));
                    ui.add(egui::Slider::new(&mut simulation.gravity, 0.0..=1000.).text("gravity"));
                },
            }
        });

        let mut central_panel_rect = egui::Rect::ZERO;

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
                egui::Rect {
                    min: painter_pos(pos2(0., 0.)),
                    max: painter_pos(pos2(response.rect.width(), response.rect.height())),
                },
                Color32::GREEN,
                "text",
            );

            let interaction_diameter = match self {
                Setup { builder, .. } => builder.interaction_radius,
                Simulate { simulation, .. } => simulation.lock().unwrap().interaction_radius,
            } * 2.;

            let mut x = 0;
            let mut y = 0;
            loop {
                let display_pos = painter_pos(pos2(
                    x as f32 * interaction_diameter,
                    y as f32 * interaction_diameter,
                ));

                if display_pos.x > response.rect.max.x {
                    x = 0;
                    y += 1;
                    continue;
                }

                if display_pos.y > response.rect.max.y {
                    break;
                }
                
                painter.rect_stroke(
                    egui::Rect {
                        min: pos2(display_pos.x, display_pos.y),
                        max: pos2(display_pos.x + interaction_diameter, display_pos.y + interaction_diameter),
                    },
                    0.,
                    Stroke::new(1., Color32::GRAY),
                    StrokeKind::Middle,
                );

                x += 1;
            }

            match self {
                Setup { positions, .. } => {
                    for position in positions.iter().map(|pos| painter_pos(*pos)) {
                        painter.circle_filled(position, 3., Color32::BLUE);
                    }
                }
                Simulate { simulation, .. } => {
                    let mut simulation = simulation.lock().unwrap();

                    simulation.boundaries = fluid_sim::Rect {
                        min: fluid_sim::Vec2 { x: 0., y: 0. },
                        max: fluid_sim::Vec2 { x: response.rect.width(), y: response.rect.height() },
                    };

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
