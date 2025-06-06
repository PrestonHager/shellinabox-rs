use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebSocket, MessageEvent};
use sugarloaf::context::{Context, SugarloafRenderer, SugarloafWindowSize};
use sugarloaf::Sugarloaf;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub async fn start(canvas: HtmlCanvasElement, ws: WebSocket) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Initialize wgpu on the provided canvas
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface_from_canvas(&canvas)? };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .ok_or("No adapter found")?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await?;

    let format = surface.get_capabilities(&adapter).formats[0];
    let size = SugarloafWindowSize::new(canvas.width(), canvas.height());
    let context = Context::new(
        &device,
        surface,
        queue,
        format,
        size,
        1.0,
        adapter.get_info(),
    );
    let sugarloaf = Rc::new(RefCell::new(Sugarloaf::new(SugarloafRenderer::default())));

    let sl_clone = sugarloaf.clone();
    let ctx_clone = context.clone();
    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            let text = String::from_utf8_lossy(&array.to_vec());
            sl_clone.borrow_mut().render_line(&ctx_clone, &text);
        }
    });
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    Ok(())
}
