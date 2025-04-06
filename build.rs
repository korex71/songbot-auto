fn main() { 
    let mut res = winres::WindowsResource::new(); 
    res.set_icon("icon.ico"); // caminho do seu ícone 
    res.compile().unwrap(); 
}