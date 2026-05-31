#![allow(clippy::empty_loop)]
// #![deny(unsafe_code)]
#![no_std]
#![no_main]

use nb::block;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{flash, pac, prelude::*, rcc, timer::Timer};
use cortex_m_rt::{entry, exception, ExceptionFrame}; 

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain(); // Flash 等待周期配置


    // 外部晶振
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // 使用 8MHz 外部晶振
            .sysclk(72.MHz()) // PLL 倍频到 72MHz
            .pclk1(36.MHz()) // APB1 分频到 36MHz
            .pclk2(72.MHz()) // APB2 不分频
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    // Hello World
    rprintln!("Hello World!");

    loop {}
}

// HardFault 处理：硬件错误时调用
// 常见原因：非法内存访问、非法指令、栈溢出等
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    // ExceptionFrame 包含故障发生时的 CPU 寄存器状态
    panic!("{:#?}", ef);
}

// 默认异常处理：未被其他处理函数捕获的异常
#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    // irqn 是中断号，负数表示系统异常，正数表示外部中断
    panic!("Unhandled exception (IRQn = {})", irqn);
}