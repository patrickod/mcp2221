use packed_struct::prelude::*;

#[derive(PrimitiveEnum, Debug, Copy, Clone, PartialEq)]
pub enum GPIODirection {
    Output = 0,
    Input = 1,
}

#[derive(PackedStruct, Clone, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct GP0Settings {
    #[packed_field(bits = "0..=2", ty = "enum")]
    mode: GP0OperatingMode,
    #[packed_field(bits = "3", ty = "enum")]
    direction: GPIODirection,
    #[packed_field(bits = "4")]
    value: Integer<u8, packed_bits::Bits1>,
}

#[derive(PrimitiveEnum, Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GP0OperatingMode {
    GPIO = 0,
    SSPND = 1,
    LED_UART_RX = 2,
}

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct GP1Settings {
    #[packed_field(bits = "0..=2", ty = "enum")]
    mode: GP1OperatingMode,
    #[packed_field(bits = "3", ty = "enum")]
    direction: GPIODirection,
    #[packed_field(bits = "4")]
    value: Integer<u8, packed_bits::Bits1>,
}
#[derive(PrimitiveEnum, Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GP1OperatingMode {
    GPIO = 0,
    CLK = 1,
    ADC1 = 2,
    LED_UART_TX = 3,
    INT = 4,
}

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct GP2Settings {
    #[packed_field(bits = "0..=2", ty = "enum")]
    mode: GP2OperatingMode,
    #[packed_field(bits = "3", ty = "enum")]
    direction: GPIODirection,
    #[packed_field(bits = "4")]
    value: Integer<u8, packed_bits::Bits1>,
}

#[derive(PrimitiveEnum, Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GP2OperatingMode {
    GPIO = 0,
    USBCFG = 1,
    ADC2 = 2,
    DAC1 = 3,
}

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct GP3Settings {
    #[packed_field(bits = "0..=2", ty = "enum")]
    mode: GP3OperatingMode,
    #[packed_field(bits = "3", ty = "enum")]
    direction: GPIODirection,
    #[packed_field(bits = "4")]
    value: Integer<u8, packed_bits::Bits1>,
}

#[derive(PrimitiveEnum, Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GP3OperatingMode {
    GPIO = 0,
    LED_I2C = 1,
    ADC3 = 2,
    DAC2 = 3,
}

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "4")]
pub struct GPSettings {
    #[packed_field(element_size_bytes = "1", bytes = "0")]
    gp0: GP0Settings,
    #[packed_field(element_size_bytes = "1", bytes = "1")]
    gp1: GP1Settings,
    #[packed_field(element_size_bytes = "1", bytes = "2")]
    gp2: GP2Settings,
    #[packed_field(element_size_bytes = "1", bytes = "3")]
    gp3: GP3Settings,
}
