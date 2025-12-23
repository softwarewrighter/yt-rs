//! yt-rs frontend application entry point.

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<yt_rs_yew_app::app::App>::new().render();
}
