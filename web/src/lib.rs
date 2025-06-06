use wasm_bindgen::prelude::*;
use web_sys::{Document, Window, WebSocket, MessageEvent, HtmlElement};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window: Window = web_sys::window().expect("no global `window`");
    let document: Document = window.document().expect("should have a document");
    let body: HtmlElement = document.body().expect("document should have a body");
    let ws = WebSocket::new("ws://localhost:3000/ws")?;
    let pre = document.create_element("pre")?;
    body.append_child(&pre)?;
    let pre = pre.dyn_into::<HtmlElement>()?;
    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            let text = String::from_utf8_lossy(&array.to_vec()).into_owned();
            let _ = pre.insert_adjacent_text("beforeend", &text);
        }
    });
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    Ok(())
}
