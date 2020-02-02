use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let elem = document.create_element("h1")?;
    elem.set_inner_html("Hello WASM from Rust!");
    body.append_child(&elem)?;

    let canvas = document.create_element("canvas")?;
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(1000);
    canvas.set_height(1000);
    body.append_child(&canvas)?;

    let ctx = canvas.get_context("2d")?.expect("canvas should get 2D ctx");
    let ctx = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    ctx.begin_path();
    ctx.stroke_rect(75.0, 140.0, 150.0, 110.0);
    ctx.stroke();

    Ok(())
}
