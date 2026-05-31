#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use nb::block;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, rcc, serial::Config};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("开始串口测试");
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    let mut gpiob = dp.GPIOB.split(&mut rcc);

    // === USART3 引脚配置（DKX 板）===
    // TX: PB10 配置为复用推挽输出
    // 复用推挽输出 = GPIO 由硬件外设控制，而非软件
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // RX: PB11 默认就是浮空输入
    let rx = gpiob.pb11;

    // 创建串口实例
    // USART3, 波特率 115200
    let mut serial = dp
        .USART3
        .serial((tx, rx), Config::default().baudrate(115200.bps()), &mut rcc);

    // === 方式 1: 使用 serial 对象直接读写 ===
    // let sent = b'X';
    // block!(serial.tx.write_u8(sent)).unwrap(); // 发送字节
    // let received = block!(serial.rx.read()).unwrap(); // 接收字节
    // assert_eq!(received, sent); // 验证
    // rprintln!("{}",received);
    // asm::bkpt(); // 断点，用调试器检查

    // === 方式 2: 拆分为独立的 TX/RX ===
    let sent = b'Y';
    let (mut tx, mut rx) = serial.split();
    block!(tx.write_u8(sent)).unwrap();
    // let received = block!(rx.read()).unwrap();
    // block!(tx.write_u8(received)).unwrap(); // 回显
    // asm::bkpt();

    // === 方式 3: 使用 split 后的独立 TX/RX 读写 ===
    // stm32f1xx_hal 的 Tx/Rx 不支持 reunite，
    // 拆分后可独立使用 tx.write_u8() 和 rx.read()
    // let sent = b'Z';
    // let (mut tx, mut rx) = serial.split();
    // block!(tx.write_u8(sent)).unwrap();


    loop {
        // 方式一
        // let received = block!(serial.rx.read()).unwrap(); // 接收字节
        // rprintln!("{}", received as char);
        // block!(serial.tx.write_u8(received)).unwrap(); // 回显

        // 方式二
        let received = block!(rx.read()).unwrap();
        block!(tx.write_u8(received)).unwrap(); // 回显
        rprintln!("{}",received as char);

        // 方式三
        // let received = block!(rx.read()).unwrap();
        // assert_eq!(received, sent);
        // block!(tx.write_u8(received)).unwrap(); // 回显
        // rprintln!("{}",received as char);
    }
}
