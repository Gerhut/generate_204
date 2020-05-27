use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
#[cfg(not(feature = "nodotenv"))]
use dotenv::dotenv;
use warp::Filter;

fn generate_204() -> impl Filter<Extract = (impl warp::Reply,)> + Copy {
    warp::get()
        .and(warp::path("generate_204"))
        .map(warp::reply)
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NO_CONTENT))
}

#[tokio::main]
async fn main() {
    let filter = generate_204();

    #[cfg(not(feature = "nodotenv"))]
    dotenv().ok();

    let ip = env::var("HOST").map(|s| s.parse().expect("Parsing HOST"))
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let port = env::var("PORT").map(|s| s.parse().expect("Parsing PORT"))
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
