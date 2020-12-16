pub const VENDOR_ID: u16 = 0x04d8;
pub const PRODUCT_ID: u16 = 0x00dd;


pub enum ClockDuty {
    ClockDuty0  = 0b00,
    ClockDuty25 = 0b01,
    ClockDuty50 = 0b10,
    ClockDuty75 = 0b11,
}

pub enum ClockDivider {
    ClockDivider2   = 0b000, // 24Mhz
    ClockDivider4   = 0b001, // 12Mhz
    ClockDivider8   = 0b010, // 6Mhz
    ClockDivider16  = 0b011, // 3Mhz
    ClockDivider32  = 0b100, // 1.5Mhz
    ClockDivider64  = 0b101, // 750Khz
    ClockDivider128 = 0b111, // 375Khz
}

pub enum DACVRM {
    VRM0    = 0b00, // Disabled
    VRM1024 = 0b01, // 1.024V
    VRM2048 = 0b10, // 2.048V
    VRM4096 = 0b11, // 4.096V
}

pub enum DACRef {
    VDD,
    VRM
}
pub enum ADCVRM {
    VRM0    = 0b00, // Disabled
    VRM1024 = 0b01, // 1.024V
    VRM2048 = 0b10, // 2.048V
    VRM4096 = 0b11, // 4.096V
}

pub enum ADCRef {
    VDD,
    VRM
}

pub enum Command {
    SetParameters = 0x10,
    ReadFlashData = 0xb0,
    WriteFlashData = 0xb1,
    SendFlashAcessPassword = 0xb2,

    I2CWriteData = 0x90,
    I2CWriteDataRepeatedStart = 0x92,
    I2CWriteDataNoStop = 0x94,

    I2CReadData = 0x91,
    I2CReadDataRepeatedStart = 0x93,

    I2CReadDataGetI2CData = 0x40,

    SetGPIOOutputValues = 0x50,
    GetGPIOOutputValues = 0x51,

    SetSRAMSettings = 0x60,
    GetSRAMSettings = 0x61,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
