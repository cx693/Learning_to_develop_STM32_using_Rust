#![allow(clippy::empty_loop)]
// #![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use cortex_m::asm::delay; // 导入指令延时（消抖用）
use cortex_m_rt::entry;
use pac::interrupt;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    gpio::{Edge, ExtiPin, GpioExt, Input, Output, PullUp},
    pac,
    prelude::*,
    rcc,
};

// 使用 MaybeUninit 存储未初始化的全局变量
static mut LED: MaybeUninit<stm32f1xx_hal::gpio::gpioc::PC13<Output>> = MaybeUninit::uninit();
// static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Input>> = MaybeUninit::uninit(); // 浮空
static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Input<PullUp>>> =
    MaybeUninit::uninit();

/// EXTI9_5 中断处理函数（覆盖 PA7、PB7、PC7 等的 EXTI7 中断）
#[interrupt]
fn EXTI9_5() {
    // 2021 -- Rust
    #[cfg(edition = "2021")]
    let (led, int_pin) = (&mut *LED.as_mut_ptr(), &mut *INT_PIN.as_mut_ptr());

    // 2024 -- Rust
    let (led, int_pin) = (unsafe { &mut *(*(&raw mut LED)).as_mut_ptr() }, unsafe {
        &mut *(*(&raw mut INT_PIN)).as_mut_ptr()
    });
    // 作者解释 --- 说白了就是 权限还给自己
    // &mut 借用本身
    // 2024 对 static mut 不能直接 &mut 需要 &raw解用
    // *(&raw mut LED) 《==》 2021的 LED --> 也就是反解 到LED本身
    // *(*(&raw mut LED)).as_mut_ptr() 《==》 *LED
    // ---------- GPT 解释如下 -------------
    // &raw mut LED
    // 获取 static mut LED 的 raw pointer
    // 类型:
    // *mut MaybeUninit<PC13<Output>>

    // *(&raw mut LED)
    // 对 raw pointer 解引用
    // 得到 LED 对应的内存位置（place）
    // 注意：
    // 这里不是“取值复制”
    // 而是回到该对象所在内存位置

    // .as_mut_ptr()
    // 将 MaybeUninit<T>
    // 转成 *mut T

    // *ptr
    // 对 *mut T 解引用
    // 得到 T 的内存位置（place）

    // &mut *ptr
    // 最终创建:
    // &mut T

    // 注意：
    // 本质仍然是 mutable reference
    // 只是绕过了:
    // &mut STATIC
    // 的直接写法

    if int_pin.check_interrupt() {
        rprintln!("进入中断加+1");
        // ====================== 消抖核心代码 ======================
        delay(72_000_000 / 1000 * 40); // 72MHz 时钟 → 延时 40ms
        // ==========================================================
        led.toggle();
        int_pin.clear_interrupt_pending_bit();
    }
}

#[entry]
fn main() -> ! {
    // 初始化 RTT 调试输出
    rtt_init_print!();
    rprintln!("程序启动...");

    let mut dp = pac::Peripherals::take().unwrap();
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

    rprintln!("开始配置");
    // 作用域 -- 初始化 中断信息配置
    {
        let mut gpioa = dp.GPIOA.split(&mut rcc);
        let mut gpioc = dp.GPIOC.split(&mut rcc);
        let _afio = dp.AFIO.constrain(&mut rcc);

        // LED
        let led = unsafe { &mut *(*(&raw mut LED)).as_mut_ptr() };
        *led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        // 按键
        let int_pin = unsafe { &mut *(*(&raw mut INT_PIN)).as_mut_ptr() };
        // *int_pin = gpioa.pa7.into_floating_input(&mut gpioa.crl); // 浮空输入 最小开发板硬件电路不支持 需要外部用电容+上拉/下拉电阻
        *int_pin = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);

        // 链接到中断 设置触发方式：上升沿和下降沿都触发
        int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Rising); // 按下接地
        // 使能该引脚的中断 -- 启动使能
        int_pin.enable_interrupt(&mut dp.EXTI);
    }

    rprintln!("结束配置！设置NVIC！");
    // 在 NVIC 中取消屏蔽 EXTI9_5 中断
    // 这一步必须在初始化完成后才能做！
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }
    rprintln!("设置完成！");
    loop {}
}
