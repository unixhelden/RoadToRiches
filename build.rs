fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        // Hier sicherstellen, dass der Pfad zu deinem Assets-Ordner passt
        res.set_icon("assets/icon.ico"); 
        res.compile().unwrap();
    }
}