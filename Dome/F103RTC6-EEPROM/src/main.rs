#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

mod i2c;
mod eeprom;

use i2c::SoftI2c;
use eeprom::{Eeprom, Partition, EEPROM_SIZE};

// ==================== 模式选择宏定义 ====================
// 修改这个值来选择工作模式（类似 C 的 #define）：
//   1 = 读 + 写 (Read + Write)
//   2 = 仅写   (Write Only)
//   3 = 仅读   (Read Only)
const EEPROM_MODE: u8 = 3;

const _: () = assert!(EEPROM_MODE >= 1 && EEPROM_MODE <= 3, "EEPROM_MODE 必须是 1/2/3");

const MODE_NAME: &str = if EEPROM_MODE == 1 {
    "Read + Write (读写)"
} else if EEPROM_MODE == 2 {
    "Write Only (仅写)"
} else {
    "Read Only (仅读)"
};

// ==================== EEPROM 分区 ====================
const PART_POEM: Partition = Partition::new(0x00, EEPROM_SIZE as u16);

// ==================== 古诗数据 ====================
const LYRIC: &[u8] = "
《静夜思》李白
床前明月光，疑是地上霜。
举头望明月，低头思故乡。

《春晓》孟浩然
春眠不觉晓，处处闻啼鸟。
夜来风雨声，花落知多少。
Rust STM32 CXi
".as_bytes();

fn safe_str(bytes: &[u8]) -> &str {
    let mut end = bytes.len();
    while end > 0 && (bytes[end - 1] == 0xFF || bytes[end - 1] == 0x00) {
        end -= 1;
    }
    match core::str::from_utf8(&bytes[..end]) {
        Ok(s) => s,
        Err(e) => {
            let n = e.valid_up_to();
            if n > 0 { core::str::from_utf8(&bytes[..n]).unwrap_or("") }
            else { "" }
        }
    }
}

// ==================== 状态机定义 ====================
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Idle,
    Write,
    WriteWait,
    Read,
    Verify,
    Done,
    Error,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("==============================");
    rprintln!("  EEPROM 古诗读写实验 (状态机)");
    rprintln!("==============================");

    let mut dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut gpioa = dp.GPIOA.split(&mut dp.RCC);
    let scl = gpioa.pa2.into_open_drain_output(&mut gpioa.crl);
    let sda = gpioa.pa3.into_open_drain_output(&mut gpioa.crl);

    let _clocks = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let i2c_bus = SoftI2c::new(scl, sda);
    let mut eeprom = Eeprom::new(i2c_bus);

    if !eeprom.check_device() {
        rprintln!("[ERROR] EEPROM 未检测到!");
        loop {}
    }
    rprintln!("[OK] EEPROM 已连接 ({} 字节)", EEPROM_SIZE);

    let data_len = PART_POEM.max_len(LYRIC.len());
    if LYRIC.len() > data_len {
        rprintln!("[WARN] 数据 {} 字节, 截断至 {} 字节", LYRIC.len(), data_len);
    }

    let mut state = State::Idle;
    let mut read_buf = [0u8; EEPROM_SIZE];

    loop {
        match state {
            State::Idle => {
                rprintln!("[状态机] 模式: {} (MODE={})", MODE_NAME, EEPROM_MODE);
                rprintln!("----------------------------");
                rprintln!("待写入古诗 ({} 字节):", data_len);
                if let Ok(s) = core::str::from_utf8(&LYRIC[..data_len]) {
                    rprintln!("{}", s);
                }
                rprintln!("----------------------------");

                match EEPROM_MODE {
                    1 | 2 => state = State::Write,
                    3 => state = State::Read,
                    _ => state = State::Error,
                }
            }

            State::Write => {
                rprintln!("[写入] @0x00 ({} 字节)...", data_len);
                let (ok, _) = eeprom.write_partition(&PART_POEM, LYRIC);
                if ok {
                    rprintln!("[写入] 成功");
                    state = State::WriteWait;
                } else {
                    rprintln!("[写入] 失败!");
                    state = State::Error;
                }
            }

            State::WriteWait => {
                rprintln!("[等待] EEPROM 内部写入周期...");
                if eeprom.wait_standby() {
                    rprintln!("[等待] 就绪");
                    match EEPROM_MODE {
                        1 => state = State::Read,
                        _ => state = State::Done,
                    }
                } else {
                    rprintln!("[等待] 超时!");
                    state = State::Error;
                }
            }

            State::Read => {
                rprintln!("[读取] EEPROM @0x00 ...");
                let (ok, n) = eeprom.read_partition(&PART_POEM, &mut read_buf);
                if ok {
                    rprintln!("----------------------------");
                    rprintln!("读取到的内容 ({} 字节):", n);
                    rprintln!("{}", safe_str(&read_buf[..n]));
                    rprintln!("----------------------------");
                    match EEPROM_MODE {
                        2 => state = State::Done,
                        _ => state = State::Verify,
                    }
                } else {
                    rprintln!("[读取] 失败!");
                    state = State::Error;
                }
            }

            State::Verify => {
                rprintln!("[校验] 比对写入与读取...");
                let pass = read_buf[..data_len] == LYRIC[..data_len];
                if pass {
                    rprintln!("[校验] PASS - 数据一致!");
                } else {
                    rprintln!("[校验] FAIL - 数据不一致!");
                    for (i, (a, b)) in read_buf[..data_len]
                        .iter()
                        .zip(LYRIC[..data_len].iter())
                        .enumerate()
                    {
                        if a != b {
                            rprintln!("  偏移 {}: 读到 0x{:02X}, 期望 0x{:02X}", i, a, b);
                        }
                    }
                }
                state = State::Done;
            }

            State::Done => {
                rprintln!("==============================");
                rprintln!("  实验完成!");
                rprintln!("==============================");
                cortex_m::asm::bkpt();
            }

            State::Error => {
                rprintln!("[错误] 操作异常");
                cortex_m::asm::bkpt();
            }
        }
    }
}
