#[stem::main]
fn main(_arg: usize) -> ! {
    stem::println!("Fetching cnn...");
    match http::HttpClient::get("https://www.cnn.com") {
        Ok(mut resp) => {
            stem::println!("Got response!");
            loop {
                match resp.read_chunk() {
                    Ok(chunk) => {
                        stem::println!("Chunk size: {}", chunk.len());
                        if chunk.is_empty() { break; }
                        stem::println!("Data: {:?}", core::str::from_utf8(&chunk[0..chunk.len().min(100)]));
                    }
                    Err(e) => {
                        stem::println!("Error: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            stem::println!("Request failed: {}", e);
        }
    }
    stem::syscall::exit(0)
}
