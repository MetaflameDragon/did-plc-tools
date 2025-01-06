#![feature(never_type)]


#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    todo!()
}
