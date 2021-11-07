// [[file:../ui.note::f69a16e3][f69a16e3]]
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = ui::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
// f69a16e3 ends here
