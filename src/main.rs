use shellinabox::server::routes;

#[tokio::main]
async fn main() {
    let routes = routes("static");
    warp::serve(routes).run(([127,0,0,1], 3000)).await;
}
