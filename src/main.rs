use eframe::egui;
use egui::*;

use fluid_sim::*;

use std::thread;
use std::time::Instant;

fn main() {
    thread::spawn(move || {
        let dt = 1.; // seconds
        let mut now = Instant::now();

        let mut accum = 0.;
        
        loop {
            accum += now.elapsed().as_secs_f32();
            now = Instant::now();

            while accum >= dt {
                println!("step");
                accum -= dt;
            }
        }
    });
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("fluid sim", native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}

#[derive(Default)]
struct MyApp {
    particles: Vec<Particle>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> MyApp {
        let mut particles = vec![];

        let hori_num = 30;
        let vert_num = 20;
        let spacing = 10;

        for hori in 0..hori_num {
            for vert in 0..vert_num {
                particles.push(Particle::new(
                    (hori * spacing) as f32,
                    (vert * spacing) as f32,
                ));
            }
        }

        MyApp { particles }
    }
}

impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

           for &particle in &self.particles {
               painter.circle_filled(pos2(particle.pos.x, particle.pos.y), 2., Color32::BLUE);
           }
       });
   }
}
