use eframe::egui;
use egui::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("water simulation");
       });

       egui::Window::new("simulation").show(ctx, |ui| {
           let (response, mut painter) =
               ui.allocate_painter(ui.available_size(), Sense::empty());

           let painter_pos = |pos: Pos2| {
               pos2(
                   pos.x + response.rect.min.x,
                   pos.y + response.rect.min.y,
               )
            };

           painter.debug_rect(
               Rect {
                   min: painter_pos(pos2(0., 0.)),
                   max: painter_pos(pos2(response.rect.width(), response.rect.height())),
               },
               Color32::GREEN,
               "text",
           );
       });
   }
}
