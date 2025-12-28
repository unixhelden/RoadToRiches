// build.rs
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico"); // Hier der Name deiner Icon-Datei
        res.compile().unwrap();
    }
}
