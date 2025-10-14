
struct api_sender {
    api_key: String,
    endpoint: String,
    timeout: u32,
    retries:u8,}


fn main(value: String , number: i32 , flag: bool) -> api_sender {
    let api_sender = api_sender {
        api_key: String::from("your_api_key"),
        endpoint: String::from("https://api.example.com"),
        timeout: 30,
        retries: 3,
    };
}
