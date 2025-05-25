fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/try.ico") // Путь к иконке
           .compile().unwrap();
    }
}