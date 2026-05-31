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

```rust
# [package] 段：定义包的元数据信息
[package]
# 包的名称，用于在 crates.io 上发布或被其他项目引用时标识
name = "stm32dome"
# 包的版本号，遵循语义化版本规范（Semantic Versioning）
version = "0.1.0"
# 使用的 Rust 版本约定，2024 是最新的版本，启用了最新的语言特性
edition = "2024"

# [dependencies] 段：定义项目的依赖项
[dependencies]
# embedded-hal：嵌入式硬件抽象层标准接口
# 版本 "1.0" 表示使用 1.0.x 的最新版本
# 定义了 GPIO、I2C、SPI、串口等通用 trait，使得驱动代码可在不同 MCU 间移植
embedded-hal = "1.0"

# nb：non-blocking（非阻塞）操作库
# 提供 Result 和 block! 宏，用于处理可能需要重试的操作（如串口发送）
nb = "1"

# cortex-m：ARM Cortex-M 处理器的底层支持库
# 提供系统寄存器访问、中断管理、系统定时器等功能
cortex-m = "0.7.7"

# cortex-m-rt：Cortex-M 运行时库
# 提供启动代码、中断向量表、内存初始化等运行时支持
# 有了它才能使用 #[entry] 宏定义程序入口点
cortex-m-rt = "0.7.5"
panic-halt = "1.0.0"
rtt-target = "0.6.2"
rtic = { version = "2", features = ["thumbv7-backend"] }
rtic-monotonics = { version = "2", features = ["cortex-m-systick"] }


# [dependencies.stm32f1xx-hal]：特定依赖的详细配置
# 使用表格语法为 stm32f1xx-hal 提供更详细的配置
[dependencies.stm32f1xx-hal]
# 指定版本号
version = "0.11.0"
# features：启用编译时特性
# "stm32f103"：选择 STM32F103 系列芯片的支持代码
# "medium"：中等密度芯片配置（64-128KB Flash），C8T6 属于此类
# 其他选项还有 "low"（低密度，16-32KB）和 "high"（高密度，256KB+）
features = ["stm32f103", "medium"]


[profile.dev]
incremental = false   # 关闭增量编译，确保嵌入式构建一致性
codegen-units = 1     # 单代码生成单元，让编译器做更多优化
opt-level = 1         # 轻度优化，避免 debug 模式下中断处理太慢
panic = "abort"       # panic 时直接终止，不展开栈（嵌入式无栈展开支持）

[profile.release]
codegen-units = 1
debug = true          # 保留调试信息（不影响性能，方便用调试器排查问题）
lto = true            # 链接时优化，跨 crate 优化以减小体积
panic = "abort"
```

.cargo/cargo.toml

```rust
[target.thumbv7m-none-eabi]
# ============================================================
# Runner — st-mem runner (跨平台，先分析内存再烧录)
# ============================================================
# st-mem runner: 分析 FLASH/RAM 占用 → probe-rs 烧录
runner = "st-mem runner --chip STM32F103C8 --protocol swd"
# ============================================================
# 不使用内存分析时，直接用 probe-rs:
# runner = "probe-rs run --chip STM32F103C8 --protocol swd"
# ============================================================
rustflags = [
  "-C", "link-arg=-Tlink.x",
#   "-C", "link-arg=-Tdefmt.x",
]

[build]
target = "thumbv7m-none-eabi"

[env]
DEFMT_LOG = "info"

[alias]
r = "run --release"
```



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

```plain
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "STM Debug",
            "cwd": "${workspaceFolder}",
            //!MODIFY (or remove)
            "speed": 24000,
            //!MODIFY (or remove)
            //   "probe": "VID:PID:<Serial>",
            "runtimeExecutable": "probe-rs",
            "runtimeArgs": [
                "dap-server"
            ],
            //!MODIFY
            "chip": "STM32F103C8", // 芯片型号 更具自己的修改！
            "flashingConfig": {
                "flashingEnabled": true,
                "haltAfterReset": false,
                "formatOptions": {
                    //!MODIFY (or remove). Valid values are: 'bin', 'hex', 'elf'(default), 'idf'
                    "binaryFormat": "elf"
                }
            },
            "coreConfigs": [
                {
                    "coreIndex": 0,
                    //!MODIFY
                    "programBinary": "target/thumbv7m-none-eabi/debug/${workspaceFolderBasename}",
                    //!MODIFY
                    // "svdFile": "Relative or fully qualified path to the CMSIS-SVD file for your target core"
                    // 开启RTT
                    "rttEnabled": true
                }
            ],
            "env": {
                //!MODIFY (or remove)
                // If you set this variable, check the VSCode console log window for the location of the log file.
                "RUST_LOG": "info"
            },
            // Info, Debug
            "consoleLogLevel": "Console"
        }
    ]
}
```



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
```rust
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
```

****

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
```rust
#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m::delay;
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

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        rprintln!("设置高电平");
        led.set_high();
        delay.delay_ms(1_800_u16);

        rprintln!("设置低电平");
        led.set_low();
        delay.delay(1.secs());
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779801980743-ec54e1f1-3737-4cd9-adfa-8e4a6a99ec8e.jpeg" width="281" title="" crop="0,0,1,1" id="u62526734" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779802010301-0b3dbe14-df37-444f-9f75-1f9dcb40b870.jpeg" width="280" title="" crop="0,0,1,1" id="u6a547c40" class="ne-image">

如上图LED闪烁

### LED闪烁-TIM2定时器延时
```rust
#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, rcc};
use cortex_m_rt::{entry}; 

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

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut delay = dp.TIM2.delay_us(&mut rcc); // 使用TIM2 实现

    loop {
        rprintln!("TIM2 定时器");
        led.set_high();
        delay.delay_ms(1_800_u16);

        led.set_low();
        delay.delay(1.secs());
    }
}
```

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
```rust
#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{gpio::PinState, pac, prelude::*, rcc};

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

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    let (mut led1, mut led2) = (
        gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High),
        gpioc
            .pc14
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::Low),
    );

    // 禁用 JTAG，释放 PA15、PB3、PB4 作为普通 GPIO
    // STM32F1 默认 PA13/PA14/PA15/PB3/PB4 是 JTAG/SWD 引脚
    // 使用普通 GPIO 前必须先释放
    let mut afio = dp.AFIO.constrain(&mut rcc);
    let (gpioa_pa15, _gpiob_pb3, _gpiob_pb4) =
        afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let key_0 = gpiob.pb12.into_pull_up_input(&mut gpiob.crh);
    let key_1 = gpioa_pa15.into_pull_up_input(&mut gpioa.crh);

    let mut key_up: bool = true;
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cp.SYST.delay(&rcc.clocks);
    loop {
        let key_result = (key_0.is_low(), key_1.is_low());
        if key_up && (key_result.0 || key_result.1) {
            key_up = false;
            delay.delay_ms(20_u8);
            rprintln!(
                "{}",
                if key_result.0 {
                    "按下了按键0"
                } else {
                    "按下了按键1"
                }
            );
            match key_result {
                (true, _) => led1.toggle(),
                (_, true) => led2.toggle(),
                (_, _) => (),
            }
        } else if !key_result.0 && !key_result.1 {
            key_up = true;
            // nanos() 纳秒; micros() 微秒; millis() 毫秒; secs() 秒; minutes() 分; hours() 时
            delay.delay(20.millis());
        } else {
            // rprintln!("出错！");
            // delay.delay(2.secs());
        }
    }
}
```

****

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

```rust
#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use nb::block;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, rcc, timer::Timer};

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
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // 创建动态引脚（可以在运行时切换输入/输出模式）
    let mut pin = gpioc.pc13.into_dynamic(&mut gpioc.crh);

    let cp = cortex_m::Peripherals::take().unwrap();

    // TOOD:1
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    timer.start(6.Hz()).unwrap();

    // TOOD:2
    // let mut timer = cp.SYST.counter_hz(&rcc.clocks);
    // timer.start(5.Hz()).unwrap();

    // TOOD:3
    // let mut timer: stm32f1xx_hal::timer::SysCounter<72000000> = cp.SYST.counter(&mut rcc.clocks);
    // timer.start(200.millis()).unwrap();

    loop {
        pin.make_floating_input(&mut gpioc.crh);
        block!(timer.wait()).unwrap();
        rprintln!("{}", pin.is_high().unwrap());

        pin.make_push_pull_output(&mut gpioc.crh);
        pin.set_high().unwrap();
        block!(timer.wait()).unwrap();
        pin.set_low().unwrap();
        block!(timer.wait()).unwrap();
    }
}
```



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

```rust
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
    // let led = unsafe { &mut *LED.as_mut_ptr() };
    // let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };

    // 2024 -- Rust
    let led = unsafe {
        &mut *(*(&raw mut LED)).as_mut_ptr()
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
    };
    let int_pin = unsafe { &mut *(*(&raw mut INT_PIN)).as_mut_ptr() };

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
```

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
```rust
#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{asm::wfi, interrupt::Mutex};

use cortex_m_rt::entry;

use panic_halt as _;

use rtt_target::{rprintln, rtt_init_print};

use stm32f1xx_hal::{
    gpio::{GpioExt, Output, PC13, PinState, PushPull},
    pac::{Interrupt, Peripherals, TIM2, interrupt},
    prelude::*,
    rcc,
    timer::{CounterHz, Event},
};

type LedPin = PC13<Output<PushPull>>;

static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

static G_TIM: Mutex<RefCell<Option<CounterHz<TIM2>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        let mut led_ref = G_LED.borrow(cs).borrow_mut();
        let mut tim_ref = G_TIM.borrow(cs).borrow_mut();

        if let (Some(led), Some(tim)) = (led_ref.as_mut(), tim_ref.as_mut()) {
            led.toggle();
            tim.clear_interrupt(Event::Update);
            rprintln!("TIM2 中断");
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("程序启动");

    let dp = Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let led = Output::new(gpioc.pc13, &mut gpioc.crh, PinState::High);

    cortex_m::interrupt::free(|cs| {
        *G_LED.borrow(cs).borrow_mut() = Some(led);
    });

    rprintln!("GPIO 初始化完成");

    let mut timer = dp.TIM2.counter_hz(&mut rcc);
    timer.start(6.Hz()).unwrap();
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| {
        *G_TIM.borrow(cs).borrow_mut() = Some(timer);
    });

    rprintln!("TIM2 初始化完成");

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    rprintln!("程序运行中");

    loop {
        // Wait For Interrupt，CPU 进入低功耗休眠 --- 生产中建议不写！
        // wfi(); // 启动后会出现 -- rprintln! 无法打印
    }
}
```

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



```rust
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
    // 导入 RTT（实时传输）调试输出宏
    // rtt_init_print! 用于初始化 RTT 输出通道
    // rprintln! 用于通过 RTT 打印调试信息（需要 J-Link/ST-Link 等调试器）
    use rtt_target::{rprintln, rtt_init_print};

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
        // 初始化 RTT 调试输出通道
        // 之后就可以用 rprintln! 打印调试信息了
        rtt_init_print!();
        rprintln!("程序启动：init 函数开始执行");

        // 获取 RCC（复位和时钟控制）外设并约束它
        // constrain() 会将 RCC 配置为默认状态，返回一个时钟配置对象
        let mut rcc = cx.device.RCC.constrain();
        rprintln!("RCC 时钟配置完成");

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
        rprintln!("PC13 LED 引脚配置完成");

        // 配置 TIM1 定时器为毫秒精度计数器
        // counter_ms() 将 TIM1 配置为毫秒精度的定时器
        let mut timer = cx.device.TIM1.counter_ms(&mut rcc);
        // 启动定时器，每 1 秒触发一次更新事件
        timer.start(1.secs()).unwrap();
        // 启用定时器的更新中断（Update Event）
        // 当定时器计数溢出时会触发中断
        timer.listen(Event::Update);
        rprintln!("TIM1 定时器配置完成，每 1 秒触发一次中断");

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
        rprintln!("进入 idle 空闲循环，CPU 等待中断中...");
        loop {
            // WFI（Wait For Interrupt）：让 CPU 进入低功耗等待状态
            // CPU 会在中断发生时自动唤醒，执行完中断处理后回到这里
            //
            // 注意：启用 wfi() 后，RTT 调试输出可能无法正常刷新
            // 如果需要查看 idle 中的 rprintln! 输出，请注释掉 wfi()
            // cortex_m::asm::dsb();
            // cortex_m::asm::wfi();
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
            rprintln!("[中断] LED 熄灭");
        } else {
            // set_low()：输出低电平（点亮 LED）
            cx.local.led.set_low();
            *cx.local.led_state = true;
            rprintln!("[中断] LED 点亮");
        }

        // 中断计数器加 1
        // 用于控制定时器频率的切换时机
        *cx.local.count += 1;
        rprintln!("[中断] 计数: {}", *cx.local.count);

        // 动态改变定时器的触发频率
        // 第 4 次中断：将定时器改为 500ms 触发一次（LED 闪烁加快）
        if *cx.local.count == 4 {
            cx.local.timer_handler.start(500.millis()).unwrap();
            rprintln!("[中断] 定时器切换为 500ms");
        }
        // 第 12 次中断：将定时器改回 1 秒触发一次（LED 闪烁恢复慢速）
        // 并重置计数器，开始新的周期
        else if *cx.local.count == 12 {
            cx.local.timer_handler.start(1.secs()).unwrap();
            *cx.local.count = 0;
            rprintln!("[中断] 定时器切换为 1s，计数器重置");
        }

        // 清除定时器的更新中断标志
        // 必须手动清除，否则中断会持续触发
        cx.local.timer_handler.clear_interrupt(Event::Update);
    }
}
```

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



```rust
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
```



输出

```rust
     Finished in 1.39s
程序启动：init 函数开始执行
RCC 时钟配置完成
PC13 LED 引脚配置完成
TIM1 定时器配置完成，每 1 秒触发一次中断
进入 idle 空闲循环，CPU 等待中断中...
[中断] LED 点亮
[中断] 计数: 1
[中断] LED 熄灭
[中断] 计数: 2
[中断] LED 点亮
[中断] 计数: 3
```



## RTIC2异步任务
使用RTIC实现2个任务的调度 异步模式

```rust
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
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045924896-cc601051-f619-4596-87f6-9a2246f5de5c.png" width="581" title="" crop="0,0,1,1" id="ud3c4bf51" class="ne-image">

如上图 4次翻转执行一次心跳！同时执行





## 串口通信
#### 普通模式
```rust
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
```

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
```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
    rcc
};
use core::fmt::Write;  // 导入 Write trait

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

    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::new(
        dp.USART3,
        (tx, rx),
        Config::default().baudrate(115200.bps()),
        &mut rcc,
    );

    let (mut tx, _rx) = serial.split();

    let mut number = 0;
    // 使用 write! 宏格式化输出
    writeln!(tx, "Hello formatted string {}", number).unwrap();
    // Windows 换行: write!(tx, "Hello formatted string {}\r\n", number)


    let mut delay = dp.TIM2.delay_us(&mut rcc); // 使用TIM2 实现

    loop {
        writeln!(tx, "Hello formatted string {}", number).unwrap();
        delay.delay_ms(2_000_u16);
        number += 1;
        rprintln!("调试反馈:Hello formatted string {}",number);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047558289-b27d36c0-42f4-4c70-b44b-6278e05bc28c.png" width="492" title="" crop="0,0,1,1" id="u784935b0" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047584627-e3e1ce36-b097-4508-959a-6fa5562f7ca5.png" width="624" title="" crop="0,0,1,1" id="uc9269666" class="ne-image">

就是如上的信息个人觉得比较完美，这个语言的优势就是框架做好了给AI完成任务！



#### 串口中断_空闲检测
主要是调用IDLE模式

```rust
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
```

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
```rust
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
```

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

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    adc,
    pac,
    prelude::*,
    rcc,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("ADC 电压采集");
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // 配置时钟：HSE 8MHz，SYSCLK 72MHz，ADCCLK 14MHz
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    rprintln!("adc freq: {}", rcc.clocks.adcclk());

    // 初始化 ADC1
    let mut adc1 = adc::Adc::new(dp.ADC1, &mut rcc);

    // 配置 PB0 为模拟输入
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut ch0 = gpiob.pb1.into_analog(&mut gpiob.crl);

    // 初始化 SysTick 延时器
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, rcc.clocks.sysclk().raw());

    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        // 参考电压 3.3V，12 位 ADC (0-4095)
        let voltage_mv = data as u32 * 3300 / 4095;
        let voltage_v = voltage_mv as f32 / 1000.0;
        rprintln!("adc1: {}  |  {}mV  |  {:.3}V", data, voltage_mv, voltage_v);
        delay.delay_ms(600u32);
    }
}
```

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

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    adc,
    pac,
    prelude::*,
    rcc,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("内部 ADC 温度传感器测试");
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
    rprintln!("sysclk freq: {}", rcc.clocks.sysclk());
    rprintln!("adc freq: {}", rcc.clocks.adcclk());

    let mut adc1 = adc::Adc::new(dp.ADC1, &mut rcc);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, rcc.clocks.sysclk().raw());

    loop {
        let temp = adc1.read_temp();
        rprintln!("temp: {} C", temp);
        delay.delay_ms(1000u32);
    }
}
```



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



```rust
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
```

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

```rust
#![allow(dead_code)]

use stm32f1xx_hal::{
    pac,
    spi::{Spi, Error as SpiError},
    rcc::Clocks,
};

pub struct ST7789<DC, RST> {
    dc: DC,
    rst: RST,
    width: u16,
    height: u16,
}

impl<DC: embedded_hal::digital::OutputPin, RST: embedded_hal::digital::OutputPin> ST7789<DC, RST> {
    pub fn new(dc: DC, rst: RST) -> Self {
        ST7789 {
            dc,
            rst,
            width: 240,
            height: 240,
        }
    }

    pub fn init<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, clocks: &Clocks) -> Result<(), SpiError> {
        self.hard_reset(clocks);

        self.write_command(spi, 0x11)?;
        self.delay_ms(120, clocks);

        self.write_command(spi, 0x36)?;
        self.write_data(spi, &[0x00])?;

        self.write_command(spi, 0x3A)?;
        self.write_data(spi, &[0x05])?;

        self.write_command(spi, 0xB2)?;
        self.write_data(spi, &[0x0C, 0x0C, 0x00, 0x33, 0x33])?;

        self.write_command(spi, 0xB7)?;
        self.write_data(spi, &[0x35])?;

        self.write_command(spi, 0xBB)?;
        self.write_data(spi, &[0x19])?;

        self.write_command(spi, 0xC0)?;
        self.write_data(spi, &[0x2C])?;

        self.write_command(spi, 0xC2)?;
        self.write_data(spi, &[0x01])?;

        self.write_command(spi, 0xC3)?;
        self.write_data(spi, &[0x12])?;

        self.write_command(spi, 0xC4)?;
        self.write_data(spi, &[0x20])?;

        self.write_command(spi, 0xC6)?;
        self.write_data(spi, &[0x0F])?;

        self.write_command(spi, 0xD0)?;
        self.write_data(spi, &[0xA4, 0xA1])?;

        self.write_command(spi, 0xE0)?;
        self.write_data(spi, &[0xD0, 0x04, 0x0D, 0x11, 0x13, 0x2B, 0x3F, 0x54, 0x4C, 0x18, 0x0D, 0x0B, 0x1F, 0x23])?;

        self.write_command(spi, 0xE1)?;
        self.write_data(spi, &[0xD0, 0x04, 0x0C, 0x11, 0x13, 0x2C, 0x3F, 0x44, 0x51, 0x2F, 0x1F, 0x1F, 0x20, 0x23])?;

        self.write_command(spi, 0x21)?;

        self.write_command(spi, 0x29)?;
        self.delay_ms(20, clocks);

        Ok(())
    }

    fn delay_ms(&self, ms: u32, clocks: &Clocks) {
        let cycles = clocks.sysclk().raw() / 1000 * ms;
        cortex_m::asm::delay(cycles);
    }

    fn hard_reset(&mut self, clocks: &Clocks) {
        let _ = self.rst.set_low();
        self.delay_ms(10, clocks);
        let _ = self.rst.set_high();
        self.delay_ms(120, clocks);
    }

    fn write_command<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, cmd: u8) -> Result<(), SpiError> {
        let _ = self.dc.set_low();
        spi.write(&[cmd])
    }

    fn write_data<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, data: &[u8]) -> Result<(), SpiError> {
        let _ = self.dc.set_high();
        spi.write(data)
    }

    pub fn set_address_window<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), SpiError> {
        self.write_command(spi, 0x2A)?;
        self.write_data(spi, &[
            (x0 >> 8) as u8,
            (x0 & 0xFF) as u8,
            (x1 >> 8) as u8,
            (x1 & 0xFF) as u8,
        ])?;

        self.write_command(spi, 0x2B)?;
        self.write_data(spi, &[
            (y0 >> 8) as u8,
            (y0 & 0xFF) as u8,
            (y1 >> 8) as u8,
            (y1 & 0xFF) as u8,
        ])?;

        self.write_command(spi, 0x2C)
    }

    pub fn fill_rect<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, w: u16, h: u16, color: u16) -> Result<(), SpiError> {
        if w == 0 || h == 0 {
            return Ok(());
        }

        self.set_address_window(spi, x, y, x + w - 1, y + h - 1)?;

        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;
        let pixel_pair = [hi, lo];
        let pixel_count = w as u32 * h as u32;

        let _ = self.dc.set_high();
        for _ in 0..pixel_count {
            spi.write(&pixel_pair)?;
        }

        Ok(())
    }

    pub fn fill_screen<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, color: u16) -> Result<(), SpiError> {
        self.fill_rect(spi, 0, 0, self.width, self.height, color)
    }

    pub fn set_pixel<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, color: u16) -> Result<(), SpiError> {
        self.fill_rect(spi, x, y, 1, 1, color)
    }
}
```

主程序

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod st7789;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc,
    spi::{Mode, Phase, Polarity},
};
use st7789::ST7789;

const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | (b as u16 >> 3)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("ST7789 240x240 驱动测试");

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

    let clocks = rcc.clocks.clone();

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    let sck = gpioa.pa5;
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7;
    let dc = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let rst = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut spi = dp.SPI1.spi(
        (Some(sck), Some(miso), Some(mosi)),
        SPI_MODE,
        16.MHz(),
        &mut rcc,
    );

    rprintln!("SPI 初始化完成, 16MHz");

    let mut display = ST7789::new(dc, rst);

    rprintln!("ST7789 初始化中...");
    display.init(&mut spi, &clocks).unwrap();
    rprintln!("ST7789 初始化完成");

    let colors = [
        rgb565(255, 0, 0),
        rgb565(0, 255, 0),
        rgb565(0, 0, 255),
        rgb565(255, 255, 0),
        rgb565(0, 255, 255),
        rgb565(255, 0, 255),
        rgb565(255, 255, 255),
        rgb565(0, 0, 0),
    ];
    let color_names = ["红", "绿", "蓝", "黄", "青", "紫", "白", "黑"];

    let mut color_idx: usize = 0;

    loop {
        let c = colors[color_idx % colors.len()];
        rprintln!("填充颜色: {} #{:04X}", color_names[color_idx % color_names.len()], c);
        display.fill_screen(&mut spi, c).unwrap();
        delay_ms(2000, &clocks);
        color_idx += 1;
    }
}

fn delay_ms(ms: u32, clocks: &rcc::Clocks) {
    let cycles = clocks.sysclk().raw() / 1000 * ms;
    asm::delay(cycles);
}
```

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


```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, rcc};

const SCAN_START: u8 = 0x08;
const SCAN_END: u8 = 0x78;

fn device_name(addr: u8) -> Option<&'static str> {
    match addr {
        0x03 => Some("General Call"),
        0x08..=0x0F => Some("HS-mode Master"),
        0x10 => Some("LED Driver (IS31FL3731)"),
        0x11 => Some("LED Driver (IS31FL3731)"),
        0x1C => Some("Accel (MMA845xQ/LIS3DH)"),
        0x1D => Some("Accel (ADXL345/MMA845xQ)"),
        0x1E => Some("Magnetometer (HMC5883L)"),
        0x20..=0x27 => Some("I/O Expander (PCF8574)"),
        0x28 => Some("IMU (MPU6050/BNO055)"),
        0x29 => Some("ToF (VL53L0X/VL53L1X)"),
        0x2A..=0x2F => Some("DAC (MCP4725)"),
        0x38 => Some("Sensor (BH1750/AHT20)"),
        0x39 => Some("Light (APDS-9960/TSL2561)"),
        0x3C => Some("OLED SSD1306 (128x32)"),
        0x3D => Some("OLED SSD1306 (128x64)"),
        0x40..=0x43 => Some("ADC/Temp (INA219/TMP102)"),
        0x44..=0x45 => Some("TH Sensor (SHT30/SHT31)"),
        0x48..=0x4B => Some("ADC (ADS1115/ADS1015)"),
        0x50..=0x57 => Some("EEPROM (AT24Cxx)"),
        0x5A => Some("IR Thermo (MLX90614)"),
        0x5C => Some("TH Sensor (AM2320/DHT12)"),
        0x60 => Some("Sensor (SGP30/SI1145)"),
        0x62 => Some("CO2 Sensor (SCD40/SCD41)"),
        0x68 => Some("RTC/IMU (DS3231/MPU6050)"),
        0x69 => Some("IMU (MPU6050 alt addr)"),
        0x6A..=0x6B => Some("IMU (LSM6DS3/LSM9DS1)"),
        0x76..=0x77 => Some("Env Sensor (BME280/BMP280)"),
        _ => None,
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let rcc_cfg = rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz())
        .pclk2(72.MHz())
        .adcclk(14.MHz());

    let mut rcc = dp.RCC.freeze(rcc_cfg, &mut flash.acr);

    cortex_m::peripheral::Peripherals::take()
        .unwrap()
        .DWT
        .enable_cycle_counter();

    let mut gpiob = dp.GPIOB.split(&mut rcc);

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let mut i2c = dp.I2C1.blocking_i2c(
        (scl, sda),
        stm32f1xx_hal::i2c::Mode::standard(100.kHz()),
        &mut rcc,
        1000,
        10,
        1000,
        1000,
    );
    rprint!("\r\n");
    rprintln!("=============================");
    rprintln!("  I2C Bus Scanner");
    rprintln!("  Range: 0x{:02X} - 0x{:02X}", SCAN_START, SCAN_END - 1);
    rprintln!("  Speed: 100kHz (Standard)");
    rprintln!("=============================");
    rprintln!();

    rprint!("     ");
    for col in 0..16u8 {
        rprint!("{:02X} ", col);
    }
    rprintln!();

    rprint!("    ");
    for _ in 0..16 {
        rprint!("---");
    }
    rprintln!();

    let mut found_count: u8 = 0;
    let mut found_addrs: [u8; 32] = [0u8; 32];

    for row in 0..8u8 {
        rprint!("{:02X}: ", row * 16);

        for col in 0..16u8 {
            let addr = row * 16 + col;

            if addr < SCAN_START || addr >= SCAN_END {
                rprint!("   ");
                continue;
            }

            let data: [u8; 1] = [0];
            match i2c.write(addr, &data) {
                Ok(_) => {
                    rprint!("{:02X}", addr);
                    if (found_count as usize) < found_addrs.len() {
                        found_addrs[found_count as usize] = addr;
                    }
                    found_count += 1;
                }
                Err(_) => {
                    rprint!("..");
                }
            }

            rprint!(" ");
        }
        rprintln!();
    }

    rprintln!();
    rprintln!("=============================");
    rprintln!("  Scan Done: {} device(s)", found_count);
    rprintln!("=============================");

    if found_count > 0 {
        rprintln!();
        for i in 0..found_count.min(32) {
            let addr = found_addrs[i as usize];
            rprint!("  [{}] 0x{:02X}", i + 1, addr);
            rprint!(" (W:0x{:02X} R:0x{:02X})", addr << 1, (addr << 1) | 1);
            if let Some(name) = device_name(addr) {
                rprint!(" {}", name);
            }
            rprintln!();
        }
    }

    rprintln!();
    rprintln!("Tip: 8-bit addr = 7-bit << 1 | R/W bit");

    loop {}
}
```

接线如图

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129746845-ddca9982-5283-4dd1-9def-439f14f69b5e.png" width="645" title="" crop="0,0,1,1" id="u6a6d07a1" class="ne-image">



## PWM波
### 输出
我们以PWM控制舵机来讲解

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780132432825-534b3892-b8fd-4f4d-ace6-0350c6c16588.png" width="589" title="" crop="0,0,1,1" id="u1a6aff10" class="ne-image">

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc,
    timer::{Channel, Tim2NoRemap},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("\r\n舵机测试");
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

    let mut afio = dp.AFIO.constrain(&mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // PA0 -> TIM2 CH1，50Hz（舵机标准频率）
    let pins = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp.TIM2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 50.Hz(), &mut rcc);
    let max = pwm.get_max_duty();
    pwm.enable(Channel::C1);

    // 舵机脉宽范围：0.5ms ~ 2.5ms（对应 0° ~ 180°）
    // 周期 20ms，占空比 = 脉宽 / 周期
    // duty_0   = max * 0.5 / 20  = max / 40
    // duty_180 = max * 2.5 / 20  = max / 8
    let duty_min = max / 40;   // 0.5ms → 0°
    let duty_max = max / 8;    // 2.5ms → 180°
    let step = (duty_max - duty_min) / 180;  // 每度对应的 duty 增量

    let mut current_duty = duty_min;
    let mut direction_up = true;

    // 72MHz 主频，每步延时约 5ms → 舵机约 0.7 秒完成 0→180
    let delay_cycles: u32 = 72_000 * 5;

    loop {
        pwm.set_duty(Channel::C1, current_duty);

        if direction_up {
            if current_duty >= duty_max {
                direction_up = false;
            } else {
                current_duty += step;
            }
        } else {
            if current_duty <= duty_min {
                direction_up = true;
            } else {
                current_duty -= step;
            }
        }
        rprintln!("--{:>3}度--",(current_duty-duty_min)/step);
        asm::delay(delay_cycles);
    }
}
```

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




```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc::{self, BusTimerClock},
    timer::pwm_input::*,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("PWM 输入检测启动");

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz()),
        &mut flash.acr,
    );

    let mut afio = dp.AFIO.constrain(&mut rcc);
    let mut dbg = dp.DBGMCU;

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // 禁用 JTAG 释放 PB4/PB5（默认被 JTAG 占用）
    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    let pb5 = gpiob.pb5;

    // TIM3 配置为 PWM 输入模式
    // PB4 = IC1（上升沿捕获，测周期）
    // PB5 = IC2（下降沿捕获，测高电平时间）
    let pwm_input = dp.TIM3.remap(&mut afio.mapr).pwm_input(
        (pb4, pb5),
        &mut dbg,
        Configuration::Frequency(10.kHz()),
        &mut rcc,
    );

    let timer_clk = pac::TIM3::timer_clock(&rcc.clocks);
    rprintln!("定时器时钟: {} Hz", timer_clk.raw());

    loop {
        match pwm_input.read_frequency(ReadMode::WaitForNextCapture, timer_clk) {
            Ok(freq) => {
                let freq_hz = freq.raw();
                match pwm_input.read_duty(ReadMode::Instant) {
                    Ok((high, period)) => {
                        let duty_pct = (high as f32 * 100.0) / period as f32;
                        rprintln!(
                            "频率: {} Hz | 占空比: {:.1}% ({}/{})",
                            freq_hz,
                            duty_pct,
                            high,
                            period,
                        );
                    }
                    Err(_) => {
                        rprintln!("频率: {} Hz | 占空比读取失败", freq_hz);
                    }
                }
            }
            Err(Error::FrequencyTooLow) => {
                rprintln!("信号频率过低或无信号");
            }
        }
    }
}
```



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



```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc,
    timer::{pwm_input::QeiOptions, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("EC11 编码器 QEI 测试");

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let gpiob = dp.GPIOB.split(&mut rcc);

    let c1 = gpiob.pb6;
    let c2 = gpiob.pb7;

    let qei = Timer::new(dp.TIM4, &mut rcc).qei((c1, c2), QeiOptions::default());
    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        let before = qei.count();
        delay.delay_ms(1_000_u16);
        let after = qei.count();

        let elapsed = after.wrapping_sub(before) as i16;
        rprintln!("脉冲: {} 方向: {:?}", elapsed, qei.direction());
    }
}
```



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780136893674-70671b8b-3b27-429a-8a79-12fdca9cbd25.png" width="499" title="" crop="0,0,1,1" id="u76a64ffa" class="ne-image">



## CRC校验
**用途：** 数据完整性校验、通信协议的 CRC 校验。

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("CRC 校验 Demo 启动");

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

    let mut crc = dp.CRC.new(&mut rcc);

    crc.reset();
    crc.write(0x12345678);
    let val = crc.read();
    rprintln!("单字 CRC: found={:08x}, expected={:08x}", val, 0xdf8a8a2b_u32);

    crc.reset();
    crc.write(0x00000001);
    crc.write(0x00000002);
    crc.write(0x00000003);
    let val = crc.read();
    rprintln!("多字 CRC: result={:08x}", val);

    crc.reset();
    let val = crc.read();
    rprintln!("复位后初始值: {:08x} (应为 ffffffff)", val);

    rprintln!("CRC 校验 Demo 完成");

    loop {}
}
```

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

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    dac::{DacExt, DacOut, DacPin},
    rcc,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("DAC数模转换测试");

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

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let pa4 = gpioa.pa4.into_analog(&mut gpioa.crl);
    let pa5 = gpioa.pa5.into_analog(&mut gpioa.crl);

    let (mut ch1, _ch2) = dp.DAC.constrain((pa4, pa5), &mut rcc);
    ch1.enable();

    let val_0_3v: u16 = (0.3_f32 / 3.3_f32 * 4095.0_f32) as u16;
    let val_1_6v: u16 = (1.6_f32 / 3.3_f32 * 4095.0_f32) as u16;
    let val_3_1v: u16 = (3.1_f32 / 3.3_f32 * 4095.0_f32) as u16;

    let delay_cycles: u32 = 72_000_000 * 3;

    loop {
        ch1.set_value(val_0_3v);
        rprintln!("DAC CH1: 0.3V (raw: {})", val_0_3v);
        cortex_m::asm::delay(delay_cycles);

        ch1.set_value(val_1_6v);
        rprintln!("DAC CH1: 1.6V (raw: {})", val_1_6v);
        cortex_m::asm::delay(delay_cycles);

        ch1.set_value(val_3_1v);
        rprintln!("DAC CH1: 3.1V (raw: {})", val_3_1v);
        cortex_m::asm::delay(delay_cycles);
    }
}
```

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
```rust
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
```

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod dht11;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

use dht11::{Delay, Dht11};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("DHT11 温湿度传感器 - PA6");

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut delay = Delay::new(cp.SYST);

    // PA6 推挽输出，初始高电平
    let mut pin = gpioa.pa6.into_push_pull_output_with_state(
        &mut gpioa.crl,
        stm32f1xx_hal::gpio::PinState::High,
    );

    delay.ms(1500);
    rprintln!("DHT11 初始化完成，开始采集...");

    loop {
        // Dht11::read 接收推挽输出引脚，归还推挽输出引脚
        let (result, returned_pin) = Dht11::read(pin, &mut gpioa.crl, &mut delay);
        pin = returned_pin;

        match result {
            Ok((humi, temp)) => {
                rprintln!("湿度: {}%RH, 温度: {}C", humi, temp);
            }
            Err(e) => {
                rprintln!("DHT11 读取失败: {:?}", e);
            }
        }

        delay.ms(2000);
    }
}
```

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



src/font_ascii.rs

```rust
#![allow(dead_code)]

/// 16x32 font bitmap
/// 64 bytes/char, column-major, LSB-first
/// 95 characters

pub const FONT_16X32: [[u8; 64]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  32 ' '
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x0F, 0x03, 0xF0, 0xFF, 0x0F, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  33 '!'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x07, 0x00, 0x00, 0xC0, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x03, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  34 '"'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x08, 0xC8, 0x01, 0x00, 0x08, 0xFF, 0x01, 0x00, 0xF8, 0x0F, 0x00, 0xE0, 0x3F, 0x08, 0x00, 0x60, 0x0C, 0x0C, 0x00, 0x00, 0x0C, 0x0C, 0x00, 0x00, 0x0C, 0x0C, 0x01, 0x00, 0x0C, 0xFC, 0x01, 0x00, 0xFC, 0x1F, 0x00, 0xE0, 0x7F, 0x04, 0x00, 0xE0, 0x07, 0x06, 0x00, 0x00, 0x04, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00], //  35 '#'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x03, 0x7C, 0x00, 0xC0, 0x0F, 0xC0, 0x00, 0x60, 0x18, 0x80, 0x00, 0x60, 0x30, 0x80, 0x00, 0xFC, 0xFF, 0xFF, 0x0F, 0x60, 0xC0, 0x80, 0x01, 0x60, 0xC0, 0x80, 0x00, 0xC0, 0x80, 0xC1, 0x00, 0x80, 0x01, 0x67, 0x00, 0x00, 0x01, 0x3E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  36 '$'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x02, 0xC0, 0x81, 0x80, 0x03, 0xC0, 0x81, 0xE0, 0x00, 0x80, 0x7F, 0x38, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x70, 0xFC, 0x01, 0x00, 0x9C, 0x9E, 0x03, 0x00, 0x87, 0x03, 0x02, 0x80, 0x01, 0x87, 0x03, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  37 '%'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00, 0x87, 0x01, 0x80, 0xBF, 0x01, 0x01, 0xC0, 0xF0, 0x00, 0x02, 0x40, 0xE0, 0x01, 0x02, 0x40, 0x30, 0x07, 0x02, 0xC0, 0x1F, 0x0C, 0x02, 0x80, 0x07, 0x18, 0x01, 0x00, 0x00, 0xB0, 0x01, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0xFC, 0x01, 0x00, 0x00, 0x8C, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  38 '&'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x03, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  39 '''
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x0F, 0x00, 0x00, 0xFE, 0x3F, 0x00, 0x80, 0x07, 0xF0, 0x00, 0xE0, 0x01, 0xC0, 0x01, 0x70, 0x00, 0x00, 0x03, 0x38, 0x00, 0x00, 0x06, 0x0C, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  40 '('
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x18, 0x00, 0x00, 0x18, 0x30, 0x00, 0x00, 0x0E, 0xE0, 0x00, 0x00, 0x07, 0x80, 0x01, 0x80, 0x01, 0x00, 0x0F, 0xF0, 0x00, 0x00, 0xFC, 0x3F, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  41 ')'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0x00, 0x00, 0x00, 0xC4, 0x00, 0x00, 0x00, 0x4C, 0x00, 0x00, 0x00, 0x68, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0xF0, 0x0F, 0x00, 0xC0, 0xFF, 0x0F, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0xCC, 0x00, 0x00, 0x00, 0x86, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  42 '*'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x02, 0x00, 0xC0, 0xFF, 0x03, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  43 '+'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x30, 0x06, 0x00, 0x00, 0xE0, 0x07, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  44 ','
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  45 '-'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  46 '.'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  47 '/'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x1F, 0x00, 0x00, 0x3C, 0x78, 0x00, 0x00, 0x06, 0x80, 0x01, 0x00, 0x03, 0x00, 0x01, 0x80, 0x01, 0x00, 0x02, 0xC0, 0x00, 0x00, 0x02, 0xC0, 0x00, 0x00, 0x02, 0xC0, 0x00, 0x00, 0x03, 0x80, 0x01, 0x00, 0x01, 0x00, 0x0F, 0xC0, 0x00, 0x00, 0xFC, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  48 '0'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0xE0, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  49 '1'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x80, 0x01, 0xC0, 0x00, 0xC0, 0x00, 0xE0, 0x01, 0x40, 0x00, 0x70, 0x01, 0x60, 0x00, 0x38, 0x01, 0x60, 0x00, 0x0C, 0x01, 0x60, 0x00, 0x06, 0x01, 0x60, 0x80, 0x03, 0x01, 0x40, 0xC0, 0x01, 0x01, 0xC0, 0xF0, 0x00, 0x01, 0x80, 0x3F, 0x00, 0x01, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  50 '2'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x38, 0x00, 0x80, 0x03, 0x60, 0x00, 0x80, 0x01, 0xC0, 0x00, 0xC0, 0x00, 0x80, 0x01, 0x60, 0x00, 0x80, 0x01, 0x60, 0x00, 0x00, 0x01, 0x20, 0x40, 0x80, 0x01, 0x20, 0xE0, 0x80, 0x01, 0x60, 0xF0, 0xC0, 0x00, 0xC0, 0x98, 0xE3, 0x00, 0x80, 0x0F, 0x7F, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  51 '3'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x80, 0x0B, 0x00, 0x00, 0xC0, 0x09, 0x00, 0x00, 0x70, 0x08, 0x00, 0x00, 0x38, 0x08, 0x00, 0x00, 0x0C, 0x08, 0x00, 0x00, 0x07, 0x08, 0x00, 0x80, 0x03, 0x08, 0x00, 0xE0, 0x00, 0x0C, 0x00, 0xE0, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00], //  52 '4'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x38, 0x70, 0x00, 0xC0, 0x7F, 0xE0, 0x00, 0xE0, 0x21, 0xC0, 0x00, 0x60, 0x20, 0x80, 0x00, 0x60, 0x20, 0x80, 0x01, 0x60, 0x20, 0x80, 0x01, 0x60, 0x20, 0x80, 0x00, 0x60, 0x60, 0xC0, 0x00, 0x60, 0xC0, 0x60, 0x00, 0x20, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  53 '5'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0xE0, 0x7F, 0x00, 0x00, 0xE0, 0xC0, 0x00, 0x00, 0x38, 0x80, 0x00, 0x00, 0x3E, 0x00, 0x01, 0x00, 0x13, 0x00, 0x01, 0xC0, 0x11, 0x00, 0x01, 0x60, 0x30, 0x00, 0x01, 0x00, 0x20, 0x80, 0x00, 0x00, 0x60, 0xC0, 0x00, 0x00, 0xC0, 0x7F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  54 '6'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0xC0, 0x00, 0x20, 0x00, 0x78, 0x00, 0x20, 0x00, 0x0E, 0x00, 0x20, 0xC0, 0x03, 0x00, 0x20, 0xF0, 0x00, 0x00, 0x20, 0x3C, 0x00, 0x00, 0x20, 0x0F, 0x00, 0x00, 0xA0, 0x03, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  55 '7'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x9E, 0xC3, 0x00, 0x00, 0xBF, 0x81, 0x00, 0x80, 0xE1, 0x00, 0x01, 0x80, 0xE0, 0x00, 0x01, 0xE0, 0xE0, 0x00, 0x01, 0x60, 0xA0, 0x00, 0x01, 0xC0, 0xB0, 0x01, 0x01, 0xC0, 0x1F, 0x83, 0x00, 0x00, 0x0F, 0xE6, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  56 '8'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x00, 0x80, 0x63, 0x00, 0x00, 0xC0, 0x80, 0x00, 0x00, 0x60, 0x00, 0x01, 0x00, 0x20, 0x00, 0x81, 0x03, 0x20, 0x00, 0xE1, 0x00, 0x20, 0x00, 0x39, 0x00, 0x20, 0x80, 0x1F, 0x00, 0x60, 0x80, 0x07, 0x00, 0xC0, 0xE0, 0x01, 0x00, 0x80, 0x71, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  57 '9'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x18, 0x00, 0x00, 0x06, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  58 ':'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x60, 0x00, 0x80, 0x01, 0x66, 0x00, 0x80, 0x01, 0x3C, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  59 ';'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0xE0, 0x03, 0x00, 0x00, 0x60, 0x03, 0x00, 0x00, 0x30, 0x06, 0x00, 0x00, 0x18, 0x0C, 0x00, 0x00, 0x0C, 0x18, 0x00, 0x00, 0x06, 0x30, 0x00, 0x00, 0x07, 0x60, 0x00, 0x00, 0x03, 0xC0, 0x00, 0x80, 0x01, 0xC0, 0x00, 0xC0, 0x00, 0x80, 0x01, 0x60, 0x00, 0x00, 0x03, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  60 '<'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x00, 0xC3, 0x00, 0x00, 0x00, 0xC1, 0x00, 0x00, 0x00, 0x41, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  61 '='
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x20, 0x00, 0x00, 0x03, 0x60, 0x00, 0x80, 0x01, 0xC0, 0x00, 0xC0, 0x00, 0x80, 0x01, 0x60, 0x00, 0x00, 0x03, 0x20, 0x00, 0x00, 0x06, 0x30, 0x00, 0x00, 0x0C, 0x18, 0x00, 0x00, 0x0C, 0x0C, 0x00, 0x00, 0x18, 0x06, 0x00, 0x00, 0x30, 0x03, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  62 '>'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x60, 0x00, 0x83, 0x01, 0x20, 0xC0, 0x83, 0x01, 0x20, 0xC0, 0x00, 0x00, 0x60, 0x60, 0x00, 0x00, 0xE0, 0x38, 0x00, 0x00, 0xC0, 0x1F, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  63 '?'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x1F, 0x00, 0x00, 0x0F, 0x78, 0x00, 0x80, 0x01, 0xE0, 0x00, 0xC0, 0x00, 0x80, 0x00, 0x40, 0xC0, 0x8F, 0x01, 0x60, 0x70, 0x08, 0x01, 0x60, 0x18, 0x08, 0x01, 0x60, 0x18, 0x0E, 0x01, 0x60, 0xC8, 0x07, 0x01, 0x40, 0xF8, 0x87, 0x01, 0xC0, 0x01, 0x8C, 0x00, 0x00, 0x07, 0x07, 0x00, 0x00, 0xFE, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  64 '@'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xF0, 0x01, 0x00, 0x00, 0x3E, 0x00, 0x00, 0xC0, 0x0F, 0x00, 0x00, 0xF8, 0x04, 0x00, 0x00, 0x1F, 0x04, 0x00, 0xE0, 0x03, 0x06, 0x00, 0xE0, 0x03, 0x06, 0x00, 0x00, 0x1F, 0x06, 0x00, 0x00, 0xF8, 0x03, 0x00, 0x00, 0xC0, 0x07, 0x00, 0x00, 0x00, 0x7C, 0x00, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  65 'A'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0x20, 0x40, 0x00, 0x01, 0x20, 0x40, 0x00, 0x01, 0x20, 0x40, 0x00, 0x01, 0x20, 0x40, 0x00, 0x01, 0x20, 0x40, 0x00, 0x01, 0x20, 0x40, 0x00, 0x01, 0x60, 0x40, 0x00, 0x01, 0x40, 0xE0, 0x80, 0x00, 0xC0, 0xB9, 0xC1, 0x00, 0x00, 0x0F, 0x73, 0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0x00, 0x00], //  66 'B'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0xFE, 0x3F, 0x00, 0x00, 0x07, 0x70, 0x00, 0x80, 0x00, 0xC0, 0x00, 0x40, 0x00, 0x80, 0x01, 0x60, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x01, 0x20, 0x00, 0x80, 0x01, 0x20, 0x00, 0x80, 0x00, 0x40, 0x00, 0x60, 0x00, 0x80, 0x01, 0x3E, 0x00, 0x00, 0x07, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  67 'C'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0xE0, 0xFF, 0xFF, 0x01, 0x20, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x01, 0x40, 0x00, 0x00, 0x01, 0x40, 0x00, 0x80, 0x00, 0x80, 0x00, 0xC0, 0x00, 0x80, 0x03, 0x60, 0x00, 0x00, 0xFE, 0x3F, 0x00, 0x00, 0xF0, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  68 'D'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xFF, 0xFF, 0x00, 0x40, 0x80, 0x80, 0x00, 0x40, 0xC0, 0x80, 0x00, 0x40, 0xC0, 0x80, 0x00, 0x40, 0xC0, 0x80, 0x00, 0x60, 0xC0, 0x80, 0x00, 0x60, 0x40, 0x80, 0x00, 0x60, 0x40, 0x80, 0x00, 0x20, 0x60, 0x80, 0x00, 0x20, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  69 'E'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xFF, 0xFF, 0x01, 0xC0, 0xFF, 0x01, 0x00, 0x40, 0x80, 0x01, 0x00, 0x40, 0x80, 0x00, 0x00, 0x40, 0x80, 0x00, 0x00, 0x40, 0xC0, 0x00, 0x00, 0x40, 0xC0, 0x00, 0x00, 0x60, 0xC0, 0x00, 0x00, 0x60, 0x40, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  70 'F'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x0F, 0x00, 0x00, 0xFF, 0x3F, 0x00, 0x80, 0x01, 0x60, 0x00, 0x40, 0x00, 0x80, 0x00, 0x60, 0x00, 0x80, 0x01, 0x20, 0x00, 0x00, 0x01, 0x20, 0x00, 0x00, 0x01, 0x20, 0x00, 0x01, 0x01, 0x20, 0x00, 0x81, 0x01, 0x40, 0x00, 0xC1, 0x00, 0xC0, 0x01, 0xE1, 0x00, 0x00, 0x0F, 0xFF, 0x01, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  71 'G'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xFF, 0xFF, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  72 'H'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0xC0, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  73 'I'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0x7C, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0xE0, 0x7F, 0x00, 0xE0, 0xFF, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  74 'J'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0xF8, 0x00, 0x00, 0x00, 0x8C, 0x01, 0x00, 0x00, 0x06, 0x07, 0x00, 0x00, 0x03, 0x1C, 0x00, 0x80, 0x01, 0x38, 0x00, 0xC0, 0x00, 0xE0, 0x00, 0x40, 0x00, 0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  75 'K'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  76 'L'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x00, 0xC0, 0x07, 0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0xF0, 0x01, 0x00, 0x00, 0xC0, 0x0F, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0xC0, 0x03, 0x00, 0x00, 0x7C, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0xC0, 0x03, 0x00, 0x00, 0xC0, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  77 'M'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0xE0, 0xFF, 0x01, 0x00, 0x80, 0x03, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x78, 0x00, 0xE0, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  78 'N'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x1F, 0x00, 0x00, 0x1E, 0x78, 0x00, 0x00, 0x03, 0xC0, 0x00, 0x80, 0x01, 0x80, 0x00, 0x80, 0x00, 0x00, 0x01, 0xE0, 0x00, 0x00, 0x01, 0x60, 0x00, 0x00, 0x01, 0xC0, 0x00, 0x80, 0x01, 0x80, 0x01, 0xC0, 0x00, 0x00, 0x0F, 0x78, 0x00, 0x00, 0xFC, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  79 'O'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x60, 0x40, 0x00, 0x00, 0x40, 0x60, 0x00, 0x00, 0xC0, 0x39, 0x00, 0x00, 0x80, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  80 'P'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x07, 0x00, 0x00, 0xFE, 0x3F, 0x00, 0x00, 0x07, 0x60, 0x00, 0x80, 0x01, 0xC0, 0x00, 0xC0, 0x00, 0x80, 0x01, 0x60, 0x00, 0x80, 0x01, 0x70, 0x00, 0x80, 0x01, 0x30, 0x00, 0x88, 0x01, 0x60, 0x00, 0xB8, 0x00, 0xC0, 0x00, 0xE0, 0x00, 0x80, 0x03, 0xF8, 0x03, 0x00, 0xFE, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  81 'Q'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0x20, 0x80, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x00, 0x00, 0x20, 0xC0, 0x01, 0x00, 0x60, 0xC0, 0x07, 0x00, 0x40, 0x60, 0x1C, 0x00, 0x80, 0x3F, 0xF0, 0x00, 0x00, 0x1F, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  82 'R'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x0F, 0x60, 0x00, 0xC0, 0x1D, 0xC0, 0x00, 0x40, 0x30, 0x80, 0x01, 0x60, 0x20, 0x80, 0x01, 0x60, 0x60, 0x80, 0x01, 0x60, 0xC0, 0x80, 0x01, 0x60, 0xC0, 0x80, 0x01, 0x60, 0x80, 0x81, 0x00, 0xC0, 0x00, 0xC3, 0x00, 0x80, 0x03, 0x7E, 0x00, 0x00, 0x03, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  83 'S'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0xC0, 0xFF, 0xFF, 0x01, 0xC0, 0xFF, 0xFF, 0x00, 0x60, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  84 'T'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xFF, 0x3F, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0xF0, 0x00, 0xE0, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  85 'U'
    [0x00, 0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0xC0, 0x03, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x7C, 0x00, 0x00, 0xC0, 0x07, 0x00, 0x00, 0xF8, 0x00, 0x00, 0x80, 0x0F, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  86 'V'
    [0x00, 0x00, 0x00, 0x00, 0xC0, 0x0F, 0x00, 0x00, 0x00, 0xFF, 0x07, 0x00, 0x00, 0x80, 0xFF, 0x00, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xF8, 0x07, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0xF8, 0x07, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xF8, 0x00, 0x00, 0xF8, 0x7F, 0x00, 0xE0, 0x7F, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  87 'W'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x20, 0x00, 0xC0, 0x01, 0xE0, 0x00, 0x70, 0x00, 0x80, 0x03, 0x38, 0x00, 0x00, 0x1E, 0x0E, 0x00, 0x00, 0xF8, 0x03, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0xB8, 0x07, 0x00, 0x00, 0x1E, 0x1E, 0x00, 0x00, 0x07, 0x38, 0x00, 0xC0, 0x01, 0xE0, 0x00, 0x60, 0x00, 0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  88 'X'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0x00, 0x00, 0xE0, 0xFF, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  89 'Y'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0xC0, 0x01, 0x20, 0x00, 0xE0, 0x01, 0x20, 0x00, 0x38, 0x01, 0x20, 0x00, 0x0E, 0x01, 0x20, 0x00, 0x07, 0x01, 0x20, 0xC0, 0x01, 0x01, 0x20, 0x70, 0x00, 0x01, 0x20, 0x3C, 0x00, 0x01, 0x20, 0x0E, 0x00, 0x01, 0xA0, 0x03, 0x00, 0x01, 0xE0, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  90 'Z'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0xFF, 0xFF, 0x1F, 0x30, 0x00, 0x00, 0x10, 0x30, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x10, 0x18, 0x00, 0x00, 0x10, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  91 '['
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x80, 0x07, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x00, 0x00, 0xC0, 0x03, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  92 '\u{5C}'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x10, 0x08, 0x00, 0x00, 0x10, 0x08, 0x00, 0x00, 0x10, 0x08, 0x00, 0x00, 0x18, 0x08, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x08, 0xF8, 0xFF, 0xFF, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  93 ']'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  94 '^'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  95 '_'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  96 '`'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x18, 0x1F, 0x00, 0x00, 0x0C, 0x23, 0x00, 0x00, 0x86, 0x21, 0x00, 0x00, 0x86, 0x21, 0x00, 0x00, 0x86, 0x31, 0x00, 0x00, 0x86, 0x18, 0x00, 0x00, 0x86, 0x08, 0x00, 0x00, 0xC4, 0x0C, 0x00, 0x00, 0xFC, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  97 'a'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0xE0, 0xFF, 0x71, 0x00, 0x00, 0xC0, 0x60, 0x00, 0x00, 0xC0, 0xC0, 0x00, 0x00, 0x60, 0x80, 0x01, 0x00, 0x20, 0x80, 0x01, 0x00, 0x30, 0x00, 0x01, 0x00, 0x30, 0x00, 0x01, 0x00, 0x20, 0x80, 0x01, 0x00, 0x60, 0xC0, 0x00, 0x00, 0xC0, 0x71, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  98 'b'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x00, 0x00, 0xFC, 0x03, 0x00, 0x00, 0x06, 0x0E, 0x00, 0x00, 0x02, 0x0C, 0x00, 0x00, 0x03, 0x18, 0x00, 0x00, 0x03, 0x18, 0x00, 0x00, 0x01, 0x18, 0x00, 0x00, 0x03, 0x18, 0x00, 0x00, 0x03, 0x0C, 0x00, 0x00, 0x06, 0x0E, 0x00, 0x00, 0x8C, 0x03, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], //  99 'c'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x80, 0xFF, 0x00, 0x00, 0xC0, 0x80, 0x01, 0x00, 0x60, 0x00, 0x03, 0x00, 0x60, 0x00, 0x02, 0x00, 0x60, 0x00, 0x02, 0x00, 0x60, 0x00, 0x02, 0x00, 0x40, 0x00, 0x01, 0x00, 0xC0, 0xC0, 0x01, 0x00, 0xC0, 0xE1, 0x01, 0xC0, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 100 'd'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x01, 0x00, 0x00, 0xFC, 0x03, 0x00, 0x00, 0x26, 0x0E, 0x00, 0x00, 0x23, 0x08, 0x00, 0x00, 0x23, 0x10, 0x00, 0x00, 0x21, 0x10, 0x00, 0x00, 0x21, 0x10, 0x00, 0x00, 0x23, 0x10, 0x00, 0x00, 0x23, 0x08, 0x00, 0x00, 0x26, 0x0C, 0x00, 0x00, 0x3C, 0x07, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 101 'e'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0x01, 0x80, 0xFF, 0xFF, 0x01, 0xC0, 0x30, 0x00, 0x00, 0x60, 0x10, 0x00, 0x00, 0x60, 0x10, 0x00, 0x00, 0x60, 0x10, 0x00, 0x00, 0x60, 0x10, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 102 'f'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x7C, 0x00, 0x00, 0xCC, 0x4F, 0x00, 0x00, 0xFF, 0x8C, 0x00, 0x00, 0x21, 0x88, 0x00, 0x00, 0x21, 0x88, 0x00, 0x80, 0x21, 0x88, 0x00, 0x80, 0x21, 0x88, 0x00, 0x00, 0x19, 0x88, 0x00, 0x00, 0x0F, 0x88, 0x00, 0x00, 0x01, 0x58, 0x00, 0x80, 0x01, 0x70, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 103 'g'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0x00, 0x80, 0x01, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0xC0, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 104 'h'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0xF0, 0x01, 0x60, 0xE0, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 105 'i'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0C, 0x18, 0xFC, 0xFF, 0x07, 0x10, 0xF8, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 106 'j'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0xC0, 0xFF, 0x0F, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x80, 0x0C, 0x00, 0x00, 0xC0, 0x18, 0x00, 0x00, 0x60, 0x60, 0x00, 0x00, 0x30, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 107 'k'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0x01, 0xC0, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 108 'l'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0xFE, 0x1F, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0xF8, 0x0F, 0x00, 0x00, 0xF8, 0x0F, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0xF8, 0x3F, 0x00, 0x00, 0xC0, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00], // 109 'm'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFC, 0x3F, 0x00, 0x00, 0xFC, 0x1F, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0xFC, 0x3F, 0x00, 0x00, 0xF8, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 110 'n'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x07, 0x00, 0x00, 0x38, 0x0C, 0x00, 0x00, 0x0C, 0x18, 0x00, 0x00, 0x0E, 0x10, 0x00, 0x00, 0x07, 0x10, 0x00, 0x00, 0x07, 0x10, 0x00, 0x00, 0x03, 0x10, 0x00, 0x00, 0x06, 0x08, 0x00, 0x00, 0x0E, 0x0C, 0x00, 0x00, 0xF8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 111 'o'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xFF, 0x00, 0x80, 0xFF, 0x03, 0x00, 0x00, 0x06, 0x03, 0x00, 0x00, 0x03, 0x06, 0x00, 0x00, 0x01, 0x04, 0x00, 0x80, 0x01, 0x0C, 0x00, 0x80, 0x01, 0x0C, 0x00, 0x80, 0x01, 0x0C, 0x00, 0x00, 0x01, 0x04, 0x00, 0x00, 0x03, 0x06, 0x00, 0x00, 0xFE, 0x03, 0x00, 0x00, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 112 'p'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x01, 0x00, 0x00, 0xFE, 0x03, 0x00, 0x00, 0x03, 0x04, 0x00, 0x80, 0x01, 0x0C, 0x00, 0x80, 0x01, 0x08, 0x00, 0x80, 0x01, 0x08, 0x00, 0x80, 0x01, 0x08, 0x00, 0x80, 0x01, 0x04, 0x00, 0x00, 0x03, 0x06, 0x00, 0x00, 0x03, 0x03, 0x00, 0x00, 0x86, 0xFF, 0x00, 0x80, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 113 'q'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x1F, 0x00, 0x00, 0xFE, 0x0F, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 114 'r'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x38, 0x18, 0x00, 0x00, 0x7C, 0x10, 0x00, 0x00, 0xC4, 0x20, 0x00, 0x00, 0xC6, 0x20, 0x00, 0x00, 0x86, 0x20, 0x00, 0x00, 0x86, 0x21, 0x00, 0x00, 0x86, 0x21, 0x00, 0x00, 0x84, 0x13, 0x00, 0x00, 0x0C, 0x1F, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 115 's'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0xC0, 0xFF, 0x7F, 0x00, 0x80, 0x0F, 0x40, 0x00, 0x00, 0x0C, 0x80, 0x00, 0x00, 0x0C, 0x80, 0x00, 0x00, 0x0C, 0x80, 0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 116 't'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0xFF, 0x07, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0xFE, 0x07, 0x00, 0x00, 0xFE, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 117 'u'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 118 'v'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0xF0, 0x01, 0x00, 0x00, 0x80, 0x0F, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0xC0, 0x07, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0xE0, 0x03, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0xF0, 0x07, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 119 'w'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x18, 0x00, 0x00, 0x0E, 0x0C, 0x00, 0x00, 0x1C, 0x06, 0x00, 0x00, 0x30, 0x03, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x70, 0x03, 0x00, 0x00, 0x18, 0x0E, 0x00, 0x00, 0x0C, 0x1C, 0x00, 0x00, 0x06, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 120 'x'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x01, 0x80, 0x00, 0x00, 0x0F, 0x80, 0x00, 0x00, 0x7C, 0x80, 0x00, 0x00, 0xE0, 0xC3, 0x00, 0x00, 0x80, 0x6F, 0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x80, 0x07, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 121 'y'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x30, 0x00, 0x00, 0x02, 0x3C, 0x00, 0x00, 0x02, 0x2E, 0x00, 0x00, 0x02, 0x23, 0x00, 0x00, 0xC2, 0x21, 0x00, 0x00, 0xE2, 0x20, 0x00, 0x00, 0x32, 0x20, 0x00, 0x00, 0x1A, 0x20, 0x00, 0x00, 0x0E, 0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 122 'z'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFE, 0xFF, 0x03, 0xF8, 0x3F, 0xFC, 0x07, 0x0C, 0x00, 0x00, 0x0C, 0x04, 0x00, 0x00, 0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 123 '{'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFE, 0xFF, 0xFF, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 124 '|'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x08, 0x0C, 0x00, 0x00, 0x0C, 0x1C, 0x00, 0x00, 0x07, 0xF8, 0x7F, 0xFF, 0x03, 0x00, 0xE0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 125 '}'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 126 '~'
];

pub fn font_16x32(c: char) -> Option<&'static [u8; 64]>
{
    let idx = c as usize;
    if idx >= 32 && idx <= 126 {
        Some(&FONT_16X32[idx - 32])
    } else {
        None
    }
}
```



src/font_cn.rs

```rust
#![allow(dead_code)]

/// 32x32 font bitmap
/// 128 bytes/char, column-major, LSB-first
/// 4 characters

pub const FONT_32X32: &[(char, [u8; 128])] = &[
    ('\u{2103}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0xE0, 0x02, 0x00, 0x00, 0x60, 0x01, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x3F, 0x00, 0x00, 0x38, 0x60, 0x00, 0x00, 0x08, 0xC0, 0x00, 0x00, 0x0C, 0x80, 0x00, 0x00, 0x04, 0x80, 0x00, 0x00, 0x04, 0xC0, 0x00, 0x00, 0x0C, 0x60, 0x00, 0x00, 0x38, 0x3C, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // ℃
    ('\u{5EA6}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0xC0, 0x1F, 0xF8, 0xFF, 0xFF, 0x03, 0xF8, 0xFF, 0x01, 0x08, 0x00, 0x00, 0x00, 0x10, 0x30, 0x02, 0x00, 0x18, 0x30, 0x02, 0x00, 0x18, 0x10, 0x82, 0x01, 0x08, 0x10, 0x02, 0x07, 0x0C, 0x10, 0x02, 0x1C, 0x0C, 0xD0, 0x7F, 0x32, 0x04, 0x10, 0x12, 0x62, 0x06, 0x10, 0x12, 0xC2, 0x02, 0x17, 0x12, 0x82, 0x03, 0x10, 0x13, 0x02, 0x03, 0x10, 0x13, 0x82, 0x03, 0x10, 0x13, 0xC2, 0x02, 0x10, 0x11, 0x62, 0x04, 0x08, 0x7F, 0x32, 0x04, 0xC8, 0x3F, 0x1E, 0x08, 0x48, 0x01, 0x06, 0x08, 0x08, 0x01, 0x00, 0x08, 0x08, 0x01, 0x00, 0x08, 0x08, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // 度
    ('\u{6E29}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x10, 0x00, 0x0C, 0x00, 0x3C, 0x08, 0x18, 0x00, 0x0F, 0x18, 0x38, 0xC0, 0x03, 0x38, 0x30, 0x7C, 0x00, 0x70, 0xC0, 0x03, 0x00, 0x60, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x07, 0x10, 0x0C, 0x20, 0xFE, 0x17, 0x38, 0x7F, 0x02, 0x10, 0xF0, 0x17, 0x02, 0x10, 0x10, 0x31, 0x02, 0x10, 0x10, 0x31, 0x02, 0x10, 0x10, 0x31, 0xFE, 0x0F, 0x10, 0x11, 0x02, 0x08, 0x90, 0x11, 0x02, 0x08, 0x90, 0x11, 0x02, 0x0A, 0x90, 0x11, 0xFF, 0x0F, 0x90, 0x11, 0xFF, 0x0F, 0x18, 0x10, 0x03, 0x08, 0xF8, 0x7F, 0x03, 0x08, 0x00, 0x70, 0xFF, 0x0F, 0x00, 0x00, 0x0F, 0x0F, 0x00, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00]), // 温
    ('\u{6E7F}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x18, 0x08, 0x0C, 0x00, 0x1E, 0x18, 0x18, 0x80, 0x07, 0x30, 0x30, 0x78, 0x00, 0x30, 0x20, 0x07, 0x00, 0x60, 0x80, 0x00, 0x10, 0xC0, 0x00, 0x00, 0x10, 0x00, 0x30, 0x06, 0x10, 0x0E, 0x7C, 0x1E, 0x18, 0xF8, 0x1F, 0xF8, 0x18, 0xE0, 0x31, 0xC0, 0x19, 0x08, 0x11, 0x00, 0x09, 0x88, 0xD1, 0xFF, 0x0F, 0x88, 0xD1, 0xFF, 0x0B, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x0B, 0x88, 0xD8, 0xFF, 0x0B, 0xC8, 0x18, 0x00, 0x08, 0x88, 0x18, 0x80, 0x09, 0xFC, 0x3F, 0xE0, 0x08, 0xFC, 0x7F, 0x38, 0x08, 0x00, 0x70, 0x0E, 0x08, 0x00, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // 湿
];

pub fn font_32x32(c: char) -> Option<&'static [u8; 128]>
{
    FONT_32X32
        .iter()
        .find(|(ch, _)| *ch == c)
        .map(|(_, data)| data)
}
```



src/st7789.rs

```rust
#![allow(dead_code)]

use stm32f1xx_hal::{
    pac,
    spi::{Spi, Error as SpiError},
    rcc::Clocks,
};

pub struct ST7789<DC, RST> {
    dc: DC,
    rst: RST,
    width: u16,
    height: u16,
}

impl<DC: embedded_hal::digital::OutputPin, RST: embedded_hal::digital::OutputPin> ST7789<DC, RST> {
    pub fn new(dc: DC, rst: RST) -> Self {
        ST7789 {
            dc,
            rst,
            width: 240,
            height: 240,
        }
    }

    pub fn init<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, clocks: &Clocks) -> Result<(), SpiError> {
        self.hard_reset(clocks);

        self.write_command(spi, 0x11)?;
        self.delay_ms(120, clocks);

        self.write_command(spi, 0x36)?;
        self.write_data(spi, &[0x00])?;

        self.write_command(spi, 0x3A)?;
        self.write_data(spi, &[0x05])?;

        self.write_command(spi, 0xB2)?;
        self.write_data(spi, &[0x0C, 0x0C, 0x00, 0x33, 0x33])?;

        self.write_command(spi, 0xB7)?;
        self.write_data(spi, &[0x35])?;

        self.write_command(spi, 0xBB)?;
        self.write_data(spi, &[0x19])?;

        self.write_command(spi, 0xC0)?;
        self.write_data(spi, &[0x2C])?;

        self.write_command(spi, 0xC2)?;
        self.write_data(spi, &[0x01])?;

        self.write_command(spi, 0xC3)?;
        self.write_data(spi, &[0x12])?;

        self.write_command(spi, 0xC4)?;
        self.write_data(spi, &[0x20])?;

        self.write_command(spi, 0xC6)?;
        self.write_data(spi, &[0x0F])?;

        self.write_command(spi, 0xD0)?;
        self.write_data(spi, &[0xA4, 0xA1])?;

        self.write_command(spi, 0xE0)?;
        self.write_data(spi, &[0xD0, 0x04, 0x0D, 0x11, 0x13, 0x2B, 0x3F, 0x54, 0x4C, 0x18, 0x0D, 0x0B, 0x1F, 0x23])?;

        self.write_command(spi, 0xE1)?;
        self.write_data(spi, &[0xD0, 0x04, 0x0C, 0x11, 0x13, 0x2C, 0x3F, 0x44, 0x51, 0x2F, 0x1F, 0x1F, 0x20, 0x23])?;

        self.write_command(spi, 0x21)?;

        self.write_command(spi, 0x29)?;
        self.delay_ms(20, clocks);

        Ok(())
    }

    fn delay_ms(&self, ms: u32, clocks: &Clocks) {
        let cycles = clocks.sysclk().raw() / 1000 * ms;
        cortex_m::asm::delay(cycles);
    }

    fn hard_reset(&mut self, clocks: &Clocks) {
        let _ = self.rst.set_low();
        self.delay_ms(10, clocks);
        let _ = self.rst.set_high();
        self.delay_ms(120, clocks);
    }

    fn write_command<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, cmd: u8) -> Result<(), SpiError> {
        let _ = self.dc.set_low();
        spi.write(&[cmd])
    }

    fn write_data<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, data: &[u8]) -> Result<(), SpiError> {
        let _ = self.dc.set_high();
        spi.write(data)
    }

    pub fn set_address_window<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), SpiError> {
        self.write_command(spi, 0x2A)?;
        self.write_data(spi, &[
            (x0 >> 8) as u8,
            (x0 & 0xFF) as u8,
            (x1 >> 8) as u8,
            (x1 & 0xFF) as u8,
        ])?;

        self.write_command(spi, 0x2B)?;
        self.write_data(spi, &[
            (y0 >> 8) as u8,
            (y0 & 0xFF) as u8,
            (y1 >> 8) as u8,
            (y1 & 0xFF) as u8,
        ])?;

        self.write_command(spi, 0x2C)
    }

    pub fn fill_rect<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, w: u16, h: u16, color: u16) -> Result<(), SpiError> {
        if w == 0 || h == 0 {
            return Ok(());
        }

        self.set_address_window(spi, x, y, x + w - 1, y + h - 1)?;

        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;
        let pixel_pair = [hi, lo];
        let pixel_count = w as u32 * h as u32;

        let _ = self.dc.set_high();
        for _ in 0..pixel_count {
            spi.write(&pixel_pair)?;
        }

        Ok(())
    }

    pub fn fill_screen<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, color: u16) -> Result<(), SpiError> {
        self.fill_rect(spi, 0, 0, self.width, self.height, color)
    }

    pub fn set_pixel<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, color: u16) -> Result<(), SpiError> {
        self.fill_rect(spi, x, y, 1, 1, color)
    }

    pub fn draw_bitmap<PULL>(
        &mut self,
        spi: &mut Spi<pac::SPI1, u8, PULL>,
        x: u16, y: u16,
        bitmap: &[u8],
        width: u16, height: u16,
        color: u16, bg_color: u16,
    ) -> Result<(), SpiError> {
        let bytes_per_col = ((height + 7) / 8) as usize;
        self.set_address_window(spi, x, y, x + width - 1, y + height - 1)?;
        let _ = self.dc.set_high();
        for row in 0..height {
            for col in 0..width {
                let byte_idx = col as usize * bytes_per_col + row as usize / 8;
                let bit_idx = row % 8;
                let px = if byte_idx < bitmap.len() && (bitmap[byte_idx] & (1 << bit_idx) != 0) {
                    color
                } else {
                    bg_color
                };
                spi.write(&[(px >> 8) as u8, (px & 0xFF) as u8])?;
            }
        }
        Ok(())
    }

    pub fn draw_bitmap_scaled2x<PULL>(
        &mut self,
        spi: &mut Spi<pac::SPI1, u8, PULL>,
        x: u16, y: u16,
        bitmap: &[u8],
        width: u16, height: u16,
        color: u16, bg_color: u16,
    ) -> Result<(), SpiError> {
        let sw = width * 2;
        let sh = height * 2;
        let bytes_per_col = ((height + 7) / 8) as usize;
        self.set_address_window(spi, x, y, x + sw - 1, y + sh - 1)?;
        let _ = self.dc.set_high();
        let hi_c = (color >> 8) as u8;
        let lo_c = (color & 0xFF) as u8;
        let hi_b = (bg_color >> 8) as u8;
        let lo_b = (bg_color & 0xFF) as u8;
        for row in 0..height {
            let mut row_buf = [0u8; 8];
            for col in 0..width {
                let byte_idx = col as usize * bytes_per_col + row as usize / 8;
                let bit_idx = row % 8;
                let on = byte_idx < bitmap.len()
                    && (bitmap[byte_idx] & (1 << bit_idx) != 0);
                let (hi, lo) = if on { (hi_c, lo_c) } else { (hi_b, lo_b) };
                let ci = col as usize * 4;
                row_buf[ci % 8] = hi;
                row_buf[ci % 8 + 1] = lo;
                row_buf[ci % 8 + 2] = hi;
                row_buf[ci % 8 + 3] = lo;
            }
            for _ in 0..2 {
                for col in 0..width {
                    let ci = (col as usize * 4) % 8;
                    spi.write(&row_buf[ci..ci + 4])?;
                }
            }
        }
        Ok(())
    }
}
```



src/main.rs

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod dht11;
mod font_ascii;
mod font_cn;
mod st7789;

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    gpio,
    pac,
    prelude::*,
    rcc,
    spi::{self, Mode, Phase, Polarity},
};

const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

type Display = st7789::ST7789<
    gpio::Pin<'A', 0, gpio::Output<gpio::PushPull>>,
    gpio::Pin<'A', 1, gpio::Output<gpio::PushPull>>,
>;
type Spi1 = spi::Spi<pac::SPI1, u8, gpio::Floating>;

const BG: u16 = 0x0000;
const FG: u16 = 0xFFFF;
const C_TEMP: u16 = 0xFFE0;
const C_HUMI: u16 = 0x07FF;
const C_TITLE: u16 = 0x07FF;
const C_ERR: u16 = 0xF800;

const LABEL_X: u16 = 24;
const CN_W: u16 = 32;
const ASC_W: u16 = 16;
const H: u16 = 32;
const TEMP_Y: u16 = 40;
const HUMI_Y: u16 = 90;
const VAL_X: u16 = 120;
const VAL_W: u16 = 32;

fn draw_cn32(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, c: char, color: u16,
) -> Result<(), spi::Error> {
    if let Some(g) = font_cn::font_32x32(c) {
        d.draw_bitmap(s, x, y, g, 32, 32, color, BG)?;
    }
    Ok(())
}

fn draw_asc(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, c: char, color: u16,
) -> Result<(), spi::Error> {
    let code = c as usize;
    if code < 32 || code > 126 { return Ok(()); }
    let glyph = &font_ascii::FONT_16X32[code - 32];
    d.draw_bitmap(s, x, y, glyph, 16, 32, color, BG)
}

fn draw_str8(
    d: &mut Display, s: &mut Spi1,
    mut x: u16, y: u16, text: &str, color: u16,
) -> Result<(), spi::Error> {
    for ch in text.chars() {
        if (ch as u32) >= 32 && (ch as u32) <= 126 {
            let glyph = &font_ascii::FONT_16X32[ch as usize - 32];
            d.draw_bitmap(s, x, y, glyph, 16, 32, color, BG)?;
            x += 16;
        }
    }
    Ok(())
}

fn draw_u8(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, val: u8, color: u16,
) -> Result<(), spi::Error> {
    draw_asc(d, s, x, y, (b'0' + val / 10) as char, color)?;
    draw_asc(d, s, x + ASC_W, y, (b'0' + val % 10) as char, color)
}

fn clear_val(d: &mut Display, s: &mut Spi1, y: u16) -> Result<(), spi::Error> {
    d.fill_rect(s, VAL_X, y, VAL_W, H, BG)
}

fn draw_layout(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_screen(s, BG)?;
    draw_str8(d, s, 40, 4, "== DHT11 ==", C_TITLE)?;
    draw_cn32(d, s, LABEL_X, TEMP_Y, '\u{6E29}', FG)?;
    draw_cn32(d, s, LABEL_X + CN_W, TEMP_Y, '\u{5EA6}', FG)?;
    draw_asc(d, s, LABEL_X + CN_W * 2, TEMP_Y, ':', FG)?;
    draw_cn32(d, s, VAL_X + VAL_W, TEMP_Y, '\u{2103}', C_TEMP)?;
    draw_cn32(d, s, LABEL_X, HUMI_Y, '\u{6E7F}', FG)?;
    draw_cn32(d, s, LABEL_X + CN_W, HUMI_Y, '\u{5EA6}', FG)?;
    draw_asc(d, s, LABEL_X + CN_W * 2, HUMI_Y, ':', FG)?;
    draw_asc(d, s, VAL_X + VAL_W, HUMI_Y, '%', C_HUMI)
}

fn draw_err_msg(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_rect(s, 24, 22, 192, 16, BG)?;
    draw_str8(d, s, 24, 22, "Sensor Error!", C_ERR)
}

fn clear_err_msg(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_rect(s, 24, 22, 192, 16, BG)
}

#[allow(dead_code)]
enum Ui {
    Fresh,
    Data { t: u8, h: u8 },
    Err { t: u8, h: u8 },
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("DHT11 + ST7789 32x32");

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc_cfg = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    let clocks = rcc_cfg.clocks.clone();
    let mut gpioa = dp.GPIOA.split(&mut rcc_cfg);

    let dc = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let res = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut spi = dp.SPI1.spi(
        (Some(gpioa.pa5), pac::SPI1::NoMiso, Some(gpioa.pa7)),
        SPI_MODE, 16.MHz(), &mut rcc_cfg,
    );
    let mut display = st7789::ST7789::new(dc, res);
    display.init(&mut spi, &clocks).unwrap();

    let mut delay = dht11::Delay::new(cp.SYST);
    let mut dht11_pin = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    dht11_pin.set_high();

    display.fill_screen(&mut spi, BG).unwrap();
    let _ = draw_str8(&mut display, &mut spi, 24, 100, "DHT11 32x32 Demo", C_TITLE);
    let _ = draw_str8(&mut display, &mut spi, 56, 140, "Wait 2s...", 0x8410);
    delay.ms(2000);

    let mut ui = Ui::Fresh;

    loop {
        let mut res = Err(dht11::Error::NoResponse);
        for attempt in 0u8..3 {
            let (r, pin) = dht11::Dht11::read(dht11_pin, &mut gpioa.crl, &mut delay);
            dht11_pin = pin;
            match r {
                Ok((h, t)) => { res = Ok((h, t)); break; }
                Err(e) => { rprintln!("try{}: {:?}", attempt + 1, e); delay.ms(1500); }
            }
        }

        match (&ui, &res) {
            (Ui::Fresh, Ok((h, t))) => {
                let _ = draw_layout(&mut display, &mut spi);
                let _ = draw_u8(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = draw_u8(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Data { t: *t, h: *h };
                rprintln!("T:{}C H:{}%", t, h);
            }
            (Ui::Fresh, Err(_)) => {
                let _ = draw_layout(&mut display, &mut spi);
                let _ = draw_err_msg(&mut display, &mut spi);
                ui = Ui::Err { t: 0, h: 0 };
            }
            (Ui::Data { t, h }, Ok((nh, nt))) => {
                if *nt != *t {
                    let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                    let _ = draw_u8(&mut display, &mut spi, VAL_X, TEMP_Y, *nt, C_TEMP);
                }
                if *nh != *h {
                    let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                    let _ = draw_u8(&mut display, &mut spi, VAL_X, HUMI_Y, *nh, C_HUMI);
                }
                ui = Ui::Data { t: *nt, h: *nh };
                rprintln!("T:{}C H:{}%", nt, nh);
            }
            (Ui::Data { t, h }, Err(_)) => {
                let _ = draw_err_msg(&mut display, &mut spi);
                let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                let _ = draw_u8(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                let _ = draw_u8(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Err { t: *t, h: *h };
            }
            (Ui::Err { .. }, Ok((h, t))) => {
                let _ = clear_err_msg(&mut display, &mut spi);
                let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                let _ = draw_u8(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                let _ = draw_u8(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Data { t: *t, h: *h };
                rprintln!("T:{}C H:{}%", t, h);
            }
            (Ui::Err { .. }, Err(_)) => {}
        }

        delay.ms(2000);
    }
}
```



font_tool.py

```python
#!/usr/bin/env python3
"""
Font Bitmap Generator — PyQt5 GUI
Supports TTF/OTF font loading, configurable sizes, i18n (zh_CN / en)
Generates Rust-compatible byte arrays for embedded displays.

Usage:
    python font_tool.py
"""

import sys
import os
from PIL import Image, ImageDraw, ImageFont
from PyQt5.QtWidgets import (
    QApplication, QMainWindow, QWidget, QVBoxLayout, QHBoxLayout,
    QGridLayout, QLabel, QPushButton, QComboBox, QLineEdit,
    QSpinBox, QTextEdit, QFileDialog, QGroupBox,
    QMessageBox, QScrollArea, QSizePolicy, QDialog,
)
from PyQt5.QtCore import Qt, pyqtSignal
from PyQt5.QtGui import QPainter, QColor, QFont, QPalette

# ============================================================
#  i18n
# ============================================================

TRANS = {
    "zh_CN": {
        "app_title": "字模生成器",
        "lang_label": "语言",
        "group_font": "字体设置",
        "font_file": "字体文件",
        "browse": "浏览...",
        "bitmap_size": "点阵尺寸 (WxH)",
        "font_pt": "字号 (pt)",
        "group_chars": "字符设置",
        "preset": "预设",
        "preset_ascii_print": "可打印 ASCII (32~126)",
        "preset_ascii_digits": "数字 0-9",
        "preset_ascii_upper": "大写字母 A-Z",
        "preset_ascii_lower": "小写字母 a-z",
        "preset_ascii_hex": "十六进制 0-9A-F",
        "preset_custom": "自定义",
        "custom_chars": "自定义字符",
        "custom_hint": "输入要生成字模的字符（支持汉字/日韩/符号等任意Unicode）...",
        "group_options": "生成选项",
        "bit_order": "比特序",
        "lsb_first": "LSB 在上 (嵌入式常用)",
        "msb_first": "MSB 在上",
        "layout": "排列方式",
        "col_major": "列优先 (每字节一列)",
        "row_major": "行优先 (每字节一行)",
        "var_name": "变量名",
        "generate": "生成字模",
        "group_preview": "预览",
        "group_code": "Rust 代码",
        "copy": "复制代码",
        "save": "保存文件",
        "clear": "清空",
        "char_info": "字符信息",
        "status_ready": "就绪",
        "status_generated": "已生成 {n} 个字符, 共 {b} 字节",
        "status_copied": "代码已复制到剪贴板",
        "status_saved": "已保存到 {path}",
        "err_no_font": "请先选择字体文件",
        "err_no_chars": "请输入至少一个字符",
        "err_font_not_found": "字体文件不存在: {path}",
        "preview_click": "点击字符查看详细位图",
        "bytes_per_char": "每字符字节数",
        "total_chars": "总字符数",
        "total_bytes": "总字节数",
        "hex_preview": "十六进制预览",
        "bitmap_detail": "位图详情",
        "preview_btn": "预览字模",
        "preview_title": "字模预览",
        "no_data": "请先点击「生成字模」",
        "draw_btn": "涂鸦字模",
        "draw_title": "涂鸦编辑器",
        "draw_char": "字符",
        "draw_char_hint": "输入字符 或 Unicode码 (如: A / U+0041)",
        "draw_add": "添加到字模",
        "draw_clear_grid": "清除画布",
        "draw_close": "关闭",
        "draw_info": "左键画点 | 右键擦除 | 拖动连续绘制",
        "draw_no_char": "请先输入字符",
        "draw_added": "已添加字符 '{ch}'",
        "draw_preview": "实时预览",
    },
    "en": {
        "app_title": "Font Bitmap Generator",
        "lang_label": "Language",
        "group_font": "Font Settings",
        "font_file": "Font File",
        "browse": "Browse...",
        "bitmap_size": "Bitmap Size (WxH)",
        "font_pt": "Font Size (pt)",
        "group_chars": "Character Settings",
        "preset": "Preset",
        "preset_ascii_print": "Printable ASCII (32~126)",
        "preset_ascii_digits": "Digits 0-9",
        "preset_ascii_upper": "Uppercase A-Z",
        "preset_ascii_lower": "Lowercase a-z",
        "preset_ascii_hex": "Hexadecimal 0-9A-F",
        "preset_custom": "Custom",
        "custom_chars": "Custom Characters",
        "custom_hint": "Enter characters (CJK, symbols, any Unicode)...",
        "group_options": "Generation Options",
        "bit_order": "Bit Order",
        "lsb_first": "LSB First (embedded common)",
        "msb_first": "MSB First",
        "layout": "Layout",
        "col_major": "Column-major (byte per column)",
        "row_major": "Row-major (byte per row)",
        "var_name": "Variable Name",
        "generate": "Generate",
        "group_preview": "Preview",
        "group_code": "Rust Code",
        "copy": "Copy Code",
        "save": "Save File",
        "clear": "Clear",
        "char_info": "Char Info",
        "status_ready": "Ready",
        "status_generated": "Generated {n} chars, {b} bytes total",
        "status_copied": "Code copied to clipboard",
        "status_saved": "Saved to {path}",
        "err_no_font": "Please select a font file first",
        "err_no_chars": "Please enter at least one character",
        "err_font_not_found": "Font file not found: {path}",
        "preview_click": "Click a character to view detail",
        "bytes_per_char": "Bytes/char",
        "total_chars": "Total chars",
        "total_bytes": "Total bytes",
        "hex_preview": "Hex Preview",
        "bitmap_detail": "Bitmap Detail",
        "preview_btn": "Preview",
        "preview_title": "Font Preview",
        "no_data": "Click 'Generate' first",
        "draw_btn": "Pixel Draw",
        "draw_title": "Pixel Editor",
        "draw_char": "Character",
        "draw_char_hint": "Enter char or Unicode (e.g. A / U+0041)",
        "draw_add": "Add to Data",
        "draw_clear_grid": "Clear Canvas",
        "draw_close": "Close",
        "draw_info": "Left: paint | Right: erase | Drag to draw",
        "draw_no_char": "Please enter a character first",
        "draw_added": "Added character '{ch}'",
        "draw_preview": "Live Preview",
    },
}

PRESETS = {
    "preset_ascii_print": (32, 127),
    "preset_ascii_digits": (ord("0"), ord("9") + 1),
    "preset_ascii_upper": (ord("A"), ord("Z") + 1),
    "preset_ascii_lower": (ord("a"), ord("z") + 1),
    "preset_ascii_hex": None,
}


class I18n:
    def __init__(self, lang="zh_CN"):
        self.lang = lang

    def t(self, key, **kw):
        txt = TRANS.get(self.lang, TRANS["en"]).get(key, key)
        for k, v in kw.items():
            txt = txt.replace("{" + k + "}", str(v))
        return txt

    def set_lang(self, lang):
        self.lang = lang


i18n = I18n("zh_CN")

# ============================================================
#  Bitmap generation core
# ============================================================


def generate_bitmaps(
    text: str,
    font_path: str,
    width: int,
    height: int,
    font_size: int,
    lsb_first: bool = True,
    col_major: bool = True,
) -> list:
    """
    Generate bitmap data for each character.

    Returns: list of (char, [u8, ...])
    """
    try:
        font = ImageFont.truetype(font_path, font_size)
    except Exception:
        font = ImageFont.load_default()

    results = []
    seen = set()
    bytes_per_col = (height + 7) // 8

    for ch in text:
        if ch in seen:
            continue
        seen.add(ch)

        img = Image.new("1", (width, height), 0)
        draw = ImageDraw.Draw(img)

        bbox = draw.textbbox((0, 0), ch, font=font)
        tw = bbox[2] - bbox[0]
        th = bbox[3] - bbox[1]
        x = (width - tw) // 2 - bbox[0]
        y = (height - th) // 2 - bbox[1]
        draw.text((x, y), ch, fill=1, font=font)

        pixels = list(img.getdata())
        byte_data = []

        if col_major:
            for col in range(width):
                for bi in range(bytes_per_col):
                    val = 0
                    for bit in range(8):
                        row = bi * 8 + bit
                        if row < height:
                            if pixels[row * width + col]:
                                val |= 1 << bit
                    byte_data.append(val)
        else:
            bytes_per_row = (width + 7) // 8
            for row in range(height):
                for bi in range(bytes_per_row):
                    val = 0
                    for bit in range(8):
                        col = bi * 8 + bit
                        if col < width:
                            if pixels[row * width + col]:
                                if lsb_first:
                                    val |= 1 << bit
                                else:
                                    val |= 1 << (7 - bit)
                    byte_data.append(val)

        results.append((ch, byte_data))

    return results


def format_rust_code(
    data: list,
    width: int,
    height: int,
    var_name: str,
    lsb_first: bool,
    col_major: bool,
) -> str:
    """Format bitmap data as Rust const array."""
    if not data:
        return "// No data"

    bpc = len(data[0][1])
    is_ascii = all(32 <= ord(c) <= 126 for c, _ in data)

    lines = []
    lines.append("#![allow(dead_code)]")
    lines.append("")

    bit_desc = "LSB在上" if lsb_first else "MSB在上"
    layout_desc = "列优先" if col_major else "行优先"
    bit_desc_en = "LSB-first" if lsb_first else "MSB-first"
    layout_desc_en = "column-major" if col_major else "row-major"

    lines.append(f"/// {width}x{height} font bitmap")
    lines.append(f"/// {bpc} bytes/char, {layout_desc_en}, {bit_desc_en}")
    lines.append(f"/// {len(data)} characters")
    lines.append("")

    if is_ascii and len(data) > 1:
        sorted_data = sorted(data, key=lambda x: ord(x[0]))
        start = ord(sorted_data[0][0])
        end = ord(sorted_data[-1][0])
        arr_name = f"{var_name}_{width}X{height}"
        lines.append(
            f"pub const {arr_name}: [[u8; {bpc}]; {len(sorted_data)}] = ["
        )
        for ch, byte_data in sorted_data:
            code = ord(ch)
            hex_str = ", ".join(f"0x{b:02X}" for b in byte_data)
            display = ch if ch.isprintable() and ch != "\\" else f"\\u{{{code:X}}}"
            lines.append(f"    [{hex_str}], // {code:3d} '{display}'")
        lines.append("];")

        lines.append("")
        lines.append(
            f"pub fn {arr_name.lower()}(c: char) -> Option<&'static [u8; {bpc}]>"
        )
        lines.append("{")
        lines.append(f"    let idx = c as usize;")
        lines.append(f"    if idx >= {start} && idx <= {end} {{")
        lines.append(
            f"        Some(&{arr_name}[idx - {start}])"
        )
        lines.append("    } else {")
        lines.append("        None")
        lines.append("    }")
        lines.append("}")
    else:
        arr_name = f"{var_name}_{width}X{height}"
        lines.append(f"pub const {arr_name}: &[(char, [u8; {bpc}])] = &[")
        for ch, byte_data in sorted(data, key=lambda x: ord(x[0])):
            code = ord(ch)
            hex_str = ", ".join(f"0x{b:02X}" for b in byte_data)
            lines.append(f"    ('\\u{{{code:X}}}', [{hex_str}]), // {ch}")
        lines.append("];")

        lines.append("")
        lines.append(
            f"pub fn {arr_name.lower()}(c: char) -> Option<&'static [u8; {bpc}]>"
        )
        lines.append("{")
        lines.append(f"    {arr_name}")
        lines.append("        .iter()")
        lines.append("        .find(|(ch, _)| *ch == c)")
        lines.append("        .map(|(_, data)| data)")
        lines.append("}")

    return "\n".join(lines)


# ============================================================
#  Preview widget
# ============================================================


class CharPreviewWidget(QWidget):
    """Grid preview of all generated character bitmaps."""

    char_clicked = pyqtSignal(str)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.chars = []
        self.char_w = 8
        self.char_h = 16
        self.scale = 2
        self.setMinimumHeight(80)
        self.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Preferred)

    def set_data(self, data, width, height, scale=2):
        self.chars = data
        self.char_w = width
        self.char_h = height
        self.scale = scale
        cols = max(1, 400 // (width * scale + 4))
        rows = (len(data) + cols - 1) // cols
        h = max(80, rows * (height * scale + 20) + 20)
        self.setMinimumHeight(h)
        self.update()

    def paintEvent(self, event):
        if not self.chars:
            return
        painter = QPainter(self)
        painter.setRenderHint(QPainter.Antialiasing, False)

        cell_w = self.char_w * self.scale + 4
        cell_h = self.char_h * self.scale + 20
        cols = max(1, self.width() // cell_w)
        margin = 4

        for i, (ch, byte_data) in enumerate(self.chars):
            col = i % cols
            row = i // cols
            ox = margin + col * cell_w
            oy = margin + row * cell_h

            for py in range(self.char_h):
                for px in range(self.char_w):
                    bpc = (self.char_h + 7) // 8
                    byte_idx = px * bpc + py // 8
                    bit_idx = py % 8
                    on = False
                    if byte_idx < len(byte_data):
                        on = byte_data[byte_idx] & (1 << bit_idx) != 0
                    color = QColor(0, 200, 100) if on else QColor(30, 30, 30)
                    painter.fillRect(
                        ox + px * self.scale,
                        oy + py * self.scale,
                        self.scale,
                        self.scale,
                        color,
                    )

            painter.setPen(QColor(180, 180, 180))
            display = ch if ch.isprintable() else "?"
            painter.drawText(
                ox, oy + self.char_h * self.scale + 12, display
            )

        painter.end()

    def mousePressEvent(self, event):
        if not self.chars:
            return
        cell_w = self.char_w * self.scale + 4
        cols = max(1, self.width() // cell_w)
        margin = 4
        col = (event.x() - margin) // cell_w
        row = (event.y() - margin) // (self.char_h * self.scale + 20)
        idx = row * cols + col
        if 0 <= idx < len(self.chars):
            self.char_clicked.emit(self.chars[idx][0])


class BitmapDetailWidget(QWidget):
    """Detailed bitmap view for a single character."""

    def __init__(self, parent=None):
        super().__init__(parent)
        self.char = None
        self.byte_data = []
        self.bmp_w = 8
        self.bmp_h = 16
        self.scale = 8
        self.setMinimumSize(200, 200)
        self.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

    def set_char(self, ch, byte_data, width, height):
        self.char = ch
        self.byte_data = byte_data
        self.bmp_w = width
        self.bmp_h = height
        self.scale = max(4, min(16, 200 // max(width, height)))
        self.setFixedSize(
            width * self.scale + 60,
            max(height * self.scale + 20, 160),
        )
        self.update()

    def paintEvent(self, event):
        if not self.char:
            return
        painter = QPainter(self)
        painter.setRenderHint(QPainter.Antialiasing, False)
        bpc = (self.bmp_h + 7) // 8
        ox, oy = 10, 10

        for py in range(self.bmp_h):
            for px in range(self.bmp_w):
                byte_idx = px * bpc + py // 8
                bit_idx = py % 8
                on = False
                if byte_idx < len(self.byte_data):
                    on = self.byte_data[byte_idx] & (1 << bit_idx) != 0
                color = QColor(0, 220, 120) if on else QColor(25, 25, 25)
                painter.fillRect(
                    ox + px * self.scale,
                    oy + py * self.scale,
                    self.scale - 1,
                    self.scale - 1,
                    color,
                )

        info_x = ox + self.bmp_w * self.scale + 10
        info_y = 20
        painter.setPen(QColor(200, 200, 200))
        painter.setFont(QFont("monospace", 9))

        display = self.char if self.char.isprintable() else f"U+{ord(self.char):04X}"
        painter.drawText(info_x, info_y, f"Char: {display}")
        info_y += 18
        painter.drawText(info_x, info_y, f"U+{ord(self.char):04X}")
        info_y += 18
        painter.drawText(info_x, info_y, f"{self.bmp_w}x{self.bmp_h}")
        info_y += 26

        painter.drawText(info_x, info_y, i18n.t("hex_preview") + ":")
        info_y += 18
        hex_str = " ".join(f"{b:02X}" for b in self.byte_data)
        for i in range(0, len(hex_str), 24):
            painter.drawText(info_x, info_y, hex_str[i : i + 24])
            info_y += 16

        painter.end()


# ============================================================
#  Preview dialog
# ============================================================


class PreviewDialog(QDialog):
    """Standalone preview window opened by a button."""

    def __init__(self, data, width, height, parent=None):
        super().__init__(parent)
        self.setWindowTitle(i18n.t("preview_title"))
        self.setMinimumSize(640, 500)
        self.resize(720, 600)

        layout = QVBoxLayout(self)

        hint = QLabel(i18n.t("preview_click"))
        hint.setStyleSheet("color: #888; font-size: 11px;")
        layout.addWidget(hint)

        self.scroll = QScrollArea()
        self.scroll.setWidgetResizable(True)
        self.char_preview = CharPreviewWidget()
        self.char_preview.char_clicked.connect(self._on_char_clicked)
        self.scroll.setWidget(self.char_preview)
        layout.addWidget(self.scroll, 1)

        self.detail = BitmapDetailWidget()
        layout.addWidget(self.detail)

        scale = max(1, min(4, 200 // max(width, height)))
        self.char_w = width
        self.char_h = height
        self.char_preview.set_data(data, width, height, scale)

    def _on_char_clicked(self, ch):
        for c, byte_data in self.char_preview.chars:
            if c == ch:
                self.detail.set_char(ch, byte_data, self.char_w, self.char_h)
                break


# ============================================================
#  Pixel editor widget
# ============================================================


class PixelEditorWidget(QWidget):
    """Grid for hand-drawing bitmaps pixel by pixel."""

    pixel_changed = pyqtSignal()

    def __init__(self, grid_w, grid_h, parent=None):
        super().__init__(parent)
        self.grid_w = grid_w
        self.grid_h = grid_h
        self.cell_size = min(28, max(12, 380 // max(grid_w, grid_h)))
        self.pixels = [[False] * grid_w for _ in range(grid_h)]
        self.drawing = False
        self.draw_val = True
        w = grid_w * self.cell_size + 1
        h = grid_h * self.cell_size + 1
        self.setFixedSize(w, h)

    def clear(self):
        self.pixels = [[False] * self.grid_w for _ in range(self.grid_h)]
        self.update()
        self.pixel_changed.emit()

    def set_pixels_from_bytes(self, byte_data, col_major=True):
        bpc = (self.grid_h + 7) // 8
        for row in range(self.grid_h):
            for col in range(self.grid_w):
                if col_major:
                    idx = col * bpc + row // 8
                    bit = row % 8
                else:
                    bpr = (self.grid_w + 7) // 8
                    idx = row * bpr + col // 8
                    bit = col % 8
                self.pixels[row][col] = (
                    idx < len(byte_data) and (byte_data[idx] & (1 << bit)) != 0
                )
        self.update()
        self.pixel_changed.emit()

    def get_byte_data(self, col_major=True, lsb_first=True):
        data = []
        bpc = (self.grid_h + 7) // 8
        if col_major:
            for col in range(self.grid_w):
                for bi in range(bpc):
                    val = 0
                    for bit in range(8):
                        row = bi * 8 + bit
                        if row < self.grid_h and self.pixels[row][col]:
                            val |= 1 << bit
                    data.append(val)
        else:
            bpr = (self.grid_w + 7) // 8
            for row in range(self.grid_h):
                for bi in range(bpr):
                    val = 0
                    for bit in range(8):
                        col = bi * 8 + bit
                        if col < self.grid_w and self.pixels[row][col]:
                            if lsb_first:
                                val |= 1 << bit
                            else:
                                val |= 1 << (7 - bit)
                    data.append(val)
        return data

    def _hit(self, x, y):
        c = x // self.cell_size
        r = y // self.cell_size
        if 0 <= c < self.grid_w and 0 <= r < self.grid_h:
            return (c, r)
        return None

    def mousePressEvent(self, e):
        pos = self._hit(e.x(), e.y())
        if pos is None:
            return
        c, r = pos
        if e.button() == Qt.LeftButton:
            self.drawing = True
            self.draw_val = not self.pixels[r][c]
            self.pixels[r][c] = self.draw_val
        elif e.button() == Qt.RightButton:
            self.pixels[r][c] = False
        self.update()
        self.pixel_changed.emit()

    def mouseMoveEvent(self, e):
        if not self.drawing:
            return
        pos = self._hit(e.x(), e.y())
        if pos is None:
            return
        c, r = pos
        self.pixels[r][c] = self.draw_val
        self.update()
        self.pixel_changed.emit()

    def mouseReleaseEvent(self, e):
        self.drawing = False

    def paintEvent(self, e):
        p = QPainter(self)
        p.setRenderHint(QPainter.Antialiasing, False)
        cs = self.cell_size
        for r in range(self.grid_h):
            for c in range(self.grid_w):
                color = QColor(0, 200, 100) if self.pixels[r][c] else QColor(30, 30, 30)
                p.fillRect(c * cs, r * cs, cs - 1, cs - 1, color)
        p.setPen(QColor(55, 55, 55))
        for r in range(self.grid_h + 1):
            p.drawLine(0, r * cs, self.grid_w * cs, r * cs)
        for c in range(self.grid_w + 1):
            p.drawLine(c * cs, 0, c * cs, self.grid_h * cs)
        p.end()


class DrawPreviewWidget(QWidget):
    """Tiny real-time preview of the drawn bitmap."""

    def __init__(self, parent=None):
        super().__init__(parent)
        self.byte_data = []
        self.bmp_w = 8
        self.bmp_h = 16
        self.scale = 4
        self.setFixedSize(200, 200)

    def update_data(self, byte_data, w, h):
        self.byte_data = byte_data
        self.bmp_w = w
        self.bmp_h = h
        self.scale = max(2, min(12, 180 // max(w, h)))
        self.setFixedSize(w * self.scale + 2, h * self.scale + 2)
        self.update()

    def paintEvent(self, e):
        if not self.byte_data:
            return
        p = QPainter(self)
        p.setRenderHint(QPainter.Antialiasing, False)
        bpc = (self.bmp_h + 7) // 8
        s = self.scale
        for r in range(self.bmp_h):
            for c in range(self.bmp_w):
                idx = c * bpc + r // 8
                bit = r % 8
                on = idx < len(self.byte_data) and (self.byte_data[idx] & (1 << bit)) != 0
                color = QColor(0, 220, 120) if on else QColor(20, 20, 20)
                p.fillRect(c * s, r * s, s, s, color)
        p.end()


class PixelEditorDialog(QDialog):
    """Dialog for hand-drawing a character bitmap."""

    def __init__(self, grid_w, grid_h, col_major, lsb_first, parent=None):
        super().__init__(parent)
        self.grid_w = grid_w
        self.grid_h = grid_h
        self.col_major = col_major
        self.lsb_first = lsb_first
        self.added_data = []
        self.setWindowTitle(f"{i18n.t('draw_title')} — {grid_w}x{grid_h}")
        self.setMinimumSize(560, 460)
        self._build()

    def _build(self):
        lay = QVBoxLayout(self)

        top = QHBoxLayout()
        top.addWidget(QLabel(i18n.t("draw_char")))
        self.txt_char = QLineEdit()
        self.txt_char.setPlaceholderText(i18n.t("draw_char_hint"))
        self.txt_char.setMaximumWidth(260)
        top.addWidget(self.txt_char)
        top.addStretch()
        lay.addLayout(top)

        mid = QHBoxLayout()
        self.editor = PixelEditorWidget(self.grid_w, self.grid_h)
        self.editor.pixel_changed.connect(self._on_pixel_changed)
        mid.addWidget(self.editor, 0, Qt.AlignTop)

        right = QVBoxLayout()
        right.addWidget(QLabel(i18n.t("draw_preview")))
        self.preview = DrawPreviewWidget()
        right.addWidget(self.preview, 0, Qt.AlignTop)

        self.lbl_hex = QLabel()
        self.lbl_hex.setWordWrap(True)
        self.lbl_hex.setFont(QFont("Menlo", 9))
        self.lbl_hex.setStyleSheet("color: #aaa;")
        self.lbl_hex.setMinimumWidth(180)
        self.lbl_hex.setMaximumWidth(220)
        right.addWidget(self.lbl_hex)
        right.addStretch()
        mid.addLayout(right)
        lay.addLayout(mid)

        info = QLabel(i18n.t("draw_info"))
        info.setStyleSheet("color: #666; font-size: 11px;")
        lay.addWidget(info)

        btns = QHBoxLayout()
        btns.addStretch()
        b_clear = QPushButton(i18n.t("draw_clear_grid"))
        b_clear.clicked.connect(self.editor.clear)
        btns.addWidget(b_clear)
        self.b_add = QPushButton(i18n.t("draw_add"))
        self.b_add.setStyleSheet(
            "QPushButton{background:#1a6b3a;}QPushButton:hover{background:#228b4a;}"
        )
        self.b_add.clicked.connect(self._add_char)
        btns.addWidget(self.b_add)
        b_close = QPushButton(i18n.t("draw_close"))
        b_close.clicked.connect(self.close)
        btns.addWidget(b_close)
        lay.addLayout(btns)

        self._on_pixel_changed()

    def _parse_char(self):
        txt = self.txt_char.text().strip()
        if not txt:
            return None
        if txt.upper().startswith("U+"):
            try:
                return chr(int(txt[2:], 16))
            except ValueError:
                return None
        if txt.upper().startswith("0X") and len(txt) > 2:
            try:
                return chr(int(txt[2:], 16))
            except ValueError:
                return None
        return txt[0]

    def _on_pixel_changed(self):
        data = self.editor.get_byte_data(self.col_major, self.lsb_first)
        self.preview.update_data(data, self.grid_w, self.grid_h)
        hex_str = " ".join(f"{b:02X}" for b in data)
        bpc = len(data)
        self.lbl_hex.setText(
            f"{i18n.t('bytes_per_char')}: {bpc}\n\n{hex_str}"
        )

    def _add_char(self):
        ch = self._parse_char()
        if ch is None:
            QMessageBox.warning(self, "", i18n.t("draw_no_char"))
            return
        data = self.editor.get_byte_data(self.col_major, self.lsb_first)
        self.added_data.append((ch, data))
        QMessageBox.information(self, "", i18n.t("draw_added", ch=ch))


# ============================================================
#  Main window
# ============================================================


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.generated_data = []
        self.rust_code = ""
        self._build_ui()
        self._retranslate()

    def _build_ui(self):
        self.setMinimumSize(680, 560)

        central = QWidget()
        self.setCentralWidget(central)
        main_layout = QVBoxLayout(central)
        main_layout.setSpacing(6)

        # --- Top bar: language ---
        top_bar = QHBoxLayout()
        top_bar.addStretch()
        top_bar.addWidget(QLabel())
        self.lbl_lang = QLabel()
        top_bar.addWidget(self.lbl_lang)
        self.cmb_lang = QComboBox()
        self.cmb_lang.addItem("简体中文", "zh_CN")
        self.cmb_lang.addItem("English", "en")
        self.cmb_lang.currentIndexChanged.connect(self._on_lang_changed)
        top_bar.addWidget(self.cmb_lang)
        main_layout.addLayout(top_bar)

        # --- Font settings ---
        grp_font = QGroupBox()
        self.grp_font = grp_font
        font_grid = QGridLayout(grp_font)

        self.lbl_font_file = QLabel()
        font_grid.addWidget(self.lbl_font_file, 0, 0)
        self.txt_font = QLineEdit()
        self.txt_font.setPlaceholderText("")
        font_grid.addWidget(self.txt_font, 0, 1)
        self.btn_browse = QPushButton()
        self.btn_browse.clicked.connect(self._browse_font)
        font_grid.addWidget(self.btn_browse, 0, 2)

        self.lbl_size = QLabel()
        font_grid.addWidget(self.lbl_size, 1, 0)
        size_layout = QHBoxLayout()
        self.spn_w = QSpinBox()
        self.spn_w.setRange(1, 256)
        self.spn_w.setValue(8)
        size_layout.addWidget(self.spn_w)
        size_layout.addWidget(QLabel("x"))
        self.spn_h = QSpinBox()
        self.spn_h.setRange(1, 256)
        self.spn_h.setValue(16)
        size_layout.addWidget(self.spn_h)
        size_layout.addStretch()
        font_grid.addLayout(size_layout, 1, 1)

        self.lbl_pt = QLabel()
        font_grid.addWidget(self.lbl_pt, 2, 0)
        self.spn_pt = QSpinBox()
        self.spn_pt.setRange(1, 512)
        self.spn_pt.setValue(16)
        font_grid.addWidget(self.spn_pt, 2, 1)

        main_layout.addWidget(grp_font)

        # --- Character settings ---
        grp_chars = QGroupBox()
        self.grp_chars = grp_chars
        chars_grid = QGridLayout(grp_chars)

        self.lbl_preset = QLabel()
        chars_grid.addWidget(self.lbl_preset, 0, 0)
        self.cmb_preset = QComboBox()
        self.cmb_preset.currentIndexChanged.connect(self._on_preset_changed)
        chars_grid.addWidget(self.cmb_preset, 0, 1)

        self.lbl_custom = QLabel()
        chars_grid.addWidget(self.lbl_custom, 1, 0)
        self.txt_chars = QLineEdit()
        chars_grid.addWidget(self.txt_chars, 1, 1)

        main_layout.addWidget(grp_chars)

        # --- Options ---
        grp_opts = QGroupBox()
        self.grp_opts = grp_opts
        opts_grid = QGridLayout(grp_opts)

        self.lbl_bit = QLabel()
        opts_grid.addWidget(self.lbl_bit, 0, 0)
        self.cmb_bit = QComboBox()
        opts_grid.addWidget(self.cmb_bit, 0, 1)

        self.lbl_layout = QLabel()
        opts_grid.addWidget(self.lbl_layout, 1, 0)
        self.cmb_layout = QComboBox()
        opts_grid.addWidget(self.cmb_layout, 1, 1)

        self.lbl_var = QLabel()
        opts_grid.addWidget(self.lbl_var, 2, 0)
        self.txt_var = QLineEdit("FONT")
        opts_grid.addWidget(self.txt_var, 2, 1)

        main_layout.addWidget(grp_opts)

        # --- Action buttons ---
        btn_layout = QHBoxLayout()
        btn_layout.addStretch()
        self.btn_gen = QPushButton()
        self.btn_gen.setMinimumSize(120, 36)
        self.btn_gen.clicked.connect(self._generate)
        btn_layout.addWidget(self.btn_gen)
        self.btn_draw = QPushButton()
        self.btn_draw.setMinimumSize(120, 36)
        self.btn_draw.clicked.connect(self._open_pixel_editor)
        btn_layout.addWidget(self.btn_draw)
        self.btn_preview = QPushButton()
        self.btn_preview.setMinimumSize(140, 36)
        self.btn_preview.clicked.connect(self._open_preview)
        btn_layout.addWidget(self.btn_preview)
        btn_layout.addStretch()
        main_layout.addLayout(btn_layout)

        # --- Code area ---
        code_group = QGroupBox()
        self.grp_code = code_group
        code_layout = QVBoxLayout(code_group)

        self.txt_code = QTextEdit()
        self.txt_code.setReadOnly(True)
        self.txt_code.setFont(QFont("Menlo", 11))
        self.txt_code.setMinimumHeight(120)
        code_layout.addWidget(self.txt_code)

        code_btn_layout = QHBoxLayout()
        self.btn_copy = QPushButton()
        self.btn_copy.clicked.connect(self._copy_code)
        code_btn_layout.addWidget(self.btn_copy)
        self.btn_save = QPushButton()
        self.btn_save.clicked.connect(self._save_code)
        code_btn_layout.addWidget(self.btn_save)
        self.btn_clear = QPushButton()
        self.btn_clear.clicked.connect(self._clear_all)
        code_btn_layout.addWidget(self.btn_clear)
        code_btn_layout.addStretch()
        code_layout.addLayout(code_btn_layout)

        main_layout.addWidget(code_group)

        # Status bar
        self.statusBar().showMessage("")

        self._default_font_path()

    def _default_font_path(self):
        candidates = [
            "/System/Library/Fonts/Menlo.ttc",
            "/System/Library/Fonts/Courier.dfont",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "C:/Windows/Fonts/consola.ttf",
        ]
        for p in candidates:
            if os.path.exists(p):
                self.txt_font.setText(p)
                break

    def _browse_font(self):
        path, _ = QFileDialog.getOpenFileName(
            self,
            i18n.t("browse"),
            "",
            "Fonts (*.ttf *.otf *.ttc *.dfont);;All (*)",
        )
        if path:
            self.txt_font.setText(path)

    def _on_preset_changed(self, idx):
        key = self.cmb_preset.currentData()
        if key == "preset_ascii_print":
            self.txt_chars.setText(
                "".join(chr(c) for c in range(32, 127))
            )
        elif key == "preset_ascii_digits":
            self.txt_chars.setText("0123456789")
        elif key == "preset_ascii_upper":
            self.txt_chars.setText(
                "".join(chr(c) for c in range(ord("A"), ord("Z") + 1))
            )
        elif key == "preset_ascii_lower":
            self.txt_chars.setText(
                "".join(chr(c) for c in range(ord("a"), ord("z") + 1))
            )
        elif key == "preset_ascii_hex":
            self.txt_chars.setText("0123456789ABCDEF")
        elif key == "preset_custom":
            self.txt_chars.clear()
            self.txt_chars.setFocus()

    def _on_lang_changed(self, idx):
        lang = self.cmb_lang.currentData()
        i18n.set_lang(lang)
        self._retranslate()

    def _open_preview(self):
        if not self.generated_data:
            QMessageBox.information(self, "", i18n.t("no_data"))
            return
        dlg = PreviewDialog(
            self.generated_data,
            self.spn_w.value(),
            self.spn_h.value(),
            self,
        )
        dlg.exec_()

    def _open_pixel_editor(self):
        w = self.spn_w.value()
        h = self.spn_h.value()
        col = self.cmb_layout.currentIndex() == 0
        lsb = self.cmb_bit.currentIndex() == 0
        dlg = PixelEditorDialog(w, h, col, lsb, self)
        dlg.exec_()
        if dlg.added_data:
            for ch, byte_data in dlg.added_data:
                exists = False
                for i, (c, _) in enumerate(self.generated_data):
                    if c == ch:
                        self.generated_data[i] = (ch, byte_data)
                        exists = True
                        break
                if not exists:
                    self.generated_data.append((ch, byte_data))
            self._regenerate_code()
            self.statusBar().showMessage(
                i18n.t("status_generated", n=len(self.generated_data), b=0)
            )

    def _regenerate_code(self):
        if not self.generated_data:
            self.txt_code.clear()
            self.rust_code = ""
            return
        w = self.spn_w.value()
        h = self.spn_h.value()
        lsb = self.cmb_bit.currentIndex() == 0
        col = self.cmb_layout.currentIndex() == 0
        var = self.txt_var.text().strip() or "FONT"
        self.rust_code = format_rust_code(self.generated_data, w, h, var, lsb, col)
        self.txt_code.setPlainText(self.rust_code)

    def _generate(self):
        font_path = self.txt_font.text().strip()
        if not font_path or not os.path.exists(font_path):
            QMessageBox.warning(self, "", i18n.t("err_no_font"))
            return

        chars = self.txt_chars.text()
        if not chars:
            QMessageBox.warning(self, "", i18n.t("err_no_chars"))
            return

        w = self.spn_w.value()
        h = self.spn_h.value()
        pt = self.spn_pt.value()
        lsb = self.cmb_bit.currentIndex() == 0
        col = self.cmb_layout.currentIndex() == 0
        var = self.txt_var.text().strip() or "FONT"

        self.generated_data = generate_bitmaps(
            chars, font_path, w, h, pt, lsb, col
        )
        self.rust_code = format_rust_code(
            self.generated_data, w, h, var, lsb, col
        )

        scale = max(1, min(4, 200 // max(w, h)))
        self.txt_code.setPlainText(self.rust_code)

        total_bytes = len(self.generated_data) * len(self.generated_data[0][1])
        self.statusBar().showMessage(
            i18n.t(
                "status_generated",
                n=len(self.generated_data),
                b=total_bytes,
            )
        )

    def _copy_code(self):
        if self.rust_code:
            QApplication.clipboard().setText(self.rust_code)
            self.statusBar().showMessage(i18n.t("status_copied"))

    def _save_code(self):
        if not self.rust_code:
            return
        path, _ = QFileDialog.getSaveFileName(
            self, i18n.t("save"), "font_data.rs", "Rust (*.rs);;All (*)"
        )
        if path:
            with open(path, "w", encoding="utf-8") as f:
                f.write(self.rust_code)
            self.statusBar().showMessage(i18n.t("status_saved", path=path))

    def _clear_all(self):
        self.generated_data = []
        self.rust_code = ""
        self.txt_code.clear()
        self.statusBar().showMessage(i18n.t("status_ready"))

    def _retranslate(self):
        self.setWindowTitle(i18n.t("app_title"))
        self.lbl_lang.setText(i18n.t("lang_label"))
        self.grp_font.setTitle(i18n.t("group_font"))
        self.lbl_font_file.setText(i18n.t("font_file"))
        self.btn_browse.setText(i18n.t("browse"))
        self.lbl_size.setText(i18n.t("bitmap_size"))
        self.lbl_pt.setText(i18n.t("font_pt"))
        self.grp_chars.setTitle(i18n.t("group_chars"))
        self.lbl_preset.setText(i18n.t("preset"))
        self.lbl_custom.setText(i18n.t("custom_chars"))
        self.txt_chars.setPlaceholderText(i18n.t("custom_hint"))
        self.grp_opts.setTitle(i18n.t("group_options"))
        self.lbl_bit.setText(i18n.t("bit_order"))
        self.lbl_layout.setText(i18n.t("layout"))
        self.lbl_var.setText(i18n.t("var_name"))
        self.btn_gen.setText(i18n.t("generate"))
        self.btn_draw.setText(i18n.t("draw_btn"))
        self.btn_preview.setText(i18n.t("preview_btn"))
        self.grp_code.setTitle(i18n.t("group_code"))
        self.btn_copy.setText(i18n.t("copy"))
        self.btn_save.setText(i18n.t("save"))
        self.btn_clear.setText(i18n.t("clear"))

        self.cmb_bit.clear()
        self.cmb_bit.addItem(i18n.t("lsb_first"))
        self.cmb_bit.addItem(i18n.t("msb_first"))

        self.cmb_layout.clear()
        self.cmb_layout.addItem(i18n.t("col_major"))
        self.cmb_layout.addItem(i18n.t("row_major"))

        cur_preset = self.cmb_preset.currentIndex()
        self.cmb_preset.blockSignals(True)
        self.cmb_preset.clear()
        for key in [
            "preset_ascii_print",
            "preset_ascii_digits",
            "preset_ascii_upper",
            "preset_ascii_lower",
            "preset_ascii_hex",
            "preset_custom",
        ]:
            self.cmb_preset.addItem(i18n.t(key), key)
        self.cmb_preset.setCurrentIndex(
            min(cur_preset, self.cmb_preset.count() - 1)
        )
        self.cmb_preset.blockSignals(False)


# ============================================================
#  Entry
# ============================================================


def main():
    app = QApplication(sys.argv)
    app.setStyle("Fusion")

    dark_palette = QPalette()
    dark_palette.setColor(QPalette.Window, QColor(45, 45, 48))
    dark_palette.setColor(QPalette.WindowText, QColor(220, 220, 220))
    dark_palette.setColor(QPalette.Base, QColor(30, 30, 30))
    dark_palette.setColor(QPalette.AlternateBase, QColor(45, 45, 48))
    dark_palette.setColor(QPalette.Text, QColor(220, 220, 220))
    dark_palette.setColor(QPalette.Button, QColor(55, 55, 58))
    dark_palette.setColor(QPalette.ButtonText, QColor(220, 220, 220))
    dark_palette.setColor(QPalette.Highlight, QColor(0, 120, 215))
    dark_palette.setColor(QPalette.HighlightedText, QColor(255, 255, 255))
    dark_palette.setColor(QPalette.Disabled, QPalette.Text, QColor(128, 128, 128))
    dark_palette.setColor(QPalette.Disabled, QPalette.ButtonText, QColor(128, 128, 128))
    app.setPalette(dark_palette)

    app.setStyleSheet("""
        QGroupBox {
            font-weight: bold;
            border: 1px solid #555;
            border-radius: 4px;
            margin-top: 8px;
            padding-top: 14px;
        }
        QGroupBox::title {
            subcontrol-origin: margin;
            left: 10px;
            padding: 0 4px;
        }
        QPushButton {
            padding: 6px 16px;
            border-radius: 3px;
            border: 1px solid #666;
            background: #3a3a3d;
        }
        QPushButton:hover {
            background: #4a4a4d;
        }
        QPushButton:pressed {
            background: #2a2a2d;
        }
        QLineEdit, QSpinBox, QComboBox {
            padding: 4px;
            border: 1px solid #555;
            border-radius: 3px;
            background: #1e1e1e;
        }
        QTextEdit {
            border: 1px solid #555;
            border-radius: 3px;
            background: #1a1a1a;
        }
        QScrollArea {
            border: 1px solid #444;
            border-radius: 3px;
        }
    """)

    win = MainWindow()
    win.show()
    sys.exit(app.exec_())


if __name__ == "__main__":
    main()
```

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


src/dht20.rs

```rust
#![allow(dead_code)]

use embedded_hal::i2c::I2c;
use stm32f1xx_hal::rcc::Clocks;

const ADDR: u8 = 0x38;

#[derive(Debug)]
pub enum Error {
    I2c,
    Crc,
    NotReady,
}

pub struct Dht20<I2C> {
    i2c: I2C,
}

impl<I2C: I2c> Dht20<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn init(&mut self, clocks: &Clocks) -> Result<(), Error> {
        Self::delay_ms(100, clocks);

        let mut status = [0u8; 1];
        self.i2c
            .write_read(ADDR, &[0x71], &mut status)
            .map_err(|_| Error::I2c)?;

        if status[0] & 0x80 != 0 {
            self.i2c
                .write(ADDR, &[0x1B, 0x1C, 0x0E, 0x09])
                .map_err(|_| Error::I2c)?;
            Self::delay_ms(10, clocks);
            self.i2c
                .write(ADDR, &[0x1B, 0x1C, 0x0E, 0x09])
                .map_err(|_| Error::I2c)?;
            Self::delay_ms(10, clocks);
            self.i2c
                .write(ADDR, &[0x1B, 0x1C, 0x0E, 0x09])
                .map_err(|_| Error::I2c)?;
            Self::delay_ms(10, clocks);
        }

        Ok(())
    }

    pub fn read(&mut self, clocks: &Clocks) -> Result<(u16, u16), Error> {
        self.i2c
            .write(ADDR, &[0xAC, 0x33, 0x00])
            .map_err(|_| Error::I2c)?;

        Self::delay_ms(80, clocks);

        let mut buf = [0u8; 7];
        self.i2c.read(ADDR, &mut buf).map_err(|_| Error::I2c)?;

        if buf[0] & 0x80 != 0 {
            return Err(Error::NotReady);
        }

        if Self::crc8(&buf[..6]) != buf[6] {
            return Err(Error::Crc);
        }

        let raw_h = ((buf[1] as u32) << 12)
            | ((buf[2] as u32) << 4)
            | ((buf[3] as u32) >> 4);
        let humidity = (raw_h * 1000 / 1048576) as u16;

        let raw_t = (((buf[3] & 0x0F) as u32) << 16)
            | ((buf[4] as u32) << 8)
            | (buf[5] as u32);
        let temp_i = (raw_t * 2000 / 1048576) as i32 - 500;
        let temperature = if temp_i < 0 { 0u16 } else { temp_i as u16 };

        Ok((humidity, temperature))
    }

    fn delay_ms(ms: u32, clocks: &Clocks) {
        let cycles = clocks.sysclk().raw() / 1000 * ms;
        cortex_m::asm::delay(cycles);
    }

    fn crc8(data: &[u8]) -> u8 {
        let mut crc: u8 = 0xFF;
        for &byte in data {
            crc ^= byte;
            for _ in 0..8 {
                if crc & 0x80 != 0 {
                    crc = (crc << 1) ^ 0x31;
                } else {
                    crc <<= 1;
                }
            }
        }
        crc
    }
}
```



src/main.rs

```rust
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod dht20;
mod font_ascii;
mod font_cn;
mod st7789;

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    gpio,
    i2c,
    pac,
    prelude::*,
    rcc,
    spi::{self, Mode, Phase, Polarity},
};

const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

type Display = st7789::ST7789<
    gpio::Pin<'A', 0, gpio::Output<gpio::PushPull>>,
    gpio::Pin<'A', 1, gpio::Output<gpio::PushPull>>,
>;
type Spi1 = spi::Spi<pac::SPI1, u8, gpio::Floating>;

const BG: u16 = 0x0000;
const FG: u16 = 0xFFFF;
const C_TEMP: u16 = 0xFFE0;
const C_HUMI: u16 = 0x07FF;
const C_TITLE: u16 = 0x07FF;
const C_ERR: u16 = 0xF800;

const LABEL_X: u16 = 24;
const CN_W: u16 = 32;
const ASC_W: u16 = 16;
const H: u16 = 32;
const TEMP_Y: u16 = 40;
const HUMI_Y: u16 = 90;
const VAL_X: u16 = 120;
const VAL_W: u16 = 64;

fn draw_cn32(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, c: char, color: u16,
) -> Result<(), spi::Error> {
    if let Some(g) = font_cn::font_32x32(c) {
        d.draw_bitmap(s, x, y, g, 32, 32, color, BG)?;
    }
    Ok(())
}

fn draw_asc(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, c: char, color: u16,
) -> Result<(), spi::Error> {
    let code = c as usize;
    if code < 32 || code > 126 { return Ok(()); }
    let glyph = &font_ascii::FONT_16X32[code - 32];
    d.draw_bitmap(s, x, y, glyph, 16, 32, color, BG)
}

fn draw_str8(
    d: &mut Display, s: &mut Spi1,
    mut x: u16, y: u16, text: &str, color: u16,
) -> Result<(), spi::Error> {
    for ch in text.chars() {
        if (ch as u32) >= 32 && (ch as u32) <= 126 {
            let glyph = &font_ascii::FONT_16X32[ch as usize - 32];
            d.draw_bitmap(s, x, y, glyph, 16, 32, color, BG)?;
            x += 16;
        }
    }
    Ok(())
}

fn draw_decimal(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, val_x10: u16, color: u16,
) -> Result<(), spi::Error> {
    let v = if val_x10 > 999 { 999 } else { val_x10 };
    let int_part = v / 10;
    let dec_part = v % 10;
    draw_asc(d, s, x, y, (b'0' + (int_part / 10) as u8) as char, color)?;
    draw_asc(d, s, x + ASC_W, y, (b'0' + (int_part % 10) as u8) as char, color)?;
    draw_asc(d, s, x + ASC_W * 2, y, '.', color)?;
    draw_asc(d, s, x + ASC_W * 3, y, (b'0' + dec_part as u8) as char, color)
}

fn clear_val(d: &mut Display, s: &mut Spi1, y: u16) -> Result<(), spi::Error> {
    d.fill_rect(s, VAL_X, y, VAL_W, H, BG)
}

fn draw_layout(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_screen(s, BG)?;
    draw_str8(d, s, 40, 4, "== DHT20 ==", C_TITLE)?;
    draw_cn32(d, s, LABEL_X, TEMP_Y, '温', FG)?;
    draw_cn32(d, s, LABEL_X + CN_W, TEMP_Y, '\u{5EA6}', FG)?;
    draw_asc(d, s, LABEL_X + CN_W * 2, TEMP_Y, ':', FG)?;
    draw_cn32(d, s, VAL_X + VAL_W, TEMP_Y, '\u{2103}', C_TEMP)?;
    draw_cn32(d, s, LABEL_X, HUMI_Y, '湿', FG)?;
    draw_cn32(d, s, LABEL_X + CN_W, HUMI_Y, '\u{5EA6}', FG)?;
    draw_asc(d, s, LABEL_X + CN_W * 2, HUMI_Y, ':', FG)?;
    draw_asc(d, s, VAL_X + VAL_W, HUMI_Y, '%', C_HUMI)
}

fn draw_err_msg(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_rect(s, 24, 22, 192, 16, BG)?;
    draw_str8(d, s, 24, 22, "Sensor Error!", C_ERR)
}

fn clear_err_msg(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_rect(s, 24, 22, 192, 16, BG)
}

#[allow(dead_code)]
enum Ui {
    Fresh,
    Data { t: u16, h: u16 },
    Err { t: u16, h: u16 },
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("DHT20 + ST7789 32x32");

    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc_cfg = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    let clocks = rcc_cfg.clocks.clone();
    let mut gpioa = dp.GPIOA.split(&mut rcc_cfg);

    let dc = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let res = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut spi = dp.SPI1.spi(
        (Some(gpioa.pa5), pac::SPI1::NoMiso, Some(gpioa.pa7)),
        SPI_MODE, 16.MHz(), &mut rcc_cfg,
    );
    let mut display = st7789::ST7789::new(dc, res);
    display.init(&mut spi, &clocks).unwrap();

    let mut gpiob = dp.GPIOB.split(&mut rcc_cfg);
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c_bus = dp.I2C1.i2c(
        (scl, sda),
        i2c::Mode::standard(100_000.Hz()),
        &mut rcc_cfg,
    ).blocking(1000, 10, 1000, 1000, &clocks);

    let mut dht20_sensor = dht20::Dht20::new(i2c_bus);

    display.fill_screen(&mut spi, BG).unwrap();
    let _ = draw_str8(&mut display, &mut spi, 24, 100, "DHT20 32x32 Demo", C_TITLE);
    let _ = draw_str8(&mut display, &mut spi, 56, 140, "Wait...", 0x8410);
    if let Err(e) = dht20_sensor.init(&clocks) {
        rprintln!("DHT20 init: {:?}", e);
    }

    let mut ui = Ui::Fresh;

    loop {
        let mut res = Err(dht20::Error::I2c);
        for attempt in 0u8..3 {
            match dht20_sensor.read(&clocks) {
                Ok((h, t)) => { res = Ok((h, t)); break; }
                Err(e) => {
                    rprintln!("try{}: {:?}", attempt + 1, e);
                    cortex_m::asm::delay(clocks.sysclk().raw() / 1000 * 1500);
                }
            }
        }

        match (&ui, &res) {
            (Ui::Fresh, Ok((h, t))) => {
                let _ = draw_layout(&mut display, &mut spi);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Data { t: *t, h: *h };
                rprintln!("T:{}.{}C H:{}.{}%", t / 10, t % 10, h / 10, h % 10);
            }
            (Ui::Fresh, Err(_)) => {
                let _ = draw_layout(&mut display, &mut spi);
                let _ = draw_err_msg(&mut display, &mut spi);
                ui = Ui::Err { t: 0, h: 0 };
            }
            (Ui::Data { t, h }, Ok((nh, nt))) => {
                if *nt != *t {
                    let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                    let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *nt, C_TEMP);
                }
                if *nh != *h {
                    let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                    let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *nh, C_HUMI);
                }
                ui = Ui::Data { t: *nt, h: *nh };
                rprintln!("T:{}.{}C H:{}.{}%", nt / 10, nt % 10, nh / 10, nh % 10);
            }
            (Ui::Data { t, h }, Err(_)) => {
                let _ = draw_err_msg(&mut display, &mut spi);
                let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Err { t: *t, h: *h };
            }
            (Ui::Err { .. }, Ok((h, t))) => {
                let _ = clear_err_msg(&mut display, &mut spi);
                let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Data { t: *t, h: *h };
                rprintln!("T:{}.{}C H:{}.{}%", t / 10, t % 10, h / 10, h % 10);
            }
            (Ui::Err { .. }, Err(_)) => {}
        }

        cortex_m::asm::delay(clocks.sysclk().raw() / 1000 * 2000);
    }
}
```



# 结语
作者邮箱：pycx0@qq.com  
其实作者是在大四即将毕业的时候来完成这个项目的，原因是China就业压力太大了，目前找工作中ing，不行了我可能会去进厂2班倒了！  
后续更新会在原文链接中！期待找个好工作！

期待Rust生态越来越好！加油地球村的“村民”们！
