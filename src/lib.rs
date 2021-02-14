#[macro_use]
extern crate bitflags;

pub const VENDOR_ID: u16 = 0x04d8;
pub const PRODUCT_ID: u16 = 0x00dd;

pub struct Mcp2221 {}

type GPIOConfig = u8;
type Response = Option<[u8; 64]>;

impl Mcp2221 {
    fn new() -> Mcp2221 {
        Mcp2221{}
    }

    fn reset(&mut self) {
        unimplemented!()
    }

    fn gpio_read(&self) -> Response {
        unimplemented!()
    }

    fn gpio_write(&self) -> Response {
        unimplemented!()
    }

    fn gp_set_mode(&mut self) -> Response {
        unimplemented!()
    }

    fn gp_set_direction(&mut self) -> Response {
        unimplemented!()
    }

    fn i2c_write(&self) -> Response {
        unimplemented!()
    }

    fn i2c_read(&self) -> Response {
        unimplemented!()
    }

    fn i2c_write_read(&self) -> Response {
        unimplemented!()
    }

    fn hid_transfer(&mut self) -> Response {
        unimplemented!()
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
