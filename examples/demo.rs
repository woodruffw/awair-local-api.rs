use std::env;

fn main() {
    // Use: demo <base url>
    // Example: demo 'http://192.168.1.10'
    let api_base = env::args().nth(1).unwrap();
    let client = awair_local_api::Awair::new(&api_base).unwrap();

    println!("{:#?}", client.config().unwrap());
    println!("{:#?}", client.poll().unwrap());
}
