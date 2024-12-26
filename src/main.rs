use eframe::egui;
use egui::*;

use fluid_sim::*;

use std::thread;
use std::time::Instant;
use std::sync::*;

fn main() {
    let app = MyApp::new();
    let world_arc = Arc::clone(&app.world);

    thread::spawn(move || {
        let dt = 1. / 60.; // seconds
        let mut now = Instant::now();

        let mut accum = 0.;
        
        loop {
            accum += now.elapsed().as_secs_f32();
            now = Instant::now();

            while accum >= dt {
                let mut world = world_arc.lock().unwrap();
                world.step(dt);

                accum -= dt;
            }
        }
    });
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("fluid sim", native_options, Box::new(|cc| Ok(Box::new(app))));
}

struct MyApp {
    world: Arc<Mutex<World>>,
}

impl MyApp {
    fn new() -> MyApp {
        let mut particles = vec![];

        let hori_num = 15;
        let vert_num = 15;
        let spacing = 10;

        for hori in 0..hori_num {
            for vert in 0..vert_num {
                particles.push(Particle::new(
                    (hori * spacing) as f32,
                    (vert * spacing) as f32,
                ));
            }
        }

        MyApp { world: Arc::new(Mutex::new(World::new(
            particles,
            fluid_sim::Vec2 { x: 400., y: 400. },
            0.5,
            1.,
        ))) }
    }
}

impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       let world = self.world.lock().unwrap();
       
       egui::CentralPanel::default().show(ctx, |ui| {
           let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

           for &particle in world.particles() {
               painter.circle_filled(pos2(particle.pos.x, particle.pos.y), 2., Color32::BLUE);
           }
       });

       ctx.request_repaint();
   }
}
