use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use std::net::SocketAddr;

// Function to forward the request to another server (with HTTPS)
async fn forward_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let uri_str = format!(
        "https://test.com{}",
        req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
    );
    let uri = uri_str.parse::<Uri>().unwrap();

    // Create an HTTPS connector
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Forward the request
    let mut new_req = Request::builder()
        .method(req.method())
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    // Forward headers
    *new_req.headers_mut() = req.headers().clone();

    // Send request and return the response
    client.request(new_req).await
}

#[tokio::main]
async fn main() {
    // Address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));

    // Create the service function to handle requests
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(forward_request)) });

    // Build and run the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
