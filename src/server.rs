use std::convert::Infallible;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use futures::{StreamExt, SinkExt};
use warp::Filter;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};

pub fn routes(static_dir: &str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let static_dir = static_dir.to_string();
    let index = warp::path::end().and(warp::fs::file(format!("{}/index.html", static_dir)));
    let ws_route = warp::path("ws").and(warp::ws()).and_then(ws_handler);
    index.or(ws_route)
}

async fn ws_handler(ws: warp::ws::Ws) -> Result<impl warp::Reply, Infallible> {
    Ok(ws.on_upgrade(handle_socket))
}

async fn handle_socket(websocket: warp::ws::WebSocket) {
    let pty_system = NativePtySystem::default();
    let pair = match pty_system.openpty(PtySize { rows: 24, cols: 80, pixel_width: 0, pixel_height: 0 }) {
        Ok(p) => p,
        Err(_) => return,
    };
    let mut cmd = CommandBuilder::new("/bin/sh");
    cmd.env("TERM", "xterm-256color");
    let mut child = match pair.slave.spawn_command(cmd) {
        Ok(c) => c,
        Err(_) => return,
    };
    let mut reader = pair.master.try_clone_reader().unwrap();
    let writer = Arc::new(Mutex::new(pair.master.take_writer().unwrap()));
    let (mut ws_tx, mut ws_rx) = websocket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

    std::thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if tx.send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
            }
        }
    });

    loop {
        tokio::select! {
            Some(data) = rx.recv() => {
                if ws_tx.send(warp::ws::Message::binary(data)).await.is_err() {
                    break;
                }
            }
            msg = ws_rx.next() => {
                match msg {
                    Some(Ok(m)) => {
                        let data = m.into_bytes();
                        let writer = writer.clone();
                        tokio::task::spawn_blocking(move || {
                            let mut w = writer.lock().unwrap();
                            let _ = w.write_all(&data);
                        }).await.ok();
                    }
                    _ => break,
                }
            }
        }
    }
    let _ = child.kill();
}
