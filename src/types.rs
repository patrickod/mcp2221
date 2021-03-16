use packed_struct::prelude::*;

#[derive(PrimitiveEnum_u8, Clone, Copy, Debup, PartialEq)]
pub enum GPIODirection {
    Output = 0,
    Input = 1,
}

#[derive(PackedStruct)]
#[packed_struct(bit_numbering = "lsb0")]
pub struct GP0Settings {
    #[packed_field(bits = "0..=2")]
    mode: GP0OperatingMode,
    #[packed_field(bits = "3")]
    direction: GPIODirection,
    value: Integer<u8, packed_bits::Bits1>,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GP0OperatingMode {
    GPIO = 0,
    SSPND = 1,
    LED_UART_RX = 2,
}

#[derive(PackedStruct)]
#[packed_struct(bit_numbering = "lsb0")]
pub struct GP1Settings {
    #[packed_field(bits = "0..=2")]
    mode: GP1OperatingMode,
    #[packed_field(bits = "3")]
    direction: GPIODirection,
    value: Integer<u8, packed_bits::Bits1>,
}
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GP1OperatingMode {
    GPIO = 0,
    CLK = 1,
    ADC1 = 2,
    LED_UART_TX = 3,
    INT = 4,
}

#[derive(PackedStruct)]
#[packed_struct(bit_numbering = "lsb0")]
pub struct GP2Settings {
    #[packed_field(bits = "0..=2")]
    mode: GP2OperatingMode,
    #[packed_field(bits = "3")]
    direction: GPIODirection,
    value: Integer<u8, packed_bits::Bits1>,
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GP2OperatingMode {
    GPIO = 0,
    USBCFG = 1,
    ADC2 = 2,
    DAC1 = 3,
}

#[derive(PackedStruct)]
#[packed_struct(bit_numbering = "lsb0")]
pub struct GP3Settings {
    #[packed_field(bits = "0..=2")]
    mode: GP3OperatingMode,
    #[packed_field(bits = "3")]
    direction: GPIODirection,
    value: Integer<u8, packed_bits::Bits1>,
}
