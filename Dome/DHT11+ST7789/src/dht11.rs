//! # DHT11 温湿度传感器驱动
//!
//! ## 可调用此驱动的引脚
//!
//! STM32F103 上 **任意 GPIO 引脚** 均可使用，包括：
//!
//! | 端口 | 引脚        | 说明                          |
//! |------|-------------|-------------------------------|
//! | GPIOA | PA0~PA15   | 最常用，PA6 为本项目默认接线  |
//! | GPIOB | PB0~PB15   | 可用，注意 PB3/PB4 需关闭 JTAG |
//! | GPIOC | PC13~PC15  | 可用，但 PC13 通常接 LED      |
//! | GPIOD | PD0~PD15   | C8T6 仅 PD0~PD2 封装引出      |
//!
//! **唯一限制**：PA13/PA14 默认是 SWD 调试引脚，PA15/PB3/PB4 默认是 JTAG 引脚。
//! 使用这些引脚需要先关闭 JTAG/SWD 复用（通过 AFIO）。
//!
//! ## 为什么代码要这样写
//!
//! ### 1. 推挽输出 → 浮空输入 切换
//!
//! DHT11 使用**单总线协议**，MCU 和传感器分时驱动同一根线：
//!
//! ```text
//! ┌─────────┐                    ┌─────────┐
//! │   MCU   │───── DATA ────────│  DHT11  │
//! └─────────┘    (上拉电阻)      └─────────┘
//! ```
//!
//! - **起始信号**：MCU 必须主动拉低 20ms → 拉高 30us（需要**推挽输出**，能主动驱动高/低）
//! - **数据读取**：DHT11 驱动总线发送数据（MCU 必须释放总线 → **浮空输入**，只读不写）
//!
//! 如果用开漏输出（OpenDrain），`set_high()` 只是释放总线（高阻态），
//! 上升沿靠上拉电阻，速度慢，DHT11 可能检测不到起始信号的上升沿。
//!
//! ### 2. SysTick 硬件定时器
//!
//! `cortex_m::asm::delay(n)` 是软件循环计数，受 **flash wait states** 影响：
//! - STM32F103 在 72MHz 下 flash 有 2 个等待周期
//! - 软件循环的每条指令实际需要 2~3 个时钟周期
//! - `delay(72)` 实际可能耗时 2~3us 而非 1us
//!
//! SysTick 是 Cortex-M 内核的 24 位硬件递减计数器，以系统时钟精确计时，
//! **不受 flash wait states 影响**。
//!
//! ### 3. 读完后保持高电平
//!
//! DHT11 总线空闲态为高电平（由外部上拉电阻维持）。
//! 如果读完后推挽输出低电平，下一次起始信号的下降沿无法被 DHT11 识别。
//!
//! ### 4. Const Generics 泛型
//!
//! `Pin<const P: char, const N: u8, MODE>` 使用 const generics：
//! - `P` = 端口名（'A', 'B', 'C', 'D'）
//! - `N` = 引脚号（0~15）
//! - `MODE` = 模式类型（Output<PushPull>, Input<Floating> 等）
//!
//! 编译器为每个具体引脚生成特化代码，**零运行时开销**。
//! 引脚号 0~7 使用 CRL 寄存器，8~15 使用 CRH 寄存器，
//! 由 `HL` trait 在编译期自动选择。

use cortex_m::peripheral::{syst::SystClkSource, SYST};
use stm32f1xx_hal::gpio::{Floating, HL, Input, Output, Pin, PinState, PushPull};

// ============================================================
//  SysTick 硬件定时器延时
// ============================================================

pub struct Delay {
    syst: SYST,
}

impl Delay {
    pub fn new(mut syst: SYST) -> Self {
        syst.set_clock_source(SystClkSource::Core);
        Self { syst }
    }

    pub fn us(&mut self, us: u32) {
        if us == 0 {
            return;
        }
        // 72MHz 下 1us = 72 ticks; SysTick 最大 0xFFFFFF ≈ 233ms
        let ticks = us.saturating_mul(72);
        if ticks == 0 || ticks > 0x00FF_FFFF {
            return;
        }
        self.syst.set_reload(ticks);
        self.syst.clear_current();
        self.syst.enable_counter();
        while !self.syst.has_wrapped() {}
        self.syst.disable_counter();
    }

    pub fn ms(&mut self, ms: u32) {
        for _ in 0..ms {
            self.us(1000);
        }
    }
}

// ============================================================
//  DHT11 错误类型
// ============================================================

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    /// 起始信号后 DHT11 无响应（检测不到低电平应答）
    NoResponse,
    /// 数据位读取超时（总线状态异常）
    ReadTimeout,
    /// 校验和不匹配
    Checksum { calc: u8, recv: u8 },
}

// ============================================================
//  DHT11 驱动
// ============================================================

pub struct Dht11;

impl Dht11 {
    /// 从 DHT11 读取一次温湿度数据
    ///
    /// # 参数
    /// - `pin` — 推挽输出模式的引脚（空闲高电平）
    /// - `cr`  — 引脚的控制寄存器引用（CRL 或 CRH，编译器自动推导）
    /// - `delay` — SysTick 延时器
    ///
    /// # 返回
    /// - `(Ok((湿度, 温度)), pin)` — 成功，湿度 0~99%RH，温度 0~50°C
    /// - `(Err(e), pin)` — 失败
    ///
    /// 无论成功失败，`pin` 都恢复为**推挽输出（高电平）**返回给调用者。
    ///
    /// # 类型约束说明
    ///
    /// `CR` 是 `HL` trait 的关联类型，由引脚号自动决定：
    /// - 引脚 0~7 → `Cr<P, false>` = CRL 寄存器
    /// - 引脚 8~15 → `Cr<P, true>` = CRH 寄存器
    ///
    /// 两个 `where` 约束确保 Output 和 Input 模式下 `CR` 类型一致，
    /// 这样同一个 `&mut cr` 可以在模式切换时共用。
    #[allow(clippy::type_complexity)]
    pub fn read<const P: char, const N: u8, CR>(
        mut pin: Pin<P, N, Output<PushPull>>,
        cr: &mut CR,
        delay: &mut Delay,
    ) -> (Result<(u8, u8), Error>, Pin<P, N, Output<PushPull>>)
    where
        Pin<P, N, Output<PushPull>>: HL<Cr = CR>,
        Pin<P, N, Input<Floating>>: HL<Cr = CR>,
    {
        // ---- 1. 主机起始信号（推挽输出，主动驱动） ----
        pin.set_low();
        delay.ms(20); // 拉低 20ms（规范 18~30ms）
        pin.set_high();
        delay.us(30); // 拉高 30us（规范 10~35us）

        // ---- 2. 切换到浮空输入，读取 DHT11 数据 ----
        let in_pin = pin.into_floating_input(cr);

        let result = Self::read_data(&in_pin, delay);

        // ---- 3. 切换回推挽输出，保持总线空闲高电平 ----
        let out_pin =
            in_pin.into_push_pull_output_with_state(cr, PinState::High);

        (result, out_pin)
    }

    // ---- 内部：等待响应 + 读取 40bit 数据 ----

    fn read_data<const P: char, const N: u8>(
        pin: &Pin<P, N, Input<Floating>>,
        delay: &mut Delay,
    ) -> Result<(u8, u8), Error> {
        // DHT11 响应：先拉低 ~80us
        if !Self::wait_level(pin, false, 100, delay) {
            return Err(Error::NoResponse);
        }
        // DHT11 响应：再拉高 ~80us
        if !Self::wait_level(pin, true, 100, delay) {
            return Err(Error::NoResponse);
        }

        // 读取 5 字节 (40 bit)
        // Byte0 = 湿度整数  Byte1 = 湿度小数
        // Byte2 = 温度整数  Byte3 = 温度小数  Byte4 = 校验和
        let mut buf = [0u8; 5];
        for slot in &mut buf {
            *slot = Self::read_byte(pin, delay).ok_or(Error::ReadTimeout)?;
        }

        // 校验：前 4 字节之和的低 8 位 == 第 5 字节
        let sum = buf[0] as u32 + buf[1] as u32 + buf[2] as u32 + buf[3] as u32;
        if (sum & 0xFF) as u8 != buf[4] {
            return Err(Error::Checksum {
                calc: (sum & 0xFF) as u8,
                recv: buf[4],
            });
        }

        Ok((buf[0], buf[2])) // (湿度, 温度)
    }

    // ---- 内部：等待引脚变为目标电平 ----

    fn wait_level<const P: char, const N: u8>(
        pin: &Pin<P, N, Input<Floating>>,
        target_high: bool,
        timeout_us: u32,
        delay: &mut Delay,
    ) -> bool {
        for _ in 0..timeout_us {
            if pin.is_high() == target_high {
                return true;
            }
            delay.us(1);
        }
        false
    }

    // ---- 内部：读取一个字节（MSB 先行） ----
    //
    // 每个 bit 的时序：
    //   ┌──── 50us ────┐┌── 28us(0) 或 70us(1) ──┐
    //   │    低电平     ││        高电平           │
    //   └───────────────┘└─────────────────────────┘
    //
    // 采样策略：等待低→高跳变后延时 40us 采样
    //   "0": 高电平 ~28us → 40us 后已变低 → 读 0
    //   "1": 高电平 ~70us → 40us 后仍为高 → 读 1

    fn read_byte<const P: char, const N: u8>(
        pin: &Pin<P, N, Input<Floating>>,
        delay: &mut Delay,
    ) -> Option<u8> {
        let mut byte: u8 = 0;
        for _ in 0..8 {
            byte <<= 1;
            // 等待低电平（每个 bit 以 ~50us 低电平开始）
            if !Self::wait_level(pin, false, 70, delay) {
                return None;
            }
            // 等待高电平（DHT11 释放总线）
            if !Self::wait_level(pin, true, 70, delay) {
                return None;
            }
            // 延时 40us 后采样
            delay.us(40);
            if pin.is_high() {
                byte |= 1;
            }
        }
        Some(byte)
    }
}
