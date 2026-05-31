#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::singleton;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
    rcc,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("开始串口 DMA 测试");
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

    // 拆分 DMA1 通道（DMA1 有 7 个通道）
    // USART3 TX -> DMA1 Channel 2
    // USART3 RX -> DMA1 Channel 3
    let channels = dp.DMA1.split(&mut rcc);

    let mut gpiob = dp.GPIOB.split(&mut rcc);

    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::new(
        dp.USART3,
        (tx, rx),
        Config::default().baudrate(115200.bps()),
        &mut rcc,
    );

    // 将串口 RX 与 DMA 通道 3 绑定（USART3 RX -> DMA1 Ch3）
    let rx_dma = serial.rx.with_dma(channels.3);
    // 将串口 TX 与 DMA 通道 2 绑定（USART3 TX -> DMA1 Ch2）
    let tx_dma = serial.tx.with_dma(channels.2);

    // singleton! 宏：在静态内存中创建唯一实例
    // DMA 需要静态生命周期的缓冲区
    let rx_buf = singleton!(: [u8; 8] = [0; 8]).unwrap();

    rprintln!("等待接收 8 字节数据...");
    // 启动 DMA 接收（阻塞等待 8 字节）
    let (buf, _rx) = rx_dma.read(rx_buf).wait();

    rprintln!("DMA 接收完成!");
    for (i, byte) in buf.iter().enumerate() {
        rprintln!("buf[{}] = 0x{:02X} -> {}", i, byte, *byte as char);
    }

    // DMA 发送示例
    let tx_buf = singleton!(: [u8; 12] = *b"Hello DMA!\r\n").unwrap();
    let (_buf, _tx) = tx_dma.write(tx_buf).wait();

    rprintln!("DMA 发送完成!");

    loop {}
}