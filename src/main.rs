use std::env;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tide::{Response, Server, StatusCode};

fn app() -> Server<()> {
    let mut app = tide::new();
    let f = |_| async { Ok(Response::new(StatusCode::NoContent)) };
    app.at("/").all(f).at("*").all(f);
    app
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ip = env::var("HOST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(80);
    let address = SocketAddr::new(ip, port);

    app().listen(address).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use tide::http::{Method, Request, Response, StatusCode};

    #[async_std::test]
    async fn get_root() -> Result<(), Box<dyn Error>> {
        let response: Response = super::app()
            .respond(Request::new(Method::Get, "https://example.com/"))
            .await?;
        assert_eq!(response.status(), StatusCode::NoContent);

        Ok(())
    }

    #[async_std::test]
    async fn get_non_root() -> Result<(), Box<dyn Error>> {
        let response: Response = super::app()
            .respond(Request::new(Method::Get, "https://example.com/generate_204"))
            .await?;
        assert_eq!(response.status(), StatusCode::NoContent);

        Ok(())
    }

    #[async_std::test]
    async fn post_root() -> Result<(), Box<dyn Error>> {
        let response: Response = super::app()
            .respond(Request::new(Method::Post, "https://example.com/"))
            .await?;
        assert_eq!(response.status(), StatusCode::NoContent);

        Ok(())
    }

    #[async_std::test]
    async fn post_non_root() -> Result<(), Box<dyn Error>> {
        let response: Response = super::app()
            .respond(Request::new(Method::Post, "https://example.com/generate_204"))
            .await?;
        assert_eq!(response.status(), StatusCode::NoContent);

        Ok(())
    }
}
