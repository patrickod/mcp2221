extern crate hidapi;
extern crate num_enum;
extern crate packed_struct;

mod types;

use std::iter;
use types::*;

use hidapi::{HidApi, HidDevice};
use num_enum::IntoPrimitive;

pub const VENDOR_ID: u16 = 0x04d8;
pub const PRODUCT_ID: u16 = 0x00dd;

pub struct Mcp2221 {
    device: HidDevice,
}

pub enum GPPin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
}

enum WriteFlashDataSubCode {
    WriteChipSettings = 0x00,
    WriteGpSettings = 0x01
}

#[derive(Debug)]
pub enum Error {
    HidUnavailable,
    DeviceNotFound,
    DeviceInaccessible,
    DeviceUnexpectedError,
}

type Response = [u8; 64];

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(IntoPrimitive)]
pub enum Command {
    // Misc commands
    STATUS = 0x10,
    READ_FLASH_DATA = 0xb0,
    WRITE_FLASH_DATA = 0xb1,

    // IC2 transfer commands
    I2C_WRITE_DATA = 0x90,
    I2C_WRITE_DATA_REPEATED_START = 0x92,
    I2C_WRITE_DATA_NO_STOP = 0x94,
    I2C_READ_DATA_COMMAND = 0x91,
    I2C_READ_DATA_COMMAND_REPEATED_START = 0x93,
    I2C_READ_DATA_GET_DATA = 0x40,

    RESET = 0x70,

}

impl Mcp2221 {
    pub fn new() -> Result<Mcp2221, Error> {
        let hidapi = match HidApi::new() {
            Ok(api) => api,
            Err(_) => {
                return Err(Error::HidUnavailable);
            }
        };

        for device in hidapi.device_list() {
            if device.vendor_id() == VENDOR_ID && device.product_id() == PRODUCT_ID {
                match device.open_device(&hidapi) {
                    Ok(device) => return Ok(Mcp2221 { device }),
                    Err(_) => return Err(Error::DeviceInaccessible),
                };
            }
        }
        Err(Error::DeviceNotFound)
    }

    fn _hid_write(&mut self, payload: &[u8]) -> Result<(), Error> {
        let mut report = vec![0u8];
        report.extend_from_slice(payload);
        report.extend(iter::repeat(0u8).take(64 - payload.len()));

        match self.device.write(&report) {
            Ok(written) => {
                if written != report.len() {
                    return Err(Error::DeviceUnexpectedError);
                } else {
                    Ok(())
                }
            }
            Err(_) => return Err(Error::DeviceUnexpectedError),
        }
    }

    fn _hid_read(&mut self, response: &mut Vec<u8>) -> Result<usize, Error> {
        let mut buffer = vec![0u8; 64];
        match self.device.read(&mut buffer) {
            Ok(count) => {
                response.extend_from_slice(&buffer);
                Ok(count)
            }
            Err(_) => Err(Error::DeviceUnexpectedError),
        }
    }

    pub fn reset(&mut self) -> Result<(), Error> {
         self._hid_write(&[Command::RESET.into()])
    }

    pub fn status(&mut self) -> Result<Vec<u8>, Error> {
        self._hid_write(&[Command::STATUS.into()])?;
        let mut buffer: Vec<u8> = Vec::new();
        match self._hid_read(&mut buffer) {
            Ok(_) => Ok(buffer.to_owned()),
            Err(_) => Err(Error::DeviceUnexpectedError)
        }
    }

    pub fn gp0_set_mode(&mut self mode: GP0OperatingMode) -> Response {
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
