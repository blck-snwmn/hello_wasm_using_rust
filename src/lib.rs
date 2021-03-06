use std::cell::Cell;
use std::f64;
use std::rc::Rc;
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

    // hello !
    let elem = create_greeting_element(&document)?;
    body.append_child(&elem)?;

    // draw on canvas
    // add canvas
    let canvas = create_canvas(&document)?;
    body.append_child(&canvas)?;

    let svg = create_svg(&document)?;
    body.append_child(&svg)?;

    let drawer = create_draw_canvas(&document)?;
    body.append_child(&drawer)?;

    // draw in svg
    // TODO rewrite.
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    let svg = svg.dyn_into::<web_sys::SvgElement>()?;
    svg.set_attribute("width", "600")?;
    svg.set_attribute("hight", "600")?;
    body.append_child(&svg)?;
    let svg_rc = Rc::new(svg);
    let svg_in_closer = svg_rc.clone();
    let svg2 = svg_rc.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        // draw circle
        let circle = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")
            .unwrap()
            .dyn_into::<web_sys::SvgElement>()
            .unwrap();
        circle.set_attribute("r", "10").unwrap();
        circle
            .set_attribute("cx", &event.offset_x().to_string())
            .unwrap();
        circle
            .set_attribute("cy", &event.offset_y().to_string())
            .unwrap();
        circle.set_attribute("fill", "blue").unwrap();
        svg_in_closer.append_child(&circle).unwrap();
    }) as Box<dyn FnMut(_)>);
    svg2.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn create_greeting_element(document: &web_sys::Document) -> Result<web_sys::Element, JsValue> {
    let elem = document.create_element("h1")?;
    elem.set_inner_html("Hello WASM from Rust!");
    return Ok(elem);
}

fn create_canvas(document: &web_sys::Document) -> Result<web_sys::HtmlCanvasElement, JsValue> {
    let canvas = document.create_element("canvas")?;
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(600);
    canvas.set_height(600);
    let ctx = canvas.get_context("2d")?.expect("canvas should get 2D ctx");
    let ctx = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    // draw
    // foundation
    ctx.stroke_rect(100.0, 400.0, 400.0, 100.0);
    // snowball
    ctx.begin_path();
    ctx.arc(300.0, 300.0, 100.0, 0.0, f64::consts::PI * 2.0)?;
    ctx.stroke();
    ctx.begin_path();
    ctx.arc(300.0, 120.0, 80.0, 0.0, f64::consts::PI * 2.0)?;
    ctx.stroke();
    ctx.begin_path();
    ctx.move_to(300.0, 130.0);
    ctx.line_to(240.0, 140.0);
    ctx.line_to(300.0, 150.0);
    ctx.close_path();
    ctx.stroke();

    Ok(canvas)
}

fn create_svg(document: &web_sys::Document) -> Result<web_sys::SvgElement, JsValue> {
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    let svg = svg.dyn_into::<web_sys::SvgElement>()?;
    svg.set_attribute("width", "600")?;
    svg.set_attribute("hight", "600")?;

    // draw circle
    let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    let circle = circle.dyn_into::<web_sys::SvgElement>()?;
    circle.set_attribute("r", "100")?;
    circle.set_attribute("fill", "blue")?;
    svg.append_child(&circle)?;

    // draw txt
    let txt = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "text")?;
    let txt = txt.dyn_into::<web_sys::SvgElement>()?;
    txt.set_text_content(Some("test"));
    txt.set_attribute("x", "100")?;
    txt.set_attribute("y", "100")?;
    svg.append_child(&txt)?;

    Ok(svg)
}

fn create_draw_canvas(document: &web_sys::Document) -> Result<web_sys::HtmlCanvasElement, JsValue> {
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(600);
    canvas.set_height(600);
    let ctx = canvas
        .get_context("2d")?
        .expect("canvas should get 2D ctx")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // see https://github.com/rustwasm/wasm-bindgen/blob/master/examples/paint/src/lib.rs
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        ctx.stroke_rect(
            event.offset_x() as f64 - 5.0,
            event.offset_y() as f64 - 5.0,
            10.0,
            10.0,
        );
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(canvas)
}
