extern crate reqwest;

use std::io::copy;


fn request(){
    let mut response = reqwest::get("https://httpbin.org/status/418")
        .expect("Failed to send request");
    println!("{}", response.status());
    for header in response.headers().iter() {
        println!("{}: {}", header.name(), header.value_string());
    }

    copy(&mut response, &mut std::io::stdout()).expect("Failed to read response");
}

fn main(){
    request()
}
