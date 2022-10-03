fn main() {
    let api_token = std::env::var("API_TOKEN")
        .expect("expected there to be an api token");

    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    
    let args: String = arg_iterator.collect();

    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", api_token), ("keyword", args)])
        .header(reqwest::header::USER_AGENT, "my cli weather app")
        .send()
        .expect("a successfull request")
        .json::<serde_json::Value>()
        .expect("expected the body to be json");

    dbg!(response);

}
