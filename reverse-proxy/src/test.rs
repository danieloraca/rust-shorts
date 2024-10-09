use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use tokio;

#[tokio::main]
async fn main() {
    // Create an HTTPS connector
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // The URI
    let uri: Uri = "http://blah-blah.net".parse().unwrap();

    // Make the request
    match client.get(uri).await {
        Ok(res) => {
            println!("Response: {}", res.status());
        }
        Err(err) => {
            println!("Error making request: {:?}", err);
        }
    }
}
