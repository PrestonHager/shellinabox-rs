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
