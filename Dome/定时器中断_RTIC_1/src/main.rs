//! 使用定时器中断以不同频率闪烁 LED
//!
//! 假设 LED 连接在 PC13 引脚上（Blue Pill 开发板默认配置）
//!
//! 注意：在没有额外硬件的情况下，不建议用 PC13 直接驱动 LED（详见参考手册 5.1.2 节）
//! 但 Blue Pill 开发板上已有板载 LED，所以没有问题

#![no_std]
#![no_main]

// 导入 panic 处理程序，发生 panic 时 CPU 会停止运行
// 可以在 `rust_begin_unwind` 函数上打断点来捕获 panic
use panic_halt as _;

// ==================== RTIC 应用入口 ====================
// #[rtic::app] 是 RTIC 框架的核心宏，用于定义一个实时中断驱动的应用
// device 参数指定使用的芯片外设包（PAC），这里使用 stm32f1xx_hal 提供的 PAC
#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    // 导入 semihosting 调试输出宏
    // hprintln! 通过 SWD 调试接口输出到 OpenOCD/ST-Link 的终端
    // 相比 RTT 不需要额外配置，但速度较慢
    use cortex_m_semihosting::hprintln;

    use stm32f1xx_hal::{
        // GPIO 相关类型：PC13 引脚、输出模式、引脚状态、推挽输出
        gpio::{gpioc::PC13, Output, PinState, PushPull},
        // PAC（外设访问crate）：直接操作硬件寄存器的底层接口
        pac,
        // prelude：预导入常用 trait（如定时器的 .counter_ms() 方法）
        prelude::*,
        // 定时器相关类型：CounterMs 是毫秒精度定时器，Event 是定时器事件枚举
        timer::{CounterMs, Event},
    };

    // ==================== 共享资源 ====================
    // #[shared] 标记的结构体定义了可在多个任务间共享的资源
    // 本例中没有需要共享的资源，所以结构体为空
    #[shared]
    struct Shared {}

    // ==================== 本地资源 ====================
    // #[local] 标记的结构体定义了只能被单个任务访问的资源
    // 每个资源在编译时就被绑定到特定任务，避免了运行时的锁开销
    #[local]
    struct Local {
        // LED 引脚（PC13，推挽输出模式）
        led: PC13<Output<PushPull>>,
        // 定时器句柄（TIM1，毫秒精度）
        timer_handler: CounterMs<pac::TIM1>,
    }

    // ==================== 初始化函数 ====================
    // #[init] 标记的函数在系统启动时执行一次，用于初始化硬件和资源
    // 返回值是 (共享资源, 本地资源) 元组
    // cx 是 RTIC 的 Context，通过 cx.device 可以访问芯片外设
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        hprintln!("程序启动：init 函数开始执行");

        // 获取 RCC（复位和时钟控制）外设并约束它
        // constrain() 会将 RCC 配置为默认状态，返回一个时钟配置对象
        let mut rcc = cx.device.RCC.constrain();
        hprintln!("RCC 时钟配置完成");

        // 获取 GPIOC 外设并将其拆分为独立的引脚
        // split() 将 GPIOC 的所有引脚封装为独立的 Pin 对象
        let mut gpioc = cx.device.GPIOC.split(&mut rcc);

        // 将 PC13 配置为推挽输出模式，初始状态为高电平（LED 灭）
        // crh 寄存器用于配置引脚 8-15（引脚 0-7 使用 crl 寄存器）
        // PushPull（推挽输出）：可以主动输出高电平或低电平
        // PinState::High：初始输出高电平（Blue Pill 上 LED 低电平点亮）
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High);
        hprintln!("PC13 LED 引脚配置完成");

        // 配置 TIM1 定时器为毫秒精度计数器
        // counter_ms() 将 TIM1 配置为毫秒精度的定时器
        let mut timer = cx.device.TIM1.counter_ms(&mut rcc);
        // 启动定时器，每 1 秒触发一次更新事件
        timer.start(1.secs()).unwrap();
        // 启用定时器的更新中断（Update Event）
        // 当定时器计数溢出时会触发中断
        timer.listen(Event::Update);
        hprintln!("TIM1 定时器配置完成，每 1 秒触发一次中断");

        // 返回初始化后的资源
        // Shared {}：共享资源（本例为空）
        // Local { led, timer_handler }：本地资源，绑定到对应任务
        (
            Shared {},
            Local {
                led,
                timer_handler: timer,
            },
        )
    }

    // ==================== 空闲函数 ====================
    // #[idle] 标记的函数在系统空闲时持续运行（没有任务需要执行时）
    // 返回类型为 `!` 表示永不返回（无限循环）
    // 参考：https://rtic.rs/dev/book/en/by-example/app_idle.html
    // 如果不声明 idle 函数，RTIC 会自动设置 SLEEPONEXIT 位让 CPU 进入睡眠
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        hprintln!("进入 idle 空闲循环，CPU 等待中断中...");
        loop {
            // WFI（Wait For Interrupt）：让 CPU 进入低功耗等待状态
            // CPU 会在中断发生时自动唤醒，执行完中断处理后回到这里
            // cortex_m::asm::dsb();
            cortex_m::asm::wfi();
        }
    }

    // ==================== 定时器中断任务 ====================
    // #[task] 定义一个任务，通过 binds 参数绑定到具体的硬件中断
    // binds = TIM1_UP：绑定到 TIM1 的更新中断（Update Interrupt）
    // priority = 1：任务优先级为 1（数字越大优先级越高）
    // local = [...]：声明本地资源列表，包括：
    //   - led：LED 引脚
    //   - timer_handler：定时器句柄
    //   - led_state: bool = false：LED 状态（初始 false = LED 灭）
    //   - count: u8 = 0：中断计数器（初始 0）
    #[task(binds = TIM1_UP, priority = 1, local = [led, timer_handler, led_state: bool = false, count: u8 = 0])]
    fn tick(cx: tick::Context) {
        // 翻转 LED 状态
        // 如果当前 LED 是亮的（led_state == true），就熄灭它
        // 如果当前 LED 是灭的（led_state == false），就点亮它
        if *cx.local.led_state {
            // set_high()：输出高电平（Blue Pill 上 LED 低电平点亮，高电平熄灭）
            cx.local.led.set_high();
            *cx.local.led_state = false;
            hprintln!("[中断] LED 熄灭");
        } else {
            // set_low()：输出低电平（点亮 LED）
            cx.local.led.set_low();
            *cx.local.led_state = true;
            hprintln!("[中断] LED 点亮");
        }

        // 中断计数器加 1
        // 用于控制定时器频率的切换时机
        *cx.local.count += 1;
        hprintln!("[中断] 计数: {}", *cx.local.count);

        // 动态改变定时器的触发频率
        // 第 4 次中断：将定时器改为 500ms 触发一次（LED 闪烁加快）
        if *cx.local.count == 4 {
            cx.local.timer_handler.start(500.millis()).unwrap();
            hprintln!("[中断] 定时器切换为 500ms");
        }
        // 第 12 次中断：将定时器改回 1 秒触发一次（LED 闪烁恢复慢速）
        // 并重置计数器，开始新的周期
        else if *cx.local.count == 12 {
            cx.local.timer_handler.start(1.secs()).unwrap();
            *cx.local.count = 0;
            hprintln!("[中断] 定时器切换为 1s，计数器重置");
        }

        // 清除定时器的更新中断标志
        // 必须手动清除，否则中断会持续触发
        cx.local.timer_handler.clear_interrupt(Event::Update);
    }
}