mod handler;

use warp::{Filter, Rejection, Reply};
use dotenv::dotenv;


#[tokio::main]
async fn main() {
  dotenv().ok();
  let route = warp::post()
    .and(warp::path("callback"))
    .and(warp::body::json())
    .and_then(handler::handle_callback);

  warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}