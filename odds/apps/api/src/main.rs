use anyhow::Result;
use tako::Method;
use tako::responder::Responder;
use tako::router::Router;
use tokio::net::TcpListener;

async fn root_route_handler() -> impl Responder {
  "Welcome to the ODDS api!".into_response()
}

#[tokio::main]
async fn main() -> Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;

  let mut router = Router::new();
  router.route(Method::GET, "/", root_route_handler);

  tako::Server::builder()
    .build()
    .spawn_http(listener, router)
    .result()
    .await
    .expect("HTTP server failed");

  Ok(())
}
