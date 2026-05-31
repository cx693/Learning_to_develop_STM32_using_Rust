<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780203075814-b3bd4b33-1fec-4f6a-ba6d-342f60f58486.png" width="1448" title="" crop="0,0,1,1" id="ue9743a87" class="ne-image">



[简体中文](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_cn.md) / [English](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_en.md) / [Русский](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_ru.md)

# Declaration
This project is licensed under CC BY-NC 4.0. Commercial use requires permission from the copyright owner at pycx0@qq.com. Commercial products based on this project must obtain a license. Non-commercial use is free!

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
Main purpose: fix infinite loop bug

```bash
cargo add panic-halt
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779700077727-2d4a69c3-fbfa-4838-9bcf-5be9c764a7a0.png" width="369" title="" crop="0,0,1,1" id="u09183fea" class="ne-image">

Check FLASH usage

```json
cargo install st-mem
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779726981642-0c7dc5e5-9c60-4e5f-a84e-f07c4be2bdd0.png" width="764" title="" crop="0,0,1,1" id="uaed0ebe6" class="ne-image">



# Compile Test
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

```rust
# [package] section: defines package metadata
[package]
# Package name, used to identify when publishing to crates.io or being referenced by other projects
name = "stm32dome"
# Package version, follows Semantic Versioning
version = "0.1.0"
# Rust edition convention, 2024 is the latest edition, enabling the newest language features
edition = "2024"

# [dependencies] section: defines project dependencies
[dependencies]
# embedded-hal: embedded hardware abstraction layer standard interface
# version "1.0" means use the latest 1.0.x release
# Defines common traits for GPIO, I2C, SPI, serial port, etc., making driver code portable across different MCUs
embedded-hal = "1.0"

# nb: non-blocking operations library
# Provides Result and block! macro for handling operations that may need retries (e.g., serial port send)
nb = "1"

# cortex-m: ARM Cortex-M processor low-level support library
# Provides system register access, interrupt management, system timer, etc.
cortex-m = "0.7.7"

# cortex-m-rt: Cortex-M runtime library
# Provides startup code, interrupt vector table, memory initialization, etc.
# Needed to use the #[entry] macro to define the program entry point
cortex-m-rt = "0.7.5"
panic-halt = "1.0.0"
rtt-target = "0.6.2"
rtic = { version = "2", features = ["thumbv7-backend"] }
rtic-monotonics = { version = "2", features = ["cortex-m-systick"] }


# [dependencies.stm32f1xx-hal]: detailed configuration for specific dependency
# Uses table syntax to provide more detailed configuration for stm32f1xx-hal
[dependencies.stm32f1xx-hal]
# Specify version number
version = "0.11.0"
# features: enable compile-time features
# "stm32f103": select support code for STM32F103 series chip
# "medium": medium density chip configuration (64-128KB Flash), C8T6 belongs to this class
# Other options include "low" (low density, 16-32KB) and "high" (high density, 256KB+)
features = ["stm32f103", "medium"]


[profile.dev]
incremental = false   # Disable incremental compilation for embedded build consistency
codegen-units = 1     # Single code generation unit, let compiler do more optimization
opt-level = 1         # Light optimization, avoid slow interrupt handling in debug mode
panic = "abort"       # Abort directly on panic, no stack unwinding (no stack unwinding support in embedded)

[profile.release]
codegen-units = 1
debug = true          # Keep debug info (no performance impact, convenient for debugging with debugger)
lto = true            # Link-time optimization, cross-crate optimization to reduce size
panic = "abort"
```

.cargo/config.toml

```rust
[target.thumbv7m-none-eabi]
# ============================================================
# Runner — st-mem runner (cross-platform, analyzes memory first then flashes)
# ============================================================
# st-mem runner: analyze FLASH/RAM usage → probe-rs flash
runner = "st-mem runner --chip STM32F103C8 --protocol swd"
# ============================================================
# Without memory analysis, use probe-rs directly:
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

<font style="color:#DF2A3F;">main.rs see Lighting the First LED</font>

## Function navigation not working
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

# Debug
Create configuration task files! Use probe-rs official website! Search

[https://probe.rs/docs/tools/debugger/](https://probe.rs/docs/tools/debugger/)



Supported chip types: [https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)



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
            "chip": "STM32F103C8", // Chip model - modify according to yours!
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
                    // Enable RTT
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



~~Currently macOS RTT debugging has issues - no terminal output!~~ Resolved!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976699740-3c2d6980-e2fa-46f9-8404-b6d5f4366b2d.png" width="1440" title="" crop="0,0,1,1" id="u90e0f0b7" class="ne-image">

Current solution: set a breakpoint at the beginning of main() and write the first line as: rtt_init_print!();

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976859955-199c20a0-8073-4eca-b109-c0a097238644.png" width="486" title="" crop="0,0,1,1" id="ue86693d6" class="ne-image">







# Project Download (Flashing)
Basic Rust build environment

```plain
cargo install cargo-binutils
rustup component add llvm-tools
```

## HEX File
Compile ELF file

```rust
cargo build --release
```

Compile HEX file

```rust
cargo objcopy --release -- -O ihex ccc.hex
cargo objcopy --release -- -O ihex <firmware_name>.hex
```

Direct download

```rust
probe-rs download --binary-format hex --chip STM32F103C8 ccc.hex
probe-rs download --binary-format hex --chip <chip_name> <firmware_name>.hex
```

Chip name lookup: [https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779867658066-603db27f-8d6c-4545-b93a-3fc240c50078.png" width="1540" title="" crop="0,0,1,1" id="ua5bbe57c" class="ne-image">

## BIN File
Compile ELF file

```rust
cargo build --release
```

Compile HEX file

```rust
cargo objcopy --release -- -O binary ccc.bin
cargo objcopy --release -- -O binary <firmware_name>.bin
```

Direct download

```rust
probe-rs download --chip STM32F103C8 --base-address 0x08000000 --binary-format bin ccc.bin
probe-rs download --chip <chip_name> --base-address <offset_address> --binary-format bin <firmware_name>.bin
```



# Release JTAG Port - Cannot Download
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897632872-17f9317c-b978-4d97-845e-7012f1e8897b.png" width="604" title="" crop="0,0,1,1" id="u888d55bd" class="ne-image">

Or if running fails -- full chip erase command

```rust
probe-rs erase --chip STM32F103C8 --speed 100 --protocol swd
probe-rs erase --chip <chip_name> --speed 100 --protocol <interface_type - optional>
probe-rs erase --chip STM32F103C8 --speed 100
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897847743-83db4c01-57bd-4529-8fef-cb60a3c3b09f.png" width="496" title="" crop="0,0,1,1" id="u2c272bf8" class="ne-image">

That means boot0-boot1 --> both 0

After executing the command, press the reset key quickly and release! It will auto-erase. If that doesn't work, hold the reset button, execute the command, then release immediately!



# Learning Resources
Textbook: [https://xxchang.github.io/book/](https://xxchang.github.io/book/)

Project repository: [https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples](https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples)



# Basic Learning
---

## Clock System Deep Dive
> **Why learn clocks first?** Because almost all peripherals depend on the clock to function. Clock configuration is the most fundamental and important step in embedded development. Incorrect configuration can cause peripherals to malfunction, inaccurate serial baud rates, USB enumeration failures, and other issues.
>

### STM32F1 Clock Tree Overview
The STM32F103 clock system is very flexible, with multiple clock sources and dividers. Here is a simplified clock tree:

```plain
                          ┌─────────────┐
                          │   HSE       │  External high-speed crystal (DKX board: 8MHz)
                          │  8 MHz      │
                          └──────┬──────┘
                                 │
                          ┌──────▼──────┐
                          │   HSI       │  Internal RC oscillator
                          │  8 MHz      │  (Low accuracy, ±1%, fast startup)
                          └──────┬──────┘
                                 │
                 ┌───────────────┼───────────────┐
                 │               │               │
                 │         ┌─────▼─────┐         │
                 │         │   PLL     │  Phase-locked loop
                 │         │  Multiplier │  ×2~×16
                 │         └─────┬─────┘         │
                 │               │               │
          ┌──────▼──────┐ ┌──────▼──────┐        │
          │ SYSCLK      │ │  USBCLK     │        │
          │ System clock │ │  USB clock  │        │
          │ Max 72MHz   │ │  Must be    │        │
          │              │ │  48MHz      │        │
          └──────┬──────┘ └─────────────┘        │
                 │                                │
        ┌────────┼────────┐                      │
        │        │        │                      │
   ┌────▼───┐ ┌──▼───┐ ┌──▼────┐          ┌─────▼─────┐
   │ AHB    │ │ APB1 │ │ APB2  │          │  SYSCLK   │
   │ Bus    │ │ Bus  │ │ Bus   │          │  Source    │
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

**Core Concept: PLL (Phase-Locked Loop)**

```plain
PLL clock = PLL input clock × PLL multiplication factor

If HSE is selected as PLL input:
  PLLCLK = HSE × multiplication factor (×2 ~ ×16)

Example (DKX board 8MHz crystal):
  HSE × 9  = 8 × 9  = 72 MHz ← Maximum system clock
  HSE × 6  = 8 × 6  = 48 MHz ← USB required
  HSE × 4  = 8 × 4  = 32 MHz

If HSI is selected as PLL input:
  PLLCLK = HSI × 2 × multiplication factor / 2
  PLLCLK = HSI × multiplication factor (×2 ~ ×16)
  But HSI must first be divided by 2 before entering PLL
```

---

### Clock Source Details
#### HSI (High Speed Internal) — Internal High-Speed Clock
```plain
Characteristics:
├── Frequency: 8 MHz (RC oscillator, temperature drift)
├── Accuracy: ±1% (factory calibrated), drifts with temperature
├── Advantages: No external components needed, available on power-up
├── Disadvantages: Low accuracy, not suitable for USB, CAN, precise baud rates
└── Default: Automatically used as system clock source after power-up
```

**When to use HSI?**

+ Simple LED blinking, button detection — scenarios not requiring precise timing
+ Backup when external crystal is damaged
+ Fast startup scenarios (HSI starts faster than HSE)

#### HSE (High Speed External) — External High-Speed Clock
```plain
Characteristics:
├── Frequency: 4-16 MHz (DKX board uses 8MHz crystal)
├── Accuracy: ±0.005% (depends on crystal quality)
├── Advantages: High accuracy, suitable for USB, CAN, precise serial baud rates
├── Disadvantages: Requires external crystal, takes time to start (hundreds of μs to ms)
└── DKX board: 8MHz passive crystal + 2 x 20pF load capacitors
```

**When to use HSE?**

+ Need USB functionality (**must** use HSE or HSE through PLL)
+ Need CAN bus (requires precise clock)
+ Need precise serial baud rates
+ Need system to run at full 72MHz

#### LSE (Low Speed External) — External Low-Speed Clock
```plain
Characteristics:
├── Frequency: 32.768 kHz (for RTC)
├── Accuracy: Very high (crystal has low temperature drift)
├── Uses: Real-time clock (RTC), watchdog
└── DKX board: LSE crystal may not be soldered (check schematic)
```

#### LSI (Low Speed Internal) — Internal Low-Speed Clock
```plain
Characteristics:
├── Frequency: Approximately 40 kHz (inaccurate)
├── Uses: Independent watchdog (IWDG), RTC backup clock
└── Accuracy: Poor (±30%)
```

#### PLL (Phase Locked Loop)
PLL is the core of the clock system, used to multiply low-frequency clocks to high frequencies.

```plain
┌─────────────────────────────────────────────────┐
│                    PLL Details                   │
├─────────────────────────────────────────────────┤
│                                                  │
│  Input source selection:                         │
│  ┌──────┐    ┌─────────┐                         │
│  │ HSI/2│───►│         │    ┌───────────┐        │
│  └──────┘    │ PLL MUX │───►│  ÷ PLLMUL │──► PLLCLK
│  ┌──────┐───►│         │    │  (×2~×16) │        │
│  │ HSE  │    └─────────┘    └───────────┘        │
│  └──────┘                                        │
│                                                  │
│  Common configurations:                          │
│  ┌──────────┬──────────┬──────────────┐          │
│  │ Input clk│ Multiply │ Output freq  │          │
│  ├──────────┼──────────┼──────────────┤          │
│  │ HSI 8MHz │ ×9       │ 36 MHz*     │          │
│  │ HSE 8MHz │ ×9       │ 72 MHz ✓    │          │
│  │ HSE 8MHz │ ×6       │ 48 MHz ✓    │          │
│  │ HSE 8MHz │ ×4       │ 32 MHz ✓    │          │
│  └──────────┴──────────┴──────────────┘          │
│                                                  │
│  * HSI is first divided by 2 (=4MHz), then ×9 = 36MHz
│                                                  │
└─────────────────────────────────────────────────┘
```

---

### Bus Clock Details
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


**Note:** SysTick clock comes from AHB (if SysTick is configured to use processor clock) or AHB/8.

#### APB1 Bus — Low-Speed Peripheral Bus
| Parameter | Description |
| --- | --- |
| Maximum Frequency | **36 MHz** (hard limit, exceeding may damage the chip) |
| Prescaler | AHB ÷ 1/2/4/8/16 |
| Connected Devices | USART2, USART3, I2C1/2, SPI2, TIM2-4, USB, CAN |
| Configuration | `rcc::Config::hsi().pclk1(36.MHz())` |


**Important:** If APB1 prescaler > 1, timer clock = APB1 × 2.

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

    // Method 1: Concise configuration
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hsi()           // Use internal 8MHz RC
            .sysclk(64.MHz())        // System clock 64MHz
            .pclk1(32.MHz())         // APB1 clock 32MHz
            .pclk2(64.MHz())         // APB2 clock 64MHz
            .adcclk(8.MHz()),        // ADC clock 8MHz
        &mut flash.acr,
    );

    // Method 2: Using external crystal + PLL
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
.hclk(72.MHz())                      // Set AHB clock (usually = SYSCLK)

// PLL automatically calculates the multiplication factor based on target frequency
// No need to set manually!
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

#### Why `flash.acr`?
Flash read speed is limited. When the system clock exceeds 24MHz, wait cycles must be inserted:

| System Clock | Flash Wait Cycles |
| --- | --- |
| 0-24 MHz | 0 wait cycles |
| 24-48 MHz | 1 wait cycle |
| 48-72 MHz | 2 wait cycles |


`freeze()` automatically sets the correct wait cycles based on the system clock frequency.

#### `constrain()` vs `freeze()` Difference
```rust
// constrain() — constrain RCC, returns a configurable object
// Used for manual configuration of each peripheral clock
let mut rcc = dp.RCC.constrain();

// freeze() — configure and freeze clock in one step
// Automatically calculates all division/multiplication parameters
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,
);
```

Generally recommend using `freeze()`, it's simpler and less error-prone.

#### Advanced Usage: Directly Specify Division/Multiplication Factors
```rust
// If you need full control over clock configuration, use RawConfig
let rcc = dp.RCC.freeze(
    rcc::RawConfig {
        hse: Some(8_000_000),       // HSE frequency
        pllmul: Some(7),            // PLL multiplication factor (×9, index starts at 0)
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

// Or use freeze as well
let rcc = dp.RCC.freeze(rcc::Config::hsi(), &mut flash.acr);
```

**Suitable for:** LED blinking, button detection, GPIO testing, and other simple scenarios  
**Not suitable for:** USB, CAN, high baud rate serial

---

#### Scheme 2: HSI Multiplied to 64MHz
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hsi()
        .sysclk(64.MHz())         // HSI × 8 = 64MHz
        .pclk1(32.MHz())          // APB1 = AHB ÷ 2
        .pclk2(64.MHz())          // APB2 = AHB (no division)
        .adcclk(8.MHz()),         // ADC = APB2 ÷ 8
    &mut flash.acr,
);
// Note: HSI can only be multiplied to a maximum of 64MHz, not 72MHz
// Because HSI is divided by 2 before entering PLL: 8/2=4, 4×16=64
```

**Suitable for:** Scenarios without an external crystal but needing higher performance  
**Not suitable for:** USB (needs precise 48MHz)

---

#### Scheme 3: HSE Multiplied to 72MHz (**Recommended! DKX board preferred**)
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())     // Use DKX board's 8MHz crystal
        .sysclk(72.MHz())         // PLL: 8 × 9 = 72MHz
        .pclk1(36.MHz())          // APB1 = 72 ÷ 2 = 36MHz (maximum)
        .pclk2(72.MHz())          // APB2 = 72MHz (no division)
        .adcclk(14.MHz()),        // ADC = 72 ÷ 6 ≈ 12MHz (actual 6 division)
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
+ Flash wait cycles = 2

---

#### Scheme 4: HSE Multiplied to 48MHz (USB Dedicated)
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
+ Flash wait cycles = 1

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

**Suitable for:** Scenarios needing both maximum performance and USB

---

### 2.6 Common Clock Configuration Errors
#### Error 1: APB1 Exceeds 36MHz
```rust
// ❌ Error! APB1 max 36MHz
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(72.MHz()),  // Error! Exceeds 36MHz
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

#### Error 2: USB Clock Imprecise
```rust
// ❌ HSI not suitable for USB
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

#### Error 3: Forgetting `flash.acr`
```rust
// ❌ Missing flash.acr parameter
let rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(72.MHz()));
// Compile error! freeze requires two parameters

// ✓ Correct
let mut flash = dp.FLASH.constrain();
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,  // Must pass!
);
```

#### Error 4: SYSCLK Exceeds 72MHz
```rust
// ❌ STM32F103 max 72MHz
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(80.MHz()),  // Error!
    &mut flash.acr,
);
// freeze will automatically select a frequency close to but not exceeding 72MHz
```

#### Error 5: ADC Clock Exceeds 14MHz
```rust
// ❌ ADC clock max 14MHz
// If pclk2 = 72MHz and no division for ADC, ADC clock = 72/6 = 12MHz ✓
// If pclk2 = 72MHz and ADC has no division, ADC clock = 72MHz ✗

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

## Lighting the First LED
```rust
//! Use STM32F103C8T6 PC13 pin to blink an LED
//!
//! This example assumes the LED is connected to PC13, as on the Blue Pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to directly drive an LED.
//! See Reference Manual Section 5.1.2 for details. However, this is not an issue on the Blue Pill board.

// Prohibit unsafe code to ensure code safety
#![deny(unsafe_code)]
// Tell the Rust compiler not to use the standard library (required for embedded environments)
#![no_std]
// Tell the Rust compiler there is no traditional main function, use the entry point provided by cortex-m-rt
#![no_main]

// Import panic handler: when an unrecoverable error occurs, stop CPU execution
use panic_halt as _;

// Import non-blocking operation tools for handling async operations
use nb::block;

// Import the entry point macro from the cortex-m runtime
use cortex_m_rt::entry;
// Import HAL library core modules
// pac: Peripheral Access Crate, provides register-level access
// prelude: pre-imports commonly used traits to simplify code
// timer: timer module
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use rtt_target::{rprintln,rtt_init_print};

// Define the program entry point, replacing the standard main function
#[entry]
fn main() -> ! {

    rtt_init_print!();

    // Get Cortex-M core peripherals (e.g., SysTick timer, NVIC, etc.)
    // take() ensures these peripherals are only acquired once, preventing reuse
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get STM32F103 device-specific peripherals (GPIO, timers, serial ports, etc.)
    let dp = pac::Peripherals::take().unwrap();

    // Get and configure the Reset and Clock Controller (RCC)
    // constrain() transforms the raw RCC structure into a high-level abstraction provided by HAL
    let mut rcc = dp.RCC.constrain();

    // Get the GPIOC port and split it into individual pins
    // split() ensures unique pin ownership, preventing multiple functions from controlling the same pin
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure PC13 as push-pull output
    // crh register configures the upper 8 bits of the port (PC8-PC15)
    // For the lower 8 bits (PC0-PC7), use the crl register
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    
    // Configure the system tick timer (SysTick) as a counter triggered at a specified frequency
    // counter_hz() configures SysTick as a frequency-based counter mode
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    
    // Start the timer, set trigger frequency to 1Hz (triggered once per second)
    timer.start(4.Hz()).unwrap();

    // Main loop: wait for timer trigger and toggle LED state
    loop {
        // Block until the first timer trigger (after 1 second)
        block!(timer.wait()).unwrap();
        // Set PC13 high to turn off the LED (Blue Pill LED is active low)
        led.set_high();
        rprintln!("OPEN THE LED");
        
        // Block until the second timer trigger (after another 1 second)
        block!(timer.wait()).unwrap();
        // Set PC13 low to turn on the LED
        led.set_low();
        rprintln!("LOW THE LED");
    }
}
```

Second version -- auto-detect chip

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
    // Acquire peripheral ownership (can only be called once, subsequent calls return None)
    let p = pac::Peripherals::take().unwrap();

    // Constrain RCC (Reset and Clock Control) registers
    // constrain() returns an object containing all configurable clocks
    let mut rcc = p.RCC.constrain();

    // Split the GPIOC port into independent pin objects
    // split() returns a separate handle for each pin
    let mut gpioc = p.GPIOC.split(&mut rcc);

    // Select different pins and levels based on chip model
    cfg_select! {
        feature = "stm32f100" => {
            // STM32F100: PC9 high to turn on
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        feature = "stm32f101" => {
            // STM32F101: PC9 high to turn on
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        _ => {
            // STM32F103 (including your DKX board): PC13 low to turn on
            // PC13 on Blue Pill/DKX boards is common anode, low = on
            gpioc.pc13.into_push_pull_output(&mut gpioc.crh).set_high(); // Actually on JLCPCB, set high = on
        }
    }

    loop {} // Keep LED state unchanged
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
    let mut flash = dp.FLASH.constrain(); // Flash wait cycle configuration


    // External crystal
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Use 8MHz external crystal
            .sysclk(72.MHz()) // PLL multiply to 72MHz
            .pclk1(36.MHz()) // APB1 divide to 36MHz
            .pclk2(72.MHz()) // APB2 no division
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    // Hello World
    rprintln!("Hello World!");

    loop {}
}

// HardFault handler: called on hardware errors
// Common causes: illegal memory access, illegal instructions, stack overflow, etc.
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    // ExceptionFrame contains CPU register state at the time of the fault
    panic!("{:#?}", ef);
}

// Default exception handler: exceptions not caught by other handlers
#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    // irqn is the interrupt number, negative means system exception, positive means external interrupt
    panic!("Unhandled exception (IRQn = {})", irqn);
}
```

****

**Key Concepts:**

+ `cortex_m::Peripherals` — Cortex-M core peripherals:
    - `SYST` — SysTick timer
    - `NVIC` — Interrupt controller
    - `DCB` — Debug control block
    - `DWT` — Data watchpoint and trace unit
+ `pac::Peripherals` — Chip-specific peripherals:
    - `RCC` — Clock control
    - `GPIOA/B/C/D` — GPIO ports
    - `USART1/2/3` — Serial ports
    - `TIM1/2/3/4` — Timers
    - `SPI1/2` — SPI interfaces
    - `I2C1/2` — I2C interfaces
    - `ADC1/2` — ADC
    - `USB` — USB peripheral
    - `CAN` — CAN controller
+ `Timer::syst()` — Create timer object using SysTick timer
+ `counter_hz()` — Create a frequency counter in Hz
+ `block!()` — Convert non-blocking operation to blocking (poll until complete)
+ `1.Hz()` — Frequency unit from the fugit library

---

## LED Blinking
### SYST Mode Delay Blinking
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
    let mut flash = dp.FLASH.constrain(); // Flash wait cycle configuration

    // External crystal
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Use 8MHz external crystal
            .sysclk(72.MHz()) // PLL multiply to 72MHz
            .pclk1(36.MHz()) // APB1 divide to 36MHz
            .pclk2(72.MHz()) // APB2 no division
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        rprintln!("Set high");
        led.set_high();
        delay.delay_ms(1_800_u16);

        rprintln!("Set low");
        led.set_low();
        delay.delay(1.secs());
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779801980743-ec54e1f1-3737-4cd9-adfa-8e4a6a99ec8e.jpeg" width="281" title="" crop="0,0,1,1" id="u62526734" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779802010301-0b3dbe14-df37-444f-9f75-1f9dcb40b870.jpeg" width="280" title="" crop="0,0,1,1" id="u6a547c40" class="ne-image">

LED blinking as shown above

### LED Blinking - TIM2 Timer Delay
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
    let mut flash = dp.FLASH.constrain(); // Flash wait cycle configuration

    // External crystal
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Use 8MHz external crystal
            .sysclk(72.MHz()) // PLL multiply to 72MHz
            .pclk1(36.MHz()) // APB1 divide to 36MHz
            .pclk2(72.MHz()) // APB2 no division
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut delay = dp.TIM2.delay_us(&mut rcc); // Use TIM2

    loop {
        rprintln!("TIM2 timer");
        led.set_high();
        delay.delay_ms(1_800_u16);

        led.set_low();
        delay.delay(1.secs());
    }
}
```

**Key Concepts:**

+ `rcc::Config::hse(8.MHz())` — Use 8MHz external high-speed crystal (DKX board's crystal)
+ `.sysclk(48.MHz())` — Set system clock to 48MHz
+ `rcc.freeze()` — Freeze clock configuration, returns an immutable clock state
+ `dp.TIM2.delay_us()` — Create microsecond delay using TIM2
+ **Advantage**: More flexible and accurate than SysTick, doesn't affect SysTick's other uses



### Delay Explanation
This is the delay function. Simply put, just call delay.delay(20.millis());

+ nanos() nanoseconds
+ micros() microseconds
+ millis() milliseconds
+ secs() seconds
+ millis() milliseconds
+ minutes() minutes
+ hours() hours



## Button LED
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
    let mut flash = dp.FLASH.constrain(); // Flash wait cycle configuration

    // External crystal
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Use 8MHz external crystal
            .sysclk(72.MHz()) // PLL multiply to 72MHz
            .pclk1(36.MHz()) // APB1 divide to 36MHz
            .pclk2(72.MHz()) // APB2 no division
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

    // Disable JTAG, release PA15, PB3, PB4 as regular GPIO
    // STM32F1 defaults PA13/PA14/PA15/PB3/PB4 as JTAG/SWD pins
    // Must release before using as regular GPIO
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
                    "Button 0 pressed"
                } else {
                    "Button 1 pressed"
                }
            );
            match key_result {
                (true, _) => led1.toggle(),
                (_, true) => led2.toggle(),
                (_, _) => (),
            }
        } else if !key_result.0 && !key_result.1 {
            key_up = true;
            // nanos() nanoseconds; micros() microseconds; millis() milliseconds; secs() seconds; minutes() minutes; hours() hours
            delay.delay(20.millis());
        } else {
            // rprintln!("Error!");
            // delay.delay(2.secs());
        }
    }
}
```

****

**JTAG Pin Description:**

+ STM32F1 defaults to JTAG/SWD debug interface
+ PA13(SWDIO), PA14(SWCLK), PA15(JTDI), PB3(JTDO), PB4(JNTRST) are occupied by JTAG by default
+ If you need to use these pins as regular GPIO, you must disable JTAG first
+ Note: PA13/PA14 are SWD interface pins, generally not recommended to disable (otherwise debugging won't work)



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779809566789-02c7ee36-cc1e-4e52-82e7-860ae68fe4bb.png" width="662" title="" crop="0,0,1,1" id="u84f8089e" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809683134-a5da8944-f17a-4c76-bf5a-e18eb1ffe579.jpeg" width="223" title="" crop="0,0,1,1" id="ufb18b746" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809699056-1341bf76-a62b-49f6-ae96-d537e2c5d09c.jpeg" width="223" title="" crop="0,0,1,1" id="u774adcc9" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809732125-60f82ecc-dd74-49c0-bee5-c640bd90733b.jpeg" width="222" title="" crop="0,0,1,1" id="u1effb711" class="ne-image">



## Dynamic GPIO Switching (Port Multiplexing)
In actual development, ports may be multiplexed, requiring the following template

**Dynamic GPIO Use Cases:**

+ Some protocols (e.g., single-wire, I2C software emulation) require changing pin direction at runtime
+ Multiplexing limited pins

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
    let mut flash = dp.FLASH.constrain(); // Flash wait cycle configuration

    // External crystal
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Use 8MHz external crystal
            .sysclk(72.MHz()) // PLL multiply to 72MHz
            .pclk1(36.MHz()) // APB1 divide to 36MHz
            .pclk2(72.MHz()) // APB2 no division
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Create dynamic pin (can switch input/output mode at runtime)
    let mut pin = gpioc.pc13.into_dynamic(&mut gpioc.crh);

    let cp = cortex_m::Peripherals::take().unwrap();

    // TODO:1
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    timer.start(6.Hz()).unwrap();

    // TODO:2
    // let mut timer = cp.SYST.counter_hz(&rcc.clocks);
    // timer.start(5.Hz()).unwrap();

    // TODO:3
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



## External Interrupt EXTI
### EXTI Interrupt Line to Pin Mapping
Each EXTI line can only connect to one pin at a time, but pins with the same number (e.g., PA0, PB0, PC0) share the same interrupt line. Therefore, **you cannot use pins with the same number from different ports as interrupt sources simultaneously.**

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
Five core steps to configure external interrupts using STM32F1xx-HAL:

| Step | Operation | Code Method | Description |
| :--- | :--- | :--- | :--- |
| 1 | **Configure pin as input** | `into_pull_up_input()` etc. | Select pull-up/pull-down/floating input based on external circuit |
| 2 | **Connect to EXTI line** | `make_interrupt_source(&mut syscfg)` | Connect pin to corresponding EXTI interrupt line |
| 3 | **Set trigger edge** | `trigger_on_edge(&mut exti, Edge::RISING)` | Choose rising edge, falling edge, or both edges |
| 4 | **Enable EXTI interrupt** | `enable_interrupt(&mut exti)` | Enable the interrupt line in the EXTI peripheral |
| 5 | **NVIC unmask** | `NVIC::unmask(pac::Interrupt::EXTI0)` | Enable the corresponding interrupt channel in NVIC |


---

```rust
#![allow(clippy::empty_loop)]
// #![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use cortex_m::asm::delay; // Import instruction delay (for debouncing)
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

// Use MaybeUninit to store uninitialized global variables
static mut LED: MaybeUninit<stm32f1xx_hal::gpio::gpioc::PC13<Output>> = MaybeUninit::uninit();
// static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Input>> = MaybeUninit::uninit(); // floating
static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Input<PullUp>>> =
    MaybeUninit::uninit();

/// EXTI9_5 interrupt handler (covers EXTI7 interrupt for PA7, PB7, PC7, etc.)
#[interrupt]
fn EXTI9_5() {
    // 2021 -- Rust
    // let led = unsafe { &mut *LED.as_mut_ptr() };
    // let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };

    // 2024 -- Rust
    let led = unsafe {
        &mut *(*(&raw mut LED)).as_mut_ptr()
        // Author's explanation -- simply put, returning ownership to itself
        // &mut borrow itself
        // 2024 cannot directly &mut static mut, need &raw to dereference
        // *(&raw mut LED) <==> 2021's LED --> dereference back to LED itself
        // *(*(&raw mut LED)).as_mut_ptr() <==> *LED
        // ---------- GPT explanation as follows -------------
        // &raw mut LED
        // Get raw pointer of static mut LED
        // Type:
        // *mut MaybeUninit<PC13<Output>>

        // *(&raw mut LED)
        // Dereference the raw pointer
        // Get the memory location (place) corresponding to LED
        // Note:
        // This is NOT a "value copy"
        // It returns to the memory location of that object

        // .as_mut_ptr()
        // Convert MaybeUninit<T>
        // to *mut T

        // *ptr
        // Dereference *mut T
        // Get the memory location (place) of T

        // &mut *ptr
        // Finally create:
        // &mut T

        // Note:
        // It's still essentially a mutable reference
        // Just bypassing:
        // &mut STATIC
        // direct syntax
    };
    let int_pin = unsafe { &mut *(*(&raw mut INT_PIN)).as_mut_ptr() };

    if int_pin.check_interrupt() {
        rprintln!("Interrupt entered +1");
        // ====================== Debounce core code ======================
        delay(72_000_000 / 1000 * 40); // 72MHz clock → delay 40ms
        // ==========================================================
        led.toggle();
        int_pin.clear_interrupt_pending_bit();
    }
}

#[entry]
fn main() -> ! {
    // Initialize RTT debug output
    rtt_init_print!();
    rprintln!("Program starting...");

    let mut dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain(); // Flash wait cycle configuration

    // External crystal
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Use 8MHz external crystal
            .sysclk(72.MHz()) // PLL multiply to 72MHz
            .pclk1(36.MHz()) // APB1 divide to 36MHz
            .pclk2(72.MHz()) // APB2 no division
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    rprintln!("Starting configuration");
    // Scope -- initialize interrupt configuration
    {
        let mut gpioa = dp.GPIOA.split(&mut rcc);
        let mut gpioc = dp.GPIOC.split(&mut rcc);
        let _afio = dp.AFIO.constrain(&mut rcc);

        // LED
        let led = unsafe { &mut *(*(&raw mut LED)).as_mut_ptr() };
        *led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        // Button
        let int_pin = unsafe { &mut *(*(&raw mut INT_PIN)).as_mut_ptr() };
        // *int_pin = gpioa.pa7.into_floating_input(&mut gpioa.crl); // Floating input - minimum development board hardware circuit doesn't support, needs external capacitor + pull-up/pull-down resistor
        *int_pin = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);

        // Link to interrupt, set trigger mode: both rising and falling edges
        int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Rising); // Press to ground
        // Enable interrupt for this pin -- startup enable
        int_pin.enable_interrupt(&mut dp.EXTI);
    }

    rprintln!("Configuration complete! Setting NVIC!");
    // Unmask EXTI9_5 interrupt in NVIC
    // This step must be done after initialization is complete!
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }
    rprintln!("Setup complete!");
    loop {}
}
```

The difficulty is the syntax differences between 2021 and 2024 editions, although

```rust
#![deny(unsafe_code)] 
```

can also solve it, but my approach is another way of thinking!



Also, automatic matching of compiler syntax updates

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
        rprintln!("Interrupt entered +1");
        // ====================== Debounce core code ======================
        delay(72_000_000 / 1000 * 40); // 72MHz clock → delay 40ms
        // ==========================================================
        led.toggle();
        int_pin.clear_interrupt_pending_bit();
    }
}
```



## Timer Interrupt
### Timer Interrupt LED Blinking
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
            rprintln!("TIM2 interrupt");
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Program starting");

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

    rprintln!("GPIO initialization complete");

    let mut timer = dp.TIM2.counter_hz(&mut rcc);
    timer.start(6.Hz()).unwrap();
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| {
        *G_TIM.borrow(cs).borrow_mut() = Some(timer);
    });

    rprintln!("TIM2 initialization complete");

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    rprintln!("Program running");

    loop {
        // Wait For Interrupt, CPU enters low-power sleep -- not recommended for production!
        // wfi(); // After starting -- rprintln! output won't work
    }
}
```

Logic diagram

```plain
┌──────────────────────────────────────────────────┐
│                    main()                        │
│  1. Create LED, Timer                            │
│  2. interrupt::free → move to G_LED, G_TIM       │
│  3. NVIC::unmask → enable interrupt              │
│  4. wfi() loop sleep                             │
└──────────────────────────────────────────────────┘
                           │
                    TIM2 interrupt triggered
                           │
┌──────────────────────────────────────────────────┐
│               TIM2() interrupt handler           │
│  1. Retrieve LED/Timer from global storage       │
│  2. LED.toggle()                                 │
│  3. timer.clear_interrupt()                      │
└──────────────────────────────────────────────────┘
```



### RTIC Timer Interrupt Mode
<font style="color:#DF2A3F;"><<Development Recommended>> RTIC</font>

#### RTIC vs Bare Metal Interrupt Comparison
##### 1. Architecture Comparison
###### Bare Metal Interrupt
```rust
// Need to manually define interrupt handler, bind to interrupt vector table
#[interrupt]
fn TIM1_UP() {
    // Manually enter critical section (disable interrupts)
    cortex_m::interrupt::free(|_| {
        // All code in one function, resource management left to programmer
        // No priority management, all manual interrupt toggling
    });
}
```

###### RTIC Interrupt
```rust
// Declarative interrupt binding via #[task(binds = ...)]
#[task(binds = TIM1_UP, priority = 1, local = [led, timer])]
fn tick(cx: tick::Context) {
    // Resources managed by RTIC framework, compile-time safety
    // Priority automatically managed by RTIC scheduler
}
```

---

##### 2. Core Differences
| Feature | Bare Metal Interrupt | RTIC |
| --- | --- | --- |
| **Resource Management** | Manual critical section protection | Compile-time automatic allocation, zero runtime overhead |
| **Priority Management** | Manual NVIC register operations | `priority = N` declarative configuration |
| **Data Sharing** | Requires `static mut` + `unsafe` | `#[shared]` + `Mutex`, compile-time safe |
| **Critical Section** | Manual interrupt enable/disable | RTIC auto-generates optimal critical section |
| **Interrupt Binding** | Modify `interrupt.rs` or `device.x` | `#[task(binds = TIM1_UP)]` one line |
| **Context Switching** | Manual save/restore registers | Hardware auto-push (Cortex-M) |
| **Code Organization** | All logic crammed into one interrupt function | Each task independent, resources separated |
| **Deadlock Prevention** | Programmer must be careful | Compile-time detection (priority ceiling protocol) |


---

##### 3. Resource Management Comparison
###### Bare Metal: `static mut` + `unsafe`
```rust
// Global mutable static variable, requires unsafe access
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

+ Bugs inside `unsafe` blocks won't be caught by the compiler
+ Multiple interrupts accessing the same variable can cause data races
+ High priority interrupts may preempt low priority ones, corrupting data consistency

###### RTIC: `#[local]` compile-time binding
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
+ Access resources with just `cx.local.xxx`

---

##### 4. Priority and Scheduling Comparison
###### Bare Metal: Manual NVIC operations
```rust
// Need to manually configure priority
fn setup_timer_interrupt() {
    unsafe {
        // Set TIM1 interrupt priority to 1
        // Need to know NVIC register address and bit fields
        let nvic = &*cortex_m::peripheral::NVIC::ptr();
        // Complex register operations...
    }
}
```

###### RTIC: Declarative Priority
```rust
// priority = 1 is all it takes
#[task(binds = TIM1_UP, priority = 1)]
fn tick(cx: tick::Context) { ... }

// High priority task can preempt low priority
#[task(binds = USART1, priority = 2)]
fn serial(cx: serial::Context) { ... }
```

**RTIC Priority Rules:**

+ Higher number = higher priority
+ High priority tasks can preempt low priority tasks
+ Same priority tasks cannot preempt each other

---

##### 5. Shared Resource Comparison
###### Bare Metal: Manual Critical Section
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
    // Forgot critical section? Data race! Compiler won't catch it!
    unsafe { SHARED_DATA += 1; }
}
```

###### RTIC: `#[shared]` + Auto Lock
```rust
#[shared]
struct Shared {
    data: u32,
}

#[task(binds = TIM1_UP, priority = 1, shared = [data])]
fn tick(cx: tick::Context) {
    // lock() automatically manages critical sections, compile-time safe
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}

#[task(binds = USART1, priority = 2, shared = [data])]
fn serial(cx: serial::Context) {
    // High priority task accessing same resource, RTIC auto-generates optimal lock
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}
```

**RTIC Lock Mechanism (Priority Ceiling Protocol PCP):**

+ When a low-priority task holds a lock, high-priority tasks wait
+ But RTIC automatically boosts the priority of the lock-holding task to prevent medium-priority tasks from cutting in line
+ All of this is determined at compile time, zero runtime overhead

---

6. Summary

| Scenario | Recommended Approach |
| --- | --- |
| Learning interrupt principles | Bare Metal (understand underlying mechanism) |
| Formal project development | RTIC (safe, efficient, maintainable) |
| Single simple interrupt | Bare Metal (similar amount of code) |
| Multiple interrupts + shared resources | RTIC (avoid data races) |
| Strict real-time requirements | RTIC (zero-cost abstraction, deterministic scheduling) |


**RTIC Core Advantages:**

+ **Zero-cost abstraction**: Everything determined at compile time, no runtime overhead
+ **Compile-time safety**: Data races, deadlocks caught at compile time
+ **Declarative programming**: Describe "what to do" with attribute macros, framework generates "how to do it"
+ **Priority Ceiling Protocol**: Optimal critical section management

<font style="color:#DF2A3F;"></font>

#### How to Use RTIC
Need to modify config.toml

```bash
# Package addition
[dependencies] 
....
rtic = { version = "2", features = ["thumbv7-backend"] }
```

**Reason:**

+ Original project missing `rtic` dependency, causing inability to find the `rtic` module
+ RTIC v2 requires specifying the backend feature. STM32F103 is Cortex-M3 architecture, using `thumbv7-backend`
+ Other optional backends: `thumbv6-backend` (Cortex-M0/M0+), `thumbv8base-backend` (Cortex-M23), `thumbv8main-backend` (Cortex-M33)



```rust
//! Use timer interrupt to blink LED at different frequencies
//!
//! Assume LED is connected to PC13 (Blue Pill board default configuration)
//!
//! Note: Without additional hardware, it's not recommended to use PC13 to directly drive an LED
//! (see Reference Manual Section 5.1.2 for details)
//! But the Blue Pill board already has an onboard LED, so it's fine

#![no_std]
#![no_main]

// Import panic handler, CPU will stop on panic
// Can set a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;

// ==================== RTIC Application Entry ====================
// #[rtic::app] is the core macro of the RTIC framework, defining a real-time interrupt-driven application
// device parameter specifies the chip peripheral crate (PAC), using the PAC provided by stm32f1xx_hal
#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    // Import RTT (Real-Time Transfer) debug output macros
    // rtt_init_print! initializes the RTT output channel
    // rprintln! prints debug information via RTT (needs J-Link/ST-Link debugger)
    use rtt_target::{rprintln, rtt_init_print};

    use stm32f1xx_hal::{
        // GPIO-related types: PC13 pin, output mode, pin state, push-pull output
        gpio::{gpioc::PC13, Output, PinState, PushPull},
        // PAC (Peripheral Access Crate): low-level interface for direct hardware register access
        pac,
        // prelude: pre-imports commonly used traits (e.g., timer's .counter_ms() method)
        prelude::*,
        // Timer-related types: CounterMs is millisecond-precision timer, Event is timer event enum
        timer::{CounterMs, Event},
    };

    // ==================== Shared Resources ====================
    // #[shared] annotated struct defines resources that can be shared across tasks
    // This example has no shared resources, so the struct is empty
    #[shared]
    struct Shared {}

    // ==================== Local Resources ====================
    // #[local] annotated struct defines resources that can only be accessed by a single task
    // Each resource is bound to a specific task at compile time, avoiding runtime lock overhead
    #[local]
    struct Local {
        // LED pin (PC13, push-pull output mode)
        led: PC13<Output<PushPull>>,
        // Timer handle (TIM1, millisecond precision)
        timer_handler: CounterMs<pac::TIM1>,
    }

    // ==================== Init Function ====================
    // #[init] annotated function executes once at system startup for hardware and resource initialization
    // Returns (Shared Resources, Local Resources) tuple
    // cx is the RTIC Context, access chip peripherals via cx.device
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Initialize RTT debug output channel
        // After this, rprintln! can be used to print debug info
        rtt_init_print!();
        rprintln!("Program starting: init function begins execution");

        // Get RCC (Reset and Clock Control) peripheral and constrain it
        // constrain() configures RCC to default state, returns a clock configuration object
        let mut rcc = cx.device.RCC.constrain();
        rprintln!("RCC clock configuration complete");

        // Get GPIOC peripheral and split it into individual pins
        // split() encapsulates all GPIOC pins as independent Pin objects
        let mut gpioc = cx.device.GPIOC.split(&mut rcc);

        // Configure PC13 as push-pull output, initial state high (LED off)
        // crh register configures pins 8-15 (pins 0-7 use crl register)
        // PushPull: can actively output high or low level
        // PinState::High: initial output high (Blue Pill LED is active low)
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High);
        rprintln!("PC13 LED pin configuration complete");

        // Configure TIM1 timer as millisecond-precision counter
        // counter_ms() configures TIM1 as a millisecond-precision timer
        let mut timer = cx.device.TIM1.counter_ms(&mut rcc);
        // Start the timer, trigger update event every 1 second
        timer.start(1.secs()).unwrap();
        // Enable timer update interrupt (Update Event)
        // Interrupt triggers when timer count overflows
        timer.listen(Event::Update);
        rprintln!("TIM1 timer configuration complete, interrupt every 1 second");

        // Return initialized resources
        // Shared {}: shared resources (empty in this example)
        // Local { led, timer_handler }: local resources, bound to their respective tasks
        (
            Shared {},
            Local {
                led,
                timer_handler: timer,
            },
        )
    }

    // ==================== Idle Function ====================
    // #[idle] annotated function runs continuously when the system is idle (no tasks to execute)
    // Return type `!` means it never returns (infinite loop)
    // Reference: https://rtic.rs/dev/book/en/by-example/app_idle.html
    // If idle function is not declared, RTIC automatically sets the SLEEPONEXIT bit to put CPU to sleep
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Entering idle loop, CPU waiting for interrupt...");
        loop {
            // WFI (Wait For Interrupt): put CPU into low-power wait state
            // CPU automatically wakes on interrupt, returns here after interrupt processing
            //
            // Note: enabling wfi() may prevent RTT debug output from refreshing properly
            // If you need to see rprintln! output in idle, comment out wfi()
            // cortex_m::asm::dsb();
            // cortex_m::asm::wfi();
        }
    }

    // ==================== Timer Interrupt Task ====================
    // #[task] defines a task, bound to a specific hardware interrupt via the binds parameter
    // binds = TIM1_UP: bound to TIM1 update interrupt (Update Interrupt)
    // priority = 1: task priority is 1 (higher number = higher priority)
    // local = [...]: declare local resource list, including:
    //   - led: LED pin
    //   - timer_handler: timer handle
    //   - led_state: bool = false: LED state (initial false = LED off)
    //   - count: u8 = 0: interrupt counter (initial 0)
    #[task(binds = TIM1_UP, priority = 1, local = [led, timer_handler, led_state: bool = false, count: u8 = 0])]
    fn tick(cx: tick::Context) {
        // Toggle LED state
        // If LED is currently on (led_state == true), turn it off
        // If LED is currently off (led_state == false), turn it on
        if *cx.local.led_state {
            // set_high(): output high (Blue Pill LED is active low, high = off)
            cx.local.led.set_high();
            *cx.local.led_state = false;
            rprintln!("[Interrupt] LED off");
        } else {
            // set_low(): output low (turn on LED)
            cx.local.led.set_low();
            *cx.local.led_state = true;
            rprintln!("[Interrupt] LED on");
        }

        // Increment interrupt counter
        // Used to control timer frequency switching timing
        *cx.local.count += 1;
        rprintln!("[Interrupt] Count: {}", *cx.local.count);

        // Dynamically change timer trigger frequency
        // 4th interrupt: change timer to 500ms (LED blink faster)
        if *cx.local.count == 4 {
            cx.local.timer_handler.start(500.millis()).unwrap();
            rprintln!("[Interrupt] Timer switched to 500ms");
        }
        // 12th interrupt: change timer back to 1 second (LED blink returns to slow)
        // and reset counter to start new cycle
        else if *cx.local.count == 12 {
            cx.local.timer_handler.start(1.secs()).unwrap();
            *cx.local.count = 0;
            rprintln!("[Interrupt] Timer switched to 1s, counter reset");
        }

        // Clear timer update interrupt flag
        // Must clear manually, otherwise interrupt will keep triggering
        cx.local.timer_handler.clear_interrupt(Event::Update);
    }
}
```

Output:

```bash
FLASH] Programming via probe-rs...

      Erasing ✔ 100% [####################]   5.00 KiB @   6.69 KiB/s (took 1s)
  Programming ✔ 100% [####################]   5.00 KiB @   4.70 KiB/s (took 1s)                                                   Finished in 1.93s
Program starting: init function begins execution
12:16:00.898: RCC clock configuration complete
12:16:00.898: PC13 LED pin configuration complete
12:16:00.898: TIM1 timer configuration complete, interrupt every 1 second
12:16:00.898: Entering idle loop, CPU waiting for interrupt...
12:16:01.855: [Interrupt] LED on
12:16:01.855: [Interrupt] Count: 1
12:16:02.932: [Interrupt] LED off
12:16:02.932: [Interrupt] Count: 2
12:16:03.896: [Interrupt] LED on
12:16:03.896: [Interrupt] Count: 3
12:16:04.849: [Interrupt] LED off
12:16:04.849: [Interrupt] Count: 4
12:16:04.849: [Interrupt] Timer switched to 500ms
12:16:05.333: [Interrupt] LED on
12:16:05.333: [Interrupt] Count: 5
12:16:05.934: [Interrupt] LED off
12:16:05.934: [Interrupt] Count: 6
12:16:06.426: [Interrupt] LED on
12:16:06.426: [Interrupt] Count: 7
12:16:06.914: [Interrupt] LED off
12:16:06.914: [Interrupt] Count: 8
12:16:07.401: [Interrupt] LED on
12:16:07.401: [Interrupt] Count: 9
12:16:07.888: [Interrupt] LED off
12:16:07.888: [Interrupt] Count: 10
12:16:08.377: [Interrupt] LED on
12:16:08.377: [Interrupt] Count: 11
12:16:08.874: [Interrupt] LED off
12:16:08.874: [Interrupt] Count: 12
12:16:08.874: [Interrupt] Timer switched to 1s, counter reset
12:16:09.830: [Interrupt] LED on
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



```rust
//! Use timer interrupt to blink LED at different frequencies
//!
//! Assume LED is connected to PC13 (Blue Pill board default configuration)
//!
//! Note: Without additional hardware, it's not recommended to use PC13 to directly drive an LED
//! (see Reference Manual Section 5.1.2 for details)
//! But the Blue Pill board already has an onboard LED, so it's fine

#![no_std]
#![no_main]

// Import panic handler, CPU will stop on panic
// Can set a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;

// ==================== RTIC Application Entry ====================
// #[rtic::app] is the core macro of the RTIC framework, defining a real-time interrupt-driven application
// device parameter specifies the chip peripheral crate (PAC), using the PAC provided by stm32f1xx_hal
#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    // Import semihosting debug output macros
    // hprintln! outputs through SWD debug interface to OpenOCD/ST-Link terminal
    // Compared to RTT, no additional configuration needed, but slower
    use cortex_m_semihosting::hprintln;

    use stm32f1xx_hal::{
        // GPIO-related types: PC13 pin, output mode, pin state, push-pull output
        gpio::{gpioc::PC13, Output, PinState, PushPull},
        // PAC (Peripheral Access Crate): low-level interface for direct hardware register access
        pac,
        // prelude: pre-imports commonly used traits (e.g., timer's .counter_ms() method)
        prelude::*,
        // Timer-related types: CounterMs is millisecond-precision timer, Event is timer event enum
        timer::{CounterMs, Event},
    };

    // ==================== Shared Resources ====================
    // #[shared] annotated struct defines resources that can be shared across tasks
    // This example has no shared resources, so the struct is empty
    #[shared]
    struct Shared {}

    // ==================== Local Resources ====================
    // #[local] annotated struct defines resources that can only be accessed by a single task
    // Each resource is bound to a specific task at compile time, avoiding runtime lock overhead
    #[local]
    struct Local {
        // LED pin (PC13, push-pull output mode)
        led: PC13<Output<PushPull>>,
        // Timer handle (TIM1, millisecond precision)
        timer_handler: CounterMs<pac::TIM1>,
    }

    // ==================== Init Function ====================
    // #[init] annotated function executes once at system startup for hardware and resource initialization
    // Returns (Shared Resources, Local Resources) tuple
    // cx is the RTIC Context, access chip peripherals via cx.device
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        hprintln!("Program starting: init function begins execution");

        // Get RCC (Reset and Clock Control) peripheral and constrain it
        // constrain() configures RCC to default state, returns a clock configuration object
        let mut rcc = cx.device.RCC.constrain();
        hprintln!("RCC clock configuration complete");

        // Get GPIOC peripheral and split it into individual pins
        // split() encapsulates all GPIOC pins as independent Pin objects
        let mut gpioc = cx.device.GPIOC.split(&mut rcc);

        // Configure PC13 as push-pull output, initial state high (LED off)
        // crh register configures pins 8-15 (pins 0-7 use crl register)
        // PushPull: can actively output high or low level
        // PinState::High: initial output high (Blue Pill LED is active low)
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High);
        hprintln!("PC13 LED pin configuration complete");

        // Configure TIM1 timer as millisecond-precision counter
        // counter_ms() configures TIM1 as a millisecond-precision timer
        let mut timer = cx.device.TIM1.counter_ms(&mut rcc);
        // Start the timer, trigger update event every 1 second
        timer.start(1.secs()).unwrap();
        // Enable timer update interrupt (Update Event)
        // Interrupt triggers when timer count overflows
        timer.listen(Event::Update);
        hprintln!("TIM1 timer configuration complete, interrupt every 1 second");

        // Return initialized resources
        // Shared {}: shared resources (empty in this example)
        // Local { led, timer_handler }: local resources, bound to their respective tasks
        (
            Shared {},
            Local {
                led,
                timer_handler: timer,
            },
        )
    }

    // ==================== Idle Function ====================
    // #[idle] annotated function runs continuously when the system is idle (no tasks to execute)
    // Return type `!` means it never returns (infinite loop)
    // Reference: https://rtic.rs/dev/book/en/by-example/app_idle.html
    // If idle function is not declared, RTIC automatically sets the SLEEPONEXIT bit to put CPU to sleep
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        hprintln!("Entering idle loop, CPU waiting for interrupt...");
        loop {
            // WFI (Wait For Interrupt): put CPU into low-power wait state
            // CPU automatically wakes on interrupt, returns here after interrupt processing
            // cortex_m::asm::dsb();
            cortex_m::asm::wfi();
        }
    }

    // ==================== Timer Interrupt Task ====================
    // #[task] defines a task, bound to a specific hardware interrupt via the binds parameter
    // binds = TIM1_UP: bound to TIM1 update interrupt (Update Interrupt)
    // priority = 1: task priority is 1 (higher number = higher priority)
    // local = [...]: declare local resource list, including:
    //   - led: LED pin
    //   - timer_handler: timer handle
    //   - led_state: bool = false: LED state (initial false = LED off)
    //   - count: u8 = 0: interrupt counter (initial 0)
    #[task(binds = TIM1_UP, priority = 1, local = [led, timer_handler, led_state: bool = false, count: u8 = 0])]
    fn tick(cx: tick::Context) {
        // Toggle LED state
        // If LED is currently on (led_state == true), turn it off
        // If LED is currently off (led_state == false), turn it on
        if *cx.local.led_state {
            // set_high(): output high (Blue Pill LED is active low, high = off)
            cx.local.led.set_high();
            *cx.local.led_state = false;
            hprintln!("[Interrupt] LED off");
        } else {
            // set_low(): output low (turn on LED)
            cx.local.led.set_low();
            *cx.local.led_state = true;
            hprintln!("[Interrupt] LED on");
        }

        // Increment interrupt counter
        // Used to control timer frequency switching timing
        *cx.local.count += 1;
        hprintln!("[Interrupt] Count: {}", *cx.local.count);

        // Dynamically change timer trigger frequency
        // 4th interrupt: change timer to 500ms (LED blink faster)
        if *cx.local.count == 4 {
            cx.local.timer_handler.start(500.millis()).unwrap();
            hprintln!("[Interrupt] Timer switched to 500ms");
        }
        // 12th interrupt: change timer back to 1 second (LED blink returns to slow)
        // and reset counter to start new cycle
        else if *cx.local.count == 12 {
            cx.local.timer_handler.start(1.secs()).unwrap();
            *cx.local.count = 0;
            hprintln!("[Interrupt] Timer switched to 1s, counter reset");
        }

        // Clear timer update interrupt flag
        // Must clear manually, otherwise interrupt will keep triggering
        cx.local.timer_handler.clear_interrupt(Event::Update);
    }
}
```



Output

```rust
     Finished in 1.39s
Program starting: init function begins execution
RCC clock configuration complete
PC13 LED pin configuration complete
TIM1 timer configuration complete, interrupt every 1 second
Entering idle loop, CPU waiting for interrupt...
[Interrupt] LED on
[Interrupt] Count: 1
[Interrupt] LED off
[Interrupt] Count: 2
[Interrupt] LED on
[Interrupt] Count: 3
```



## RTIC2 Async Tasks
Using RTIC to implement scheduling of 2 tasks in async mode

```rust
//! RTIC2 async task example — two async tasks running concurrently
//!
//! Features:
//!   - blink task: toggle PC13 onboard LED every 500ms
//!   - heartbeat task: print heartbeat via RTT every 2 seconds
//!
//! The two tasks alternate execution without blocking each other, demonstrating the core value of async:
//! With synchronous blocking (e.g., `block!(...)`), the heartbeat's 2-second wait would block blink.
//! But with async, CPU is automatically yielded during wait, allowing the scheduler to run other tasks.
//!
//! Hardware: Blue Pill (STM32F103C8T6), 8MHz external crystal, PC13 LED

// ==================== Rust embedded basic attributes ====================

// Embedded programs don't have standard library main entry
// Program entry is generated by the #[rtic::app] macro from the RTIC framework
#![no_main]

// Embedded environments have no OS, don't use standard library (std)
// Only use core library (core), such as Option, Result, loops, arithmetic, etc.
#![no_std]

// ==================== Dependency crate imports ====================

// panic_halt: panic handler
// When an unrecoverable error occurs, directly halt the CPU
// `use ... as _` means only import side effects (panic handler), don't use the specific type
use panic_halt as _;

// rtic_monotonics::systick::prelude imports:
//   - systick_monotonic! macro: create a SysTick-based monotonic timer type
//   - Monotonic trait: interface that a timer must implement
//   - fugit::ExtU32: add .millis(), .secs() etc. time unit methods to u32
use rtic_monotonics::systick::prelude::*;

// ==================== Monotonic Timer Configuration ====================

// The systick_monotonic! macro expands to generate a struct named Mono
// Internally:
//   1. Defines the SysTick interrupt handler (extern "C" fn SysTick())
//   2. Implements the Monotonic trait, providing delay, now, and other methods
//
// Parameter 1_000 means tick frequency = 1000Hz (i.e., SysTick interrupt every 1ms)
// Mono::start() also needs the system clock frequency to calculate the reload value:
//   reload = sysclk / tick_rate - 1 = 72_000_000 / 1_000 - 1 = 71_999
//   SysTick triggers an interrupt every 72_000 clock cycles (1ms)
//
// Note: the first parameter is tick frequency (Hz), NOT system clock!
//   systick_monotonic!(Mono, 1_000)    → 1kHz, 1ms precision ✓
//   systick_monotonic!(Mono, 48_000_000) → 48MHz, 48 million interrupts per second ✗ (too frequent!)
systick_monotonic!(Mono, 1_000);

// ==================== RTIC Application Definition ====================

// #[rtic::app] is the core macro of the RTIC framework, defining a real-time application
//   device = stm32f1xx_hal::pac: specifies the chip's PAC (Peripheral Access Crate)
//     PAC provides low-level access to all hardware registers
//   dispatchers = [USART1]: specifies the dispatch interrupt for software tasks
//     RTIC borrows USART1's interrupt vector to run the async task scheduler
//     USART1 is chosen simply because we don't use it — any unused interrupt works
#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [USART1])]
mod app {
    // RTT (Real-Time Transfer) debug output
    // Zero-intrusion real-time printing via J-Link/ST-Link debugger
    //   rtt_init_print!(): initialize RTT up-channel (only needs to be called once)
    //   rprintln!(): print a line via RTT (similar to println!, but no OS needed)
    use rtt_target::{rprintln, rtt_init_print};

    // Import Mono timer and panic_halt from the parent module (outside mod app)
    use super::*;

    // Hardware abstraction layer types from stm32f1xx-hal
    use stm32f1xx_hal::{
        gpio::{Output, PC13},  // PC13 pin output mode type
        prelude::*,             // Pre-import traits, unlock .counter_ms(), .MHz() and other methods
        rcc::Config,            // Clock configuration struct (HSE/HSI/PLL selection)
    };

    // ==================== Shared Resources ====================
    // #[shared] defines resources accessible by multiple tasks
    // RTIC guarantees safe access to shared resources at compile time (priority-based lock-free mutex)
    // This example has no shared resources, so struct is empty
    #[shared]
    struct Shared {}

    // ==================== Local Resources ====================
    // #[local] defines resources that can only be accessed by a single task
    // Each resource is bound to a specific task at compile time, completely avoiding runtime overhead
    #[local]
    struct Local {
        // PC13 pin, push-pull output mode (drives LED)
        led: PC13<Output>,
    }

    // ==================== Init Function ====================
    // #[init] annotated function executes once (and only once) at system startup
    // It runs before all interrupts are enabled, return value is assigned to shared and local resources
    // ctx is the RTIC context object:
    //   ctx.device → PAC peripherals (FLASH, RCC, GPIO registers, etc.)
    //   ctx.core   → Cortex-M core peripherals (SYST, NVIC, etc.)
    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        // Initialize RTT debug channel, rprintln! works after this
        rtt_init_print!();
        rprintln!("Start");

        // ==================== Clock System Configuration ====================
        // STM32F103 clock tree:
        //   HSE (8MHz external crystal) → PLL ×9 multiply → SYSCLK = 72MHz
        //                        ├→ AHB  → APB1 (72÷2 = 36MHz, low-speed peripherals)
        //                        └→ AHB  → APB2 (72MHz, high-speed peripherals)
        //
        // constrain() wraps peripheral registers into safe Rust types
        // freeze() locks clock configuration, cannot be modified afterwards
        let mut flash = ctx.device.FLASH.constrain();
        let mut rcc = ctx.device.RCC.freeze(
            Config::hse(8.MHz())  // Use 8MHz external crystal (HSE)
                .sysclk(72.MHz()), // PLL multiply to 72MHz (STM32F103 max frequency)
            &mut flash.acr,       // Flash access control register (needs wait cycle configuration)
        );

        // Start the SysTick monotonic timer
        // First parameter: SysTick peripheral (Cortex-M core's built-in 24-bit down counter)
        // Second parameter: system clock frequency (72MHz), used to calculate each tick time
        // After starting, SysTick triggers an interrupt every 1ms (determined by systick_monotonic!'s 1_000Hz)
        Mono::start(ctx.core.SYST, 72_000_000);

        // ==================== GPIO Configuration ====================
        // split() splits the GPIOC peripheral into independent pin objects
        // into_push_pull_output() configures PC13 as push-pull output mode
        //   Push-pull output: can actively output high (3.3V) or low (0V)
        //   Blue Pill onboard LED is active low
        let mut gpioc = ctx.device.GPIOC.split(&mut rcc);
        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        // ==================== Start Async Tasks ====================
        // spawn() submits async tasks to the RTIC scheduler
        // Tasks won't execute immediately, but are scheduled by the scheduler after init returns
        // .ok() ignores possible errors (e.g., task pool full)
        blink::spawn().ok();
        heartbeat::spawn().ok();

        rprintln!("Startup complete");

        // Return resources: shared resources (empty) and local resources (LED pin)
        // RTIC will assign the led to the task that declared it in its local
        (Shared {}, Local { led })
    }

    // ==================== Async Task 1: LED Blink ====================
    // #[task] defines an async task
    //   local = [led, count: u32 = 0]:
    //     - led: acquired from the Local returned by init
    //     - count: u32 = 0: declares a task-private counter, initial value 0
    //         This inline initialization syntax doesn't need declaration in struct Local
    //
    // async fn means this function can pause at .await, yielding CPU
    // RTIC scheduler automatically resumes execution when the delay expires
    #[task(local = [led, count: u32 = 0])]
    async fn blink(ctx: blink::Context) {
        loop {
            // toggle(): flip pin level (high→low or low→high)
            ctx.local.led.toggle();

            // ctx.local.count is task-private state, no lock needed
            *ctx.local.count += 1;
            rprintln!("[blink] count={}", *ctx.local.count);

            // .delay(500.millis()).await: async wait 500 milliseconds
            // Key: this does NOT block the CPU!
            //   1. Mono timer records wake-up time point
            //   2. Current task yields CPU (state saved in task's Future)
            //   3. RTIC scheduler runs other ready tasks (e.g., heartbeat)
            //   4. After 500ms, SysTick interrupt wakes this task, continues execution
            Mono::delay(500.millis()).await;
        }
    }

    // ==================== Async Task 2: Heartbeat Print ====================
    // Another independent async task, running concurrently with blink
    // local = [beat: u32 = 0]: task-private heartbeat counter
    #[task(local = [beat: u32 = 0])]
    async fn heartbeat(ctx: heartbeat::Context) {
        loop {
            *ctx.local.beat += 1;
            rprintln!("[heartbeat] beat={}", *ctx.local.beat);

            // Wait 2 seconds, during which blink task runs normally
            // If this were synchronous blocking (e.g., for-loop idling 2 seconds),
            // the entire system would be unresponsive to any other task during those 2 seconds
            // But async .await just "sleeps for a while", other tasks are unaffected
            Mono::delay(2.secs()).await;
        }
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045924896-cc601051-f619-4596-87f6-9a2246f5de5c.png" width="581" title="" crop="0,0,1,1" id="ud3c4bf51" class="ne-image">

As shown above, 4 toggles execute one heartbeat! Running simultaneously!




## Serial Communication
#### Normal Mode
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
    rprintln!("Serial test starting");
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

    // === USART3 Pin Configuration (DKX Board) ===
    // TX: PB10 configured as alternate push-pull output
    // Alternate push-pull output = GPIO controlled by hardware peripheral, not software
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // RX: PB11 defaults to floating input
    let rx = gpiob.pb11;

    // Create serial instance
    // USART3, baud rate 115200
    let mut serial = dp
        .USART3
        .serial((tx, rx), Config::default().baudrate(115200.bps()), &mut rcc);

    // === Method 1: Use serial object directly for read/write ===
    // let sent = b'X';
    // block!(serial.tx.write_u8(sent)).unwrap(); // send byte
    // let received = block!(serial.rx.read()).unwrap(); // receive byte
    // assert_eq!(received, sent); // verify
    // rprintln!("{}",received);
    // asm::bkpt(); // breakpoint, check with debugger

    // === Method 2: Split into separate TX/RX ===
    let sent = b'Y';
    let (mut tx, mut rx) = serial.split();
    block!(tx.write_u8(sent)).unwrap();
    // let received = block!(rx.read()).unwrap();
    // block!(tx.write_u8(received)).unwrap(); // echo
    // asm::bkpt();

    // === Method 3: Read/write with split TX/RX ===
    // stm32f1xx_hal's Tx/Rx does not support reunite,
    // After split, can independently use tx.write_u8() and rx.read()
    // let sent = b'Z';
    // let (mut tx, mut rx) = serial.split();
    // block!(tx.write_u8(sent)).unwrap();


    loop {
        // Method 1
        // let received = block!(serial.rx.read()).unwrap(); // receive byte
        // rprintln!("{}", received as char);
        // block!(serial.tx.write_u8(received)).unwrap(); // echo

        // Method 2
        let received = block!(rx.read()).unwrap();
        block!(tx.write_u8(received)).unwrap(); // echo
        rprintln!("{}",received as char);

        // Method 3
        // let received = block!(rx.read()).unwrap();
        // assert_eq!(received, sent);
        // block!(tx.write_u8(received)).unwrap(); // echo
        // rprintln!("{}",received as char);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045838370-24ec16fe-462e-4267-92b7-5ef5b4c3f955.png" width="1451" title="" crop="0,0,1,1" id="u5f48f617" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045850995-183a5b47-560b-4c32-96d1-59dd0bc6484e.png" width="372" title="" crop="0,0,1,1" id="ud58583c3" class="ne-image">

**Available pins per USART (F103):**

| USART | TX Pin | RX Pin | Notes |
| --- | --- | --- | --- |
| USART1 | PA9 or PB6(remap) | PA10 or PB7(remap) | APB2 |
| USART2 | PA2 | PA3 | APB1 |
| USART3 | PB10 | PB11 | APB1 |


**Key Concepts:**

+ `into_alternate_push_pull()` — alternate push-pull output, pin controlled by hardware peripheral
+ `Config::default().baudrate()` — serial configuration (baud rate, data bits, stop bits, etc.)
+ `.split()` — split into separate `Tx` and `Rx` objects
+ `.reunite()` — re-merge `Tx` and `Rx`



#### Serial Communication - fmt Mode
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
use core::fmt::Write;  // Import Write trait

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Serial test starting");
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
    // Use write! macro for formatted output
    writeln!(tx, "Hello formatted string {}", number).unwrap();
    // Windows newline: write!(tx, "Hello formatted string {}\r\n", number)


    let mut delay = dp.TIM2.delay_us(&mut rcc); // Using TIM2

    loop {
        writeln!(tx, "Hello formatted string {}", number).unwrap();
        delay.delay_ms(2_000_u16);
        number += 1;
        rprintln!("Debug:Hello formatted string {}",number);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047558289-b27d36c0-42f4-4c70-b44b-6278e05bc28c.png" width="492" title="" crop="0,0,1,1" id="u784935b0" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047584627-e3e1ce36-b097-4508-959a-6fa5562f7ca5.png" width="624" title="" crop="0,0,1,1" id="uc9269666" class="ne-image">

The above information is quite good in my opinion. The advantage of this language is that once the framework is built, AI can complete the task!



#### Serial Interrupt - IDLE Detection
Mainly using IDLE mode

```rust
// USART3 Interrupt + IDLE Line Detection — Receive variable-length data and echo back
//
// Principle:
//   1. Each received byte triggers RXNE interrupt, stored in BUFFER
//   2. Bus idle (no new bytes) triggers IDLE interrupt, indicating "end of frame"
//   3. On IDLE, echo back the entire frame via TX
//
// Use Mutex<RefCell<>> instead of static mut, compatible with Rust 2024 edition

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

// Global shared state: wrapped in Mutex<RefCell<>>, safely shared between interrupt and main
static RX: Mutex<RefCell<Option<Rx<USART3>>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Tx<USART3>>>> = Mutex::new(RefCell::new(None));

const BUFFER_LEN: usize = 4096;
static BUFFER: Mutex<RefCell<[u8; BUFFER_LEN]>> = Mutex::new(RefCell::new([0; BUFFER_LEN]));
static WIDX: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Serial_Interrupt_IDLE");

    let p = pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut afio = p.AFIO.constrain(&mut rcc);
    let mut gpiob = p.GPIOB.split(&mut rcc);

    // USART3 Pins: PB10(TX), PB11(RX)
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    // Initialize USART3, baud rate 115200, split into separate TX/RX
    let (mut tx, mut rx) = p
        .USART3
        .remap(&mut afio.mapr)
        .serial((tx, rx), 115_200.bps(), &mut rcc)
        .split();

    // Enable three interrupt sources
    tx.listen();       // TXE — Transmit register empty interrupt (unused in this example, keep default enabled)
    rx.listen();       // RXNE — Receive register not empty interrupt (triggers on each byte received)
    rx.listen_idle();  // IDLE — Bus idle detection interrupt (triggers when a frame is complete)

    // Store TX/RX into global static variables inside critical section for use by ISR
    cortex_m::interrupt::free(|cs| {
        TX.borrow(cs).replace(Some(tx));
        RX.borrow(cs).replace(Some(rx));
    });

    // Enable USART3 interrupt in NVIC (NVIC::unmask is the only unsafe call in cortex-m)
    #[allow(unsafe_code)]
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART3);
    }

    // Main loop: WFI sleep, wake on interrupt
    loop {
        cortex_m::asm::wfi()
    }
}

/// Send all bytes in buf via TX (blocking, byte by byte)
fn write(cs: &cortex_m::interrupt::CriticalSection, buf: &[u8]) {
    let mut tx_ref = TX.borrow(cs).borrow_mut();
    if let Some(tx) = tx_ref.as_mut() {
        buf.iter()
            .for_each(|w| if let Err(_err) = nb::block!(tx.write(*w)) {})
    }
}

/// USART3 Interrupt Handler
///
/// Two interrupt sources share the same entry point, distinguished by flags:
///   - RXNE (Receive Not Empty): read byte by byte and store in BUFFER
///   - IDLE (Bus Idle): end of frame, echo back all received data
#[interrupt]
fn USART3() {
    cortex_m::interrupt::free(|cs| {
        let mut rx_ref = RX.borrow(cs).borrow_mut();
        if let Some(rx) = rx_ref.as_mut() {
            if rx.is_rx_not_empty() {
                // RXNE: received 1 byte, store in buffer
                if let Ok(w) = nb::block!(rx.read()) {
                    let widx = *WIDX.borrow(cs).borrow();
                    BUFFER.borrow(cs).borrow_mut()[widx] = w;
                    let new_widx = widx + 1;
                    if new_widx >= BUFFER_LEN - 1 {
                        // Buffer full: immediately echo back entire block, reset write pointer
                        let buf = BUFFER.borrow(cs).borrow();
                        write(cs, &buf[..new_widx]);
                        drop(buf);
                        *WIDX.borrow(cs).borrow_mut() = 0;
                    } else {
                        *WIDX.borrow(cs).borrow_mut() = new_widx;
                    }
                }
                rx.listen_idle(); // Re-enable IDLE detection after each RXNE
            } else if rx.is_idle() {
                // IDLE: Bus idle → frame received, echo back and clear buffer
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



**9-bit data mode, using the 9th bit to mark address/data.**

```rust
// Configure as 9-bit data
let serial = p.USART3.serial::<PushPull>(
    (tx_pin, rx_pin),
    Config::default()
        .baudrate(9600.bps())
        .wordlength_9bits()    // 9-bit data
        .parity_none(),        // no parity
    &mut rcc,
);

// 9th bit = 1 indicates address byte
// 9th bit = 0 indicates data byte
block!(serial_tx.write(SLAVE_ADDR as u16 | 0x100)).unwrap();  // send address
block!(serial_tx.write(data_byte)).unwrap();                   // send data
```

**Usage:** Distinguish address frames from data frames in multi-device communication.



#### Serial DMA Reception
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
    rprintln!("Starting serial DMA test");
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

    // Split DMA1 channels (DMA1 has 7 channels)
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

    // Bind serial RX to DMA channel 3 (USART3 RX -> DMA1 Ch3)
    let rx_dma = serial.rx.with_dma(channels.3);
    // Bind serial TX to DMA channel 2 (USART3 TX -> DMA1 Ch2)
    let tx_dma = serial.tx.with_dma(channels.2);

    // singleton! macro: create a unique instance in static memory
    // DMA requires static lifetime buffer
    let rx_buf = singleton!(: [u8; 8] = [0; 8]).unwrap();

    rprintln!("Waiting to receive 8 bytes...");
    // Start DMA receive (blocking, wait for 8 bytes)
    let (buf, _rx) = rx_dma.read(rx_buf).wait();

    rprintln!("DMA receive complete!");
    for (i, byte) in buf.iter().enumerate() {
        rprintln!("buf[{}] = 0x{:02X} -> {}", i, byte, *byte as char);
    }

    // DMA transmit example
    let tx_buf = singleton!(: [u8; 12] = *b"Hello DMA!\r\n").unwrap();
    let (_buf, _tx) = tx_dma.write(tx_buf).wait();

    rprintln!("DMA transmit complete!");

    loop {}
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071793247-2af14a44-7e27-459a-8ed3-05ec1a3abc0a.png" width="342" title="" crop="0,0,1,1" id="u7dbb3119" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071860321-d37bfaff-e264-4099-a590-ef16f984731d.png" width="328" title="" crop="0,0,1,1" id="uabd331a7" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071685474-f076938b-fd10-4f3b-8796-4be8b770bf0e.png" width="306" title="" crop="0,0,1,1" id="u3ad590c6" class="ne-image">

Explains how the trigger happens only after 8 bytes are stored!



## ADC Acquisition
**ADC Key Parameters:**

+ Resolution: 12-bit (0-4095)
+ Conversion time: depends on ADC clock
+ Reference voltage: VDDA (typically 3.3V)
+ Formula: `Voltage = Reading / 4095 * 3.3V`

**Available ADC Channels on DKX Board:**

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


### External Voltage Measurement
Measure voltage on PB01 pin

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
    rprintln!("ADC voltage measurement");
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // Configure clock: HSE 8MHz, SYSCLK 72MHz, ADCCLK 14MHz
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    rprintln!("adc freq: {}", rcc.clocks.adcclk());

    // Initialize ADC1
    let mut adc1 = adc::Adc::new(dp.ADC1, &mut rcc);

    // Configure PB1 as analog input
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut ch0 = gpiob.pb1.into_analog(&mut gpiob.crl);

    // Initialize SysTick delay
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, rcc.clocks.sysclk().raw());

    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        // Reference voltage 3.3V, 12-bit ADC (0-4095)
        let voltage_mv = data as u32 * 3300 / 4095;
        let voltage_v = voltage_mv as f32 / 1000.0;
        rprintln!("adc1: {}  |  {}mV  |  {:.3}V", data, voltage_mv, voltage_v);
        delay.delay_ms(600u32);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073349544-41a745c3-faaa-44c9-8ada-70e709ed8da6.png" width="514" title="" crop="0,0,1,1" id="ub9d403d9" class="ne-image">

Grounded

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073451277-a7b6496a-b345-4f55-8089-a2763cb2c721.png" width="536" title="" crop="0,0,1,1" id="u294fb1ca" class="ne-image">

Connected to 3.3V

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073526370-be39c8d3-3198-4e0e-81fd-9e58c6035ba3.png" width="529" title="" crop="0,0,1,1" id="u69367ae2" class="ne-image">

Voltage measured at midpoint of two 1K resistors in series



### Internal ADC Temperature Measurement
**Internal Temperature Sensor:**

+ Connected to ADC1 channel 16
+ Low accuracy (±1.5°C), suitable for rough monitoring
+ Conversion time requires at least 17.1μs

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
    rprintln!("Internal ADC temperature sensor test");
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



### ADC DMA Circular Acquisition
**Circular DMA Working Principle:**

```plain
     Buffer A          Buffer B
┌─────────────┐  ┌─────────────┐
│ [0] [1] ... │  │ [0] [1] ... │
│    [7]      │  │    [7]      │
└─────────────┘  └─────────────┘
       ↑ DMA write   ↑ DMA write
       └── Alternating ──┘

Half::First → Buffer A readable
Half::Second → Buffer B readable
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
    rprintln!("ADC DMA circular acquisition test");

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

    // Split DMA1 channel 1
    let dma_ch1 = dp.DMA1.split(&mut rcc).1;

    let adc1 = adc::Adc::new(dp.ADC1, &mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let adc_ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    // Bind ADC with DMA
    let adc_dma = adc1.with_dma(adc_ch0, dma_ch1);

    // Create double buffer (circular mode needs two half buffers)
    // singleton! ensures buffer is in static memory
    let buf = singleton!(: [[u16; 8]; 2] = [[0; 8]; 2]).unwrap();

    // Start circular DMA read
    let mut circ_buffer = adc_dma.circ_read(buf);

    // Note: In DMA circular mode, don't insert time-consuming operations between readable_half() calls
    // Otherwise DMA may complete a full cycle, causing both HTIF and TCIF to set → Overrun panic

    while circ_buffer.readable_half().unwrap() != Half::First {}
    let first_half = circ_buffer.peek(|half, _| *half).unwrap();

    while circ_buffer.readable_half().unwrap() != Half::Second {}
    let second_half = circ_buffer.peek(|half, _| *half).unwrap();

    rprintln!("First half buffer: {:?}", first_half);
    rprintln!("Second half buffer: {:?}", second_half);

    let (_buf, adc_dma) = circ_buffer.stop();
    let (_adc1, _adc_ch0, _dma_ch1) = adc_dma.split();

    rprintln!("ADC DMA circular acquisition complete");
    cortex_m::asm::bkpt();
    loop {}
}
```

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



### Driving ST7789 Display
240*240

**Hardware Pin Connections**

| Display Pin | MCU Pin | Description |
| :--- | :--- | :--- |
| SCL | PA5 | SPI Clock (SPI1_SCK) |
| SDA | PA7 | SPI Data Output (SPI1_MOSI) |
| DC | PA0 | Command/Data Select |
| RES | PA1 | Hardware Reset |
| CS | GND | Chip Select pulled low (always selected) |


> **Note:** "SCL" and "SDA" in the table are typically I²C bus signal names, but here they are connected to the SPI interface, actually corresponding to SPI's **SCK** and **MOSI**. This naming is common on some LCD modules; just use the pins according to their actual function. CS connected to GND means the SPI device is always selected, no software chip select control needed.
>

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780126926273-223244e9-d923-467e-be77-c109bd054a91.png" width="272" title="" crop="0,0,1,1" id="ucea3e5ce" class="ne-image">

ST7789 Driver Code ---- src/st7789.rs

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

Main Program

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
    rprintln!("ST7789 240x240 driver test");

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

    rprintln!("SPI initialized, 16MHz");

    let mut display = ST7789::new(dc, rst);

    rprintln!("ST7789 initializing...");
    display.init(&mut spi, &clocks).unwrap();
    rprintln!("ST7789 initialization complete");

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
    let color_names = ["Red", "Green", "Blue", "Yellow", "Cyan", "Purple", "White", "Black"];

    let mut color_idx: usize = 0;

    loop {
        let c = colors[color_idx % colors.len()];
        rprintln!("Fill color: {} #{:04X}", color_names[color_idx % color_names.len()], c);
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



## I2C Communication
Using address scanning as an example!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129573451-695873a0-2895-4ae6-82d9-760b0735e6d3.png" width="547" title="" crop="0,0,1,1" id="uf3139d39" class="ne-image">

**Wiring**

| MCU | Device |
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

Wiring as shown

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129746845-ddca9982-5283-4dd1-9def-439f14f69b5e.png" width="645" title="" crop="0,0,1,1" id="u6a6d07a1" class="ne-image">



## PWM Wave
### Output
Let's use PWM-controlled servo as an example

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
    rprintln!("\r\nServo test");
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

    // PA0 -> TIM2 CH1, 50Hz (standard servo frequency)
    let pins = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp.TIM2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 50.Hz(), &mut rcc);
    let max = pwm.get_max_duty();
    pwm.enable(Channel::C1);

    // Servo pulse width range: 0.5ms ~ 2.5ms (corresponds to 0° ~ 180°)
    // Period 20ms, duty cycle = pulse width / period
    // duty_0   = max * 0.5 / 20  = max / 40
    // duty_180 = max * 2.5 / 20  = max / 8
    let duty_min = max / 40;   // 0.5ms → 0°
    let duty_max = max / 8;    // 2.5ms → 180°
    let step = (duty_max - duty_min) / 180;  // duty increment per degree

    let mut current_duty = duty_min;
    let mut direction_up = true;

    // 72MHz main clock, ~5ms delay per step → servo completes 0→180 in ~0.7s
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
        rprintln!("--{:>3}°--",(current_duty-duty_min)/step);
        asm::delay(delay_cycles);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780132461432-60a818a5-4ab8-4cf6-afa6-a0009cb11d23.png" width="342" title="" crop="0,0,1,1" id="u22f9ccbe" class="ne-image">



### Input
Let's use EC11 encoder as an example

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780133888771-3dfd0d4c-f65c-4ded-af72-16faef0fcbed.png" width="394" title="" crop="0,0,1,1" id="ue032c32d" class="ne-image">

EC11 (with button)

**Wiring**

| MCU | Device |
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
    rprintln!("PWM input detection started");

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

    // Disable JTAG to release PB4/PB5 (occupied by JTAG by default)
    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    let pb5 = gpiob.pb5;

    // TIM3 configured as PWM input mode
    // PB4 = IC1 (rising edge capture, measure period)
    // PB5 = IC2 (falling edge capture, measure high time)
    let pwm_input = dp.TIM3.remap(&mut afio.mapr).pwm_input(
        (pb4, pb5),
        &mut dbg,
        Configuration::Frequency(10.kHz()),
        &mut rcc,
    );

    let timer_clk = pac::TIM3::timer_clock(&rcc.clocks);
    rprintln!("Timer clock: {} Hz", timer_clk.raw());

    loop {
        match pwm_input.read_frequency(ReadMode::WaitForNextCapture, timer_clk) {
            Ok(freq) => {
                let freq_hz = freq.raw();
                match pwm_input.read_duty(ReadMode::Instant) {
                    Ok((high, period)) => {
                        let duty_pct = (high as f32 * 100.0) / period as f32;
                        rprintln!(
                            "Frequency: {} Hz | Duty: {:.1}% ({}/{})",
                            freq_hz,
                            duty_pct,
                            high,
                            period,
                        );
                    }
                    Err(_) => {
                        rprintln!("Frequency: {} Hz | Duty read failed", freq_hz);
                    }
                }
            }
            Err(Error::FrequencyTooLow) => {
                rprintln!("Signal frequency too low or no signal");
            }
        }
    }
}
```



Rotating the encoder detects data! No rotation, no output!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780134186205-bb66094b-f5f1-49a0-8f13-673b832f66e1.png" width="480" title="" crop="0,0,1,1" id="u43c0b949" class="ne-image">

Output detection info



### EC11 Encoder Reading
**Wiring**

| MCU | Device |
| --- | --- |
| PB6 | S1 |
| PB7 | S2 |


**Usage:** Speed/position measurement for quadrature encoder devices such as motor encoders and rotary knobs.



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
    rprintln!("EC11 encoder QEI test");

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
        rprintln!("Pulses: {} Direction: {:?}", elapsed, qei.direction());
    }
}
```



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780136893674-70671b8b-3b27-429a-8a79-12fdca9cbd25.png" width="499" title="" crop="0,0,1,1" id="u76a64ffa" class="ne-image">



## CRC Checksum
**Usage:** Data integrity verification, CRC checksum for communication protocols.

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
    rprintln!("CRC demo started");

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
    rprintln!("Single-word CRC: found={:08x}, expected={:08x}", val, 0xdf8a8a2b_u32);

    crc.reset();
    crc.write(0x00000001);
    crc.write(0x00000002);
    crc.write(0x00000003);
    let val = crc.read();
    rprintln!("Multi-word CRC: result={:08x}", val);

    crc.reset();
    let val = crc.read();
    rprintln!("Initial value after reset: {:08x} (expected ffffffff)", val);

    rprintln!("CRC demo complete");

    loop {}
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780138752178-cbb34d1f-7689-4eb2-a05c-651295f45c4d.png" width="590" title="" crop="0,0,1,1" id="u603a4730" class="ne-image">



## DAC (Digital-to-Analog Conversion)
> Note: STM32F103C8T6 (DKX Board) **does not have DAC**, DAC is only available on high-density devices (STM32F103xC/D/E).
>

Note: C8T6 does not support it, so we choose STM32F103RCT6

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

Change the configuration in config.toml

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780139752540-46050614-f41b-4e59-a165-a384a1b0b9ab.png" width="659" title="" crop="0,0,1,1" id="ud3f9d220" class="ne-image">

```rust
# stm32f1xx-hal: Hardware Abstraction Layer for STM32F1 series
# Provides high-level Rust API for RCC, GPIO, TIM, USART and other peripherals
[dependencies.stm32f1xx-hal]
version = "0.11.0"
features = [
    "stm32f103",  # STM32F103 series chip
    "high",       # high-density (256KB Flash or above), RCT6 is this type
]
```

Code as follows

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
    rprintln!("DAC test");

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

Results as follows

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140722259-090abde4-4b38-4c94-b484-0cf40c1bf96a.png" width="396" title="" crop="0,0,1,1" id="u15e94679" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140832880-63b745a7-86fc-43cd-8947-01980100a6dc.png" width="560" title="" crop="0,0,1,1" id="u415533ee" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140776299-cfdce37d-a05f-46cd-a8e9-85e1fe758aa5.png" width="563" title="" crop="0,0,1,1" id="ud77ae8bf" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140863989-0e947147-c83d-4fa5-b09c-59dbb4a05e95.png" width="565" title="" crop="0,0,1,1" id="u57f96e96" class="ne-image">

Measurement results are within acceptable error



## CAN Bus (No Device Verification - Untested)
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

    // Configure filter (accept all frames)
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters);

    // Enable CAN
    let mut can = can1;
    block!(can.enable_non_blocking()).unwrap();

    // Loopback test: immediately send back received frames
    loop {
        if let Ok(frame) = block!(can.receive()) {
            block!(can.transmit(&frame)).unwrap();
        }
    }
}
```

**CAN Pins (DKX Board):** PA11(CAN RX), PA12(CAN TX) — Note shared with USB pins



## USB Serial (No Device Verification - Untested)
### USB Polling Serial (No Device Verification - Untested)
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

    assert!(rcc.clocks.usbclk_valid());  // verify USB clock is valid

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_high();  // turn off LED

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // USB D+ line has pull-up resistor
    // For development, pull D+ low to trigger USB RESET
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();                           // pull D+ low
    delay(rcc.clocks.sysclk().raw() / 100);     // short delay

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
                led.set_low();  // turn on LED
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
        led.set_high();  // turn off LED
    }
}
```

**USB Key Points:**

+ **Must use 48MHz system clock** (USB protocol requires accurate clock)
+ PA11 = USB D-, PA12 = USB D+
+ Manual USB RESET trigger required during development
+ VID/PID `0x16c0:0x27dd` are unofficial test IDs
+ Needs release mode build (debug mode FLASH overflows)

---

### USB Interrupt Serial (No Device Verification - Untested)
Handle USB communication using interrupt mode.

```rust
// Global USB objects
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;

#[entry]
fn main() -> ! {
    // ... USB initialization code ...

    // Enable USB interrupt
    unsafe {
        NVIC::unmask(Interrupt::USB_HP_CAN_TX);   // high priority
        NVIC::unmask(Interrupt::USB_LP_CAN_RX0);  // low priority
    }

    loop { wfi(); }  // all work done in interrupt
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
                    *c &= !0x20;  // convert to uppercase
                }
            }
            serial.write(&buf[0..count]).ok();
        }
        _ => {}
    }
}
```

**Polling vs Interrupt:**

+ Polling: simple, but CPU busy-waits
+ Interrupt: CPU can sleep, saving power



# Practical Projects
## DHT11
```rust
//! # DHT11 Temperature and Humidity Sensor Driver
//!
//! ## Compatible Pins
//!
//! **Any GPIO pin** on STM32F103 can be used, including:
//!
//! | Port | Pins        | Notes                         |
//! |------|-------------|-------------------------------|
//! | GPIOA | PA0~PA15   | Most commonly used, PA6 is default for this project |
//! | GPIOB | PB0~PB15   | Available, PB3/PB4 need JTAG disabled |
//! | GPIOC | PC13~PC15  | Available, but PC13 is usually connected to LED |
//! | GPIOD | PD0~PD15   | C8T6 only exposes PD0~PD2 |
//!
//! **Only limitation**: PA13/PA14 default to SWD debug pins, PA15/PB3/PB4 default to JTAG pins.
//! Using these pins requires disabling JTAG/SWD remapping (via AFIO).
//!
//! ## Why the Code is Written This Way
//!
//! ### 1. Push-Pull Output → Floating Input Switching
//!
//! DHT11 uses **single-bus protocol**, MCU and sensor time-share the same line:
//!
//! ```text
//! ┌─────────┐                    ┌─────────┐
//! │   MCU   │───── DATA ────────│  DHT11  │
//! └─────────┘    (pull-up resistor)      └─────────┘
//! ```
//!
//! - **Start Signal**: MCU must actively pull low for 20ms → pull high for 30us (requires **push-pull output**, can actively drive high/low)
//! - **Data Read**: DHT11 drives the bus to send data (MCU must release bus → **floating input**, read-only)
//!
//! If open-drain output is used, `set_high()` only releases the bus (high-impedance),
//! rising edge relies on pull-up resistor, slow, DHT11 may not detect the start signal's rising edge.
//!
//! ### 2. SysTick Hardware Timer
//!
//! `cortex_m::asm::delay(n)` is a software loop count, affected by **flash wait states**:
//! - STM32F103 flash has 2 wait cycles at 72MHz
//! - Each instruction in the software loop actually takes 2~3 clock cycles
//! - `delay(72)` may actually take 2~3us instead of 1us
//!
//! SysTick is a Cortex-M core 24-bit hardware down-counter, accurately timing with the system clock,
//! **not affected by flash wait states**.
//!
//! ### 3. Keep High After Reading
//!
//! DHT11 bus idle state is high (maintained by external pull-up resistor).
//! If push-pull outputs low after reading, the next start signal's falling edge won't be recognized by DHT11.
//!
//! ### 4. Const Generics
//!
//! `Pin<const P: char, const N: u8, MODE>` uses const generics:
//! - `P` = port name ('A', 'B', 'C', 'D')
//! - `N` = pin number (0~15)
//! - `MODE` = mode type (Output<PushPull>, Input<Floating>, etc.)
//!
//! The compiler generates specialized code for each specific pin, **zero runtime overhead**.
//! Pins 0~7 use CRL register, 8~15 use CRH register,
//! automatically selected at compile time by the `HL` trait.

use cortex_m::peripheral::{syst::SystClkSource, SYST};
use stm32f1xx_hal::gpio::{Floating, HL, Input, Output, Pin, PinState, PushPull};

// ============================================================
//  SysTick Hardware Timer Delay
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
        // At 72MHz, 1us = 72 ticks; SysTick max 0xFFFFFF ≈ 233ms
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
//  DHT11 Error Types
// ============================================================

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    /// No response from DHT11 after start signal (no low-level acknowledgment detected)
    NoResponse,
    /// Data bit read timeout (abnormal bus state)
    ReadTimeout,
    /// Checksum mismatch
    Checksum { calc: u8, recv: u8 },
}

// ============================================================
//  DHT11 Driver
// ============================================================

pub struct Dht11;

impl Dht11 {
    /// Read temperature and humidity data from DHT11 once
    ///
    /// # Parameters
    /// - `pin` — Push-pull output mode pin (idle high)
    /// - `cr`  — Pin control register reference (CRL or CRH, auto-deduced by compiler)
    /// - `delay` — SysTick delay timer
    ///
    /// # Returns
    /// - `(Ok((humidity, temperature)), pin)` — Success, humidity 0~99%RH, temperature 0~50°C
    /// - `(Err(e), pin)` — Failure
    ///
    /// Regardless of success or failure, `pin` is restored to **push-pull output (high)** and returned to the caller.
    ///
    /// # Type Constraints
    ///
    /// `CR` is an associated type of the `HL` trait, automatically determined by the pin number:
    /// - Pins 0~7 → `Cr<P, false>` = CRL register
    /// - Pins 8~15 → `Cr<P, true>` = CRH register
    ///
    /// Two `where` constraints ensure `CR` type is consistent in both Output and Input modes,
    /// so the same `&mut cr` can be shared during mode switching.
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
        // ---- 1. Master Start Signal (push-pull output, active drive) ----
        pin.set_low();
        delay.ms(20); // pull low for 20ms (spec: 18~30ms)
        pin.set_high();
        delay.us(30); // pull high for 30us (spec: 10~35us)

        // ---- 2. Switch to floating input, read DHT11 data ----
        let in_pin = pin.into_floating_input(cr);

        let result = Self::read_data(&in_pin, delay);

        // ---- 3. Switch back to push-pull output, keep bus idle high ----
        let out_pin =
            in_pin.into_push_pull_output_with_state(cr, PinState::High);

        (result, out_pin)
    }

    // ---- Internal: Wait for response + read 40-bit data ----

    fn read_data<const P: char, const N: u8>(
        pin: &Pin<P, N, Input<Floating>>,
        delay: &mut Delay,
    ) -> Result<(u8, u8), Error> {
        // DHT11 response: first pull low ~80us
        if !Self::wait_level(pin, false, 100, delay) {
            return Err(Error::NoResponse);
        }
        // DHT11 response: then pull high ~80us
        if !Self::wait_level(pin, true, 100, delay) {
            return Err(Error::NoResponse);
        }

        // Read 5 bytes (40 bits)
        // Byte0 = Humidity integer  Byte1 = Humidity decimal
        // Byte2 = Temperature integer  Byte3 = Temperature decimal  Byte4 = Checksum
        let mut buf = [0u8; 5];
        for slot in &mut buf {
            *slot = Self::read_byte(pin, delay).ok_or(Error::ReadTimeout)?;
        }

        // Checksum: lower 8 bits of sum of first 4 bytes == 5th byte
        let sum = buf[0] as u32 + buf[1] as u32 + buf[2] as u32 + buf[3] as u32;
        if (sum & 0xFF) as u8 != buf[4] {
            return Err(Error::Checksum {
                calc: (sum & 0xFF) as u8,
                recv: buf[4],
            });
        }

        Ok((buf[0], buf[2])) // (humidity, temperature)
    }

    // ---- Internal: Wait for pin to reach target level ----

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

    // ---- Internal: Read one byte (MSB first) ----
    //
    // Timing for each bit:
    //   ┌──── 50us ────┐┌── 28us(0) or 70us(1) ──┐
    //   │    Low Level   ││       High Level         │
    //   └───────────────┘└─────────────────────────┘
    //
    // Sampling strategy: wait for low→high transition, then delay 40us and sample
    //   "0": High ~28us → after 40us already low → read 0
    //   "1": High ~70us → after 40us still high → read 1

    fn read_byte<const P: char, const N: u8>(
        pin: &Pin<P, N, Input<Floating>>,
        delay: &mut Delay,
    ) -> Option<u8> {
        let mut byte: u8 = 0;
        for _ in 0..8 {
            byte <<= 1;
            // Wait for low level (each bit starts with ~50us low)
            if !Self::wait_level(pin, false, 70, delay) {
                return None;
            }
            // Wait for high level (DHT11 releases the bus)
            if !Self::wait_level(pin, true, 70, delay) {
                return None;
            }
            // Sample after 40us delay
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
    rprintln!("DHT11 temperature/humidity sensor - PA6");

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

    // PA6 push-pull output, initially high
    let mut pin = gpioa.pa6.into_push_pull_output_with_state(
        &mut gpioa.crl,
        stm32f1xx_hal::gpio::PinState::High,
    );

    delay.ms(1500);
    rprintln!("DHT11 initialized, starting acquisition...");

    loop {
        // Dht11::read accepts a push-pull output pin, returns a push-pull output pin
        let (result, returned_pin) = Dht11::read(pin, &mut gpioa.crl, &mut delay);
        pin = returned_pin;

        match result {
            Ok((humi, temp)) => {
                rprintln!("Humidity: {}%RH, Temperature: {}C", humi, temp);
            }
            Err(e) => {
                rprintln!("DHT11 read failed: {:?}", e);
            }
        }

        delay.ms(2000);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780160254218-22ef0a3f-9b61-4350-b558-f75ed994c24f.png" width="450" title="" crop="0,0,1,1" id="uecb29377" class="ne-image">



## DHT11+ST7789 LCD Thermometer
| Display & DHT11 | MCU Pin | Function |
| :--- | :--- | :--- |
| SCL | PA5 | SPI clock (SPI1_SCK) |
| SDA | PA7 | SPI data output (SPI1_MOSI) |
| DC | PA0 | Command/data select |
| RES | PA1 | Hardware reset |
| CS | GND | Chip select low (always selected) |
| DATA | PA6 | DHT11 data line |


Actual result

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164028402-54ee6f36-3789-4788-b616-7f4f67a666ce.png" width="628" title="" crop="0,0,1,1" id="u132cff35" class="ne-image">

Project structure

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
    ('\u{5EA6}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0xC0, 0x1F, 0xF8, 0xFF, 0xFF, 0x03, 0xF8, 0xFF, 0x01, 0x08, 0x00, 0x00, 0x00, 0x10, 0x30, 0x02, 0x00, 0x18, 0x30, 0x02, 0x00, 0x18, 0x10, 0x82, 0x01, 0x08, 0x10, 0x02, 0x07, 0x0C, 0x10, 0x02, 0x1C, 0x0C, 0xD0, 0x7F, 0x32, 0x04, 0x10, 0x12, 0x62, 0x06, 0x10, 0x12, 0xC2, 0x02, 0x17, 0x12, 0x82, 0x03, 0x10, 0x13, 0x02, 0x03, 0x10, 0x13, 0x82, 0x03, 0x10, 0x13, 0xC2, 0x02, 0x10, 0x11, 0x62, 0x04, 0x08, 0x7F, 0x32, 0x04, 0xC8, 0x3F, 0x1E, 0x08, 0x48, 0x01, 0x06, 0x08, 0x08, 0x01, 0x00, 0x08, 0x08, 0x01, 0x00, 0x08, 0x08, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // degree
    ('\u{6E29}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x10, 0x00, 0x0C, 0x00, 0x3C, 0x08, 0x18, 0x00, 0x0F, 0x18, 0x38, 0xC0, 0x03, 0x38, 0x30, 0x7C, 0x00, 0x70, 0xC0, 0x03, 0x00, 0x60, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x07, 0x10, 0x0C, 0x20, 0xFE, 0x17, 0x38, 0x7F, 0x02, 0x10, 0xF0, 0x17, 0x02, 0x10, 0x10, 0x31, 0x02, 0x10, 0x10, 0x31, 0x02, 0x10, 0x10, 0x31, 0xFE, 0x0F, 0x10, 0x11, 0x02, 0x08, 0x90, 0x11, 0x02, 0x08, 0x90, 0x11, 0x02, 0x0A, 0x90, 0x11, 0xFF, 0x0F, 0x90, 0x11, 0xFF, 0x0F, 0x18, 0x10, 0x03, 0x08, 0xF8, 0x7F, 0x03, 0x08, 0x00, 0x70, 0xFF, 0x0F, 0x00, 0x00, 0x0F, 0x0F, 0x00, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00]), // temperature
    ('\u{6E7F}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x18, 0x08, 0x0C, 0x00, 0x1E, 0x18, 0x18, 0x80, 0x07, 0x30, 0x30, 0x78, 0x00, 0x30, 0x20, 0x07, 0x00, 0x60, 0x80, 0x00, 0x10, 0xC0, 0x00, 0x00, 0x10, 0x00, 0x30, 0x06, 0x10, 0x0E, 0x7C, 0x1E, 0x18, 0xF8, 0x1F, 0xF8, 0x18, 0xE0, 0x31, 0xC0, 0x19, 0x08, 0x11, 0x00, 0x09, 0x88, 0xD1, 0xFF, 0x0F, 0x88, 0xD1, 0xFF, 0x0B, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x0B, 0x88, 0xD8, 0xFF, 0x0B, 0xC8, 0x18, 0x00, 0x08, 0x88, 0x18, 0x80, 0x09, 0xFC, 0x3F, 0xE0, 0x08, 0xFC, 0x7F, 0x38, 0x08, 0x00, 0x70, 0x0E, 0x08, 0x00, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // humidity
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

[font_tool.py](https://www.yuque.com/attachments/yuque/0/2026/py/67055297/1780215790197-e93dd665-cc2d-4706-8d8d-d349b64c53df.py)


<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164408965-82bc27da-8093-4c6a-9042-138ae538676f.png" width="680" title="" crop="0,0,1,1" id="u7c1aa107" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164569870-c81ed584-799f-4093-8e82-20d2ca0bd4bd.png" width="720" title="" crop="0,0,1,1" id="u1d403555" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164484495-b510887a-e182-4bc3-b09e-29dadc3900a7.png" width="577" title="" crop="0,0,1,1" id="u612677a5" class="ne-image">

Don't ask why I used Python — it's fast!



## DHT20+ST7789 LCD Thermometer
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780166394105-ec6c2ced-4711-4e04-9992-2d3939656899.png" width="547" title="" crop="0,0,1,1" id="u7f142962" class="ne-image">

| Display & DHT20 | MCU Pin | Function |
| :--- | :--- | :--- |
| SCL (display) | PA5 | SPI clock (SPI1_SCK) |
| SDA (display) | PA7 | SPI data output (SPI1_MOSI) |
| DC | PA0 | Command/data select |
| RES | PA1 | Hardware reset |
| CS | GND | Chip select low (always selected) |
| SCL (DHT20) | PB7 | DHT20 clock line |
| SDA (DHT20) | PB6 | DHT20 data line |


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



# Conclusion
Author email: pycx0@qq.com  
The author completed this project as a senior about to graduate, because the job market pressure in China is immense. Currently job hunting — if things don't work out, I might end up working factory shifts!  
Follow-up updates will be at the original link! Hoping to find a good job!

Looking forward to the Rust ecosystem getting better and better! Keep it up, global villagers!
