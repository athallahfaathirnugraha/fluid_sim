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

        let hori_num = 20;
        let vert_num = 20;
        let spacing = 1;

        for hori in 0..hori_num {
            for vert in 0..vert_num {
                particles.push(Particle::new(
                    (hori * spacing) as f32,
                    (vert * spacing) as f32,
                ));
            }
        }

        MyApp { world: Arc::new(Mutex::new(World::with_particles(particles))) }
    }
}

impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       let world = self.world.lock().unwrap();
       
       egui::CentralPanel::default().show(ctx, |ui| {
           let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

           painter.rect_filled(response.rect, 5., Color32::BLACK);

           painter.debug_rect(
               Rect { min: Pos2::ZERO, max: pos2(world.boundaries.x, world.boundaries.y) },
               Color32::GREEN,
               "boundaries",
           );

           for &particle in world.particles() {
               painter.circle_filled(pos2(particle.pos.x, particle.pos.y), 2., Color32::BLUE);
           }
       });

       ctx.request_repaint();
   }
}
