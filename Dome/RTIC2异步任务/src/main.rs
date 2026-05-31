//! RTIC2 异步任务示例 —— 两个 async 任务并发运行
//!
//! 功能：
//!   - blink 任务：每 500ms 翻转 PC13 板载 LED
//!   - heartbeat 任务：每 2 秒通过 RTT 打印心跳
//!
//! 两个任务互不阻塞地交替执行，体现 async 的核心价值：
//! 如果用同步阻塞（如 `block!(...)`），heartbeat 的 2 秒等待会卡死 blink。
//! 而 async 在等待期间自动让出 CPU，调度器可以运行其他任务。
//!
//! 硬件：Blue Pill（STM32F103C8T6），8MHz 外部晶振，PC13 LED

// ==================== Rust 嵌入式基础属性 ====================

// 嵌入式程序没有标准库的 main 函数入口
// 程序入口由 RTIC 框架的 #[rtic::app] 宏生成
#![no_main]

// 嵌入式环境没有操作系统，不使用标准库（std）
// 只使用核心库（core），如 Option、Result、循环、算术等
#![no_std]

// ==================== 依赖 crate 导入 ====================

// panic_halt：panic 处理程序
// 当程序发生不可恢复错误时，直接让 CPU 停机（Halt）
// `use ... as _` 表示只引入副作用（panic handler），不使用具体类型
use panic_halt as _;

// rtic_monotonics::systick::prelude 导入：
//   - systick_monotonic! 宏：创建基于 SysTick 的单调度定时器类型
//   - Monotonic trait：定时器必须实现的接口
//   - fugit::ExtU32：为 u32 添加 .millis()、.secs() 等时间单位方法
use rtic_monotonics::systick::prelude::*;

// ==================== 单调度定时器（Monotonic）配置 ====================

// systick_monotonic! 宏展开后会生成一个名为 Mono 的结构体
// 它内部：
//   1. 定义了 SysTick 中断处理函数（extern "C" fn SysTick()）
//   2. 实现了 Monotonic trait，提供 delay、now 等方法
//
// 参数 1_000 表示 tick 频率 = 1000Hz（即每 1ms 产生一次 SysTick 中断）
// Mono::start() 还需要传入系统时钟频率，用于计算 reload 值：
//   reload = sysclk / tick_rate - 1 = 72_000_000 / 1_000 - 1 = 71_999
//   SysTick 每 72_000 个时钟周期（1ms）触发一次中断
//
// 注意：第一个参数是 tick 频率（Hz），不是系统时钟！
//   systick_monotonic!(Mono, 1_000)    → 1kHz，1ms 精度 ✓
//   systick_monotonic!(Mono, 48_000_000) → 48MHz，每秒 4800 万次中断 ✗（太频繁！）
systick_monotonic!(Mono, 1_000);

// ==================== RTIC 应用定义 ====================

// #[rtic::app] 是 RTIC 框架的核心宏，定义一个实时应用
//   device = stm32f1xx_hal::pac：指定芯片的 PAC（外设访问 crate）
//     PAC 提供了对所有硬件寄存器的底层访问接口
//   dispatchers = [USART1]：指定软件任务的派发中断
//     RTIC 借用 USART1 的中断向量来运行异步任务的调度器
//     选 USART1 只是因为我们没用它，任何未使用的中断都可以
#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [USART1])]
mod app {
    // RTT（Real-Time Transfer）调试输出
    // 通过 J-Link/ST-Link 调试器实现零侵入的实时打印
    //   rtt_init_print!()：初始化 RTT 上行通道（只需调用一次）
    //   rprintln!()：通过 RTT 打印一行（类似 println!，但不需要操作系统）
    use rtt_target::{rprintln, rtt_init_print};

    // 从父模块（mod app 外部）导入 Mono 定时器和 panic_halt
    use super::*;

    // stm32f1xx-hal 提供的硬件抽象层类型
    use stm32f1xx_hal::{
        gpio::{Output, PC13},  // PC13 引脚的输出模式类型
        prelude::*,             // 预导入 trait，解锁 .counter_ms()、.MHz() 等方法
        rcc::Config,            // 时钟配置结构体（HSE/HSI/PLL 选择）
    };

    // ==================== 共享资源 ====================
    // #[shared] 定义可被多个任务访问的资源
    // RTIC 在编译时保证共享资源的访问安全（基于优先级的无锁互斥）
    // 本例没有共享资源，所以结构体为空
    #[shared]
    struct Shared {}

    // ==================== 本地资源 ====================
    // #[local] 定义只能被单个任务访问的资源
    // 每个资源在编译时绑定到特定任务，完全避免运行时开销
    #[local]
    struct Local {
        // PC13 引脚，推挽输出模式（驱动 LED）
        led: PC13<Output>,
    }

    // ==================== 初始化函数 ====================
    // #[init] 标记的函数在系统启动时执行一次（且仅一次）
    // 它在所有中断开启之前运行，返回值分配给 shared 和 local 资源
    // ctx 是 RTIC 的上下文对象：
    //   ctx.device → PAC 外设（FLASH、RCC、GPIO 等寄存器）
    //   ctx.core   → Cortex-M 核心外设（SYST、NVIC 等）
    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        // 初始化 RTT 调试通道，之后 rprintln! 才能工作
        rtt_init_print!();
        rprintln!("Start");

        // ==================== 时钟系统配置 ====================
        // STM32F103 时钟树：
        //   HSE (8MHz 外部晶振) → PLL ×9 倍频 → SYSCLK = 72MHz
        //                        ├→ AHB  → APB1 (72÷2 = 36MHz，低速外设)
        //                        └→ AHB  → APB2 (72MHz，高速外设)
        //
        // constrain() 将外设寄存器封装为安全的 Rust 类型
        // freeze() 锁定时钟配置，之后不能再修改
        let mut flash = ctx.device.FLASH.constrain();
        let mut rcc = ctx.device.RCC.freeze(
            Config::hse(8.MHz())  // 使用 8MHz 外部晶振（HSE）
                .sysclk(72.MHz()), // PLL 倍频到 72MHz（STM32F103 最大频率）
            &mut flash.acr,       // Flash 访问控制寄存器（需要配置等待周期）
        );

        // 启动 SysTick 单调度定时器
        // 第一个参数：SysTick 外设（Cortex-M 核心自带的 24 位倒计数器）
        // 第二个参数：系统时钟频率（72MHz），用于计算每次 tick 的时间
        // 启动后，SysTick 每 1ms 触发一次中断（由 systick_monotonic! 的 1_000Hz 决定）
        Mono::start(ctx.core.SYST, 72_000_000);

        // ==================== GPIO 配置 ====================
        // split() 将 GPIOC 外设拆分为独立的引脚对象
        // into_push_pull_output() 将 PC13 配置为推挽输出模式
        //   推挽输出：可以主动输出高电平（3.3V）或低电平（0V）
        //   Blue Pill 板载 LED 低电平点亮（active low）
        let mut gpioc = ctx.device.GPIOC.split(&mut rcc);
        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        // ==================== 启动异步任务 ====================
        // spawn() 将异步任务提交给 RTIC 调度器
        // 任务不会立即执行，而是等 init 返回后由调度器调度
        // .ok() 忽略可能的错误（如任务池已满）
        blink::spawn().ok();
        heartbeat::spawn().ok();

        rprintln!("启动完毕");

        // 返回资源：共享资源（空）和本地资源（LED 引脚）
        // RTIC 会把 led 分配给声明了它 local 的任务
        (Shared {}, Local { led })
    }

    // ==================== 异步任务 1：LED 闪烁 ====================
    // #[task] 定义一个异步任务
    //   local = [led, count: u32 = 0]：
    //     - led：从 init 返回的 Local 中获取
    //     - count: u32 = 0：声明一个任务私有的计数器，初始值为 0
    //         这种内联初始化语法无需在 struct Local 中声明
    //
    // async fn 意味着这个函数可以在 .await 处暂停，让出 CPU
    // RTIC 调度器会在 delay 到期后自动恢复执行
    #[task(local = [led, count: u32 = 0])]
    async fn blink(ctx: blink::Context) {
        loop {
            // toggle()：翻转引脚电平（高→低 或 低→高）
            ctx.local.led.toggle();

            // ctx.local.count 是任务私有状态，不需要锁
            *ctx.local.count += 1;
            rprintln!("[blink] count={}", *ctx.local.count);

            // .delay(500.millis()).await：异步等待 500 毫秒
            // 关键：这里不阻塞 CPU！
            //   1. Mono 定时器记录唤醒时间点
            //   2. 当前任务让出 CPU（状态保存在任务的 Future 中）
            //   3. RTIC 调度器运行其他就绪任务（如 heartbeat）
            //   4. 500ms 后 SysTick 中断唤醒此任务，继续执行
            Mono::delay(500.millis()).await;
        }
    }

    // ==================== 异步任务 2：心跳打印 ====================
    // 另一个独立的异步任务，与 blink 并发运行
    // local = [beat: u32 = 0]：任务私有的心跳计数器
    #[task(local = [beat: u32 = 0])]
    async fn heartbeat(ctx: heartbeat::Context) {
        loop {
            *ctx.local.beat += 1;
            rprintln!("[heartbeat] beat={}", *ctx.local.beat);

            // 等待 2 秒，期间 blink 任务正常运行
            // 如果这是同步阻塞（如 for 循环空转 2 秒），
            // 整个系统在这 2 秒内无法响应任何其他任务
            // 而 async .await 只是"睡一会儿"，其他任务不受影响
            Mono::delay(2.secs()).await;
        }
    }
}
