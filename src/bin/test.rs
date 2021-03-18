use packed_struct::prelude::*;

use mcp2221::types::GPSettings;
use mcp2221::Mcp2221;

fn main() {
    let mut mcp2221 = match Mcp2221::new() {
        Ok(d) => d,
        Err(e) => {
            panic!("can't open device: {:?}", e);
        }
    };
    let status = match mcp2221.status() {
        Ok(b) => b,
        Err(_) => panic!("can't read status"),
    };

    let settings = match GPSettings::unpack_from_slice(&status[8..12]) {
        Ok(s) => s,
        Err(e) => panic!("unable to unpack settings: {:?}", e),
    };

    println!("settings: {:?}", settings);
}
