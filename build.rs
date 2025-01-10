fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        if std::path::Path::new("assets/app.ico").exists() {
            res.set_icon("assets/app.ico");
        }
        res.compile().unwrap_or_else(|e| {
            println!("Warning: Failed to compile resources: {}", e);
        });
    }
} 