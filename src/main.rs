use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let proxy_addr = ([127, 0, 0, 1], 3000).into();
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });
    let server = Server::bind(&proxy_addr).serve(make_service);

    println!("Reverse proxy listening on http://{}", proxy_addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri_string = format!(
        "http://127.0.0.1:9001{}",
        req.uri()
            .path_and_query()
            .map(|x| x.as_str())
            .unwrap_or("/")
    );

    let uri = match uri_string.parse::<Uri>() {
        Ok(uri) => uri,
        Err(err) => {
            eprintln!("Error parsing URI: {}", err);
            return Ok(Response::new(Body::from("Bad Request")));
        }
    };

    let mut new_req = Request::builder().method(req.method()).uri(uri);

    for (key, value) in req.headers() {
        new_req = new_req.header(key, value);
    }

    let new_req = match new_req.body(req.into_body()) {
        Ok(req) => req,
        Err(err) => {
            eprintln!("Error building request: {}", err);
            return Ok(Response::new(Body::from("Internal Server Error")));
        }
    };

    match client.request(new_req).await {
        Ok(res) => Ok(res),
        Err(err) => {
            eprintln!("Error forwarding request: {}", err);
            Ok(Response::new(Body::from("Internal Server Error")))
        }
    }
}
