use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("fluid sim", native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> MyApp {
        MyApp::default()
    }
}

impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}
