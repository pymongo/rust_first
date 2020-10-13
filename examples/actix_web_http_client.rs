use actix_web::client::Client;

const ECHO_SERVER: &str = "http://postman-echo.com/post";

#[derive(serde::Serialize, serde::Deserialize)]
struct Form {
    user_id: u32,
}

#[actix_web::main]
async fn main() {
    http_post_echo_server().await;
}

async fn http_post_echo_server() {
    let form = Form { user_id: 1 };
    let client = Client::default();
    let response = client.post(ECHO_SERVER).send_form(&form).await;
    let resp_body_bytes = response.unwrap().body().await.unwrap();
    let resp_string = std::str::from_utf8(&resp_body_bytes).unwrap();
    println!("response = {}", resp_string);
    let resp_json: serde_json::Value = serde_json::from_str(resp_string).unwrap();
    println!(
        "response_in_json = {response}",
        response = serde_json::to_string_pretty(&resp_json).unwrap()
    );
}

/**
http_client make a https(SSL) request
openssl = "*" # for examples/http_client https example
actix-web = { version = "*", features = ["openssl"] }
*/
#[cfg(not)]
async fn https_ssl_request() {
    const JSON_DATA_URL: &str = "http://jsonplaceholder.typicode.com/posts/1";
    use actix_web::client::Connector;
    use openssl::ssl::{SslConnector, SslMethod};

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::build()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let response = client
        .get(JSON_DATA_URL.replace("http", "https"))
        .send()
        .await;
    let resp_body_bytes = response.unwrap().body().await.unwrap();
    let resp_string = std::str::from_utf8(&resp_body_bytes).unwrap();
    println!("response = {}", resp_string);
    let resp_json: serde_json::Value = serde_json::from_str(resp_string).unwrap();
    println!(
        "response_in_json = {response}",
        response = serde_json::to_string_pretty(&resp_json).unwrap()
    );
}
