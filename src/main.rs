extern crate reqwest;

use std::io::Read;


fn request(){
    let mut response = reqwest::get("https://httpbin.org/status/418")
        .expect("Failed to send request");
    println!("{}", response.status());
    for header in response.headers().iter() {
        println!("{}: {}", header.name(), header.value_string());
    }

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);
}

fn main(){
    request()
}
