use wasm_bindgen::prelude::*;
use web_sys::{Document, Window, WebSocket, MessageEvent, HtmlElement, HtmlCanvasElement};
use sugarloaf::{Sugarloaf, SugarloafRenderer, SugarloafWindow, SugarloafWindowSize, layout::RootStyle, font::{FontLibrary, fonts::SugarloafFont}};
use raw_window_handle::{WebHandle, WebDisplayHandle};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window: Window = web_sys::window().expect("no global `window`");
    let document: Document = window.document().expect("should have a document");
    let body: HtmlElement = document.body().expect("document should have a body");
    let ws = WebSocket::new("ws://localhost:3000/ws")?;
    let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
    body.append_child(&canvas)?;

    let window_handle = WebHandle::from_canvas(canvas.clone());
    let display_handle = WebDisplayHandle::new();
    let sl_window = SugarloafWindow {
        handle: window_handle.into(),
        display: display_handle.into(),
        size: SugarloafWindowSize { width: canvas.width() as f32, height: canvas.height() as f32 },
        scale: window.device_pixel_ratio(),
    };
    let font_library = FontLibrary::new(vec![SugarloafFont::default()]);
    let mut sugar = Sugarloaf::new(sl_window, SugarloafRenderer::default(), &font_library, RootStyle::default()).unwrap();

    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            let text = String::from_utf8_lossy(&array.to_vec()).into_owned();
            sugar.content().push_str(0, &text);
        }
    });
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    Ok(())
}
