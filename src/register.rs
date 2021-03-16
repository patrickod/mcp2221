use bitflags::bitflags;

bitflags! {
    pub struct CHIP_SETTING0Bits: u8 {
        const CDCSNEN = 0b1000_0000;
        const CHIP_PROTECT_UNPROTECTED = 0b0000_0000;
        const CHIP_PROTECT_PASSWORD_PROTECTED = 0b0000_0001;
        const CHIP_PROTECT_PERMANENTLY_LOCKED = 0b0000_0010;
        const CHIP_PROTECT_RESERVED = 0b0000_0011;
    }

    pub struct CHIP_SETTINGS1Bits: u8 {
        /// Clock-Out duty cycle (%)
        const CLKDC0 = 0b0000_0000;
        const CLKDC25 = 0b0000_1000;
        const CLKDC50 = 0b0001_0000;
        const CLKDC75 = 0b0001_1000;

        /// Clock-Out frequency (Hz)
        const CLKDIV375_KHZ = 0b0000_0111;
        const CLKDIV750_KHZ = 0b0000_0110;
        const CLKDIV1_5_MHZ = 0b0000_0101;
        const CLKDIV3_MHZ   = 0b0000_0100;
        const CLKDIV6_MHZ   = 0b0000_0011;
        const CLKDIV12_MHZ  = 0b0000_0010;
        const CLKDIV24_MHZ  = 0b0000_0001;
    }

    pub struct CHIP_SETTINGS2Bits: u8 {
        /// DAC Internal Voltage Reference (DAC VRM) configuration
        const DACVRM_4V       = 0b1100_0000;
        const DACVRM_2V       = 0b1000_0000;
        const DACVRM_1V       = 0b0100_0000;
        const DACVRM_DISABLED = 0b0000_0000;
    }
}
