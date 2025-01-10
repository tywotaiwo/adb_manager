#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico"); // Replace this with your icon path
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {} 