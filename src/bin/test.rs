use mcp2221::Mcp2221;

fn main() {
    let mut mcp2221 = match Mcp2221::new() {
        Ok(d) => d,
        Err(e) => { panic!("can't open device: {:?}", e); }
    };
    match mcp2221.status() {
        Ok(b) => println!("bytes: {:?}", b),
        Err(_) => panic!("can't read status")
    }
}
