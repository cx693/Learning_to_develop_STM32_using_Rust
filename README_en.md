<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780203075814-b3bd4b33-1fec-4f6a-ba6d-342f60f58486.png" width="1448" title="" crop="0,0,1,1" id="ue9743a87" class="ne-image">



[简体中文](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_cn.md) / [English](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_en.md) / [Русский](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_ru.md)

# Declaration
This project uses the CC BY-NC 4.0 license. Commercial use requires contacting the copyright holder pycx0@qq.com for authorization. Commercial products developed based on this project must obtain authorization; non-commercial use is free!

# Basic Environment Setup
## Install probe-rs
```bash
cargo install probe-rs-tools --locked
```



## Install Compiler
```bash
rustup target install thumbv7m-none-eabi
```

## Detect with probe-rs
DAP mode!

```bash
probe-rs info --protocol swd
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779699156280-0601cf1f-d586-4e82-8763-adf851dc2ccc.png" width="700" title="" crop="0,0,1,1" id="u0cb46cfc" class="ne-image">



## Install Package
The main purpose is to fix the infinite loop bug

```bash
cargo add panic-halt
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779700077727-2d4a69c3-fbfa-4838-9bcf-5be9c764a7a0.png" width="369" title="" crop="0,0,1,1" id="u09183fea" class="ne-image">

View FLASH usage

```json
cargo install st-mem
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779726981642-0c7dc5e5-9c60-4e5f-a84e-f07c4be2bdd0.png" width="764" title="" crop="0,0,1,1" id="uaed0ebe6" class="ne-image">



# Compile and Test
Project structure

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

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8B%B7%E8%B4%9D%E6%A8%A1%E7%89%88/Cargo.toml)



.cargo/cargo.toml

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8B%B7%E8%B4%9D%E6%A8%A1%E7%89%88/.cargo/config.toml)



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779701739431-cfc7d6f5-96bb-4911-8ad2-ae2660fe538b.png" width="336" title="" crop="0,0,1,1" id="u909b0bf4" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779701756541-3d0fa057-0cea-450e-9797-80f999b79c5b.png" width="674" title="" crop="0,0,1,1" id="u8ea4bf36" class="ne-image">

<font style="color:#DF2A3F;">main.rs see "Light Up the First LED"</font>

## Function Cannot Jump to Definition
Solution!

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

# Debugging
Create the configuration task file! Use the probe-rs official website! Search

[https://probe.rs/docs/tools/debugger/](https://probe.rs/docs/tools/debugger/)



Supported chip types: [https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)



.vscode/launch.json

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8B%B7%E8%B4%9D%E6%A8%A1%E7%89%88/.vscode/launch.json)



~~Currently RTT debugging on macOS has issues — no terminal output!~~ Resolved!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976699740-3c2d6980-e2fa-46f9-8404-b6d5f4366b2d.png" width="1440" title="" crop="0,0,1,1" id="u90e0f0b7" class="ne-image">

The current solution is: set a breakpoint at the main() entry point, and add this line at the very beginning:  rtt_init_print!();

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976859955-199c20a0-8073-4eca-b109-c0a097238644.png" width="486" title="" crop="0,0,1,1" id="ue86693d6" class="ne-image">






# Project Download (Flash)
Basic Rust compilation environment

```plain
cargo install cargo-binutils
rustup component add llvm-tools
```

## HEX File
Compile to ELF file

```rust
cargo build --release
```

Compile to HEX file

```rust
cargo objcopy --release -- -O ihex ccc.hex
cargo objcopy --release -- -O ihex <firmware_name>.hex
```

Direct download

```rust
probe-rs download --binary-format hex --chip STM32F103C8 ccc.hex
probe-rs download --binary-format hex --chip <chip_name>	<firmware_name>.hex
```

Chip name lookup: [https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779867658066-603db27f-8d6c-4545-b93a-3fc240c50078.png" width="1540" title="" crop="0,0,1,1" id="ua5bbe57c" class="ne-image">

## BIN File
Compile to ELF file

```rust
cargo build --release
```

Compile to HEX file

```rust
cargo objcopy --release -- -O binary ccc.bin
cargo objcopy --release -- -O binary <firmware_name>.bin
```

Direct download

```rust
probe-rs download --chip STM32F103C8 --base-address 0x08000000 --binary-format bin ccc.bin
probe-rs download --chip <chip_name> --base-address <offset_address> --binary-format bin <firmware_name>.bin
```



# Unlock JTAG Port — Unable to Download
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897632872-17f9317c-b978-4d97-845e-7012f1e8897b.png" width="604" title="" crop="0,0,1,1" id="u888d55bd" class="ne-image">

Or if execution fails — full chip erase command

```rust
probe-rs erase --chip STM32F103C8 --speed 100 --protocol swd
probe-rs erase --chip <chip_name> --speed 100 --protocol <interface_type-can_be_omitted>
probe-rs erase --chip STM32F103C8 --speed 100
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897847743-83db4c01-57bd-4529-8fef-cb60a3c3b09f.png" width="496" title="" crop="0,0,1,1" id="u2c272bf8" class="ne-image">

That is, boot0-boot1 --> both set to 0

After executing the command, press the reset button and release quickly! It will erase automatically. If it doesn't work, hold down the reset button, execute the command, then release immediately!



# Learning Resources
Textbook: [https://xxchang.github.io/book/](https://xxchang.github.io/book/)

Project source: [https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples](https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples)



# Basic Learning
---

## Clock System Detailed Explanation
> **Why learn the clock first?** Because almost all peripherals depend on the clock to operate. Clock configuration is the most fundamental and important step in embedded development. Incorrect configuration can cause peripherals to malfunction, inaccurate UART baud rates, USB enumeration failures, and other issues.
>

### STM32F1 Clock Tree Overview
The clock system of STM32F103 is very flexible, with multiple clock sources and dividers. Here is a simplified clock tree:

```plain
                          ┌─────────────┐
                          │   HSE       │  External high-speed crystal (DKX board: 8MHz)
                          │  8 MHz      │
                          └──────┬──────┘
                                 │
                          ┌──────▼──────┐
                          │   HSI       │  Internal RC oscillator
                          │  8 MHz      │  (low accuracy, ±1%, fast startup)
                          └──────┬──────┘
                                 │
                 ┌───────────────┼───────────────┐
                 │               │               │
                 │         ┌─────▼─────┐         │
                 │         │   PLL     │  Phase Locked Loop
                 │         │  Multiplier│  ×2~×16
                 │         └─────┬─────┘         │
                 │               │               │
          ┌──────▼──────┐ ┌──────▼──────┐        │
          │ SYSCLK      │ │  USBCLK     │        │
          │ System Clock │ │  USB Clock  │        │
          │ Max 72MHz    │ │  Must 48MHz │        │
          └──────┬──────┘ └─────────────┘        │
                 │                                │
        ┌────────┼────────┐                      │
        │        │        │                      │
   ┌────▼───┐ ┌──▼───┐ ┌──▼────┐          ┌─────▼─────┐
   │ AHB    │ │ APB1 │ │ APB2  │          │  SYSCLK   │
   │ Bus    │ │ Bus  │ │ Bus   │          │  Source   │
   │≤72MHz  │ │≤36MHz│ │≤72MHz │          │  Selection│
   └───┬────┘ └──┬───┘ └──┬────┘          │ HSI/HSE/PLL
       │         │        │               └───────────┘
  ┌────▼───┐ ┌───▼────┐ ┌─▼──────┐
  │ Cortex │ │USART2/3│ │USART1  │
  │ SysTick│ │TIM2-4  │ │SPI1    │
  │ DMA    │ │I2C1/2  │ │ADC1/2  │
  │ Flash  │ │SPI2    │ │TIM1    │
  └────────┘ │USB     │ │GPIO    │
             └────────┘ └────────┘
```

**Core Concept: PLL (Phase Locked Loop)**

```plain
PLL Clock = PLL Input Clock × PLL Multiplier

If HSE is selected as PLL input:
  PLLCLK = HSE × Multiplier (×2 ~ ×16)

Example (DKX board 8MHz crystal):
  HSE × 9  = 8 × 9  = 72 MHz ← Maximum system clock
  HSE × 6  = 8 × 6  = 48 MHz ← Required for USB
  HSE × 4  = 8 × 4  = 32 MHz

If HSI is selected as PLL input:
  PLLCLK = HSI × 2 × Multiplier / 2
  PLLCLK = HSI × Multiplier (×2 ~ ×16)
  But HSI must be divided by 2 before entering PLL
```

---

### Clock Source Detailed Explanation
#### HSI (High Speed Internal) — Internal High-Speed Clock
```plain
Features:
├── Frequency: 8 MHz (RC oscillator, temperature drift)
├── Accuracy: ±1% (factory calibrated), drifts with temperature
├── Advantage: No external components needed, available immediately at power-up
├── Disadvantage: Low accuracy, not suitable for USB, CAN, precise baud rates
└── Default: Automatically used as system clock source at power-up
```

**When to use HSI?**

+ Simple LED blinking, button detection, and other scenarios that don't require precise clocks
+ Backup option when external crystal is damaged
+ Fast startup scenarios (HSI starts faster than HSE)

#### HSE (High Speed External) — External High-Speed Clock
```plain
Features:
├── Frequency: 4-16 MHz (DKX board uses 8MHz crystal)
├── Accuracy: ±0.005% (depends on crystal quality)
├── Advantage: High accuracy, suitable for USB, CAN, precise UART baud rates
├── Disadvantage: Requires external crystal, startup takes time (hundreds of microseconds ~ a few milliseconds)
└── DKX board: 8MHz passive crystal + 2 x 20pF load capacitors
```

**When to use HSE?**

+ When USB functionality is needed (**must** use HSE or HSE through PLL)
+ When CAN bus is needed (requires precise clock)
+ When precise UART baud rate is needed
+ When running at full speed 72MHz

#### LSE (Low Speed External) — External Low-Speed Clock
```plain
Features:
├── Frequency: 32.768 kHz (used for RTC)
├── Accuracy: Very high (small crystal temperature drift)
├── Usage: Real-Time Clock (RTC), watchdog
└── DKX board: May not have LSE crystal soldered (check schematic)
```

#### LSI (Low Speed Internal) — Internal Low-Speed Clock
```plain
Features:
├── Frequency: ~40 kHz (inaccurate)
├── Usage: Independent Watchdog (IWDG), RTC backup clock
└── Accuracy: Poor (±30%)
```

#### PLL (Phase Locked Loop) — Phase Locked Loop
PLL is the core of the clock system, used to multiply low-frequency clocks to high frequencies.

```plain
┌─────────────────────────────────────────────────┐
│                    PLL Details                   │
├─────────────────────────────────────────────────┤
│                                                  │
│  Input Source Selection:                         │
│  ┌──────┐    ┌─────────┐                         │
│  │ HSI/2│───►│         │    ┌───────────┐        │
│  └──────┘    │ PLL MUX │───►│  ÷ PLLMUL │──► PLLCLK
│  ┌──────┐───►│         │    │  (×2~×16) │        │
│  │ HSE  │    └─────────┘    └───────────┘        │
│  └──────┘                                        │
│                                                  │
│  Common Configurations:                          │
│  ┌──────────┬──────────┬──────────────┐          │
│  │ Input Clk│ Multiplier│ Output Freq  │          │
│  ├──────────┼──────────┼──────────────┤          │
│  │ HSI 8MHz │ ×9       │ 36 MHz*     │          │
│  │ HSE 8MHz │ ×9       │ 72 MHz ✓    │          │
│  │ HSE 8MHz │ ×6       │ 48 MHz ✓    │          │
│  │ HSE 8MHz │ ×4       │ 32 MHz ✓    │          │
│  └──────────┴──────────┴──────────────┘          │
│                                                  │
│  * HSI is first divided by 2 (=4MHz), then ×9 = 36MHz │
│                                                  │
└─────────────────────────────────────────────────┘
```

---

### Bus Clock Detailed Explanation
```plain
                         SYSCLK (System Clock)
                              │
                ┌─────────────┼─────────────┐
                │             │             │
           ┌────▼────┐  ┌────▼────┐  ┌─────▼─────┐
           │  AHB    │  │  APB1   │  │   APB2    │
           │  Bus    │  │  Bus    │  │   Bus     │
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

#### AHB Bus — High-Speed Bus
| Parameter | Description |
| --- | --- |
| Maximum Frequency | 72 MHz |
| Prescaler | SYSCLK ÷ 1/2/4/8/16/64/128/256/512 |
| Connected Devices | Cortex-M3 core, DMA, Flash, GPIO, SysTick |
| Configuration | `rcc::Config::hsi().hclk(72.MHz())` |


**Note:** SysTick clock comes from AHB (if SysTick is configured to use processor clock), or AHB/8.

#### APB1 Bus — Low-Speed Peripheral Bus
| Parameter | Description |
| --- | --- |
| Maximum Frequency | **36 MHz** (hard limit, exceeding may damage the chip) |
| Prescaler | AHB ÷ 1/2/4/8/16 |
| Connected Devices | USART2, USART3, I2C1/2, SPI2, TIM2-4, USB, CAN |
| Configuration | `rcc::Config::hsi().pclk1(36.MHz())` |


**Important:** If APB1 prescaler > 1, then timer clock = APB1 × 2.

#### APB2 Bus — High-Speed Peripheral Bus
| Parameter | Description |
| --- | --- |
| Maximum Frequency | **72 MHz** |
| Prescaler | AHB ÷ 1/2/4/8/16 |
| Connected Devices | USART1, SPI1, ADC1/2, TIM1, GPIOA~D, EXTI, AFIO |
| Configuration | `rcc::Config::hsi().sysclk(72.MHz()).pclk2(72.MHz())` |


#### ADC Clock
| Parameter | Description |
| --- | --- |
| Maximum Frequency | **14 MHz** |
| Clock Source | APB2 ÷ 2/4/6/8 |
| Configuration | `rcc::Config::hsi().adcclk(14.MHz())` |


#### USB Clock
| Parameter | Description |
| --- | --- |
| Required Frequency | **48 MHz** (must be precise) |
| Clock Source | PLL output (PLLCLK ÷ 1 or 1.5) |
| Configuration Requirement | SYSCLK must be 48MHz or 72MHz |


---

### Clock Configuration in stm32f1xx-hal
The HAL library uses the **Builder pattern** for clock configuration, which is very intuitive:

#### Basic Usage
```rust
use stm32f1xx_hal::{pac, prelude::*, rcc};

fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();  // Flash wait cycle configuration

    // Method 1: Concise configuration method
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hsi()           // Use internal 8MHz RC
            .sysclk(64.MHz())        // System clock 64MHz
            .pclk1(32.MHz())         // APB1 clock 32MHz
            .pclk2(64.MHz())         // APB2 clock 64MHz
            .adcclk(8.MHz()),        // ADC clock 8MHz
        &mut flash.acr,
    );

    // Method 2: Use external crystal + PLL
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())    // Use 8MHz external crystal
            .sysclk(72.MHz())        // PLL multiply to 72MHz
            .pclk1(36.MHz())         // APB1 divide to 36MHz
            .pclk2(72.MHz())         // APB2 no division
            .adcclk(14.MHz()),       // ADC 14MHz
        &mut flash.acr,
    );
}
```

#### `rcc::Config` Builder Methods
```rust
// All available configuration methods (* indicates commonly used)
rcc::Config::hsi()                    // Select HSI as clock source
rcc::Config::hse(8.MHz())            // Select HSE as clock source, specify frequency

.sysclk(72.MHz())    *               // Set target system clock frequency
.pclk1(36.MHz())    *                // Set APB1 target frequency
.pclk2(72.MHz())    *                // Set APB2 target frequency
.adcclk(14.MHz())   *                // Set ADC clock frequency
.hclk(72.MHz())                       // Set AHB clock (usually = SYSCLK)

// PLL automatically calculates the multiplier based on target frequency
// No manual setting needed!
```

#### Getting Clock Info After Freeze
```rust
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,
);

// Get actual clock frequencies
rprintln!("SYSCLK: {}", rcc.clocks.sysclk());   // System clock
rprintln!("HCLK:   {}", rcc.clocks.hclk());     // AHB clock
rprintln!("PCLK1:  {}", rcc.clocks.pclk1());    // APB1 clock
rprintln!("PCLK2:  {}", rcc.clocks.pclk2());    // APB2 clock
rprintln!("ADCCLK: {}", rcc.clocks.adcclk());   // ADC clock
rprintln!("USBCLK valid: {}", rcc.clocks.usbclk_valid()); // USB clock valid?
```

#### Why is `flash.acr` Needed?
Flash read speed is limited. When the system clock exceeds 24MHz, wait states need to be inserted:

| System Clock | Flash Wait States |
| --- | --- |
| 0-24 MHz | 0 wait states |
| 24-48 MHz | 1 wait state |
| 48-72 MHz | 2 wait states |


`freeze()` automatically sets the correct wait states based on the system clock frequency.

#### Difference Between `constrain()` and `freeze()`
```rust
// constrain() — Constrain RCC, returns a configurable object
// Used for manually configuring each peripheral clock
let mut rcc = dp.RCC.constrain();

// freeze() — Complete clock configuration and freeze in one step
// Automatically calculates all divider/multiplier parameters
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,
);
```

Generally `freeze()` is recommended — simpler and less error-prone.

#### Advanced Usage: Directly Specify Divider/Multiplier
```rust
// If you need full control over clock configuration, use RawConfig
let rcc = dp.RCC.freeze(
    rcc::RawConfig {
        hse: Some(8_000_000),       // HSE frequency
        pllmul: Some(7),            // PLL multiplier (×9, index starts from 0)
        hpre: rcc::HPre::Div1,     // AHB prescaler = no division
        ppre1: rcc::PPre::Div2,    // APB1 prescaler = AHB ÷ 2
        ppre2: rcc::PPre::Div1,    // APB2 prescaler = no division
        usbpre: rcc::UsbPre::Div1_5, // USB prescaler
        adcpre: rcc::AdcPre::Div2,  // ADC prescaler = APB2 ÷ 2
        ..Default::default()
    },
    &mut flash.acr,
);
```

---

### Common Clock Configuration Schemes
#### Scheme 1: Simplest Configuration (HSI Default)
```rust
// Power-on default: HSI 8MHz, no PLL
// SYSCLK = 8MHz, APB1 = 8MHz, APB2 = 8MHz
let mut rcc = dp.RCC.constrain();

// Or use freeze
let rcc = dp.RCC.freeze(rcc::Config::hsi(), &mut flash.acr);
```

**Suitable for:** LED blinking, button detection, GPIO testing, and other simple scenarios  
**Not suitable for:** USB, CAN, high baud rate UART

---

#### Scheme 2: HSI Multiply to 64MHz
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hsi()
        .sysclk(64.MHz())         // HSI × 8 = 64MHz
        .pclk1(32.MHz())          // APB1 = AHB ÷ 2
        .pclk2(64.MHz())          // APB2 = AHB (no division)
        .adcclk(8.MHz()),         // ADC = APB2 ÷ 8
    &mut flash.acr,
);
// Note: HSI can only multiply up to 64MHz, not 72MHz
// Because HSI is divided by 2 before entering PLL, 8/2=4, 4×16=64
```

**Suitable for:** Scenarios without external crystal but needing higher performance  
**Not suitable for:** USB (requires precise 48MHz)

---

#### Scheme 3: HSE Multiply to 72MHz (**Recommended! DKX board first choice**)
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())     // Use DKX board's 8MHz crystal
        .sysclk(72.MHz())         // PLL: 8 × 9 = 72MHz
        .pclk1(36.MHz())          // APB1 = 72 ÷ 2 = 36MHz (maximum)
        .pclk2(72.MHz())          // APB2 = 72MHz (no division)
        .adcclk(14.MHz()),        // ADC = 72 ÷ 6 ≈ 12MHz (actually 6 division)
    &mut flash.acr,
);
```

**Suitable for:** Almost all scenarios, highest performance configuration  
**Clock:**

+ SYSCLK = 72 MHz
+ AHB = 72 MHz
+ APB1 = 36 MHz
+ APB2 = 72 MHz
+ ADC = 12 MHz
+ Flash wait states = 2

---

#### Scheme 4: HSE Multiply to 48MHz (USB Dedicated)
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())     // 8MHz crystal
        .sysclk(48.MHz())         // PLL: 8 × 6 = 48MHz
        .pclk1(24.MHz())          // APB1 = 48 ÷ 2 = 24MHz
        .pclk2(48.MHz()),         // APB2 = 48MHz
    &mut flash.acr,
);

// Verify USB clock
assert!(rcc.clocks.usbclk_valid());  // USB needs precise 48MHz
```

**Suitable for:** USB applications  
**Clock:**

+ SYSCLK = 48 MHz
+ USBCLK = 48 MHz ✓ (precise)
+ APB1 = 24 MHz
+ APB2 = 48 MHz
+ Flash wait states = 1

---

#### Scheme 5: 72MHz + USB (Advanced)
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())         // PLL: 8 × 9 = 72MHz
        .pclk1(36.MHz())          // APB1 = 36MHz
        .pclk2(72.MHz()),         // APB2 = 72MHz
    &mut flash.acr,
);

// USB clock = PLLCLK ÷ 1.5 = 72 ÷ 1.5 = 48MHz ✓
assert!(rcc.clocks.usbclk_valid());
```

**Suitable for:** Scenarios requiring both maximum performance and USB

---

### 2.6 Common Clock Configuration Errors
#### Error 1: APB1 Exceeds 36MHz
```rust
// ❌ Wrong! APB1 max 36MHz
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(72.MHz()),  // Wrong! Exceeds 36MHz
    &mut flash.acr,
);

// ✓ Correct
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz()),  // Correct
    &mut flash.acr,
);
```

#### Error 2: Inaccurate USB Clock
```rust
// ❌ HSI is not suitable for USB
let mut rcc = dp.RCC.freeze(
    rcc::Config::hsi().sysclk(48.MHz()),
    &mut flash.acr,
);
// HSI accuracy ±1%, USB requires ±0.25%, will cause enumeration failure

// ✓ Must use HSE
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(48.MHz()),
    &mut flash.acr,
);
```

#### Error 3: Missing `flash.acr`
```rust
// ❌ Missing flash.acr parameter
let rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(72.MHz()));
// Compile error! freeze requires two parameters

// ✓ Correct
let mut flash = dp.FLASH.constrain();
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,  // Must be passed in!
);
```

#### Error 4: SYSCLK Exceeds 72MHz
```rust
// ❌ STM32F103 max 72MHz
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(80.MHz()),  // Wrong!
    &mut flash.acr,
);
// freeze will automatically select a frequency close to but not exceeding 72MHz
```

#### Error 5: ADC Clock Exceeds 14MHz
```rust
// ❌ ADC clock max 14MHz
// If pclk2 = 72MHz, and no division for ADC, ADC clock = 72/6 = 12MHz ✓
// If pclk2 = 72MHz, and ADC no division, ADC clock = 72MHz ✗

// HAL library handles this automatically, but understanding the principle is important
```

---

### Clock Configuration Quick Reference Table
| Scenario | Configuration | SYSCLK | APB1 | APB2 | USB |
| --- | --- | --- | --- | --- | --- |
| LED/Button | `Config::hsi()` | 8 MHz | 8 MHz | 8 MHz | ✗ |
| General | `hse(8).sysclk(72)` | 72 MHz | 36 MHz | 72 MHz | ✓ |
| USB | `hse(8).sysclk(48)` | 48 MHz | 24 MHz | 48 MHz | ✓ |
| Low Power | `hsi().sysclk(8)` | 8 MHz | 8 MHz | 8 MHz | ✗ |
| No External Crystal | `hsi().sysclk(64)` | 64 MHz | 32 MHz | 64 MHz | ✗ |


---

## Light Up the First LED
```rust
//! Blink LED using PC13 pin of STM32F103C8T6
//!
//! This example assumes the LED is connected to PC13 pin, as on the Blue Pill development board.
//!
//! Note: Without additional hardware, PC13 should not be directly used to drive an LED.
//! See section 5.1.2 of the reference manual for details. However, this is not an issue on the Blue Pill board.

// Disallow unsafe code to ensure code safety
#![deny(unsafe_code)]
// Tell Rust compiler not to use standard library (required for embedded)
#![no_std]
// Tell Rust compiler there is no traditional main function, use entry point from cortex-m-rt
#![no_main]

// Import panic handler: stop CPU when an unrecoverable error occurs
use panic_halt as _;

// Import utility module for non-blocking operations, used for async operations
use nb::block;

// Import entry point macro provided by cortex-m runtime
use cortex_m_rt::entry;
// Import core modules from HAL library
// pac: Peripheral Access Crate, provides register-level access
// prelude: Pre-imports common traits to simplify code
// timer: Timer module
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use rtt_target::{rprintln,rtt_init_print};

// Define program entry point, replacing the standard main function
#[entry]
fn main() -> ! {

    rtt_init_print!();

    // Get Cortex-M core peripherals (such as SysTick timer, NVIC, etc.)
    // take() method ensures these peripherals are only acquired once, preventing reuse
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get device-specific peripherals of STM32F103 (GPIO, timers, UART, etc.)
    let dp = pac::Peripherals::take().unwrap();

    // Get and configure Reset and Clock Controller (RCC)
    // constrain() method converts the raw RCC structure into a HAL-provided high-level abstraction
    let mut rcc = dp.RCC.constrain();

    // Get GPIOC port and split it into individual pins
    // split() method ensures unique pin ownership, preventing multiple functions from controlling the same pin
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure PC13 pin as push-pull output mode
    // crh register is used to configure the upper 8 pins of the port (PC8-PC15)
    // For the lower 8 pins (PC0-PC7), pass the crl register
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    
    // Configure SysTick timer as a counter, triggering at specified frequency
    // counter_hz() configures SysTick as a frequency-based counter mode
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    
    // Start the timer, set trigger frequency to 1Hz (once per second)
    timer.start(4.Hz()).unwrap();

    // Main loop: wait for timer trigger and toggle LED state
    loop {
        // Block and wait for timer first trigger (after 1 second)
        block!(timer.wait()).unwrap();
        // Set PC13 to high level, turn off LED (Blue Pill LED is active low)
        led.set_high();
        rprintln!("OPEN THE LED");
        
        // Block and wait for timer second trigger (another 1 second)
        block!(timer.wait()).unwrap();
        // Set PC13 to low level, turn on LED
        led.set_low();
        rprintln!("LOW THE LED");
    }
}
```

Second version — auto-detect chip

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
    // Get peripheral ownership (can only be called once, subsequent calls return None)
    let p = pac::Peripherals::take().unwrap();

    // Constrain RCC (Reset and Clock Control) registers
    // constrain() returns an object containing all configurable clocks
    let mut rcc = p.RCC.constrain();

    // Split GPIOC port into individual pin objects
    // split() returns independent handles for each pin
    let mut gpioc = p.GPIOC.split(&mut rcc);

    // Select different pins and levels based on chip model
    cfg_select! {
        feature = "stm32f100" => {
            // STM32F100: PC9 high level to turn on
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        feature = "stm32f101" => {
            // STM32F101: PC9 high level to turn on
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        _ => {
            // STM32F103 (including your DKX board): PC13 low level to turn on
            // PC13 on Blue Pill/DKX board is common anode, low level = on
            gpioc.pc13.into_push_pull_output(&mut gpioc.crh).set_high(); // Actually for JLCPCB board, set high to turn on
        }
    }

    loop {} // Keep LED state unchanged
}
```



## Hello World
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/hello/src/main.rs)

**Key Concept Details:**

+ `cortex_m::Peripherals` — Cortex-M core peripherals:
    - `SYST` — SysTick timer
    - `NVIC` — Interrupt controller
    - `DCB` — Debug control block
    - `DWT` — Data watchpoint and trace unit
+ `pac::Peripherals` — Chip-specific peripherals:
    - `RCC` — Clock control
    - `GPIOA/B/C/D` — GPIO ports
    - `USART1/2/3` — UART
    - `TIM1/2/3/4` — Timers
    - `SPI1/2` — SPI interface
    - `I2C1/2` — I2C interface
    - `ADC1/2` — ADC
    - `USB` — USB peripheral
    - `CAN` — CAN controller
+ `Timer::syst()` — Create timer object using SysTick timer
+ `counter_hz()` — Create frequency counter in Hz units
+ `block!()` — Convert non-blocking operation to blocking (poll until complete)
+ `1.Hz()` — Frequency unit using fugit library

---

## LED Blinking
### SYST Mode Delay Blinking
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/LED%E9%97%AA%E7%83%81/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779801980743-ec54e1f1-3737-4cd9-adfa-8e4a6a99ec8e.jpeg" width="281" title="" crop="0,0,1,1" id="u62526734" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779802010301-0b3dbe14-df37-444f-9f75-1f9dcb40b870.jpeg" width="280" title="" crop="0,0,1,1" id="u6a547c40" class="ne-image">

LED blinking as shown above

### LED Blinking — TIM2 Timer Delay
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/LED%E9%97%AA%E7%83%81_TIM2/src/main.rs)

**Key Concepts:**

+ `rcc::Config::hse(8.MHz())` — Use 8MHz external high-speed crystal (crystal on DKX board)
+ `.sysclk(48.MHz())` — Set system clock to 48MHz
+ `rcc.freeze()` — Freeze clock configuration, returns an immutable clock state
+ `dp.TIM2.delay_us()` — Use TIM2 to create microsecond-level delays
+ **Advantages**: More flexible than SysTick, higher precision, does not affect other uses of SysTick



### Delay Explanation
This is a delay function. To put it simply, just use delay.delay(20.millis());

+ nanos() nanoseconds; 
+ micros() microseconds;
+ millis() milliseconds; 
+ secs() seconds; 
+ millis() milliseconds; 
+ minutes() minutes; 
+ hours() hours



## Button Controlled LED
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E6%8C%89%E9%94%AE%E4%B8%8ELED/src/main.rs)

**JTAG Pin Description:**

+ STM32F1 uses JTAG/SWD debug interface by default
+ PA13(SWDIO), PA14(SWCLK), PA15(JTDI), PB3(JTDO), PB4(JNTRST) are occupied by JTAG by default
+ To use these pins as regular GPIO, JTAG must be disabled first
+ Note: PA13/PA14 are SWD interface, generally not recommended to disable (otherwise debugging becomes impossible)



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779809566789-02c7ee36-cc1e-4e52-82e7-860ae68fe4bb.png" width="662" title="" crop="0,0,1,1" id="u84f8089e" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809683134-a5da8944-f17a-4c76-bf5a-e18eb1ffe579.jpeg" width="223" title="" crop="0,0,1,1" id="ufb18b746" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809699056-1341bf76-a62b-49f6-ae96-d537e2c5d09c.jpeg" width="223" title="" crop="0,0,1,1" id="u774adcc9" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809732125-60f82ecc-dd74-49c0-bee5-c640bd90733b.jpeg" width="222" title="" crop="0,0,1,1" id="u1effb711" class="ne-image">



## Dynamic GPIO Switching (Multiplexed Ports)
In actual development, ports may be multiplexed, requiring the following template

**Dynamic GPIO Use Cases:**

+ Some protocols (such as 1-Wire, software I2C emulation) require switching pin direction at runtime
+ Multiplexing of limited pins

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%8A%A8%E6%80%81GPIO%E5%88%87%E6%8D%A2/src/main.rs)



## External Interrupt EXTI
### EXTI Interrupt Line and Pin Mapping
Each EXTI line can only connect to one pin at a time, but pins with the same number (e.g., PA0, PB0, PC0) share the same interrupt line, so **pins with the same number from different ports cannot be used as interrupt sources simultaneously**.

| EXTI Line | Available Pins | Interrupt Handler Name |
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

### Key Steps Summary
Five core steps for configuring external interrupts using STM32F1xx-HAL:

| Step | Operation | Code Method | Description |
| :--- | :--- | :--- | :--- |
| 1 | **Configure pin as input** | `into_pull_up_input()` etc. | Select pull-up/pull-down/floating input based on external circuit |
| 2 | **Connect to EXTI line** | `make_interrupt_source(&mut syscfg)` | Connect pin to the corresponding EXTI interrupt line |
| 3 | **Set trigger edge** | `trigger_on_edge(&mut exti, Edge::RISING)` | Choose rising edge, falling edge, or both edges trigger |
| 4 | **Enable EXTI interrupt** | `enable_interrupt(&mut exti)` | Enable that interrupt line in the EXTI peripheral |
| 5 | **NVIC unmask** | `NVIC::unmask(pac::Interrupt::EXTI0)` | Enable the corresponding interrupt channel in NVIC |


---

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%A4%96%E9%83%A8%E4%B8%AD%E6%96%ADEXTI/src/main.rs)

The difficulty lies in the syntax differences between Rust 2021 and 2024 editions, although 

```rust
#![deny(unsafe_code)] 
```

can also solve it, but my approach is another way of thinking!



Also, automatically matching the compiler's syntax updates

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
        rprintln!("Enter interrupt +1");
        // ====================== Debounce core code ======================
        delay(72_000_000 / 1000 * 40); // 72MHz clock → 40ms delay
        // ===============================================================
        led.toggle();
        int_pin.clear_interrupt_pending_bit();
    }
}
```



## Timer Interrupt
### Timer Interrupt LED Blinking
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%AE%9A%E6%97%B6%E5%99%A8%E4%B8%AD%E6%96%AD/src/main.rs)

Logic framework diagram

```plain
┌──────────────────────────────────────────────────┐
│                    main()                        │
│  1. Create LED, Timer                             │
│  2. interrupt::free → Move into G_LED, G_TIM     │
│  3. NVIC::unmask → Enable interrupt               │
│  4. wfi() loop sleep                              │
└──────────────────────────────────────────────────┘
                           │
                    TIM2 interrupt trigger
                           │
┌──────────────────────────────────────────────────┐
│               TIM2() Interrupt Handler            │
│  1. Retrieve LED/Timer from global storage (first time) │
│  2. LED.toggle()                                 │
│  3. timer.clear_interrupt()                      │
└──────────────────────────────────────────────────┘
```



### RTIC Timer Interrupt Mode
<font style="color:#DF2A3F;"><<Recommended for development>> RTIC</font>

#### RTIC vs Bare Metal Interrupt Comparison
##### I. Architecture Comparison
###### Bare Metal Interrupt
```rust
// Need to manually define interrupt handler function, bind to interrupt vector table
#[interrupt]
fn TIM1_UP() {
    // Manually enter critical section (disable interrupts)
    cortex_m::interrupt::free(|_| {
        // All code in one function, resource management depends on the programmer
        // No priority management, rely entirely on manual interrupt enable/disable
    });
}
```

###### RTIC Interrupt
```rust
// Declarative interrupt binding via #[task(binds = ...)]
#[task(binds = TIM1_UP, priority = 1, local = [led, timer])]
fn tick(cx: tick::Context) {
    // Resources managed by RTIC framework, safety guaranteed at compile time
    // Priority automatically managed by RTIC scheduler
}
```

---

##### II. Core Differences
| Feature | Bare Metal Interrupt | RTIC |
| --- | --- | --- |
| **Resource Management** | Manual `critical section` protection | Compile-time automatic allocation, zero runtime overhead |
| **Priority Management** | Manual NVIC register operation | `priority = N` declarative configuration |
| **Data Sharing** | Requires `static mut` + `unsafe` | `#[shared]` + `Mutex`, compile-time safety |
| **Critical Section** | Manual interrupt enable/disable | RTIC automatically generates optimal critical section |
| **Interrupt Binding** | Modify `interrupt.rs` or `device.x` | `#[task(binds = TIM1_UP)]` one line done |
| **Context Switch** | Manual register save/restore | Hardware automatic stack push (Cortex-M) |
| **Code Organization** | All logic crammed in one interrupt function | Each task independent, resources separated |
| **Deadlock Prevention** | Programmer must be careful | Compile-time detection (based on Priority Ceiling Protocol) |


---

##### III. Resource Management Comparison
###### Bare Metal Approach: `static mut` + `unsafe`
```rust
// Global mutable static variables, require unsafe access
static mut LED_STATE: bool = false;
static mut COUNT: u8 = 0;

#[interrupt]
fn TIM1_UP() {
    unsafe {
        if LED_STATE {
            // Operate LED...
            LED_STATE = false;
        } else {
            // Operate LED...
            LED_STATE = true;
        }
        COUNT += 1;
    }
}
```

**Problems:**

+ Bugs in `unsafe` blocks are not caught by the compiler
+ Multiple interrupts accessing the same variable easily cause data races
+ High-priority interrupts may preempt low-priority ones, corrupting data consistency

###### RTIC Approach: `#[local]` Compile-Time Binding
```rust
#[task(binds = TIM1_UP, priority = 1, local = [led, led_state: bool = false, count: u8 = 0])]
fn tick(cx: tick::Context) {
    // Each resource is bound to this task at compile time
    // Other tasks cannot access it, naturally avoiding data races
    if *cx.local.led_state {
        cx.local.led.set_high();
        *cx.local.led_state = false;
    }
    *cx.local.count += 1;
}
```

**Advantages:**

+ Zero `unsafe`, compiler guarantees correctness
+ Resource binding determined at compile time, zero runtime overhead
+ Access resources simply with `cx.local.xxx`

---

##### IV. Priority and Scheduling Comparison
###### Bare Metal Approach: Manual NVIC Operation
```rust
// Need to manually configure priority
fn setup_timer_interrupt() {
    unsafe {
        // Set TIM1 interrupt priority to 1
        // Need to know NVIC register addresses and bit fields
        let nvic = &*cortex_m::peripheral::NVIC::ptr();
        // Complex register operations...
    }
}
```

###### RTIC Approach: Declarative Priority
```rust
// priority = 1 and done
#[task(binds = TIM1_UP, priority = 1)]
fn tick(cx: tick::Context) { ... }

// High priority task can preempt low priority
#[task(binds = USART1, priority = 2)]
fn serial(cx: serial::Context) { ... }
```

**RTIC Priority Rules:**

+ Higher number = higher priority
+ High priority tasks can preempt low priority tasks
+ Tasks with the same priority do not preempt each other

---

##### V. Shared Resource Comparison
###### Bare Metal Approach: Manual Critical Section
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
    // Forgot to add critical section? Data race! Compiler doesn't report error!
    unsafe { SHARED_DATA += 1; }
}
```

###### RTIC Approach: `#[shared]` + Automatic Lock
```rust
#[shared]
struct Shared {
    data: u32,
}

#[task(binds = TIM1_UP, priority = 1, shared = [data])]
fn tick(cx: tick::Context) {
    // lock() automatically manages critical section, safety guaranteed at compile time
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}

#[task(binds = USART1, priority = 2, shared = [data])]
fn serial(cx: serial::Context) {
    // High priority task accesses the same resource, RTIC automatically generates optimal lock
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}
```

**RTIC Lock Mechanism (Priority Ceiling Protocol PCP):**

+ When a low priority task holds a lock, high priority task waits
+ But RTIC automatically raises the priority of the lock-holding task to prevent mid-priority tasks from jumping the queue
+ All of this is determined at compile time, zero runtime overhead

---

VI. Summary

| Scenario | Recommended Approach |
| --- | --- |
| Learning interrupt principles | Bare metal (understand low-level mechanisms) |
| Formal project development | RTIC (safe, efficient, maintainable) |
| Single simple interrupt | Bare metal (similar code volume) |
| Multiple interrupts + shared resources | RTIC (avoid data races) |
| Strict real-time requirements | RTIC (zero-overhead abstraction, deterministic scheduling) |


**RTIC Core Advantages:**

+ **Zero-overhead abstraction**: Everything determined at compile time, no runtime overhead
+ **Compile-time safety**: Data races, deadlocks caught at compile time
+ **Declarative programming**: Use attribute macros to describe "what to do", framework generates "how to do it"
+ **Priority Ceiling Protocol**: Optimal critical section management

<font style="color:#DF2A3F;"></font>

#### How to Use RTIC
Need to modify config.toml here

```bash
# Package addition
[dependencies] 
....
rtic = { version = "2", features = ["thumbv7-backend"] }
```

**Reason:**

+ Original project lacks the `rtic` dependency, causing the `rtic` module to not be found
+ RTIC v2 requires specifying the backend feature, STM32F103 is Cortex-M3 architecture, uses `thumbv7-backend`
+ Other optional backends: `thumbv6-backend` (Cortex-M0/M0+), `thumbv8base-backend` (Cortex-M23), `thumbv8main-backend` (Cortex-M33)

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%AE%9A%E6%97%B6%E5%99%A8%E4%B8%AD%E6%96%AD_RTIC_1/src/main.rs)

Output:

```bash
FLASH] Programming via probe-rs...

      Erasing ✔ 100% [####################]   5.00 KiB @   6.69 KiB/s (took 1s)
  Programming ✔ 100% [####################]   5.00 KiB @   4.70 KiB/s (took 1s)                                                   Finished in 1.93s
Program started: init function begins execution
12:16:00.898: RCC clock configuration complete
12:16:00.898: PC13 LED pin configuration complete
12:16:00.898: TIM1 timer configuration complete, triggers interrupt every 1 second
12:16:00.898: Entering idle loop, CPU waiting for interrupts...
12:16:01.855: [Interrupt] LED turned on
12:16:01.855: [Interrupt] Count: 1
12:16:02.932: [Interrupt] LED turned off
12:16:02.932: [Interrupt] Count: 2
12:16:03.896: [Interrupt] LED turned on
12:16:03.896: [Interrupt] Count: 3
12:16:04.849: [Interrupt] LED turned off
12:16:04.849: [Interrupt] Count: 4
12:16:04.849: [Interrupt] Timer switched to 500ms
12:16:05.333: [Interrupt] LED turned on
12:16:05.333: [Interrupt] Count: 5
12:16:05.934: [Interrupt] LED turned off
12:16:05.934: [Interrupt] Count: 6
12:16:06.426: [Interrupt] LED turned on
12:16:06.426: [Interrupt] Count: 7
12:16:06.914: [Interrupt] LED turned off
12:16:06.914: [Interrupt] Count: 8
12:16:07.401: [Interrupt] LED turned on
12:16:07.401: [Interrupt] Count: 9
12:16:07.888: [Interrupt] LED turned off
12:16:07.888: [Interrupt] Count: 10
12:16:08.377: [Interrupt] LED turned on
12:16:08.377: [Interrupt] Count: 11
12:16:08.874: [Interrupt] LED turned off
12:16:08.874: [Interrupt] Count: 12
12:16:08.874: [Interrupt] Timer switched to 1s, counter reset
12:16:09.830: [Interrupt] LED turned on
12:16:09.830: [Interrupt] Count: 1
```

  
<font style="color:#DF2A3F;">Using hprintln can avoid the output issue caused by cortex_m::asm::wfi();</font>

```rust
config.toml

[dependencies]
cortex-m-semihosting = "0.5.0"
----------------

use cortex_m_semihosting::hprintln;
...
hprintln!("...");
```




## RTIC2 Async Tasks
Use RTIC to implement 2-task scheduling in async mode

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/RTIC2%E5%BC%82%E6%AD%A5%E4%BB%BB%E5%8A%A1/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045924896-cc601051-f619-4596-87f6-9a2246f5de5c.png" width="581" title="" crop="0,0,1,1" id="ud3c4bf51" class="ne-image">

As shown above, 4 toggles execute one heartbeat! Executed simultaneously




## UART Communication
#### Normal Mode
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045838370-24ec16fe-462e-4267-92b7-5ef5b4c3f955.png" width="1451" title="" crop="0,0,1,1" id="u5f48f617" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045850995-183a5b47-560b-4c32-96d1-59dd0bc6484e.png" width="372" title="" crop="0,0,1,1" id="ud58583c3" class="ne-image">

**Available pins for each UART (F103):**

| UART | TX Pin | RX Pin | Notes |
| --- | --- | --- | --- |
| USART1 | PA9 or PB6(remap) | PA10 or PB7(remap) | APB2 |
| USART2 | PA2 | PA3 | APB1 |
| USART3 | PB10 | PB11 | APB1 |


**Key Concepts:**

+ `into_alternate_push_pull()` — Alternate push-pull output, pin controlled by hardware peripheral
+ `Config::default().baudrate()` — UART configuration (baud rate, data bits, stop bits, etc.)
+ `.split()` — Split into independent `Tx` and `Rx` objects
+ `.reunite()` — Recombine `Tx` and `Rx`



#### UART Communication_fmt Mode
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1_fmt/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047558289-b27d36c0-42f4-4c70-b44b-6278e05bc28c.png" width="492" title="" crop="0,0,1,1" id="u784935b0" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047584627-e3e1ce36-b097-4508-959a-6fa5562f7ca5.png" width="624" title="" crop="0,0,1,1" id="uc9269666" class="ne-image">

The information shown above is quite perfect in my opinion. The advantage of this language is that once the framework is set up, let AI complete the tasks!



#### UART Interrupt — Idle Detection
Mainly uses IDLE mode

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1_%E4%B8%AD%E6%96%AD_IDLE/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780070623115-035fd2f2-c6db-41b9-87e6-58d7dbe34ef2.png" width="638" title="" crop="0,0,1,1" id="uec811e07" class="ne-image">



**9-bit data mode, using the 9th bit to mark address/data.**

```rust
// Configure for 9-bit data
let serial = p.USART3.serial::<PushPull>(
    (tx_pin, rx_pin),
    Config::default()
        .baudrate(9600.bps())
        .wordlength_9bits()    // 9-bit data
        .parity_none(),        // No parity
    &mut rcc,
);

// 9th bit = 1 means address byte
// 9th bit = 0 means data byte
block!(serial_tx.write(SLAVE_ADDR as u16 | 0x100)).unwrap();  // Send address
block!(serial_tx.write(data_byte)).unwrap();                   // Send data
```

**Usage:** Distinguishing address frames from data frames in multi-device communication.



#### UART DMA Reception
[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E4%B8%B2%E5%8F%A3%E9%80%9A%E4%BF%A1_fmt_DMA/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071793247-2af14a44-7e27-459a-8ed3-05ec1a3abc0a.png" width="342" title="" crop="0,0,1,1" id="u7dbb3119" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071860321-d37bfaff-e264-4099-a590-ef16f984731d.png" width="328" title="" crop="0,0,1,1" id="uabd331a7" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071685474-f076938b-fd10-4f3b-8796-4be8b770bf0e.png" width="306" title="" crop="0,0,1,1" id="u3ad590c6" class="ne-image">

Explains how it only triggers after 8 bytes of data are stored!



## ADC Sampling
**ADC Key Parameters:**

+ Resolution: 12-bit (0-4095)
+ Conversion time: Depends on ADC clock
+ Reference voltage: VDDA (typically 3.3V)
+ Formula: `Voltage = Reading / 4095 * 3.3V`

**DKX Board Available ADC Channels:**

| Pin | ADC Channel |
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


### External Voltage Sampling
Sample PB01 pin voltage

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/ADC%E7%94%B5%E5%8E%8B%E9%87%87%E9%9B%86/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073349544-41a745c3-faaa-44c9-8ada-70e709ed8da6.png" width="514" title="" crop="0,0,1,1" id="ub9d403d9" class="ne-image">

Connected to ground

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073451277-a7b6496a-b345-4f55-8089-a2763cb2c721.png" width="536" title="" crop="0,0,1,1" id="u294fb1ca" class="ne-image">

Connected to 3.3V

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073526370-be39c8d3-3198-4e0e-81fd-9e58c6035ba3.png" width="529" title="" crop="0,0,1,1" id="u69367ae2" class="ne-image">

2 x 1K resistors in series, measuring the midpoint voltage



### Internal ADC Temperature Conversion
**Internal Temperature Sensor:**

+ Connected to ADC1 channel 16
+ Low accuracy (±1.5°C), suitable for rough monitoring
+ Conversion time requires at least 17.1μs

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/%E5%86%85%E9%83%A8ADC%E8%BD%AC%E6%8D%A2%E6%B8%A9%E5%BA%A6/src/main.rs)



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780074073750-6029e96f-c1bf-4b19-856a-141ae6e4a1a5.png" width="400" title="" crop="0,0,1,1" id="ubecbebec" class="ne-image">



### ADC_DMA Continuous Sampling
**Continuous DMA Working Principle:**

```plain
     Buffer A          Buffer B
┌─────────────┐  ┌─────────────┐
│ [0] [1] ... │  │ [0] [1] ... │
│    [7]      │  │    [7]      │
└─────────────┘  └─────────────┘
       ↑ DMA write    ↑ DMA write
       └── Alternating ──┘

Half::First  → Buffer A readable
Half::Second → Buffer B readable
```

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/ADC_DMA%E5%BE%AA%E7%8E%AF%E9%87%87%E9%9B%86/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780123010814-cbd8950f-c10d-4fd8-b2e0-13887353b25e.png" width="625" title="" crop="0,0,1,1" id="uc37cf9c3" class="ne-image">



## SPI Protocol
**SPI Mode Details:**

| Mode | CPOL | CPHA | Clock Idle | Data Sampling |
| --- | --- | --- | --- | --- |
| Mode 0 | 0 | 0 | Low | First edge |
| Mode 1 | 0 | 1 | Low | Second edge |
| Mode 2 | 1 | 0 | High | First edge |
| Mode 3 | 1 | 1 | High | Second edge |


**JLCPCB STM32F103C8T6 Board SPI1 Pins:** PA5(SCK), PA6(MISO), PA7(MOSI), PA4(CS)



### Light Up ST7789 Screen
240*240

**Hardware Pin Connection Table**

| Display Pin | MCU Pin | Function Description |
| :--- | :--- | :--- |
| SCL | PA5 | SPI clock line (SPI1_SCK) |
| SDA | PA7 | SPI data output (SPI1_MOSI) |
| DC | PA0 | Command/data selection |
| RES | PA1 | Hardware reset |
| CS | GND | Chip select pulled low (always selected) |


> **Note**: "SCL" and "SDA" in the table are typically I²C bus signal names, but here they connect to the SPI interface, corresponding to SPI's **SCK** and **MOSI**. This naming is common on some LCD modules; just use them according to the pin names. CS connected to GND means this SPI device is always selected, no software chip select control needed.
>

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780126926273-223244e9-d923-467e-be77-c109bd054a91.png" width="272" title="" crop="0,0,1,1" id="ucea3e5ce" class="ne-image">

ST7789 driver code----src/st7789.rs

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/SPI/src/st7789.rs)

Main program

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/SPI/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780127246884-25c80e3f-a8bb-4171-8cb5-e03b616cd2af.png" width="543" title="" crop="0,0,1,1" id="u63cb4aa1" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780127264032-9cb415a8-b939-4689-a366-1cc4a698b1a0.png" width="391" title="" crop="0,0,1,1" id="u71c90ac3" class="ne-image">



## I2C Communication
Using address scanning as an example!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129573451-695873a0-2895-4ae6-82d9-760b0735e6d3.png" width="547" title="" crop="0,0,1,1" id="uf3139d39" class="ne-image">

**Wiring**

| MCU | Device |
| --- | --- |
| PB6 | SCL |
| PB7 | SDA |


[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/I2C%E5%9C%B0%E5%9D%80%E6%89%AB%E6%8F%8F/src/main.rs)

Wiring as shown

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129746845-ddca9982-5283-4dd1-9def-439f14f69b5e.png" width="645" title="" crop="0,0,1,1" id="u6a6d07a1" class="ne-image">



## PWM Wave
### Output
We use PWM to control a servo as an example

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780132432825-534b3892-b8fd-4f4d-ace6-0350c6c16588.png" width="589" title="" crop="0,0,1,1" id="u1a6aff10" class="ne-image">

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/PWM%E8%88%B5%E6%9C%BA/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780132461432-60a818a5-4ab8-4cf6-afa6-a0009cb11d23.png" width="342" title="" crop="0,0,1,1" id="u22f9ccbe" class="ne-image">



### Input
We use the EC11 encoder as an example

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780133888771-3dfd0d4c-f65c-4ded-af72-16faef0fcbed.png" width="394" title="" crop="0,0,1,1" id="ue032c32d" class="ne-image">

EC11 (with button)

**Wiring**

| MCU | Device |
| --- | --- |
| PB4 | S2 |
| PB5 | S2 |


[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/PWM%E8%BE%93%E5%85%A5%E6%A3%80%E6%B5%8B/src/main.rs)

Rotary encoder detects data! No output when not rotating!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780134186205-bb66094b-f5f1-49a0-8f13-673b832f66e1.png" width="480" title="" crop="0,0,1,1" id="u43c0b949" class="ne-image">

Output detection info



### EC11 Encoder Reading
**Wiring**

| MCU | Device |
| --- | --- |
| PB6 | S1 |
| PB7 | S2 |


**Usage:** Motor encoder, rotary knob and other quadrature encoder devices for speed/position measurement.

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/EC11%E7%BC%96%E7%A0%81%E5%99%A8/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780136893674-70671b8b-3b27-429a-8a79-12fdca9cbd25.png" width="499" title="" crop="0,0,1,1" id="u76a64ffa" class="ne-image">



## CRC Verification
**Usage:** Data integrity verification, CRC verification in communication protocols.

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/CRC%E6%A0%A1%E9%AA%8C/src/main.rs)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780138752178-cbb34d1f-7689-4eb2-a05c-651295f45c4d.png" width="590" title="" crop="0,0,1,1" id="u603a4730" class="ne-image">



## DAC Digital-to-Analog Conversion
> Note: STM32F103C8T6 (DKX board) **has no DAC**. DAC is only available on high-density devices (STM32F103xC/D/E).
>

Note that C8T6 does not support it, so we choose STM32F103RCT6

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

Change the file configuration in config.toml

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780139752540-46050614-f41b-4e59-a165-a384a1b0b9ab.png" width="659" title="" crop="0,0,1,1" id="ud3f9d220" class="ne-image">

```rust
# stm32f1xx-hal: Hardware abstraction layer for STM32F1 series
# Provides high-level Rust API for RCC, GPIO, TIM, USART, etc.
[dependencies.stm32f1xx-hal]
version = "0.11.0"
features = [
    "stm32f103",  # STM32F103 series chip
    "high",       # High density (256KB Flash or above), RCT6 belongs to this type
]
```

Code as follows

[See code here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/DAC%E6%95%B0%E6%A8%A1%E8%BD%AC%E6%8D%A2/src/main.rs)

Results as follows

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140722259-090abde4-4b38-4c94-b484-0cf40c1bf96a.png" width="396" title="" crop="0,0,1,1" id="u15e94679" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140832880-63b745a7-86fc-43cd-8947-01980100a6dc.png" width="560" title="" crop="0,0,1,1" id="u415533ee" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140776299-cfdce37d-a05f-46cd-a8e9-85e1fe758aa5.png" width="563" title="" crop="0,0,1,1" id="ud77ae8bf" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140863989-0e947147-c83d-4fa5-b09c-59dbb4a05e95.png" width="565" title="" crop="0,0,1,1" id="u57f96e96" class="ne-image">

Measurement results are within acceptable error range



## CAN Bus (No Device Verification — Not Tested)
```markdown
use bxcan::Fifo;
use bxcan::filter::Mask32;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // CAN requires external crystal for clock accuracy
    let mut rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()), &mut flash.acr);

    let mut can1 = {
        let gpioa = dp.GPIOA.split(&mut rcc);
        let rx = gpioa.pa11;  // CAN RX
        let tx = gpioa.pa12;  // CAN TX

        let can = dp.CAN.can(dp.USB, (tx, rx), &mut rcc);

        // Configure bit timing: 125kBit/s, sample point 87.5%
        bxcan::Can::builder(can)
            .set_bit_timing(0x001c_0003)
            .leave_disabled()
    };

    // Configure filter (receive all frames)
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters);

    // Enable CAN
    let mut can = can1;
    block!(can.enable_non_blocking()).unwrap();

    // Loopback test: receive frame and immediately send back
    loop {
        if let Ok(frame) = block!(can.receive()) {
            block!(can.transmit(&frame)).unwrap();
        }
    }
}
```

**CAN Pins (DKX board):** PA11(CAN RX), PA12(CAN TX) — Note shared with USB pins



## USB Serial (No Device Verification — Not Tested)
### USB Polling Serial (No Device Verification — Not Tested)
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

    // USB must use 48MHz system clock
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()).sysclk(48.MHz()).pclk1(24.MHz()),
        &mut flash.acr,
    );

    assert!(rcc.clocks.usbclk_valid());  // Verify USB clock is valid

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_high();  // Turn off LED

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // USB D+ line has a pull-up resistor
    // During development, need to pull D+ low to trigger USB RESET
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();                           // Pull D+ low
    delay(rcc.clocks.sysclk().raw() / 100);     // Brief delay

    // Configure USB peripheral
    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,                                  // USB DM = PA11
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),  // USB DP = PA12
    };
    let usb_bus = UsbBus::new(usb);

    // Create CDC-ACM serial device
    let mut serial = SerialPort::new(&usb_bus);

    // Build USB device
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
                led.set_low();  // Turn on LED
                // Convert received characters to uppercase and echo back
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;  // 'a'~'z' → 'A'~'Z'
                    }
                }
                // Write back (may need multiple writes)
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
        led.set_high();  // Turn off LED
    }
}
```

**USB Key Points:**

+ **Must have 48MHz system clock** (USB protocol requires precise clock)
+ PA11 = USB D-, PA12 = USB D+
+ During development, need to manually trigger USB RESET
+ VID/PID `0x16c0:0x27dd` is an informal test ID
+ Requires release mode compilation (debug mode will overflow FLASH)

---

### USB Interrupt Serial (No Device Verification — Not Tested)
Use interrupt-driven approach for USB communication.

```rust
// Global USB objects
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;

#[entry]
fn main() -> ! {
    // ... USB initialization code ...

    // Enable USB interrupts
    unsafe {
        NVIC::unmask(Interrupt::USB_HP_CAN_TX);   // High priority
        NVIC::unmask(Interrupt::USB_LP_CAN_RX0);  // Low priority
    }

    loop { wfi(); }  // All work done in interrupts
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
            // Process received data
            for c in buf[0..count].iter_mut() {
                if 0x61 <= *c && *c <= 0x7a {
                    *c &= !0x20;  // Convert to uppercase
                }
            }
            serial.write(&buf[0..count]).ok();
        }
        _ => {}
    }
}
```

**Polling vs Interrupt:**

+ Polling: Simple, but CPU is always busy waiting
+ Interrupt: CPU can sleep, saving power



# Actual Projects
## DHT11
[Project source here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/tree/main/Dome/DHT11)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780160254218-22ef0a3f-9b61-4350-b558-f75ed994c24f.png" width="450" title="" crop="0,0,1,1" id="uecb29377" class="ne-image">



## DHT11 + ST7789 LCD Thermometer
| Display & DHT11 | MCU Pin | Function Description |
| :--- | :--- | :--- |
| SCL | PA5 | SPI clock line (SPI1_SCK) |
| SDA | PA7 | SPI data output (SPI1_MOSI) |
| DC | PA0 | Command/data selection |
| RES | PA1 | Hardware reset |
| CS | GND | Chip select pulled low (always selected) |
| DATA | PA6 | DHT11 data line |


Actual result

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164028402-54ee6f36-3789-4788-b616-7f4f67a666ce.png" width="628" title="" crop="0,0,1,1" id="u132cff35" class="ne-image">

Project structure

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164081628-3762ec47-4c6c-4b87-ad2f-7dbf972396b9.png" width="812" title="" crop="0,0,1,1" id="u5e153928" class="ne-image">

 [Project source here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/tree/main/Dome/DHT11%2BST7789)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164408965-82bc27da-8093-4c6a-9042-138ae538676f.png" width="680" title="" crop="0,0,1,1" id="u7c1aa107" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164569870-c81ed584-799f-4093-8e82-20d2ca0bd4bd.png" width="720" title="" crop="0,0,1,1" id="u1d403555" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164484495-b510887a-e182-4bc3-b09e-29dadc3900a7.png" width="577" title="" crop="0,0,1,1" id="u612677a5" class="ne-image">

Don't ask me why I used Python — because it's fast!



## DHT20 + ST7789 LCD Thermometer
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780166394105-ec6c2ced-4711-4e04-9992-2d3939656899.png" width="547" title="" crop="0,0,1,1" id="u7f142962" class="ne-image">

| Display & DHT20 | MCU Pin | Function Description |
| :--- | :--- | :--- |
| SCL (Display) | PA5 | SPI clock line (SPI1_SCK) |
| SDA (Display) | PA7 | SPI data output (SPI1_MOSI) |
| DC | PA0 | Command/data selection |
| RES | PA1 | Hardware reset |
| CS | GND | Chip select pulled low (always selected) |
| SCL (DHT20) | PB7 | DHT20 clock line |
| SDA (DHT20) | PB6 | DHT20 data line |


[Project source here](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/tree/main/Dome/DHT20%2BST7789)

## ST7789 3D Cube (No Color — Framework)
Render a rotating 3D cube on ST7789 240x240 display, supporting colored faces, wireframe rendering, and FPS counter.

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780395668801-d4b9c570-0ae4-419c-9862-219bafb97cdc.png" width="445" title="" crop="0,0,1,1" id="u355847a2" class="ne-image">

**Hardware Pin Connection (same as SPI display):**

| Display Pin | MCU Pin | Function Description |
| :--- | :--- | :--- |
| SCL | PA5 | SPI clock line (SPI1_SCK) |
| SDA | PA7 | SPI data output (SPI1_MOSI) |
| DC | PA0 | Command/data selection |
| RES | PA1 | Hardware reset |
| CS | GND | Chip select pulled low (always selected) |

**Features:**

+ 6-face colored cube (red, yellow, green, cyan, blue, purple)
+ Wireframe edge rendering with dirty rectangle optimization
+ Real-time FPS counter display
+ Configurable cube size, field of view, rotation speed
+ SPI clock: 36MHz

[Project source code](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/Dome/ST7789_cube3d_No_Color)


# Conclusion
Author email: pycx0@qq.com  
The author actually completed this project during the senior year just before graduation, because the employment pressure in China is really intense. Currently job hunting!  
Future updates will be in the original link! Hoping to find a good job!

Looking forward to the Rust ecosystem getting better and better! Go for it, "villagers" of the global village!
