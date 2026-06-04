use stm32f1xx_hal::gpio::{OpenDrain, Output, Pin};

pub struct SoftI2c {
    scl: Pin<'A', 2, Output<OpenDrain>>,
    sda: Pin<'A', 3, Output<OpenDrain>>,
}

impl SoftI2c {
    pub fn new(
        scl: Pin<'A', 2, Output<OpenDrain>>,
        sda: Pin<'A', 3, Output<OpenDrain>>,
    ) -> Self {
        Self { scl, sda }
    }

    #[inline]
    fn delay() {
        cortex_m::asm::delay(360);
    }

    pub fn start(&mut self) {
        self.sda.set_high();
        Self::delay();
        self.scl.set_high();
        Self::delay();
        self.sda.set_low();
        Self::delay();
        self.scl.set_low();
        Self::delay();
    }

    pub fn stop(&mut self) {
        self.sda.set_low();
        Self::delay();
        self.scl.set_high();
        Self::delay();
        self.sda.set_high();
        Self::delay();
    }

    pub fn write_byte(&mut self, byte: u8) -> bool {
        for i in (0..8).rev() {
            if byte & (1 << i) != 0 {
                self.sda.set_high();
            } else {
                self.sda.set_low();
            }
            Self::delay();
            self.scl.set_high();
            Self::delay();
            self.scl.set_low();
            Self::delay();
        }
        self.sda.set_high();
        Self::delay();
        self.scl.set_high();
        Self::delay();
        let ack = self.sda.is_low();
        self.scl.set_low();
        Self::delay();
        ack
    }

    pub fn read_byte(&mut self, ack: bool) -> u8 {
        let mut val = 0u8;
        self.sda.set_high();
        for _ in 0..8 {
            val <<= 1;
            self.scl.set_high();
            Self::delay();
            if self.sda.is_high() {
                val |= 1;
            }
            self.scl.set_low();
            Self::delay();
        }
        if ack {
            self.sda.set_low();
        } else {
            self.sda.set_high();
        }
        Self::delay();
        self.scl.set_high();
        Self::delay();
        self.scl.set_low();
        Self::delay();
        self.sda.set_high();
        val
    }
}
