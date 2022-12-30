use cookie::Cookie;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Request, Response, Server, StatusCode};
use std::net::IpAddr;
use std::{convert::Infallible, net::SocketAddr};

fn debug_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

fn unauthenticated(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    return Ok(Response::builder()
        .status(302)
        .header("Location", "/trustorg")
        .body(Body::empty())
        .unwrap());
}

async fn handle_proxy(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri().path().starts_with("/") {
        // will forward requests to port 13902
        match hyper_reverse_proxy::call(client_ip, "http://127.0.0.1:3000", req).await {
            Ok(response) => Ok(response),
            Err(_error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()),
        }
    } else {
        debug_request(req)
    }
}

fn has_valid_auth_cookie(header: &HeaderMap) -> bool {
    let cookies = header.get("cookie");
    match cookies {
        Some(value) => {
            let parsed_cookie = Cookie::parse(value.to_str().unwrap().to_string());
            match parsed_cookie {
                Ok(parsed_value) => {
                    let named = parsed_value.name_value();
                    match named {
                        ("test", _) => true,
                        _ => {
                            println!("no key found!");
                            return false;
                        }
                    }
                }
                _ => {
                    println!("parse failed!");
                    return false;
                }
            }
        }
        None => {
            println!("no cookies!");
            return false;
        }
    }
}

async fn handle(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri().path().starts_with("/trustorg") {
        // will forward requests to port 13901
        match hyper_reverse_proxy::call(client_ip, "http://127.0.0.1:8070", req).await {
            Ok(response) => Ok(response),
            Err(_error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()),
        }
    } else {
        let headers = req.headers();
        let is_auth = has_valid_auth_cookie(&headers);
        if is_auth {
            return handle_proxy(client_ip, req).await;
        }
        return unauthenticated(req);
    }
}

#[tokio::main]
pub async fn start() {
    let bind_addr = "127.0.0.1:8000";
    let addr: SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");

    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle(remote_addr, req))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("proxy server http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
