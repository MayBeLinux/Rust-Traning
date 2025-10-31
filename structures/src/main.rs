
struct api_sender {
    api_key: String,
    endpoint: String,
    timeout: u32,
    retries:u8,}


fn main() {
    let api = api_sender {
        api_key: String::from("your_api_key"),
        endpoint: String::from("https://api.example.com"),
        timeout: 30,
        retries: 3,
    };
    afficher(&api)
}

fn afficher(afficher: &api_sender) {
    println!("API Key: {}", afficher.api_key);
    println!("Endpoint: {}", afficher.endpoint);
    println!("Timeout: {}", afficher.timeout);
    println!("Retries: {}", afficher.retries);
}

