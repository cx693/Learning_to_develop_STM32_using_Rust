// USART3 中断 + IDLE 空闲检测 — 接收不定长数据并回传
//
// 原理：
//   1. 每收到 1 字节触发 RXNE 中断，存入 BUFFER
//   2. 总线上出现一段空闲（无新字节）触发 IDLE 中断，表示"一帧结束"
//   3. IDLE 触发时将整帧数据通过 TX 回传（回显）
//
// 使用 Mutex<RefCell<>> 替代 static mut，兼容 Rust 2024 edition

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    pac::{self, interrupt, USART3},
    prelude::*,
    serial::{Rx, Tx},
};

// 全局共享状态：用 Mutex<RefCell<>> 包装，可在中断和主函数间安全共享
static RX: Mutex<RefCell<Option<Rx<USART3>>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Tx<USART3>>>> = Mutex::new(RefCell::new(None));

const BUFFER_LEN: usize = 4096;
static BUFFER: Mutex<RefCell<[u8; BUFFER_LEN]>> = Mutex::new(RefCell::new([0; BUFFER_LEN]));
static WIDX: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("串口通信_中断_IDLE");

    let p = pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut afio = p.AFIO.constrain(&mut rcc);
    let mut gpiob = p.GPIOB.split(&mut rcc);

    // USART3 引脚：PB10(TX), PB11(RX)
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    // 初始化 USART3，波特率 115200，拆分为独立的 TX/RX
    let (mut tx, mut rx) = p
        .USART3
        .remap(&mut afio.mapr)
        .serial((tx, rx), 115_200.bps(), &mut rcc)
        .split();

    // 使能三种中断源
    tx.listen();       // TXE  — 发送寄存器空中断（本例未用，保持默认使能）
    rx.listen();       // RXNE — 接收寄存器非空中断（每收 1 字节触发）
    rx.listen_idle();  // IDLE — 总线空闲检测中断（一帧数据接收完毕触发）

    // 在临界区内将 TX/RX 存入全局静态变量，供中断处理函数使用
    cortex_m::interrupt::free(|cs| {
        TX.borrow(cs).replace(Some(tx));
        RX.borrow(cs).replace(Some(rx));
    });

    // 放行 USART3 中断到 NVIC（NVIC::unmask 是 cortex-m 中唯一的 unsafe 调用）
    #[allow(unsafe_code)]
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART3);
    }

    // 主循环：WFI 休眠，等待中断唤醒
    loop {
        cortex_m::asm::wfi()
    }
}

/// 通过 TX 发送 buf 中的所有字节（阻塞式逐字节发送）
fn write(cs: &cortex_m::interrupt::CriticalSection, buf: &[u8]) {
    let mut tx_ref = TX.borrow(cs).borrow_mut();
    if let Some(tx) = tx_ref.as_mut() {
        buf.iter()
            .for_each(|w| if let Err(_err) = nb::block!(tx.write(*w)) {})
    }
}

/// USART3 中断处理函数
///
/// 两种中断源共用同一个中断入口，通过标志位区分：
///   - RXNE（接收非空）：逐字节读取并存入 BUFFER
///   - IDLE（总线空闲）：一帧结束，回传已接收的全部数据
#[interrupt]
fn USART3() {
    cortex_m::interrupt::free(|cs| {
        let mut rx_ref = RX.borrow(cs).borrow_mut();
        if let Some(rx) = rx_ref.as_mut() {
            if rx.is_rx_not_empty() {
                // RXNE：收到 1 字节，存入环形缓冲区
                if let Ok(w) = nb::block!(rx.read()) {
                    let widx = *WIDX.borrow(cs).borrow();
                    BUFFER.borrow(cs).borrow_mut()[widx] = w;
                    let new_widx = widx + 1;
                    if new_widx >= BUFFER_LEN - 1 {
                        // 缓冲区满：立即回传整块数据，重置写指针
                        let buf = BUFFER.borrow(cs).borrow();
                        write(cs, &buf[..new_widx]);
                        drop(buf);
                        *WIDX.borrow(cs).borrow_mut() = 0;
                    } else {
                        *WIDX.borrow(cs).borrow_mut() = new_widx;
                    }
                }
                rx.listen_idle(); // 每次 RXNE 后重新使能 IDLE 检测
            } else if rx.is_idle() {
                // IDLE：总线空闲 → 一帧数据接收完毕，回传并清空缓冲区
                rx.unlisten_idle();
                let widx = *WIDX.borrow(cs).borrow();
                let buf = BUFFER.borrow(cs).borrow();
                write(cs, &buf[..widx]);
                drop(buf);
                *WIDX.borrow(cs).borrow_mut() = 0;
            }
        }
    })
}
