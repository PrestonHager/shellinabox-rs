use shellinabox::server::routes;
use warp::http::StatusCode;

#[tokio::test]
async fn test_get_index() {
    let filter = routes("static");
    let resp = warp::test::request()
        .method("GET")
        .path("/")
        .reply(&filter)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_ws_handshake() {
    let filter = routes("static");
    let result = warp::test::ws()
        .path("/ws")
        .handshake(filter.clone())
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_static_file() {
    use std::fs;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let pkg_dir = dir.path().join("pkg");
    fs::create_dir(&pkg_dir).unwrap();
    let js_path = pkg_dir.join("foo.js");
    fs::write(&js_path, "console.log('hi');").unwrap();

    let filter = routes(dir.path().to_str().unwrap());
    let resp = warp::test::request()
        .method("GET")
        .path("/pkg/foo.js")
        .reply(&filter)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}
