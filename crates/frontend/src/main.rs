//! yt-rs frontend application entry point.

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<yt_rs_frontend::app::App>::new().render();
}
