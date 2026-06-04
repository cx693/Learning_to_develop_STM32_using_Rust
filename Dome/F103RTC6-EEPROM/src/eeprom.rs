use crate::i2c::SoftI2c;

const EEPROM_DEV_ADDR: u8 = 0xA0;
const EEPROM_PAGE_SIZE: usize = 8;
pub const EEPROM_SIZE: usize = 256;

pub struct Partition {
    pub offset: u8,
    pub size: u16,
}

impl Partition {
    pub const fn new(offset: u8, size: u16) -> Self {
        Self { offset, size }
    }

    pub fn max_len(&self, data_len: usize) -> usize {
        core::cmp::min(data_len, self.size as usize)
    }
}

pub struct Eeprom {
    i2c: SoftI2c,
}

impl Eeprom {
    pub fn new(i2c: SoftI2c) -> Self {
        Self { i2c }
    }

    pub fn check_device(&mut self) -> bool {
        self.i2c.start();
        let ack = self.i2c.write_byte(EEPROM_DEV_ADDR);
        self.i2c.stop();
        ack
    }

    pub fn wait_standby(&mut self) -> bool {
        for _ in 0..1000 {
            if self.check_device() {
                return true;
            }
        }
        false
    }

    pub fn write_bytes(&mut self, addr: u8, data: &[u8]) -> bool {
        let max_len = EEPROM_SIZE.saturating_sub(addr as usize);
        let write_len = core::cmp::min(data.len(), max_len);
        let mut written = 0usize;
        while written < write_len {
            let abs_addr = addr as usize + written;
            let page_offset = abs_addr % EEPROM_PAGE_SIZE;
            let page_remaining = EEPROM_PAGE_SIZE - page_offset;
            let chunk = core::cmp::min(page_remaining, write_len - written);

            if written > 0 && !self.wait_standby() {
                return false;
            }

            self.i2c.start();
            if !self.i2c.write_byte(EEPROM_DEV_ADDR) {
                self.i2c.stop();
                return false;
            }
            if !self.i2c.write_byte(addr.wrapping_add(written as u8)) {
                self.i2c.stop();
                return false;
            }
            for i in 0..chunk {
                if !self.i2c.write_byte(data[written + i]) {
                    self.i2c.stop();
                    return false;
                }
            }
            self.i2c.stop();
            written += chunk;
        }
        self.wait_standby()
    }

    pub fn read_bytes(&mut self, addr: u8, buf: &mut [u8]) -> bool {
        let max_len = EEPROM_SIZE.saturating_sub(addr as usize);
        let read_len = core::cmp::min(buf.len(), max_len);
        if read_len == 0 {
            return false;
        }
        self.i2c.start();
        if !self.i2c.write_byte(EEPROM_DEV_ADDR) {
            self.i2c.stop();
            return false;
        }
        if !self.i2c.write_byte(addr) {
            self.i2c.stop();
            return false;
        }
        self.i2c.start();
        if !self.i2c.write_byte(EEPROM_DEV_ADDR | 1) {
            self.i2c.stop();
            return false;
        }
        for i in 0..read_len {
            let ack = i < read_len - 1;
            buf[i] = self.i2c.read_byte(ack);
        }
        self.i2c.stop();
        true
    }

    pub fn write_partition(&mut self, part: &Partition, data: &[u8]) -> (bool, usize) {
        let len = part.max_len(data.len());
        if !self.write_bytes(part.offset, &data[..len]) {
            return (false, len);
        }
        let mut addr = part.offset.wrapping_add(len as u8);
        let mut remaining = part.size as usize - len;
        while remaining > 0 {
            let chunk = core::cmp::min(remaining, EEPROM_PAGE_SIZE);
            let zeros = [0x00u8; EEPROM_PAGE_SIZE];
            if !self.write_bytes(addr, &zeros[..chunk]) {
                return (false, len);
            }
            addr = addr.wrapping_add(chunk as u8);
            remaining -= chunk;
        }
        (true, len)
    }

    pub fn read_partition(&mut self, part: &Partition, buf: &mut [u8]) -> (bool, usize) {
        let len = core::cmp::min(buf.len(), part.size as usize);
        (self.read_bytes(part.offset, &mut buf[..len]), len)
    }
}
