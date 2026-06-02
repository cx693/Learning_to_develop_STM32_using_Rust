<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780203075814-b3bd4b33-1fec-4f6a-ba6d-342f60f58486.png" width="1448" title="" crop="0,0,1,1" id="ue9743a87" class="ne-image">



[简体中文](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_cn.md) / [English](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_en.md) / [Русский](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_ru.md)

# 声明
此项目采用 CC BY-NC 4.0 许可证，商业使用需联系版权所有者 pycx0@qq.com 获得授权。基于本项目开发的商业产品必须取得授权，非商业用途免费！

# 基础环境搭建
## 安装probe-rs
```bash
cargo install probe-rs-tools --locked
```



## 安装编译器
```bash
rustup target install thumbv7m-none-eabi
```

## 使用probe-rs检测
DAP模式！

```bash
probe-rs info --protocol swd
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779699156280-0601cf1f-d586-4e82-8763-adf851dc2ccc.png" width="700" title="" crop="0,0,1,1" id="u0cb46cfc" class="ne-image">



## 安装软件包
主要作用就是修护死循环bug

```bash
cargo add panic-halt
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779700077727-2d4a69c3-fbfa-4838-9bcf-5be9c764a7a0.png" width="369" title="" crop="0,0,1,1" id="u09183fea" class="ne-image">

查看FLASH占用

```json
cargo install st-mem
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779726981642-0c7dc5e5-9c60-4e5f-a84e-f07c4be2bdd0.png" width="764" title="" crop="0,0,1,1" id="uaed0ebe6" class="ne-image">



# 编译测试
项目结构

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779701275826-2b47bfc1-5fd2-46f6-81dd-959f9f744c6a.png" width="289" title="" crop="0,0,1,1" id="UhWsP" class="ne-image">

memory.x

```bash
/* Linker script for the STM32F103C8T6 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
```

cargo.toml

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8B%B7%E8%B4%9D%E6%A8%A1%E7%89%88/Cargo.toml)



.cargo/cargo.toml

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8B%B7%E8%B4%9D%E6%A8%A1%E7%89%88/.cargo/config.toml)



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779701739431-cfc7d6f5-96bb-4911-8ad2-ae2660fe538b.png" width="336" title="" crop="0,0,1,1" id="u909b0bf4" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779701756541-3d0fa057-0cea-450e-9797-80f999b79c5b.png" width="674" title="" crop="0,0,1,1" id="u8ea4bf36" class="ne-image">

<font style="color:#DF2A3F;">main.rs 见 点亮第一颗 LED</font>

## 函数无法跳转
解决方法!

.vscode/settings.json

```json
{
  "rust.all_targets": false,
  "rust.target": "thumbv7m-none-eabi",
  "rust.all_features": false,
  "rust.features": [],
  "rust-analyzer.checkOnSave.allTargets": false,
  "rust-analyzer.checkOnSave.extraArgs": ["--target", "thumbv7m-none-eabi"],
  "rust-analyzer.cargo.features": []
}
```

# 调试
配置文件任务文件创建! 使用probe-rs官网进行！搜索

[https://probe.rs/docs/tools/debugger/](https://probe.rs/docs/tools/debugger/)



支持的芯片类型：[https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)



.vscode/launch.json

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8B%B7%E8%B4%9D%E6%A8%A1%E7%89%88/.vscode/launch.json)



~~目前 macos rtt调试有问题 无法终端输出！~~ 已解决！

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976699740-3c2d6980-e2fa-46f9-8404-b6d5f4366b2d.png" width="1440" title="" crop="0,0,1,1" id="u90e0f0b7" class="ne-image">

目前解决方案是：mian()入口就打断电 开头滴一行写：  rtt_init_print!();

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976859955-199c20a0-8073-4eca-b109-c0a097238644.png" width="486" title="" crop="0,0,1,1" id="ue86693d6" class="ne-image">







# 项目下载（固化）
基础rust编译环境

```plain
cargo install cargo-binutils
rustup component add llvm-tools
```

## HEX文件
编译出ELF文件

```rust
cargo build --release
```

编译出HEX文件

```rust
cargo objcopy --release -- -O ihex ccc.hex
cargo objcopy --release -- -O ihex <固件名>.hex
```

直接下载

```rust
probe-rs download --binary-format hex --chip STM32F103C8 ccc.hex
probe-rs download --binary-format hex --chip <芯片名>	<固件名>.hex
```

芯片名查询地址：[https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779867658066-603db27f-8d6c-4545-b93a-3fc240c50078.png" width="1540" title="" crop="0,0,1,1" id="ua5bbe57c" class="ne-image">

## BIN文件
编译出ELF文件

```rust
cargo build --release
```

编译出HEX文件

```rust
cargo objcopy --release -- -O binary ccc.bin
cargo objcopy --release -- -O binary <固件名>.bin
```

直接下载

```rust
probe-rs download --chip STM32F103C8 --base-address 0x08000000 --binary-format bin ccc.bin
probe-rs download --chip <芯片名> --base-address <偏移地址> --binary-format bin <固件名>.bin
```



# 解除JTAG端口-无法下载
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897632872-17f9317c-b978-4d97-845e-7012f1e8897b.png" width="604" title="" crop="0,0,1,1" id="u888d55bd" class="ne-image">

或者运行失败--全盘插除命令

```rust
probe-rs erase --chip STM32F103C8 --speed 100 --protocol swd
probe-rs erase --chip <芯片名> --speed 100 --protocol <接口类型-可省略>
probe-rs erase --chip STM32F103C8 --speed 100
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897847743-83db4c01-57bd-4529-8fef-cb60a3c3b09f.png" width="496" title="" crop="0,0,1,1" id="u2c272bf8" class="ne-image">

也就是boot0-boot1 --> 都为0

执行指令后按一下复位键快速弹起！会自动插除，不行就先按住复位按键执行指令马上松开！



# 学习地址
教材：[https://xxchang.github.io/book/](https://xxchang.github.io/book/)

项目地址：[https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples](https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples)



# 基础学习
---

## 时钟系统详解
> **为什么先学时钟？** 因为几乎所有外设都依赖时钟才能工作。时钟配置是嵌入式开发中最基础、最重要的一步。配置错误会导致外设工作异常、串口波特率不准、USB 无法枚举等问题。
>

### STM32F1 时钟树总览
STM32F103 的时钟系统非常灵活，有多个时钟源和分频器。以下是简化的时钟树：

```plain
                          ┌─────────────┐
                          │   HSE       │  外部高速晶振 (DKX 板: 8MHz)
                          │  8 MHz      │
                          └──────┬──────┘
                                 │
                          ┌──────▼──────┐
                          │   HSI       │  内部 RC 振荡器
                          │  8 MHz      │  (精度差，±1%，启动快)
                          └──────┬──────┘
                                 │
                 ┌───────────────┼───────────────┐
                 │               │               │
                 │         ┌─────▼─────┐         │
                 │         │   PLL     │  锁相环  │
                 │         │  倍频器   │  ×2~×16 │
                 │         └─────┬─────┘         │
                 │               │               │
          ┌──────▼──────┐ ┌──────▼──────┐        │
          │ SYSCLK      │ │  USBCLK     │        │
          │ 系统时钟     │ │  USB 时钟   │        │
          │ 最大 72MHz   │ │  必须 48MHz │        │
          └──────┬──────┘ └─────────────┘        │
                 │                                │
        ┌────────┼────────┐                      │
        │        │        │                      │
   ┌────▼───┐ ┌──▼───┐ ┌──▼────┐          ┌─────▼─────┐
   │ AHB    │ │ APB1 │ │ APB2  │          │  SYSCLK   │
   │ 总线   │ │ 总线  │ │ 总线  │          │  来源选择  │
   │≤72MHz  │ │≤36MHz│ │≤72MHz │          │ HSI/HSE/PLL
   └───┬────┘ └──┬───┘ └──┬────┘          └───────────┘
       │         │        │
  ┌────▼───┐ ┌───▼────┐ ┌─▼──────┐
  │ Cortex │ │USART2/3│ │USART1  │
  │ SysTick│ │TIM2-4  │ │SPI1    │
  │ DMA    │ │I2C1/2  │ │ADC1/2  │
  │ Flash  │ │SPI2    │ │TIM1    │
  └────────┘ │USB     │ │GPIO    │
             └────────┘ └────────┘
```

**核心概念：PLL（锁相环）**

```plain
PLL 时钟 = PLL 输入时钟 × PLL 倍频系数

如果选择 HSE 作为 PLL 输入:
  PLLCLK = HSE × 倍频系数 (×2 ~ ×16)

示例（DKX 板 8MHz 晶振）:
  HSE × 9  = 8 × 9  = 72 MHz ← 最大系统时钟
  HSE × 6  = 8 × 6  = 48 MHz ← USB 需要
  HSE × 4  = 8 × 4  = 32 MHz

如果选择 HSI 作为 PLL 输入:
  PLLCLK = HSI × 2 × 倍频系数 / 2
  PLLCLK = HSI × 倍频系数 (×2 ~ ×16)
  但 HSI 必须先被 2 分频再进入 PLL
```

---

### 时钟源详解
#### HSI (High Speed Internal) — 内部高速时钟
```plain
特点：
├── 频率：8 MHz（RC 振荡器，有温漂）
├── 精度：±1%（出厂校准），温度变化会漂移
├── 优点：不需要外部元件，上电即可使用
├── 缺点：精度差，不适合 USB、CAN、精确波特率
└── 默认：上电后自动作为系统时钟源
```

**什么时候用 HSI？**

+ 简单的 LED 闪烁、按键检测等不需要精确时钟的场景
+ 外部晶振损坏时的备用方案
+ 快速启动场景（HSI 比 HSE 启动快）

#### HSE (High Speed External) — 外部高速时钟
```plain
特点：
├── 频率：4-16 MHz（DKX 板使用 8MHz 晶振）
├── 精度：±0.005%（取决于晶振质量）
├── 优点：精度高，适合 USB、CAN、精确串口波特率
├── 缺点：需要外部晶振，启动需要时间（几百微秒~几毫秒）
└── DKX 板：8MHz 无源晶振 + 2 个 20pF 负载电容
```

**什么时候用 HSE？**

+ 需要 USB 功能（**必须**用 HSE 或 HSE 通过 PLL）
+ 需要 CAN 总线（需要精确时钟）
+ 需要精确的串口波特率
+ 需要系统满频 72MHz 运行

#### LSE (Low Speed External) — 外部低速时钟
```plain
特点：
├── 频率：32.768 kHz（用于 RTC）
├── 精度：很高（晶振温漂小）
├── 用途：实时时钟 (RTC)、看门狗
└── DKX 板：可能没有焊接 LSE 晶振（需确认原理图）
```

#### LSI (Low Speed Internal) — 内部低速时钟
```plain
特点：
├── 频率：约 40 kHz（不精确）
├── 用途：独立看门狗 (IWDG)、RTC 备用时钟
└── 精度：较差（±30%）
```

#### PLL (Phase Locked Loop) — 锁相环
PLL 是时钟系统的核心，用于将低频时钟倍频到高频。

```plain
┌─────────────────────────────────────────────────┐
│                    PLL 详解                      │
├─────────────────────────────────────────────────┤
│                                                  │
│  输入源选择:                                      │
│  ┌──────┐    ┌─────────┐                         │
│  │ HSI/2│───►│         │    ┌───────────┐        │
│  └──────┘    │ PLL MUX │───►│  ÷ PLLMUL │──► PLLCLK
│  ┌──────┐───►│         │    │  (×2~×16) │        │
│  │ HSE  │    └─────────┘    └───────────┘        │
│  └──────┘                                        │
│                                                  │
│  常用配置:                                        │
│  ┌──────────┬──────────┬──────────────┐          │
│  │ 输入时钟 │ 倍频系数  │ 输出频率     │          │
│  ├──────────┼──────────┼──────────────┤          │
│  │ HSI 8MHz │ ×9       │ 36 MHz*     │          │
│  │ HSE 8MHz │ ×9       │ 72 MHz ✓    │          │
│  │ HSE 8MHz │ ×6       │ 48 MHz ✓    │          │
│  │ HSE 8MHz │ ×4       │ 32 MHz ✓    │          │
│  └──────────┴──────────┴──────────────┘          │
│                                                  │
│  * HSI 先被 2 分频(=4MHz)，再 ×9 = 36MHz         │
│                                                  │
└─────────────────────────────────────────────────┘
```

---

### 各总线时钟详解
```plain
                         SYSCLK (系统时钟)
                              │
                ┌─────────────┼─────────────┐
                │             │             │
           ┌────▼────┐  ┌────▼────┐  ┌─────▼─────┐
           │  AHB    │  │  APB1   │  │   APB2    │
           │  总线   │  │  总线   │  │   总线    │
           │ ÷HPRE  │  │ ÷PPRE1 │  │  ÷PPRE2  │
           │(1/2/4..)│  │(1/2/4..)│  │ (1/2/4..) │
           └────┬────┘  └────┬────┘  └─────┬─────┘
                │            │              │
                │            │              │
          ┌─────▼─────┐ ┌────▼────┐  ┌──────▼──────┐
          │Core, DMA  │ │USART2/3 │  │  USART1     │
          │SysTick    │ │I2C1/2   │  │  SPI1       │
          │Flash      │ │SPI2     │  │  ADC1/2     │
          │GPIO A~D   │ │TIM2-4   │  │  TIM1       │
          │           │ │USB      │  │  EXTI       │
          │           │ │CAN      │  │  AFIO       │
          └───────────┘ └─────────┘  └─────────────┘
```

#### AHB 总线 — 高速总线
| 参数 | 说明 |
| --- | --- |
| 最大频率 | 72 MHz |
| 预分频器 | SYSCLK ÷ 1/2/4/8/16/64/128/256/512 |
| 连接设备 | Cortex-M3 内核、DMA、Flash、GPIO、SysTick |
| 配置方法 | `rcc::Config::hsi().hclk(72.MHz())` |


**注意：** SysTick 时钟来自 AHB（如果 SysTick 配置为使用处理器时钟），或 AHB/8。

#### APB1 总线 — 低速外设总线
| 参数 | 说明 |
| --- | --- |
| 最大频率 | **36 MHz**（硬限制，超过会损坏芯片） |
| 预分频器 | AHB ÷ 1/2/4/8/16 |
| 连接设备 | USART2, USART3, I2C1/2, SPI2, TIM2-4, USB, CAN |
| 配置方法 | `rcc::Config::hsi().pclk1(36.MHz())` |


**重要：** 如果 APB1 预分频系数 > 1，则定时器时钟 = APB1 × 2。

#### APB2 总线 — 高速外设总线
| 参数 | 说明 |
| --- | --- |
| 最大频率 | **72 MHz** |
| 预分频器 | AHB ÷ 1/2/4/8/16 |
| 连接设备 | USART1, SPI1, ADC1/2, TIM1, GPIOA~D, EXTI, AFIO |
| 配置方法 | `rcc::Config::hsi().sysclk(72.MHz()).pclk2(72.MHz())` |


#### ADC 时钟
| 参数 | 说明 |
| --- | --- |
| 最大频率 | **14 MHz** |
| 时钟来源 | APB2 ÷ 2/4/6/8 |
| 配置方法 | `rcc::Config::hsi().adcclk(14.MHz())` |


#### USB 时钟
| 参数 | 说明 |
| --- | --- |
| 要求频率 | **48 MHz**（必须精确） |
| 时钟来源 | PLL 输出（PLLCLK ÷ 1 或 1.5） |
| 配置要求 | SYSCLK 必须是 48MHz 或 72MHz |


---

### stm32f1xx-hal 中的时钟配置
HAL 库使用 **Builder 模式** 配置时钟，非常直观：

#### 基本用法
```rust
use stm32f1xx_hal::{pac, prelude::*, rcc};

fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();  // Flash 等待周期配置

    // 方式 1: 简洁的配置方法
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hsi()           // 使用内部 8MHz RC
            .sysclk(64.MHz())        // 系统时钟 64MHz
            .pclk1(32.MHz())         // APB1 时钟 32MHz
            .pclk2(64.MHz())         // APB2 时钟 64MHz
            .adcclk(8.MHz()),        // ADC 时钟 8MHz
        &mut flash.acr,
    );

    // 方式 2: 使用外部晶振 + PLL
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())    // 使用 8MHz 外部晶振
            .sysclk(72.MHz())        // PLL 倍频到 72MHz
            .pclk1(36.MHz())         // APB1 分频到 36MHz
            .pclk2(72.MHz())         // APB2 不分频
            .adcclk(14.MHz()),       // ADC 14MHz
        &mut flash.acr,
    );
}
```

#### `rcc::Config` 的 builder 方法
```rust
// 所有可用的配置方法（带 * 表示常用）
rcc::Config::hsi()                    // 选择 HSI 作为时钟源
rcc::Config::hse(8.MHz())            // 选择 HSE 作为时钟源，指定频率

.sysclk(72.MHz())    *               // 设置目标系统时钟频率
.pclk1(36.MHz())    *                // 设置 APB1 目标频率
.pclk2(72.MHz())    *                // 设置 APB2 目标频率
.adcclk(14.MHz())   *                // 设置 ADC 时钟频率
.hclk(72.MHz())                       // 设置 AHB 时钟（通常 = SYSCLK）

// PLL 会根据目标频率自动计算倍频系数
// 无需手动设置！
```

#### 冻结后获取时钟信息
```rust
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,
);

// 获取实际的时钟频率
rprintln!("SYSCLK: {}", rcc.clocks.sysclk());   // 系统时钟
rprintln!("HCLK:   {}", rcc.clocks.hclk());     // AHB 时钟
rprintln!("PCLK1:  {}", rcc.clocks.pclk1());    // APB1 时钟
rprintln!("PCLK2:  {}", rcc.clocks.pclk2());    // APB2 时钟
rprintln!("ADCCLK: {}", rcc.clocks.adcclk());   // ADC 时钟
rprintln!("USBCLK valid: {}", rcc.clocks.usbclk_valid()); // USB 时钟是否有效
```

#### 为什么需要 `flash.acr`？
Flash 的读取速度有限，当系统时钟超过 24MHz 时，需要插入等待周期：

| 系统时钟 | Flash 等待周期 |
| --- | --- |
| 0-24 MHz | 0 等待周期 |
| 24-48 MHz | 1 等待周期 |
| 48-72 MHz | 2 等待周期 |


`freeze()` 会自动根据系统时钟频率设置正确的等待周期。

#### `constrain()` vs `freeze()` 的区别
```rust
// constrain() — 约束 RCC，返回可配置的对象
// 用于手动配置每个外设时钟
let mut rcc = dp.RCC.constrain();

// freeze() — 一步完成时钟配置并冻结
// 自动计算所有分频/倍频参数
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,
);
```

一般推荐用 `freeze()`，更简单且不容易出错。

#### 高级用法：直接指定分频/倍频系数
```rust
// 如果你需要完全控制时钟配置，可以使用 RawConfig
let rcc = dp.RCC.freeze(
    rcc::RawConfig {
        hse: Some(8_000_000),       // HSE 频率
        pllmul: Some(7),            // PLL 倍频系数 (×9，索引从 0 开始)
        hpre: rcc::HPre::Div1,     // AHB 预分频 = 不分频
        ppre1: rcc::PPre::Div2,    // APB1 预分频 = AHB ÷ 2
        ppre2: rcc::PPre::Div1,    // APB2 预分频 = 不分频
        usbpre: rcc::UsbPre::Div1_5, // USB 预分频
        adcpre: rcc::AdcPre::Div2,  // ADC 预分频 = APB2 ÷ 2
        ..Default::default()
    },
    &mut flash.acr,
);
```

---

### 常用时钟配置方案
#### 方案 1: 最简配置（HSI 默认）
```rust
// 上电默认：HSI 8MHz，不使用 PLL
// SYSCLK = 8MHz, APB1 = 8MHz, APB2 = 8MHz
let mut rcc = dp.RCC.constrain();

// 或者用 freeze 也行
let rcc = dp.RCC.freeze(rcc::Config::hsi(), &mut flash.acr);
```

**适用：** LED 闪烁、按键检测、GPIO 测试等简单场景  
**不适合：** USB、CAN、高波特率串口

---

#### 方案 2: HSI 倍频到 64MHz
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hsi()
        .sysclk(64.MHz())         // HSI × 8 = 64MHz
        .pclk1(32.MHz())          // APB1 = AHB ÷ 2
        .pclk2(64.MHz())          // APB2 = AHB（不分频）
        .adcclk(8.MHz()),         // ADC = APB2 ÷ 8
    &mut flash.acr,
);
// 注意：HSI 最高只能倍频到 64MHz，不能到 72MHz
// 因为 HSI 进入 PLL 前会被 2 分频，8/2=4, 4×16=64
```

**适用：** 没有外部晶振但需要较高性能的场景  
**不适合：** USB（需要精确 48MHz）

---

#### 方案 3: HSE 倍频到 72MHz（**推荐！DKX 板首选**）
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())     // 使用 DKX 板的 8MHz 晶振
        .sysclk(72.MHz())         // PLL: 8 × 9 = 72MHz
        .pclk1(36.MHz())          // APB1 = 72 ÷ 2 = 36MHz（最大值）
        .pclk2(72.MHz())          // APB2 = 72MHz（不分频）
        .adcclk(14.MHz()),        // ADC = 72 ÷ 6 ≈ 12MHz（实际取 6 分频）
    &mut flash.acr,
);
```

**适用：** 几乎所有场景，最高性能配置  
**时钟：**

+ SYSCLK = 72 MHz
+ AHB = 72 MHz
+ APB1 = 36 MHz
+ APB2 = 72 MHz
+ ADC = 12 MHz
+ Flash 等待周期 = 2

---

#### 方案 4: HSE 倍频到 48MHz（USB 专用）
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())     // 8MHz 晶振
        .sysclk(48.MHz())         // PLL: 8 × 6 = 48MHz
        .pclk1(24.MHz())          // APB1 = 48 ÷ 2 = 24MHz
        .pclk2(48.MHz()),         // APB2 = 48MHz
    &mut flash.acr,
);

// 验证 USB 时钟
assert!(rcc.clocks.usbclk_valid());  // USB 需要精确 48MHz
```

**适用：** USB 应用  
**时钟：**

+ SYSCLK = 48 MHz
+ USBCLK = 48 MHz ✓（精确）
+ APB1 = 24 MHz
+ APB2 = 48 MHz
+ Flash 等待周期 = 1

---

#### 方案 5: 72MHz + USB（进阶）
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())         // PLL: 8 × 9 = 72MHz
        .pclk1(36.MHz())          // APB1 = 36MHz
        .pclk2(72.MHz()),         // APB2 = 72MHz
    &mut flash.acr,
);

// USB 时钟 = PLLCLK ÷ 1.5 = 72 ÷ 1.5 = 48MHz ✓
assert!(rcc.clocks.usbclk_valid());
```

**适用：** 既需要最高性能又需要 USB 的场景

---

### 2.6 时钟配置常见错误
#### 错误 1: APB1 超过 36MHz
```rust
// ❌ 错误！APB1 最大 36MHz
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(72.MHz()),  // 错误！超过 36MHz
    &mut flash.acr,
);

// ✓ 正确
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz()),  // 正确
    &mut flash.acr,
);
```

#### 错误 2: USB 时钟不精确
```rust
// ❌ HSI 不适合 USB
let mut rcc = dp.RCC.freeze(
    rcc::Config::hsi().sysclk(48.MHz()),
    &mut flash.acr,
);
// HSI 精度 ±1%，USB 需要 ±0.25%，会导致枚举失败

// ✓ 必须使用 HSE
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(48.MHz()),
    &mut flash.acr,
);
```

#### 错误 3: 忘记 `flash.acr`
```rust
// ❌ 缺少 flash.acr 参数
let rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(72.MHz()));
// 编译错误！freeze 需要两个参数

// ✓ 正确
let mut flash = dp.FLASH.constrain();
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,  // 必须传入！
);
```

#### 错误 4: SYSCLK 超过 72MHz
```rust
// ❌ STM32F103 最大 72MHz
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(80.MHz()),  // 错误！
    &mut flash.acr,
);
// freeze 会自动选择接近但不超过 72MHz 的频率
```

#### 错误 5: ADC 时钟超过 14MHz
```rust
// ❌ ADC 时钟最大 14MHz
// 如果 pclk2 = 72MHz，且不分频给 ADC，ADC 时钟 = 72/6 = 12MHz ✓
// 如果 pclk2 = 72MHz，且 ADC 不分频，ADC 时钟 = 72MHz ✗

// HAL 库会自动处理，但了解原理很重要
```

---

### 时钟配置速查表
| 场景 | 配置 | SYSCLK | APB1 | APB2 | USB |
| --- | --- | --- | --- | --- | --- |
| LED/按键 | `Config::hsi()` | 8 MHz | 8 MHz | 8 MHz | ✗ |
| 通用 | `hse(8).sysclk(72)` | 72 MHz | 36 MHz | 72 MHz | ✓ |
| USB | `hse(8).sysclk(48)` | 48 MHz | 24 MHz | 48 MHz | ✓ |
| 低功耗 | `hsi().sysclk(8)` | 8 MHz | 8 MHz | 8 MHz | ✗ |
| 无外部晶振 | `hsi().sysclk(64)` | 64 MHz | 32 MHz | 64 MHz | ✗ |


---

## 点亮第一颗LED
```rust
//! 使用 STM32F103C8T6 的 PC13 引脚闪烁 LED
//!
//! 此示例假设 LED 连接到 PC13 引脚，就像 Blue Pill 开发板上的情况一样。
//!
//! 注意：在没有额外硬件的情况下，PC13 不应直接用于驱动 LED，
//! 详细说明请参考参考手册第 5.1.2 节。不过在 Blue Pill 开发板上这不是问题。

// 禁止使用 unsafe 代码，确保代码安全性
#![deny(unsafe_code)]
// 告诉 Rust 编译器不使用标准库（嵌入式环境必需）
#![no_std]
// 告诉 Rust 编译器没有传统的 main 函数，使用 cortex-m-rt 提供的入口点
#![no_main]

// 导入 panic 处理函数：当程序发生不可恢复的错误时，停止 CPU 运行
use panic_halt as _;

// 导入非阻塞操作的工具模块，用于处理异步操作
use nb::block;

// 导入 cortex-m 运行时提供的入口点宏
use cortex_m_rt::entry;
// 导入 HAL 库的核心模块
// pac: 外设访问层（Peripheral Access Crate），提供对寄存器级别的访问
// prelude: 预导入常用的 trait，简化代码
// timer: 定时器模块
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use rtt_target::{rprintln,rtt_init_print};

// 定义程序入口点，替代标准的 main 函数
#[entry]
fn main() -> ! {

    rtt_init_print!();

    // 获取 Cortex-M 核心外设（如 SysTick 定时器、NVIC 等）
    // take() 方法确保这些外设只被获取一次，防止重复使用
    let cp = cortex_m::Peripherals::take().unwrap();
    // 获取 STM32F103 的设备特定外设（GPIO、定时器、串口等）
    let dp = pac::Peripherals::take().unwrap();

    // 获取并配置复位和时钟控制器（RCC）
    // constrain() 方法将原始的 RCC 结构转换为 HAL 提供的高级抽象
    let mut rcc = dp.RCC.constrain();

    // 获取 GPIOC 端口，并将其拆分为独立的引脚
    // split() 方法确保引脚所有权唯一，防止多个函数同时控制同一引脚
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // 将 PC13 引脚配置为推挽输出模式
    // crh 寄存器用于配置端口的高 8 位引脚（PC8-PC15）
    // 对于低 8 位引脚（PC0-PC7），应该传入 crl 寄存器
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    
    // 配置系统滴答定时器（SysTick）作为计数器，按指定频率触发
    // counter_hz() 将 SysTick 配置为基于频率的计数器模式
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    
    // 启动定时器，设置触发频率为 1Hz（每秒触发一次）
    timer.start(4.Hz()).unwrap();

    // 主循环：等待定时器触发并切换 LED 状态
    loop {
        // 阻塞等待定时器第一次触发（1 秒后）
        block!(timer.wait()).unwrap();
        // 将 PC13 设置为高电平，熄灭 LED（Blue Pill 的 LED 是低电平点亮）
        led.set_high();
        rprintln!("OPEN THE LED");
        
        // 阻塞等待定时器第二次触发（再过 1 秒）
        block!(timer.wait()).unwrap();
        // 将 PC13 设置为低电平，点亮 LED
        led.set_low();
        rprintln!("LOW THE LED");
    }
}
```

第二版本--自动读取芯片

```rust
#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*}; // pac = Peripheral Access Crate

#[entry]
fn main() -> ! {
    // 获取外设所有权（只能调用一次，后续调用返回 None）
    let p = pac::Peripherals::take().unwrap();

    // 约束 RCC（复位和时钟控制）寄存器
    // constrain() 返回一个包含所有可配置时钟的对象
    let mut rcc = p.RCC.constrain();

    // 拆分 GPIOC 端口为独立的引脚对象
    // split() 返回每个引脚的独立句柄
    let mut gpioc = p.GPIOC.split(&mut rcc);

    // 根据芯片型号选择不同的引脚和电平
    cfg_select! {
        feature = "stm32f100" => {
            // STM32F100: PC9 高电平点亮
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        feature = "stm32f101" => {
            // STM32F101: PC9 高电平点亮
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        _ => {
            // STM32F103 (包括你的 DKX 板): PC13 低电平点亮
            // PC13 在 Blue Pill/DKX 板上是共阳接法，低电平 = 亮
            gpioc.pc13.into_push_pull_output(&mut gpioc.crh).set_high(); // 实际上 嘉立创的是 设置为高电平 亮
        }
    }

    loop {} // 保持 LED 状态不变
}
```



## Hello World
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/hello/src/main.rs)

**关键概念详解：**

+ `cortex_m::Peripherals` — Cortex-M 核心外设：
    - `SYST` — SysTick 定时器
    - `NVIC` — 中断控制器
    - `DCB` — 调试控制块
    - `DWT` — 数据观察点和触发单元
+ `pac::Peripherals` — 芯片特意外设：
    - `RCC` — 时钟控制
    - `GPIOA/B/C/D` — GPIO 端口
    - `USART1/2/3` — 串口
    - `TIM1/2/3/4` — 定时器
    - `SPI1/2` — SPI 接口
    - `I2C1/2` — I2C 接口
    - `ADC1/2` — ADC
    - `USB` — USB 外设
    - `CAN` — CAN 控制器
+ `Timer::syst()` — 使用 SysTick 定时器创建定时器对象
+ `counter_hz()` — 创建以 Hz 为单位的频率计数器
+ `block!()` — 将非阻塞操作转为阻塞（轮询等待直到完成）
+ `1.Hz()` — 使用 fugit 库的频率单位

---

## LED闪烁
### SYST 模式延时闪烁
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/LED%E9%97%AA%E7%83%81/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779801980743-ec54e1f1-3737-4cd9-adfa-8e4a6a99ec8e.jpeg" width="281" title="" crop="0,0,1,1" id="u62526734" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779802010301-0b3dbe14-df37-444f-9f75-1f9dcb40b870.jpeg" width="280" title="" crop="0,0,1,1" id="u6a547c40" class="ne-image">

如上图LED闪烁

### LED闪烁-TIM2定时器延时
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/LED%E9%97%AA%E7%83%81_TIM2/src/main.rs)

**关键概念：**

+ `rcc::Config::hse(8.MHz())` — 使用 8MHz 外部高速晶振（DKX 板上的晶振）
+ `.sysclk(48.MHz())` — 设置系统时钟为 48MHz
+ `rcc.freeze()` — 冻结时钟配置，返回一个不可变的时钟状态
+ `dp.TIM2.delay_us()` — 使用 TIM2 创建微秒级延迟
+ **优势**：比 SysTick 更灵活，精度更高，不影响 SysTick 的其他用途



### 延时讲解
这种就是延时函数 如果直述 就是直接 delay.delay(20.millis());

+ nanos() 纳秒; 
+ micros() 微秒;
+ millis() 毫秒; 
+ secs() 秒; 
+ millis() 毫秒; 
+ minutes() 分; 
+ hours() 时



## 按键点灯
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8C%89%E9%94%AE%E4%B8%8ELED/src/main.rs)

**JTAG 引脚说明：**

+ STM32F1 默认使用 JTAG/SWD 调试接口
+ PA13(SWDIO), PA14(SWCLK), PA15(JTDI), PB3(JTDO), PB4(JNTRST) 默认被 JTAG 占用
+ 如果要用这些引脚做普通 GPIO，必须先禁用 JTAG
+ 注意：PA13/PA14 是 SWD 接口，一般不建议禁用（否则无法调试）



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779809566789-02c7ee36-cc1e-4e52-82e7-860ae68fe4bb.png" width="662" title="" crop="0,0,1,1" id="u84f8089e" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809683134-a5da8944-f17a-4c76-bf5a-e18eb1ffe579.jpeg" width="223" title="" crop="0,0,1,1" id="ufb18b746" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809699056-1341bf76-a62b-49f6-ae96-d537e2c5d09c.jpeg" width="223" title="" crop="0,0,1,1" id="u774adcc9" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809732125-60f82ecc-dd74-49c0-bee5-c640bd90733b.jpeg" width="222" title="" crop="0,0,1,1" id="u1effb711" class="ne-image">



## 动态GPIO切换（复用端口）
实际开发中端口可能会被复用使用就需要使用下面的模版进行

**动态 GPIO 用途：**

+ 某些协议（如单总线、I2C 软件模拟）需要在运行时切换引脚方向
+ 有限引脚的复用

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%8A%A8%E6%80%81GPIO%E5%88%87%E6%8D%A2/src/main.rs)



## 外部中断EXTI
### EXTI 中断线与引脚对应关系
每个 EXTI 线只能同时连接一个引脚，但同一编号的引脚（如 PA0、PB0、PC0）共享同一条中断线，因此**不能同时使用不同端口的同编号引脚作为中断源**。

| EXTI 线 | 可用引脚 | 中断处理函数名 |
| :--- | :--- | :--- |
| EXTI0 | PA0, PB0, PC0, PD0, PE0 | `EXTI0` |
| EXTI1 | PA1, PB1, PC1, PD1, PE1 | `EXTI1` |
| EXTI2 | PA2, PB2, PC2, PD2, PE2 | `EXTI2` |
| EXTI3 | PA3, PB3, PC3, PD3, PE3 | `EXTI3` |
| EXTI4 | PA4, PB4, PC4, PD4, PE4 | `EXTI4` |
| EXTI5 | PA5, PB5, PC5, PD5, PE5 | `EXTI9_5` |
| EXTI6 | PA6, PB6, PC6, PD6, PE6 | `EXTI9_5` |
| EXTI7 | PA7, PB7, PC7, PD7, PE7 | `EXTI9_5` |
| EXTI8 | PA8, PB8, PC8, PD8, PE8 | `EXTI9_5` |
| EXTI9 | PA9, PB9, PC9, PD9, PE9 | `EXTI9_5` |
| EXTI10 | PA10, PB10, PC10, PD10, PE10 | `EXTI15_10` |
| EXTI11 | PA11, PB11, PC11, PD11, PE11 | `EXTI15_10` |
| EXTI12 | PA12, PB12, PC12, PD12, PE12 | `EXTI15_10` |
| EXTI13 | PA13, PB13, PC13, PD13, PE13 | `EXTI15_10` |
| EXTI14 | PA14, PB14, PC14, PD14, PE14 | `EXTI15_10` |
| EXTI15 | PA15, PB15, PC15, PD15, PE15 | `EXTI15_10` |


---

### 关键步骤总结
使用 STM32F1xx-HAL 配置外部中断的五个核心步骤：

| 步骤 | 操作 | 代码方法 | 说明 |
| :--- | :--- | :--- | :--- |
| 1 | **配置引脚为输入** | `into_pull_up_input()` 等 | 根据外部电路选择上拉/下拉/浮空输入 |
| 2 | **连接到 EXTI 线** | `make_interrupt_source(&mut syscfg)` | 将引脚连接到对应的 EXTI 中断线 |
| 3 | **设置触发边沿** | `trigger_on_edge(&mut exti, Edge::RISING)` | 可选择上升沿、下降沿或双沿触发 |
| 4 | **使能 EXTI 中断** | `enable_interrupt(&mut exti)` | 在 EXTI 外设中启用该中断线 |
| 5 | **NVIC 取消屏蔽** | `NVIC::unmask(pac::Interrupt::EXTI0)` | 在 NVIC 中启用对应的中断通道 |


---

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%A4%96%E9%83%A8%E4%B8%AD%E6%96%ADEXTI/src/main.rs)

难点就是 2021 与 2024 的语法书写 格式，虽然 

```rust
#![deny(unsafe_code)] 
```

也可以解决但是我这种也是一种思路！



还有就是自动匹配 编译器的语法更新

```rust
#[interrupt]
fn EXTI9_5() {
    // 2021 -- Rust
    #[cfg(edition = "2021")]
    let (led, int_pin) = (&mut *LED.as_mut_ptr(), &mut *INT_PIN.as_mut_ptr());

    // 2024 -- Rust
    let (led, int_pin) = (unsafe { &mut *(*(&raw mut LED)).as_mut_ptr() }, unsafe {
        &mut *(*(&raw mut INT_PIN)).as_mut_ptr()
    });

    if int_pin.check_interrupt() {
        rprintln!("进入中断加+1");
        // ====================== 消抖核心代码 ======================
        delay(72_000_000 / 1000 * 40); // 72MHz 时钟 → 延时 40ms
        // ==========================================================
        led.toggle();
        int_pin.clear_interrupt_pending_bit();
    }
}
```



## 定时器中断
### 定时器中断闪烁LED
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%AE%9A%E6%97%B6%E5%99%A8%E4%B8%AD%E6%96%AD/src/main.rs)

逻辑框架图

```plain
┌──────────────────────────────────────────────────┐
│                    main()                        │
│  1. 创建 LED, Timer                               │
│  2. interrupt::free → 移入 G_LED, G_TIM           │
│  3. NVIC::unmask → 使能中断                       │
│  4. wfi() 循环休眠                                │
└──────────────────────────────────────────────────┘
                           │
                    TIM2 中断触发
                           │
┌──────────────────────────────────────────────────┐
│               TIM2() 中断处理                     │
│  1. 从全局存储取出 LED/Timer（首次）                │
│  2. LED.toggle()                                 │
│  3. timer.clear_interrupt()                      │
└──────────────────────────────────────────────────┘
```



### RTIC定时器中断模式
<font style="color:#DF2A3F;"><<开发推荐>> RTIC</font>

#### RTIC vs 裸机中断 对比
##### 一、架构对比
###### 裸机中断（Bare Metal）
```rust
// 需要手动定义中断处理函数，绑定到中断向量表
#[interrupt]
fn TIM1_UP() {
    // 手动进入临界区（关中断）
    cortex_m::interrupt::free(|_| {
        // 所有代码都在一个函数里，资源管理靠程序员自己
        // 没有优先级管理，全靠手动开关中断
    });
}
```

###### RTIC 中断
```rust
// 通过 #[task(binds = ...)] 声明式绑定中断
#[task(binds = TIM1_UP, priority = 1, local = [led, timer])]
fn tick(cx: tick::Context) {
    // 资源由 RTIC 框架管理，编译时保证安全
    // 优先级由 RTIC 调度器自动管理
}
```

---

##### 二、核心区别
| 特性 | 裸机中断 | RTIC |
| --- | --- | --- |
| **资源管理** | 手动用 `critical section` 保护 | 编译时自动分配，零运行时开销 |
| **优先级管理** | 手动操作 NVIC 寄存器 | `priority = N` 声明式配置 |
| **数据共享** | 需要 `static mut` + `unsafe` | `#[shared]` + `Mutex`，编译时安全 |
| **临界区** | 手动开关中断 | RTIC 自动生成最优临界区 |
| **中断绑定** | 修改 `interrupt.rs` 或 `device.x` | `#[task(binds = TIM1_UP)]` 一行搞定 |
| **上下文切换** | 手动保存/恢复寄存器 | 硬件自动压栈（Cortex-M） |
| **代码组织** | 全部逻辑挤在一个中断函数里 | 每个任务独立，资源分离 |
| **死锁防护** | 程序员自己小心 | 编译时检测（基于优先级天花板协议） |


---

##### 三、资源管理对比
###### 裸机方式：`static mut` + `unsafe`
```rust
// 全局可变静态变量，需要 unsafe 访问
static mut LED_STATE: bool = false;
static mut COUNT: u8 = 0;

#[interrupt]
fn TIM1_UP() {
    unsafe {
        if LED_STATE {
            // 操作 LED...
            LED_STATE = false;
        } else {
            // 操作 LED...
            LED_STATE = true;
        }
        COUNT += 1;
    }
}
```

**问题：**

+ `unsafe` 块里出 bug 编译器不管
+ 多个中断访问同一变量容易数据竞争
+ 中断优先级高时可能打断低优先级，破坏数据一致性

###### RTIC 方式：`#[local]` 编译时绑定
```rust
#[task(binds = TIM1_UP, priority = 1, local = [led, led_state: bool = false, count: u8 = 0])]
fn tick(cx: tick::Context) {
    // 每个资源在编译时就绑定到这个任务
    // 其他任务无法访问，天然避免数据竞争
    if *cx.local.led_state {
        cx.local.led.set_high();
        *cx.local.led_state = false;
    }
    *cx.local.count += 1;
}
```

**优势：**

+ 零 `unsafe`，编译器保证正确
+ 资源绑定在编译时确定，零运行时开销
+ 访问资源只需 `cx.local.xxx`

---

##### 四、优先级与调度对比
###### 裸机方式：手动操作 NVIC
```rust
// 需要手动配置优先级
fn setup_timer_interrupt() {
    unsafe {
        // 设置 TIM1 中断优先级为 1
        // 需要知道 NVIC 的寄存器地址和位域
        let nvic = &*cortex_m::peripheral::NVIC::ptr();
        // 复杂的寄存器操作...
    }
}
```

###### RTIC 方式：声明式优先级
```rust
// priority = 1 就搞定了
#[task(binds = TIM1_UP, priority = 1)]
fn tick(cx: tick::Context) { ... }

// 高优先级任务可以抢占低优先级
#[task(binds = USART1, priority = 2)]
fn serial(cx: serial::Context) { ... }
```

**RTIC 优先级规则：**

+ 数字越大优先级越高
+ 高优先级任务可以抢占低优先级任务
+ 同优先级任务不会互相抢占

---

##### 五、共享资源对比
###### 裸机方式：手动临界区
```rust
static mut SHARED_DATA: u32 = 0;

#[interrupt]
fn LOW_PRIORITY() {
    cortex_m::interrupt::free(|_| {
        unsafe { SHARED_DATA += 1; }
    });
}

#[interrupt]
fn HIGH_PRIORITY() {
    // 忘记加临界区？数据竞争！编译器不报错！
    unsafe { SHARED_DATA += 1; }
}
```

###### RTIC 方式：`#[shared]` + 自动锁
```rust
#[shared]
struct Shared {
    data: u32,
}

#[task(binds = TIM1_UP, priority = 1, shared = [data])]
fn tick(cx: tick::Context) {
    // lock() 自动管理临界区，编译时保证安全
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}

#[task(binds = USART1, priority = 2, shared = [data])]
fn serial(cx: serial::Context) {
    // 高优先级任务访问同一资源，RTIC 自动生成最优锁
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}
```

**RTIC 锁机制（优先级天花板协议 PCP）：**

+ 低优先级任务持锁时，高优先级任务等待
+ 但 RTIC 会自动提升持锁任务的优先级，避免中间优先级任务插队
+ 所有这些都是编译时确定的，零运行时开销

---

六、总结

| 场景 | 推荐方式 |
| --- | --- |
| 学习中断原理 | 裸机（理解底层机制） |
| 正式项目开发 | RTIC（安全、高效、可维护） |
| 单个简单中断 | 裸机（代码量差不多） |
| 多中断 + 共享资源 | RTIC（避免数据竞争） |
| 需要严格实时性 | RTIC（零开销抽象，确定性调度） |


**RTIC 的核心优势：**

+ **零开销抽象**：编译时确定一切，运行时无额外开销
+ **编译时安全**：数据竞争、死锁在编译期就被拦截
+ **声明式编程**：用属性宏描述"做什么"，框架生成"怎么做"
+ **优先级天花板协议**：最优化的临界区管理

<font style="color:#DF2A3F;"></font>

#### 如何使用RTIC
这里需要修改config.toml

```bash
# 包添加
[dependencies] 
....
rtic = { version = "2", features = ["thumbv7-backend"] }
```

**原因：**

+ 原项目缺少 `rtic` 依赖，导致无法找到 `rtic` 模块
+ RTIC v2 需要指定后端特性，STM32F103 是 Cortex-M3 架构，使用 `thumbv7-backend`
+ 其他可选后端：`thumbv6-backend`（Cortex-M0/M0+）、`thumbv8base-backend`（Cortex-M23）、`thumbv8main-backend`（Cortex-M33）

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%AE%9A%E6%97%B6%E5%99%A8%E4%B8%AD%E6%96%AD_RTIC_1/src/main.rs)

输出：

```bash
FLASH] Programming via probe-rs...

      Erasing ✔ 100% [####################]   5.00 KiB @   6.69 KiB/s (took 1s)
  Programming ✔ 100% [####################]   5.00 KiB @   4.70 KiB/s (took 1s)                                                   Finished in 1.93s
程序启动：init 函数开始执行
12:16:00.898: RCC 时钟配置完成
12:16:00.898: PC13 LED 引脚配置完成
12:16:00.898: TIM1 定时器配置完成，每 1 秒触发一次中断
12:16:00.898: 进入 idle 空闲循环，CPU 等待中断中...
12:16:01.855: [中断] LED 点亮
12:16:01.855: [中断] 计数: 1
12:16:02.932: [中断] LED 熄灭
12:16:02.932: [中断] 计数: 2
12:16:03.896: [中断] LED 点亮
12:16:03.896: [中断] 计数: 3
12:16:04.849: [中断] LED 熄灭
12:16:04.849: [中断] 计数: 4
12:16:04.849: [中断] 定时器切换为 500ms
12:16:05.333: [中断] LED 点亮
12:16:05.333: [中断] 计数: 5
12:16:05.934: [中断] LED 熄灭
12:16:05.934: [中断] 计数: 6
12:16:06.426: [中断] LED 点亮
12:16:06.426: [中断] 计数: 7
12:16:06.914: [中断] LED 熄灭
12:16:06.914: [中断] 计数: 8
12:16:07.401: [中断] LED 点亮
12:16:07.401: [中断] 计数: 9
12:16:07.888: [中断] LED 熄灭
12:16:07.888: [中断] 计数: 10
12:16:08.377: [中断] LED 点亮
12:16:08.377: [中断] 计数: 11
12:16:08.874: [中断] LED 熄灭
12:16:08.874: [中断] 计数: 12
12:16:08.874: [中断] 定时器切换为 1s，计数器重置
12:16:09.830: [中断] LED 点亮
12:16:09.830: [中断] 计数: 1
```

  
<font style="color:#DF2A3F;">使用hprintln可以规避cortex_m::asm::wfi(); 导致的无法输出问题</font>

```rust
config.toml

[dependencies]
cortex-m-semihosting = "0.5.0"
----------------

use cortex_m_semihosting::hprintln;
...
hprintln!("...");
```





## RTIC2异步任务
使用RTIC实现2个任务的调度 异步模式

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/RTIC2%E5%BC%82%E6%AD%A5%E4%BB%BB%E5%8A%A1/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045924896-cc601051-f619-4596-87f6-9a2246f5de5c.png" width="581" title="" crop="0,0,1,1" id="ud3c4bf51" class="ne-image">

如上图 4次翻转执行一次心跳！同时执行





## 串口通信
#### 普通模式
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045838370-24ec16fe-462e-4267-92b7-5ef5b4c3f955.png" width="1451" title="" crop="0,0,1,1" id="u5f48f617" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045850995-183a5b47-560b-4c32-96d1-59dd0bc6484e.png" width="372" title="" crop="0,0,1,1" id="ud58583c3" class="ne-image">

**各串口可用引脚（F103）：**

| 串口 | TX 引脚 | RX 引脚 | 备注 |
| --- | --- | --- | --- |
| USART1 | PA9 或 PB6(remap) | PA10 或 PB7(remap) | APB2 |
| USART2 | PA2 | PA3 | APB1 |
| USART3 | PB10 | PB11 | APB1 |


**关键概念：**

+ `into_alternate_push_pull()` — 复用推挽输出，引脚由硬件外设控制
+ `Config::default().baudrate()` — 串口配置（波特率、数据位、停止位等）
+ `.split()` — 拆分为独立的 `Tx` 和 `Rx` 对象
+ `.reunite()` — 将 `Tx` 和 `Rx` 重新合并



#### 串口通信_fmt 模式
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1_fmt/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047558289-b27d36c0-42f4-4c70-b44b-6278e05bc28c.png" width="492" title="" crop="0,0,1,1" id="u784935b0" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047584627-e3e1ce36-b097-4508-959a-6fa5562f7ca5.png" width="624" title="" crop="0,0,1,1" id="uc9269666" class="ne-image">

就是如上的信息个人觉得比较完美，这个语言的优势就是框架做好了给AI完成任务！



#### 串口中断_空闲检测
主要是调用IDLE模式

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1_%E4%B8%AD%E6%96%AD_IDLE/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780070623115-035fd2f2-c6db-41b9-87e6-58d7dbe34ef2.png" width="638" title="" crop="0,0,1,1" id="uec811e07" class="ne-image">



**9位数据位模式，用第 9 位标记地址/数据。**

```rust
// 配置为 9 位数据位
let serial = p.USART3.serial::<PushPull>(
    (tx_pin, rx_pin),
    Config::default()
        .baudrate(9600.bps())
        .wordlength_9bits()    // 9 位数据位
        .parity_none(),        // 无校验
    &mut rcc,
);

// 第 9 位 = 1 表示地址字节
// 第 9 位 = 0 表示数据字节
block!(serial_tx.write(SLAVE_ADDR as u16 | 0x100)).unwrap();  // 发送地址
block!(serial_tx.write(data_byte)).unwrap();                   // 发送数据
```

**用途：** 多机通信中区分地址帧和数据帧。



#### 串口DMA接收
[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1_fmt_DMA/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071793247-2af14a44-7e27-459a-8ed3-05ec1a3abc0a.png" width="342" title="" crop="0,0,1,1" id="u7dbb3119" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071860321-d37bfaff-e264-4099-a590-ef16f984731d.png" width="328" title="" crop="0,0,1,1" id="uabd331a7" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071685474-f076938b-fd10-4f3b-8796-4be8b770bf0e.png" width="306" title="" crop="0,0,1,1" id="u3ad590c6" class="ne-image">

解释了如何存了8位数据才会触发！



## ADC采集
**ADC 关键参数：**

+ 分辨率：12 位（0-4095）
+ 转换时间：取决于 ADC 时钟
+ 参考电压：VDDA（通常 3.3V）
+ 公式：`电压 = 读数 / 4095 * 3.3V`

**DKX 板可用 ADC 通道：**

| 引脚 | ADC 通道 |
| --- | --- |
| PA0 | ADC1_IN0 |
| PA1 | ADC1_IN1 |
| PA2 | ADC1_IN2 |
| PA3 | ADC1_IN3 |
| PA4 | ADC1_IN4 |
| PA5 | ADC1_IN5 |
| PA6 | ADC1_IN6 |
| PA7 | ADC1_IN7 |
| PB0 | ADC1_IN8 |
| PB1 | ADC1_IN9 |


### 采集外部电压
采集PB01引脚电压

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/ADC%E7%94%B5%E5%8E%8B%E9%87%87%E9%9B%86/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073349544-41a745c3-faaa-44c9-8ada-70e709ed8da6.png" width="514" title="" crop="0,0,1,1" id="ub9d403d9" class="ne-image">

接地

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073451277-a7b6496a-b345-4f55-8089-a2763cb2c721.png" width="536" title="" crop="0,0,1,1" id="u294fb1ca" class="ne-image">

接3.3V

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073526370-be39c8d3-3198-4e0e-81fd-9e58c6035ba3.png" width="529" title="" crop="0,0,1,1" id="u69367ae2" class="ne-image">

2个1K 电阻串联测的中间电压



### 内部ADC采集转换温度
**内部温度传感器：**

+ 连接到 ADC1 通道 16
+ 精度不高（±1.5°C），适合粗略监测
+ 转换时间需要 17.1μs 以上

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%86%85%E9%83%A8ADC%E8%BD%AC%E6%8D%A2%E6%B8%A9%E5%BA%A6/src/main.rs)



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780074073750-6029e96f-c1bf-4b19-856a-141ae6e4a1a5.png" width="400" title="" crop="0,0,1,1" id="ubecbebec" class="ne-image">



### ADC_DMA_循环采集
**循环 DMA 工作原理：**

```plain
     缓冲区 A          缓冲区 B
┌─────────────┐  ┌─────────────┐
│ [0] [1] ... │  │ [0] [1] ... │
│    [7]      │  │    [7]      │
└─────────────┘  └─────────────┘
       ↑ DMA 写入    ↑ DMA 写入
       └── 交替进行 ──┘

Half::First  → 缓冲区 A 可读
Half::Second → 缓冲区 B 可读
```

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/ADC_DMA%E5%BE%AA%E7%8E%AF%E9%87%87%E9%9B%86/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780123010814-cbd8950f-c10d-4fd8-b2e0-13887353b25e.png" width="625" title="" crop="0,0,1,1" id="uc37cf9c3" class="ne-image">



## SPI协议
**SPI 模式详解：**

| 模式 | CPOL | CPHA | 时钟空闲 | 数据采样 |
| --- | --- | --- | --- | --- |
| Mode 0 | 0 | 0 | 低 | 第一个边沿 |
| Mode 1 | 0 | 1 | 低 | 第二个边沿 |
| Mode 2 | 1 | 0 | 高 | 第一个边沿 |
| Mode 3 | 1 | 1 | 高 | 第二个边沿 |


**嘉立创STM32F103C8T6板 SPI1 引脚：** PA5(SCK), PA6(MISO), PA7(MOSI), PA4(CS)



### 点亮ST7789屏幕
240*240

**硬件引脚连接表**

| 显示屏引脚 | MCU 引脚 | 功能说明 |
| :--- | :--- | :--- |
| SCL | PA5 | SPI 时钟线 (SPI1_SCK) |
| SDA | PA7 | SPI 数据输出 (SPI1_MOSI) |
| DC | PA0 | 命令/数据选择 |
| RES | PA1 | 硬件复位 |
| CS | GND | 片选拉低（始终选中） |


> **说明**：表中 “SCL” 和 “SDA” 通常为 I²C 总线信号名，但此处连接的是 SPI 接口，实际对应 SPI 的 **SCK** 和 **MOSI**。这种命名常见于某些 LCD 模块，实际功能按引脚名称使用即可。CS 接 GND 表示该 SPI 设备始终被选中，无需软件控制片选。
>

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780126926273-223244e9-d923-467e-be77-c109bd054a91.png" width="272" title="" crop="0,0,1,1" id="ucea3e5ce" class="ne-image">

ST7789驱动代码----src/st7789.rs

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/SPI/src/st7789.rs)

主程序

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/SPI/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780127246884-25c80e3f-a8bb-4171-8cb5-e03b616cd2af.png" width="543" title="" crop="0,0,1,1" id="u63cb4aa1" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780127264032-9cb415a8-b939-4689-a366-1cc4a698b1a0.png" width="391" title="" crop="0,0,1,1" id="u71c90ac3" class="ne-image">



## I2C通信
以地址扫描为案例！

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129573451-695873a0-2895-4ae6-82d9-760b0735e6d3.png" width="547" title="" crop="0,0,1,1" id="uf3139d39" class="ne-image">

**接线**

| MCU | 设备 |
| --- | --- |
| PB6 | SCL |
| PB7 | SDA |


[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/I2C%E5%9C%B0%E5%9D%80%E6%89%AB%E6%8F%8F/src/main.rs)

接线如图

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129746845-ddca9982-5283-4dd1-9def-439f14f69b5e.png" width="645" title="" crop="0,0,1,1" id="u6a6d07a1" class="ne-image">



## PWM波
### 输出
我们以PWM控制舵机来讲解

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780132432825-534b3892-b8fd-4f4d-ace6-0350c6c16588.png" width="589" title="" crop="0,0,1,1" id="u1a6aff10" class="ne-image">

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/PWM%E8%88%B5%E6%9C%BA/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780132461432-60a818a5-4ab8-4cf6-afa6-a0009cb11d23.png" width="342" title="" crop="0,0,1,1" id="u22f9ccbe" class="ne-image">



### 输入
我们以EC11编码器来讲解

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780133888771-3dfd0d4c-f65c-4ded-af72-16faef0fcbed.png" width="394" title="" crop="0,0,1,1" id="ue032c32d" class="ne-image">

EC11(带按键)

**接线**

| MCU | 设备 |
| --- | --- |
| PB4 | S2 |
| PB5 | S2 |


[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/PWM%E8%BE%93%E5%85%A5%E6%A3%80%E6%B5%8B/src/main.rs)

旋转编码器检测出数据！不旋转无输出！

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780134186205-bb66094b-f5f1-49a0-8f13-673b832f66e1.png" width="480" title="" crop="0,0,1,1" id="u43c0b949" class="ne-image">

输出检测信息



### EC11编码器读取
**接线**

| MCU | 设备 |
| --- | --- |
| PB6 | S1 |
| PB7 | S2 |


**用途：** 电机编码器、旋转旋钮等正交编码器设备的速度/位置测量。

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/EC11%E7%BC%96%E7%A0%81%E5%99%A8/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780136893674-70671b8b-3b27-429a-8a79-12fdca9cbd25.png" width="499" title="" crop="0,0,1,1" id="u76a64ffa" class="ne-image">



## CRC校验
**用途：** 数据完整性校验、通信协议的 CRC 校验。

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/CRC%E6%A0%A1%E9%AA%8C/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780138752178-cbb34d1f-7689-4eb2-a05c-651295f45c4d.png" width="590" title="" crop="0,0,1,1" id="u603a4730" class="ne-image">



## DAC 数模转换
> 注意：STM32F103C8T6 (DKX 板) **没有 DAC**，DAC 仅在高密度设备（STM32F103xC/D/E）上可用。
>

注意这里C8T6不支持，所以我们选择STM32F103RCT6

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780139242972-43a95357-bf90-4333-a02f-a114f1654d5c.png" width="1299" title="" crop="0,0,1,1" id="u48c78d02" class="ne-image">

memory.x

```rust
/* Linker script for the STM32F103RCT6  https://probe.rs/targets/?q=&p=0 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 48K
}
```

更改config.tom中的文件配置

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780139752540-46050614-f41b-4e59-a165-a384a1b0b9ab.png" width="659" title="" crop="0,0,1,1" id="ud3f9d220" class="ne-image">

```rust
# stm32f1xx-hal：STM32F1 系列的硬件抽象层
# 提供 RCC、GPIO、TIM、USART 等外设的高级 Rust API
[dependencies.stm32f1xx-hal]
version = "0.11.0"
features = [
    "stm32f103",  # STM32F103 系列芯片
    "high",       # 高密度（256KB Flash 或以上），RCT6 属于此类型
]
```

代码如下

[代码见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/DAC%E6%95%B0%E6%A8%A1%E8%BD%AC%E6%8D%A2/src/main.rs)

结果如下

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140722259-090abde4-4b38-4c94-b484-0cf40c1bf96a.png" width="396" title="" crop="0,0,1,1" id="u15e94679" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140832880-63b745a7-86fc-43cd-8947-01980100a6dc.png" width="560" title="" crop="0,0,1,1" id="u415533ee" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140776299-cfdce37d-a05f-46cd-a8e9-85e1fe758aa5.png" width="563" title="" crop="0,0,1,1" id="ud77ae8bf" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140863989-0e947147-c83d-4fa5-b09c-59dbb4a05e95.png" width="565" title="" crop="0,0,1,1" id="u57f96e96" class="ne-image">

测量结果在可以承受的误差内



## CAN 总线（无设备验证-未测试）
```markdown
use bxcan::Fifo;
use bxcan::filter::Mask32;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // CAN 需要外部晶振保证时钟精度
    let mut rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()), &mut flash.acr);

    let mut can1 = {
        let gpioa = dp.GPIOA.split(&mut rcc);
        let rx = gpioa.pa11;  // CAN RX
        let tx = gpioa.pa12;  // CAN TX

        let can = dp.CAN.can(dp.USB, (tx, rx), &mut rcc);

        // 配置位时序：125kBit/s, 采样点 87.5%
        bxcan::Can::builder(can)
            .set_bit_timing(0x001c_0003)
            .leave_disabled()
    };

    // 配置过滤器（接收所有帧）
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters);

    // 使能 CAN
    let mut can = can1;
    block!(can.enable_non_blocking()).unwrap();

    // 回环测试：接收帧后立即发回
    loop {
        if let Ok(frame) = block!(can.receive()) {
            block!(can.transmit(&frame)).unwrap();
        }
    }
}
```

**CAN 引脚（DKX 板）：** PA11(CAN RX), PA12(CAN TX) — 注意与 USB 引脚共用



## USB 串口（无设备验证-未测试）
### USB轮询串口（无设备验证-未测试）
```rust
#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f1xx_hal::usb::{Peripheral, UsbBus};
use stm32f1xx_hal::{pac, prelude::*, rcc};
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // USB 必须使用 48MHz 系统时钟
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()).sysclk(48.MHz()).pclk1(24.MHz()),
        &mut flash.acr,
    );

    assert!(rcc.clocks.usbclk_valid());  // 验证 USB 时钟有效

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_high();  // 关灯

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // USB D+ 线上有上拉电阻
    // 开发时需要拉低 D+ 触发 USB RESET
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();                           // 拉低 D+
    delay(rcc.clocks.sysclk().raw() / 100);     // 短暂延时

    // 配置 USB 外设
    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,                                  // USB DM = PA11
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),  // USB DP = PA12
    };
    let usb_bus = UsbBus::new(usb);

    // 创建 CDC-ACM 串口设备
    let mut serial = SerialPort::new(&usb_bus);

    // 构建 USB 设备
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(USB_CLASS_CDC)
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake Company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];
        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_low();  // 点亮 LED
                // 将收到的字符转为大写后回显
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;  // 'a'~'z' → 'A'~'Z'
                    }
                }
                // 写回（可能需要多次写入）
                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        led.set_high();  // 关灯
    }
}
```

**USB 关键点：**

+ **必须 48MHz 系统时钟**（USB 协议要求精确时钟）
+ PA11 = USB D-, PA12 = USB D+
+ 开发时需要手动触发 USB RESET
+ VID/PID `0x16c0:0x27dd` 是测试用的非正式 ID
+ 需要 release 模式编译（debug 模式 FLASH 会溢出）

---

### USB中断串口（无设备验证-未测试）
使用中断方式处理 USB 通信。

```rust
// 全局 USB 对象
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;

#[entry]
fn main() -> ! {
    // ... USB 初始化代码 ...

    // 使能 USB 中断
    unsafe {
        NVIC::unmask(Interrupt::USB_HP_CAN_TX);   // 高优先级
        NVIC::unmask(Interrupt::USB_LP_CAN_RX0);  // 低优先级
    }

    loop { wfi(); }  // 所有工作在中断中完成
}

#[interrupt]
fn USB_HP_CAN_TX() {
    usb_interrupt();
}

#[interrupt]
fn USB_LP_CAN_RX0() {
    usb_interrupt();
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev.poll(&mut [serial]) {
        return;
    }

    let mut buf = [0u8; 8];
    match serial.read(&mut buf) {
        Ok(count) if count > 0 => {
            // 处理接收到的数据
            for c in buf[0..count].iter_mut() {
                if 0x61 <= *c && *c <= 0x7a {
                    *c &= !0x20;  // 转大写
                }
            }
            serial.write(&buf[0..count]).ok();
        }
        _ => {}
    }
}
```

**轮询 vs 中断：**

+ 轮询：简单，但 CPU 一直在忙等
+ 中断：CPU 可以休眠，节省功耗



# 实际项目
## DHT11
[项目地址见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/tree/main/Dome/DHT11)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780160254218-22ef0a3f-9b61-4350-b558-f75ed994c24f.png" width="450" title="" crop="0,0,1,1" id="uecb29377" class="ne-image">



## DHT11+ST7789液晶屏温度计
| 显示屏&DHT11 | MCU 引脚 | 功能说明 |
| :--- | :--- | :--- |
| SCL | PA5 | SPI 时钟线 (SPI1_SCK) |
| SDA | PA7 | SPI 数据输出 (SPI1_MOSI) |
| DC | PA0 | 命令/数据选择 |
| RES | PA1 | 硬件复位 |
| CS | GND | 片选拉低（始终选中） |
| DATA | PA6 | DHT11数据线 |


实际效果图

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164028402-54ee6f36-3789-4788-b616-7f4f67a666ce.png" width="628" title="" crop="0,0,1,1" id="u132cff35" class="ne-image">

项目结构图

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164081628-3762ec47-4c6c-4b87-ad2f-7dbf972396b9.png" width="812" title="" crop="0,0,1,1" id="u5e153928" class="ne-image">

 [项目地址见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/tree/main/Dome/DHT11%2BST7789)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164408965-82bc27da-8093-4c6a-9042-138ae538676f.png" width="680" title="" crop="0,0,1,1" id="u7c1aa107" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164569870-c81ed584-799f-4093-8e82-20d2ca0bd4bd.png" width="720" title="" crop="0,0,1,1" id="u1d403555" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164484495-b510887a-e182-4bc3-b09e-29dadc3900a7.png" width="577" title="" crop="0,0,1,1" id="u612677a5" class="ne-image">

别问我为啥用python 因为它快！



## DHT20+ST7789液晶屏温度计
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780166394105-ec6c2ced-4711-4e04-9992-2d3939656899.png" width="547" title="" crop="0,0,1,1" id="u7f142962" class="ne-image">

| 显示屏&DHT20 | MCU 引脚 | 功能说明 |
| :--- | :--- | :--- |
| SCL（显示器） | PA5 | SPI 时钟线 (SPI1_SCK) |
| SDA（显示器） | PA7 | SPI 数据输出 (SPI1_MOSI) |
| DC | PA0 | 命令/数据选择 |
| RES | PA1 | 硬件复位 |
| CS | GND | 片选拉低（始终选中） |
| SCL（DHT20） | PB7 | DHT20时钟线 |
| SDA（DHT20） | PB6 | DHT20数据线 |


[项目地址见这里](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/tree/main/Dome/DHT20%2BST7789)

## ST7789 3D立方体（无颜色-框架）
在ST7789 240x240显示屏上渲染旋转的3D立方体，支持彩色面、线框渲染和FPS计数器。

**硬件引脚连接（同SPI显示屏）：**

| 显示屏引脚 | MCU 引脚 | 功能说明 |
| :--- | :--- | :--- |
| SCL | PA5 | SPI 时钟线 (SPI1_SCK) |
| SDA | PA7 | SPI 数据输出 (SPI1_MOSI) |
| DC | PA0 | 命令/数据选择 |
| RES | PA1 | 硬件复位 |
| CS | GND | 片选拉低（始终选中） |

**功能特性：**

+ 6面彩色立方体（红、黄、绿、青、蓝、紫）
+ 线框边缘渲染，带脏矩形优化
+ 实时FPS计数器显示
+ 可配置立方体大小、视场角、旋转速度
+ SPI时钟：36MHz

[项目源代码](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/ST7789_cube3d_No_Color)



# 结语
作者邮箱：pycx0@qq.com  
其实作者是在大四即将毕业的时候来完成这个项目的，原因是China就业压力太大了，目前找工作中ing！  
后续更新会在原文链接中！期待找个好工作！

期待Rust生态越来越好！加油地球村的“村民”们！
