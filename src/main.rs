#[cfg(not(feature = "nodotenv"))]
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::http::StatusCode;
use warp::{Filter, Reply};

fn generate_204() -> impl Filter<Extract = (impl Reply,)> + Copy {
    warp::get()
        .and(warp::path("generate_204"))
        .map(warp::reply)
        .map(|reply| warp::reply::with_status(reply, StatusCode::NO_CONTENT))
}

#[tokio::main]
async fn main() {
    let filter = generate_204();

    #[cfg(not(feature = "nodotenv"))]
    dotenv().ok();

    let ip = env::var("HOST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(80);
    let address = SocketAddr::new(ip, port);

    warp::serve(filter).run(address).await;
}

#[tokio::test]
async fn test() {
    let filter = generate_204();
    let reply = warp::test::request()
        .method("GET")
        .path("/generate_204")
        .reply(&filter)
        .await;
    assert_eq!(reply.status(), 204);
}
