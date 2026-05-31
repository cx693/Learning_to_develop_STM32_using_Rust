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
    adc,
    rcc,
    dma::Half,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("ADC DMA 循环采集测试");

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

    // 拆分 DMA1 通道 1
    let dma_ch1 = dp.DMA1.split(&mut rcc).1;

    let adc1 = adc::Adc::new(dp.ADC1, &mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let adc_ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    // 将 ADC 与 DMA 绑定
    let adc_dma = adc1.with_dma(adc_ch0, dma_ch1);

    // 创建双缓冲区（循环模式需要两个半缓冲区）
    // singleton! 确保缓冲区在静态内存中
    let buf = singleton!(: [[u16; 8]; 2] = [[0; 8]; 2]).unwrap();

    // 启动循环 DMA 读取
    let mut circ_buffer = adc_dma.circ_read(buf);

    // 注意：DMA 循环模式下，不能在两次 readable_half() 调用之间插入耗时操作
    // 否则 DMA 可能跑完一整圈，导致 HTIF 和 TCIF 同时置位 → Overrun panic

    while circ_buffer.readable_half().unwrap() != Half::First {}
    let first_half = circ_buffer.peek(|half, _| *half).unwrap();

    while circ_buffer.readable_half().unwrap() != Half::Second {}
    let second_half = circ_buffer.peek(|half, _| *half).unwrap();

    rprintln!("第一半缓冲区: {:?}", first_half);
    rprintln!("第二半缓冲区: {:?}", second_half);

    let (_buf, adc_dma) = circ_buffer.stop();
    let (_adc1, _adc_ch0, _dma_ch1) = adc_dma.split();

    rprintln!("ADC DMA 循环采集完成");
    cortex_m::asm::bkpt();
    loop {}
}
