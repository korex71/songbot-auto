use std::path::Path;

fn main() { 
    if !cfg!(target_os = "windows") {
        println!("cargo:warning=Ignoring build.rs.");
        return;
    }

    let icon_path = "icon.ico";
    if Path::new(icon_path).exists() {
        let mut res = winres::WindowsResource::new();
        res.set_icon(icon_path);
        res.compile().unwrap();
    } else {
        println!("cargo:warning=Icon file not found, skipping resource embedding");
    }
}