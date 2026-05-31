<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780203075814-b3bd4b33-1fec-4f6a-ba6d-342f60f58486.png" width="1448" title="" crop="0,0,1,1" id="ue9743a87" class="ne-image">



[简体中文](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_cn.md) / [English](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_en.md) / [Русский](https://github.com/cx693/Learning_to_develop_STM32_using_Rust/blob/main/README_ru.md)

# Лицензия
Данный проект распространяется по лицензии CC BY-NC 4.0. Коммерческое использование требует получения разрешения от владельца авторских прав pycx0@qq.com. Коммерческие продукты, основанные на этом проекте, должны получить лицензию. Некоммерческое использование бесплатно!

# Настройка базовой среды
## Установка probe-rs
```bash
cargo install probe-rs-tools --locked
```



## Установка компилятора
```bash
rustup target install thumbv7m-none-eabi
```

## Проверка с probe-rs
Режим DAP!

```bash
probe-rs info --protocol swd
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779699156280-0601cf1f-d586-4e82-8763-adf851dc2ccc.png" width="700" title="" crop="0,0,1,1" id="u0cb46cfc" class="ne-image">



## Установка пакетов
В основном для исправления бага с бесконечным циклом в панике

```bash
cargo add panic-halt
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779700077727-2d4a69c3-fbfa-4838-9bcf-5be9c764a7a0.png" width="369" title="" crop="0,0,1,1" id="u09183fea" class="ne-image">

Просмотр занимаемой памяти FLASH

```json
cargo install st-mem
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779726981642-0c7dc5e5-9c60-4e5f-a84e-f07c4be2bdd0.png" width="764" title="" crop="0,0,1,1" id="uaed0ebe6" class="ne-image">



# Компиляция и тестирование
Структура проекта

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
# [package] — метаданные пакета
[package]
# Имя пакета, используется при публикации на crates.io или для ссылок из других проектов
name = "stm32dome"
# Версия пакета, соответствует семантическому версионированию (Semantic Versioning)
version = "0.1.0"
# Используемая версия Rust, 2024 — последняя версия, включает новейшие языковые возможности
edition = "2024"

# [dependencies] — зависимости проекта
[dependencies]
# embedded-hal: стандартный интерфейс аппаратной абстракции для встраиваемых систем
# Версия "1.0" означает последнюю версию в диапазоне 1.0.x
# Определяет общие типажи для GPIO, I2C, SPI, UART и т.д., обеспечивая переносимость кода между разными МК
embedded-hal = "1.0"

# nb: библиотека для неблокирующих операций
# Предоставляет Result и макрос block! для обработки операций, требующих повторных попыток (например, отправка UART)
nb = "1"

# cortex-m: базовая библиотека поддержки для ARM Cortex-M процессоров
# Предоставляет доступ к системным регистрам, управление прерываниями, системный таймер и т.д.
cortex-m = "0.7.7"

# cortex-m-rt: библиотека времени выполнения для Cortex-M
# Предоставляет стартовый код, таблицу векторов прерываний, инициализацию памяти и т.д.
# Без неё нельзя использовать макрос #[entry] для определения точки входа программы
cortex-m-rt = "0.7.5"
panic-halt = "1.0.0"
rtt-target = "0.6.2"
rtic = { version = "2", features = ["thumbv7-backend"] }
rtic-monotonics = { version = "2", features = ["cortex-m-systick"] }


# [dependencies.stm32f1xx-hal]: детальная конфигурация конкретной зависимости
# Используется табличный синтаксис для более подробной настройки stm32f1xx-hal
[dependencies.stm32f1xx-hal]
# Указание версии
version = "0.11.0"
# features: включаемые возможности на этапе компиляции
# "stm32f103": выбор поддержки для микроконтроллеров серии STM32F103
# "medium": конфигурация для микросхем средней плотности (64-128 КБ Flash), C8T6 относится к этому классу
# Другие варианты: "low" (низкая плотность, 16-32 КБ) и "high" (высокая плотность, 256+ КБ)
features = ["stm32f103", "medium"]


[profile.dev]
incremental = false   # Отключение инкрементальной компиляции для обеспечения воспроизводимости сборки
codegen-units = 1     # Один модуль генерации кода, позволяющий компилятору выполнять больше оптимизаций
opt-level = 1         # Небольшая оптимизация, чтобы избежать слишком медленной обработки прерываний в режиме debug
panic = "abort"       # При панике сразу завершать, без размотки стека (во встраиваемых системах нет поддержки размотки)

[profile.release]
codegen-units = 1
debug = true          # Сохранять отладочную информацию (не влияет на производительность, удобно для отладки)
lto = true            # Оптимизация на этапе компоновки, межмодульная оптимизация для уменьшения размера
panic = "abort"
```

.cargo/config.toml

```rust
[target.thumbv7m-none-eabi]
# ============================================================
# Runner — st-mem runner (кроссплатформенный, сначала анализ памяти, затем прошивка)
# ============================================================
# st-mem runner: анализ занятости FLASH/RAM → прошивка через probe-rs
runner = "st-mem runner --chip STM32F103C8 --protocol swd"
# ============================================================
# Без анализа памяти, напрямую через probe-rs:
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

<font style="color:#DF2A3F;">main.rs см. раздел «Зажигаем первый светодиод»</font>

## Функция перехода не работает
Решение!

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

# Отладка
Создание файлов конфигурации задач! Используйте официальный сайт probe-rs! Поиск

[https://probe.rs/docs/tools/debugger/](https://probe.rs/docs/tools/debugger/)



Поддерживаемые типы микросхем: [https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)



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
            "chip": "STM32F103C8", // Модель микросхемы — измените под свою!
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
                    // Включение RTT
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



~~В настоящее время на macos есть проблема с отладкой RTT — нет вывода в терминал!~~ Исправлено!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976699740-3c2d6980-e2fa-46f9-8404-b6d5f4366b2d.png" width="1440" title="" crop="0,0,1,1" id="u90e0f0b7" class="ne-image">

Текущее решение: на входе в main() сразу поставить точку останова, первой строкой написать: rtt_init_print!();

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779976859955-199c20a0-8073-4eca-b109-c0a097238644.png" width="486" title="" crop="0,0,1,1" id="ue86693d6" class="ne-image">






# Загрузка проекта (прошивка)
Базовая среда компиляции Rust

```plain
cargo install cargo-binutils
rustup component add llvm-tools
```

## HEX-файл
Компиляция ELF-файла

```rust
cargo build --release
```

Компиляция HEX-файла

```rust
cargo objcopy --release -- -O ihex ccc.hex
cargo objcopy --release -- -O ihex <имя_прошивки>.hex
```

Непосредственная загрузка

```rust
probe-rs download --binary-format hex --chip STM32F103C8 ccc.hex
probe-rs download --binary-format hex --chip <имя_микросхемы> <имя_прошивки>.hex
```

Адрес для поиска микросхем: [https://probe.rs/targets/?q=&p=0](https://probe.rs/targets/?q=&p=0)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779867658066-603db27f-8d6c-4545-b93a-3fc240c50078.png" width="1540" title="" crop="0,0,1,1" id="ua5bbe57c" class="ne-image">

## BIN-файл
Компиляция ELF-файла

```rust
cargo build --release
```

Компиляция BIN-файла

```rust
cargo objcopy --release -- -O binary ccc.bin
cargo objcopy --release -- -O binary <имя_прошивки>.bin
```

Непосредственная загрузка

```rust
probe-rs download --chip STM32F103C8 --base-address 0x08000000 --binary-format bin ccc.bin
probe-rs download --chip <имя_микросхемы> --base-address <адрес_смещения> --binary-format bin <имя_прошивки>.bin
```



# Освобождение портов JTAG — невозможность загрузки
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897632872-17f9317c-b978-4d97-845e-7012f1e8897b.png" width="604" title="" crop="0,0,1,1" id="u888d55bd" class="ne-image">

Или при ошибке выполнения — команда полного стирания

```rust
probe-rs erase --chip STM32F103C8 --speed 100 --protocol swd
probe-rs erase --chip <имя_микросхемы> --speed 100 --protocol <тип_интерфейса-можно_опустить>
probe-rs erase --chip STM32F103C8 --speed 100
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779897847743-83db4c01-57bd-4529-8fef-cb60a3c3b09f.png" width="496" title="" crop="0,0,1,1" id="u2c272bf8" class="ne-image">

То есть boot0-boot1 → оба = 0

После выполнения команды быстро нажать и отпустить кнопку сброса! Она автоматически сотрёт. Если не срабатывает, сначала удерживайте кнопку сброса, выполните команду, затем сразу отпустите!



# Учебные ссылки
Материалы: [https://xxchang.github.io/book/](https://xxchang.github.io/book/)

Репозиторий проекта: [https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples](https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples)



# Основы обучения
---

## Подробно о системе тактирования
> **Почему сначала тактирование?** Потому что почти вся периферия зависит от тактового сигнала. Конфигурация тактирования — самый базовый и важный шаг в разработке встраиваемых систем. Ошибки конфигурации приведут к некорректной работе периферии, неверной скорости UART, невозможности enumeration USB и т.д.
>

### Общая схема тактирования STM32F1
Система тактирования STM32F103 очень гибкая, имеет несколько источников тактового сигнала и делителей. Ниже приведена упрощённая схема тактирования:

```plain
                          ┌─────────────┐
                          │   HSE       │   Внешний высокочастотный кварц (плата DKX: 8 МГц)
                          │  8 MHz      │
                          └──────┬──────┘
                                 │
                          ┌──────▼──────┐
                          │   HSI       │   Внутренний RC-генератор
                          │  8 MHz      │  (низкая точность, ±1%, быстрый запуск)
                          └──────┬──────┘
                                 │
                 ┌───────────────┼───────────────┐
                 │               │               │
                 │         ┌─────▼─────┐         │
                 │         │   PLL     │  ФАПЧ   │
                 │         │ Умножитель│  ×2~×16 │
                 │         └─────┬─────┘         │
                 │               │               │
          ┌──────▼──────┐ ┌──────▼──────┐        │
          │ SYSCLK      │ │  USBCLK     │        │
          │ Системная   │ │  Тактовый   │        │
          │ частота     │ │  сигнал USB │        │
          │ макс. 72 МГц│ │  обязат.   │        │
          │             │ │  48 МГц    │        │
          └──────┬──────┘ └─────────────┘        │
                 │                                │
        ┌────────┼────────┐                      │
        │        │        │                      │
   ┌────▼───┐ ┌──▼───┐ ┌──▼────┐          ┌─────▼─────┐
   │ AHB    │ │ APB1 │ │ APB2  │          │  SYSCLK   │
   │ Шина   │ │ Шина  │ │ Шина  │          │  Выбор    │
   │≤72 МГц │ │≤36 МГц│ │≤72 МГц│          │источника  │
   └───┬────┘ └──┬───┘ └──┬────┘          │HSI/HSE/PLL│
       │         │        │               └───────────┘
  ┌────▼───┐ ┌───▼────┐ ┌─▼──────┐
  │ Cortex │ │USART2/3│ │USART1  │
  │ SysTick│ │TIM2-4  │ │SPI1    │
  │ DMA    │ │I2C1/2  │ │ADC1/2  │
  │ Flash  │ │SPI2    │ │TIM1    │
  └────────┘ │USB     │ │GPIO    │
             └────────┘ └────────┘
```

**Ключевая концепция: PLL (ФАПЧ — фазовая автоподстройка частоты)**

```plain
Тактовая частота PLL = Входная частота PLL × Коэффициент умножения PLL

Если выбран HSE как вход PLL:
  PLLCLK = HSE × Коэффициент умножения (×2 ~ ×16)

Пример (плата DKX, кварц 8 МГц):
  HSE × 9  = 8 × 9  = 72 МГц ← максимальная системная частота
  HSE × 6  = 8 × 6  = 48 МГц ← требуется для USB
  HSE × 4  = 8 × 4  = 32 МГц

Если выбран HSI как вход PLL:
  PLLCLK = HSI × 2 × Коэффициент умножения / 2
  PLLCLK = HSI × Коэффициент умножения (×2 ~ ×16)
  Но HSI сначала делится на 2, затем поступает в PLL
```

---

### Подробно об источниках тактового сигнала
#### HSI (High Speed Internal) — внутренний высокочастотный генератор
```plain
Характеристики:
├── Частота: 8 МГц (RC-генератор, с температурным дрейфом)
├── Точность: ±1% (калибровка на заводе), дрейфует при изменении температуры
├── Преимущества: не требует внешних компонентов, доступен сразу после подачи питания
├── Недостатки: низкая точность, не подходит для USB, CAN, точных скоростей
└── По умолчанию: после подачи питания автоматически становится источником системного тактового сигнала
```

**Когда использовать HSI?**

+ Простое мигание светодиодом, опрос кнопок и другие сценарии, не требующие точного тактирования
+ Резервный вариант при повреждении внешнего кварца
+ Сценарии быстрого запуска (HSI запускается быстрее HSE)

#### HSE (High Speed External) — внешний высокочастотный кварц
```plain
Характеристики:
├── Частота: 4-16 МГц (на плате DKX используется кварц 8 МГц)
├── Точность: ±0,005% (зависит от качества кварца)
├── Преимущества: высокая точность, подходит для USB, CAN, точных скоростей UART
├── Недостатки: требуется внешний кварц, время запуска (сотни микросекунд — несколько миллисекунд)
└── Плата DKX: пассивный кварц 8 МГц + 2 нагрузочных конденсатора по 20 пФ
```

**Когда использовать HSE?**

+ Требуется USB (**обязательно** использовать HSE или HSE через PLL)
+ Требуется шина CAN (нужно точное тактирование)
+ Требуется точная скорость UART
+ Требуется работа системы на полной частоте 72 МГц

#### LSE (Low Speed External) — внешний низкочастотный кварц
```plain
Характеристики:
├── Частота: 32,768 кГц (для RTC)
├── Точность: очень высокая (малый температурный дрейф кварца)
├── Назначение: часы реального времени (RTC), сторожевой таймер
└── Плата DKX: возможно, не припаян кварц LSE (нужно проверить по схеме)
```

#### LSI (Low Speed Internal) — внутренний низкочастотный генератор
```plain
Характеристики:
├── Частота: около 40 кГц (неточная)
├── Назначение: независимый сторожевой таймер (IWDG), резервный тактовый сигнал для RTC
└── Точность: низкая (±30%)
```

#### PLL (Phase Locked Loop) — фазовая автоподстройка частоты
PLL — это ядро системы тактирования, используется для умножения низкочастотного тактового сигнала до высокой частоты.

```plain
┌─────────────────────────────────────────────────┐
│                 Подробно о PLL                   │
├─────────────────────────────────────────────────┤
│                                                  │
│  Выбор источника входа:                          │
│  ┌──────┐    ┌─────────┐                        │
│  │ HSI/2│───►│         │    ┌───────────┐       │
│  └──────┘    │ PLL MUX │───►│  ÷ PLLMUL │──►PLLCLK
│  ┌──────┐───►│         │    │  (×2~×16) │       │
│  │ HSE  │    └─────────┘    └───────────┘       │
│  └──────┘                                        │
│                                                  │
│  Типовые конфигурации:                           │
│  ┌──────────┬──────────┬──────────────┐         │
│  │ Входной  │ Коэфф.   │ Выходная     │         │
│  │ сигнал   │ умнож.   │ частота      │         │
│  ├──────────┼──────────┼──────────────┤         │
│  │ HSI 8МГц │ ×9       │ 36 МГц*     │         │
│  │ HSE 8МГц │ ×9       │ 72 МГц ✓    │         │
│  │ HSE 8МГц │ ×6       │ 48 МГц ✓    │         │
│  │ HSE 8МГц │ ×4       │ 32 МГц ✓    │         │
│  └──────────┴──────────┴──────────────┘         │
│                                                  │
│  * HSI сначала делится на 2 (=4 МГц), затем ×9 = 36 МГц│
│                                                  │
└─────────────────────────────────────────────────┘
```

---

### Подробно о тактировании шин
```plain
                         SYSCLK (Системные часы)
                              │
                ┌─────────────┼─────────────┐
                │             │             │
           ┌────▼────┐  ┌────▼────┐  ┌─────▼─────┐
           │  AHB    │  │  APB1   │  │   APB2    │
           │  Шина   │  │  Шина   │  │   Шина    │
           │ ÷HPRE   │  │ ÷PPRE1  │  │  ÷PPRE2   │
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

#### Шина AHB — высокоскоростная шина
| Параметр | Описание |
| --- | --- |
| Максимальная частота | 72 МГц |
| Предделитель | SYSCLK ÷ 1/2/4/8/16/64/128/256/512 |
| Подключенные устройства | Ядро Cortex-M3, DMA, Flash, GPIO, SysTick |
| Метод настройки | `rcc::Config::hsi().hclk(72.MHz())` |


**Примечание:** Тактовый сигнал SysTick поступает от AHB (если SysTick настроен на использование тактов процессора), или AHB/8.

#### Шина APB1 — шина низкоскоростной периферии
| Параметр | Описание |
| --- | --- |
| Максимальная частота | **36 МГц** (жёсткое ограничение, превышение может повредить микросхему) |
| Предделитель | AHB ÷ 1/2/4/8/16 |
| Подключенные устройства | USART2, USART3, I2C1/2, SPI2, TIM2-4, USB, CAN |
| Метод настройки | `rcc::Config::hsi().pclk1(36.MHz())` |


**Важно:** Если коэффициент предделителя APB1 > 1, тактовая частота таймеров = APB1 × 2.

#### Шина APB2 — высокоскоростная шина периферии
| Параметр | Описание |
| --- | --- |
| Максимальная частота | **72 МГц** |
| Предделитель | AHB ÷ 1/2/4/8/16 |
| Подключенные устройства | USART1, SPI1, ADC1/2, TIM1, GPIOA~D, EXTI, AFIO |
| Метод настройки | `rcc::Config::hsi().sysclk(72.MHz()).pclk2(72.MHz())` |


#### Тактовый сигнал ADC
| Параметр | Описание |
| --- | --- |
| Максимальная частота | **14 МГц** |
| Источник тактов | APB2 ÷ 2/4/6/8 |
| Метод настройки | `rcc::Config::hsi().adcclk(14.MHz())` |


#### Тактовый сигнал USB
| Параметр | Описание |
| --- | --- |
| Требуемая частота | **48 МГц** (должна быть точной) |
| Источник тактов | Выход PLL (PLLCLK ÷ 1 или 1,5) |
| Требование к конфигурации | SYSCLK должна быть 48 МГц или 72 МГц |


---

### Конфигурация тактирования в stm32f1xx-hal
Библиотека HAL использует **шаблон Builder** для конфигурации тактирования, что очень интуитивно:

#### Базовое использование
```rust
use stm32f1xx_hal::{pac, prelude::*, rcc};

fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();  // Конфигурация циклов ожидания Flash

    // Способ 1: краткий метод конфигурации
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hsi()           // Использовать внутренний RC 8 МГц
            .sysclk(64.MHz())        // Системная частота 64 МГц
            .pclk1(32.MHz())         // Частота APB1 32 МГц
            .pclk2(64.MHz())         // Частота APB2 64 МГц
            .adcclk(8.MHz()),        // Частота ADC 8 МГц
        &mut flash.acr,
    );

    // Способ 2: использование внешнего кварца + PLL
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())    // Использовать внешний кварц 8 МГц
            .sysclk(72.MHz())        // PLL умножение до 72 МГц
            .pclk1(36.MHz())         // APB1 деление до 36 МГц
            .pclk2(72.MHz())         // APB2 без деления
            .adcclk(14.MHz()),       // ADC 14 МГц
        &mut flash.acr,
    );
}
```

#### Методы Builder для `rcc::Config`
```rust
// Все доступные методы конфигурации (со * — часто используемые)
rcc::Config::hsi()                    // Выбор HSI как источника тактов
rcc::Config::hse(8.MHz())            // Выбор HSE как источника тактов, указание частоты

.sysclk(72.MHz())    *               // Установка целевой системной частоты
.pclk1(36.MHz())    *                // Установка целевой частоты APB1
.pclk2(72.MHz())    *                // Установка целевой частоты APB2
.adcclk(14.MHz())   *                // Установка частоты ADC
.hclk(72.MHz())                       // Установка тактов AHB (обычно = SYSCLK)

// PLL автоматически вычисляет коэффициент умножения по целевой частоте
// Ручная настройка не требуется!
```

#### Получение информации о тактах после freeze
```rust
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,
);

// Получение фактической тактовой частоты
rprintln!("SYSCLK: {}", rcc.clocks.sysclk());   // Системные часы
rprintln!("HCLK:   {}", rcc.clocks.hclk());     // Такты AHB
rprintln!("PCLK1:  {}", rcc.clocks.pclk1());    // Такты APB1
rprintln!("PCLK2:  {}", rcc.clocks.pclk2());    // Такты APB2
rprintln!("ADCCLK: {}", rcc.clocks.adcclk());   // Такты ADC
rprintln!("USBCLK valid: {}", rcc.clocks.usbclk_valid()); // Валидность тактов USB
```

#### Зачем нужен `flash.acr`?
Скорость чтения Flash ограничена. При системной частоте выше 24 МГц необходимо вставлять циклы ожидания:

| Системная частота | Циклы ожидания Flash |
| --- | --- |
| 0-24 МГц | 0 циклов ожидания |
| 24-48 МГц | 1 цикл ожидания |
| 48-72 МГц | 2 цикла ожидания |


`freeze()` автоматически устанавливает правильные циклы ожидания в зависимости от системной частоты.

#### Разница между `constrain()` и `freeze()`
```rust
// constrain() — ограничение RCC, возвращает настраиваемый объект
// Используется для ручной настройки тактирования каждой периферии
let mut rcc = dp.RCC.constrain();

// freeze() — однократная конфигурация тактирования и фиксация
// Автоматически вычисляет все параметры деления/умножения
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,
);
```

Обычно рекомендуется использовать `freeze()`, это проще и менее подвержено ошибкам.

#### Продвинутое использование: прямое указание коэффициентов деления/умножения
```rust
// Если требуется полный контроль над конфигурацией тактирования, можно использовать RawConfig
let rcc = dp.RCC.freeze(
    rcc::RawConfig {
        hse: Some(8_000_000),       // Частота HSE
        pllmul: Some(7),            // Коэффициент умножения PLL (×9, индекс с 0)
        hpre: rcc::HPre::Div1,     // Предделитель AHB = без деления
        ppre1: rcc::PPre::Div2,    // Предделитель APB1 = AHB ÷ 2
        ppre2: rcc::PPre::Div1,    // Предделитель APB2 = без деления
        usbpre: rcc::UsbPre::Div1_5, // Предделитель USB
        adcpre: rcc::AdcPre::Div2,  // Предделитель ADC = APB2 ÷ 2
        ..Default::default()
    },
    &mut flash.acr,
);
```

---

### Типовые схемы конфигурации тактирования
#### Схема 1: Минимальная конфигурация (HSI по умолчанию)
```rust
// По умолчанию после включения: HSI 8 МГц, без PLL
// SYSCLK = 8 МГц, APB1 = 8 МГц, APB2 = 8 МГц
let mut rcc = dp.RCC.constrain();

// Или можно использовать freeze
let rcc = dp.RCC.freeze(rcc::Config::hsi(), &mut flash.acr);
```

**Применение:** мигание LED, опрос кнопок, тестирование GPIO и другие простые сценарии  
**Не подходит для:** USB, CAN, высокоскоростной UART

---

#### Схема 2: HSI с умножением до 64 МГц
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hsi()
        .sysclk(64.MHz())         // HSI × 8 = 64 МГц
        .pclk1(32.MHz())          // APB1 = AHB ÷ 2
        .pclk2(64.MHz())          // APB2 = AHB (без деления)
        .adcclk(8.MHz()),         // ADC = APB2 ÷ 8
    &mut flash.acr,
);
// Примечание: HSI можно умножить максимум до 64 МГц, не до 72 МГц
// Потому что HSI перед поступлением в PLL делится на 2: 8/2=4, 4×16=64
```

**Применение:** сценарии, где нет внешнего кварца, но требуется высокая производительность  
**Не подходит для:** USB (требуется точная частота 48 МГц)

---

#### Схема 3: HSE с умножением до 72 МГц (**Рекомендуется! Первый выбор для платы DKX**)
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())     // Использовать кварц 8 МГц на плате DKX
        .sysclk(72.MHz())         // PLL: 8 × 9 = 72 МГц
        .pclk1(36.MHz())          // APB1 = 72 ÷ 2 = 36 МГц (максимум)
        .pclk2(72.MHz())          // APB2 = 72 МГц (без деления)
        .adcclk(14.MHz()),        // ADC = 72 ÷ 6 ≈ 12 МГц (фактически делитель 6)
    &mut flash.acr,
);
```

**Применение:** практически все сценарии, конфигурация максимальной производительности  
**Тактовые частоты:**

+ SYSCLK = 72 МГц
+ AHB = 72 МГц
+ APB1 = 36 МГц
+ APB2 = 72 МГц
+ ADC = 12 МГц
+ Циклы ожидания Flash = 2

---

#### Схема 4: HSE с умножением до 48 МГц (специально для USB)
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())     // Кварц 8 МГц
        .sysclk(48.MHz())         // PLL: 8 × 6 = 48 МГц
        .pclk1(24.MHz())          // APB1 = 48 ÷ 2 = 24 МГц
        .pclk2(48.MHz()),         // APB2 = 48 МГц
    &mut flash.acr,
);

// Проверка тактов USB
assert!(rcc.clocks.usbclk_valid());  // USB требуется точная частота 48 МГц
```

**Применение:** приложения с USB  
**Тактовые частоты:**

+ SYSCLK = 48 МГц
+ USBCLK = 48 МГц ✓ (точная)
+ APB1 = 24 МГц
+ APB2 = 48 МГц
+ Циклы ожидания Flash = 1

---

#### Схема 5: 72 МГц + USB (продвинутая)
```rust
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())         // PLL: 8 × 9 = 72 МГц
        .pclk1(36.MHz())          // APB1 = 36 МГц
        .pclk2(72.MHz()),         // APB2 = 72 МГц
    &mut flash.acr,
);

// Такты USB = PLLCLK ÷ 1,5 = 72 ÷ 1,5 = 48 МГц ✓
assert!(rcc.clocks.usbclk_valid());
```

**Применение:** сценарии, где нужна и максимальная производительность, и USB

---

### 2.6 Типичные ошибки конфигурации тактирования
#### Ошибка 1: APB1 превышает 36 МГц
```rust
// ❌ Ошибка! Максимум APB1 — 36 МГц
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(72.MHz()),  // Ошибка! Превышение 36 МГц
    &mut flash.acr,
);

// ✓ Правильно
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz()),  // Правильно
    &mut flash.acr,
);
```

#### Ошибка 2: Неточные такты USB
```rust
// ❌ HSI не подходит для USB
let mut rcc = dp.RCC.freeze(
    rcc::Config::hsi().sysclk(48.MHz()),
    &mut flash.acr,
);
// Точность HSI ±1%, для USB требуется ±0,25%, это приведёт к сбою enumeration

// ✓ Необходимо использовать HSE
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(48.MHz()),
    &mut flash.acr,
);
```

#### Ошибка 3: Забыли `flash.acr`
```rust
// ❌ Отсутствует параметр flash.acr
let rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()).sysclk(72.MHz()));
// Ошибка компиляции! freeze требует два параметра

// ✓ Правильно
let mut flash = dp.FLASH.constrain();
let rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz()).sysclk(72.MHz()),
    &mut flash.acr,  // Обязательно передать!
);
```

#### Ошибка 4: SYSCLK превышает 72 МГц
```rust
// ❌ STM32F103 максимум 72 МГц
let mut rcc = dp.RCC.freeze(
    rcc::Config::hse(8.MHz())
        .sysclk(80.MHz()),  // Ошибка!
    &mut flash.acr,
);
// freeze автоматически выберет частоту, близкую к 72 МГц, но не превышающую её
```

#### Ошибка 5: Такты ADC превышают 14 МГц
```rust
// ❌ Такты ADC максимум 14 МГц
// Если pclk2 = 72 МГц и без деления для ADC, то ADC такты = 72/6 = 12 МГц ✓
// Если pclk2 = 72 МГц и ADC без деления, то ADC такты = 72 МГц ✗

// HAL библиотека обрабатывает это автоматически, но важно понимать принцип
```

---

### Таблица быстрого выбора конфигурации тактирования
| Сценарий | Конфигурация | SYSCLK | APB1 | APB2 | USB |
| --- | --- | --- | --- | --- | --- |
| LED/кнопки | `Config::hsi()` | 8 МГц | 8 МГц | 8 МГц | ✗ |
| Универсальный | `hse(8).sysclk(72)` | 72 МГц | 36 МГц | 72 МГц | ✓ |
| USB | `hse(8).sysclk(48)` | 48 МГц | 24 МГц | 48 МГц | ✓ |
| Низкое потребление | `hsi().sysclk(8)` | 8 МГц | 8 МГц | 8 МГц | ✗ |
| Без внешнего кварца | `hsi().sysclk(64)` | 64 МГц | 32 МГц | 64 МГц | ✗ |


---

## Зажигаем первый светодиод
```rust
//! Мигание светодиодом на выводе PC13 микроконтроллера STM32F103C8T6
//!
//! В этом примере предполагается, что светодиод подключён к выводу PC13,
//! как на плате Blue Pill.
//!
//! Примечание: без дополнительного оборудования не рекомендуется использовать PC13
//! для непосредственного управления светодиодом.
//! Подробнее см. раздел 5.1.2 справочного руководства. На плате Blue Pill это не проблема.

// Запрет unsafe кода для обеспечения безопасности
#![deny(unsafe_code)]
// Указание компилятору Rust не использовать стандартную библиотеку (обязательно для встраиваемых систем)
#![no_std]
// Указание компилятору Rust, что нет традиционной функции main,
// используется точка входа от cortex-m-rt
#![no_main]

// Импорт обработчика паники: при невосстановимой ошибке программы останавливает CPU
use panic_halt as _;

// Импорт модуля для неблокирующих операций
use nb::block;

// Импорт макроса точки входа из библиотеки cortex-m
use cortex_m_rt::entry;
// Импорт основных модулей HAL библиотеки
// pac: уровень доступа к периферии (Peripheral Access Crate), предоставляет доступ на уровне регистров
// prelude: предварительно импортированные часто используемые типажи, упрощает код
// timer: модуль таймеров
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use rtt_target::{rprintln, rtt_init_print};

// Определение точки входа программы, заменяет стандартную main
#[entry]
fn main() -> ! {

    rtt_init_print!();

    // Получение ядерной периферии Cortex-M (SysTick, NVIC и т.д.)
    // Метод take() гарантирует, что периферия будет получена только один раз, предотвращая повторное использование
    let cp = cortex_m::Peripherals::take().unwrap();
    // Получение специфичной для STM32F103 периферии устройства (GPIO, таймеры, UART и т.д.)
    let dp = pac::Peripherals::take().unwrap();

    // Получение и настройка контроллера сброса и тактирования (RCC)
    // Метод constrain() преобразует сырую структуру RCC в высокоуровневую абстракцию HAL
    let mut rcc = dp.RCC.constrain();

    // Получение порта GPIOC и разделение его на отдельные выводы
    // Метод split() гарантирует уникальное владение выводами, предотвращая одновременное управление
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Настройка вывода PC13 как двухтактного выхода
    // Регистр crh используется для конфигурации старших 8 выводов порта (PC8-PC15)
    // Для младших 8 выводов (PC0-PC7) следует использовать регистр crl
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    
    // Настройка системного таймера SysTick как счётчика с заданной частотой
    // counter_hz() настраивает SysTick в режим счётчика на основе частоты
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    
    // Запуск таймера с частотой 4 Гц
    timer.start(4.Hz()).unwrap();

    // Основной цикл: ожидание срабатывания таймера и переключение состояния LED
    loop {
        // Блокировка до первого срабатывания таймера
        block!(timer.wait()).unwrap();
        // Установка высокого уровня на PC13 — гашение LED (на Blue Pill LED загорается низким уровнем)
        led.set_high();
        rprintln!("OPEN THE LED");
        
        // Блокировка до второго срабатывания таймера
        block!(timer.wait()).unwrap();
        // Установка низкого уровня на PC13 — зажигание LED
        led.set_low();
        rprintln!("LOW THE LED");
    }
}
```

Вторая версия — автоматическое определение микросхемы

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
    // Получение владения периферией (можно вызвать только один раз, последующие вызовы вернут None)
    let p = pac::Peripherals::take().unwrap();

    // Ограничение регистров RCC (сброс и тактирование)
    // constrain() возвращает объект, содержащий все настраиваемые тактовые сигналы
    let mut rcc = p.RCC.constrain();

    // Разделение порта GPIOC на отдельные объекты выводов
    // split() возвращает отдельные дескрипторы каждого вывода
    let mut gpioc = p.GPIOC.split(&mut rcc);

    // Выбор различных выводов и уровней в зависимости от модели микросхемы
    cfg_select! {
        feature = "stm32f100" => {
            // STM32F100: PC9 зажигается высоким уровнем
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        feature = "stm32f101" => {
            // STM32F101: PC9 зажигается высоким уровнем
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh).set_high();
        }
        _ => {
            // STM32F103 (включая вашу плату DKX): PC13 зажигается низким уровнем
            // PC13 на плате Blue Pill/DKX подключён по схеме с общим анодом, низкий уровень = горит
            gpioc.pc13.into_push_pull_output(&mut gpioc.crh).set_high(); // На самом деле на JieLiChuang высокий уровень = горит
        }
    }

    loop {} // Сохранение состояния LED
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
    let mut flash = dp.FLASH.constrain(); // Конфигурация циклов ожидания Flash


    // Внешний кварц
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Использование внешнего кварца 8 МГц
            .sysclk(72.MHz()) // PLL умножение до 72 МГц
            .pclk1(36.MHz()) // APB1 деление до 36 МГц
            .pclk2(72.MHz()) // APB2 без деления
            .adcclk(14.MHz()), // ADC 14 МГц
        &mut flash.acr,
    );

    // Hello World
    rprintln!("Hello World!");

    loop {}
}

// Обработчик HardFault: вызывается при аппаратной ошибке
// Распространённые причины: недопустимый доступ к памяти, недопустимая инструкция, переполнение стека и т.д.
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    // ExceptionFrame содержит состояние регистров CPU в момент сбоя
    panic!("{:#?}", ef);
}

// Обработчик исключений по умолчанию: для исключений, не перехваченных другими обработчиками
#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    // irqn — номер прерывания, отрицательный = системное исключение, положительный = внешнее прерывание
    panic!("Unhandled exception (IRQn = {})", irqn);
}
```

****

**Подробное описание ключевых концепций:**

+ `cortex_m::Peripherals` — ядерная периферия Cortex-M:
    - `SYST` — таймер SysTick
    - `NVIC` — контроллер прерываний
    - `DCB` — блок отладки
    - `DWT` — блок наблюдения за данными и триггеров
+ `pac::Peripherals` — специфичная периферия микросхемы:
    - `RCC` — управление тактированием
    - `GPIOA/B/C/D` — порты GPIO
    - `USART1/2/3` — UART
    - `TIM1/2/3/4` — таймеры
    - `SPI1/2` — интерфейс SPI
    - `I2C1/2` — интерфейс I2C
    - `ADC1/2` — АЦП
    - `USB` — USB периферия
    - `CAN` — контроллер CAN
+ `Timer::syst()` — создание объекта таймера с использованием SysTick
+ `counter_hz()` — создание счётчика частоты в герцах
+ `block!()` — преобразование неблокирующей операции в блокирующую (ожидание через опрос)
+ `1.Hz()` — единица частоты из библиотеки fugit

---

## Мигание светодиодом
### Мигание с задержкой через SYST
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
    let mut flash = dp.FLASH.constrain(); // Конфигурация циклов ожидания Flash

    // Внешний кварц
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Использование внешнего кварца 8 МГц
            .sysclk(72.MHz()) // PLL умножение до 72 МГц
            .pclk1(36.MHz()) // APB1 деление до 36 МГц
            .pclk2(72.MHz()) // APB2 без деления
            .adcclk(14.MHz()), // ADC 14 МГц
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        rprintln!("Установка высокого уровня");
        led.set_high();
        delay.delay_ms(1_800_u16);

        rprintln!("Установка низкого уровня");
        led.set_low();
        delay.delay(1.secs());
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779801980743-ec54e1f1-3737-4cd9-adfa-8e4a6a99ec8e.jpeg" width="281" title="" crop="0,0,1,1" id="u62526734" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779802010301-0b3dbe14-df37-444f-9f75-1f9dcb40b870.jpeg" width="280" title="" crop="0,0,1,1" id="u6a547c40" class="ne-image">

На рисунке выше показано мигание светодиода

### Мигание светодиодом — задержка через таймер TIM2
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
    let mut flash = dp.FLASH.constrain(); // Конфигурация циклов ожидания Flash

    // Внешний кварц
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Использование внешнего кварца 8 МГц
            .sysclk(72.MHz()) // PLL умножение до 72 МГц
            .pclk1(36.MHz()) // APB1 деление до 36 МГц
            .pclk2(72.MHz()) // APB2 без деления
            .adcclk(14.MHz()), // ADC 14 МГц
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut delay = dp.TIM2.delay_us(&mut rcc); // Использование TIM2

    loop {
        rprintln!("TIM2 таймер");
        led.set_high();
        delay.delay_ms(1_800_u16);

        led.set_low();
        delay.delay(1.secs());
    }
}
```

**Ключевые концепции:**

+ `rcc::Config::hse(8.MHz())` — использование внешнего высокочастотного кварца 8 МГц (кварц на плате DKX)
+ `.sysclk(48.MHz())` — установка системной частоты 48 МГц
+ `rcc.freeze()` — фиксация конфигурации тактирования, возвращает неизменяемое состояние тактов
+ `dp.TIM2.delay_us()` — создание задержки в микросекундах с использованием TIM2
+ **Преимущество:** гибче, чем SysTick, выше точность, не влияет на другие применения SysTick



### Пояснение по задержкам
Это функция задержки — если кратко, то просто `delay.delay(20.millis());`

+ nanos() наносекунды;
+ micros() микросекунды;
+ millis() миллисекунды;
+ secs() секунды;
+ minutes() минуты;
+ hours() часы



## Управление светодиодом кнопкой
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
    let mut flash = dp.FLASH.constrain(); // Конфигурация циклов ожидания Flash

    // Внешний кварц
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Использование внешнего кварца 8 МГц
            .sysclk(72.MHz()) // PLL умножение до 72 МГц
            .pclk1(36.MHz()) // APB1 деление до 36 МГц
            .pclk2(72.MHz()) // APB2 без деления
            .adcclk(14.MHz()), // ADC 14 МГц
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

    // Отключение JTAG, освобождение PA15, PB3, PB4 как обычных GPIO
    // STM32F1 по умолчанию PA13/PA14/PA15/PB3/PB4 — выводы JTAG/SWD
    // Перед использованием как обычные GPIO необходимо освободить
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
                    "Нажата кнопка 0"
                } else {
                    "Нажата кнопка 1"
                }
            );
            match key_result {
                (true, _) => led1.toggle(),
                (_, true) => led2.toggle(),
                (_, _) => (),
            }
        } else if !key_result.0 && !key_result.1 {
            key_up = true;
            // nanos() наносекунды; micros() микросекунды; millis() миллисекунды; secs() секунды; minutes() минуты; hours() часы
            delay.delay(20.millis());
        } else {
            // rprintln!("Ошибка!");
            // delay.delay(2.secs());
        }
    }
}
```

****

**Пояснение по выводам JTAG:**

+ STM32F1 по умолчанию использует отладочный интерфейс JTAG/SWD
+ PA13(SWDIO), PA14(SWCLK), PA15(JTDI), PB3(JTDO), PB4(JNTRST) по умолчанию заняты JTAG
+ Чтобы использовать эти выводы как обычные GPIO, необходимо сначала отключить JTAG
+ Примечание: PA13/PA14 — интерфейс SWD, обычно не рекомендуется отключать (иначе отладка станет невозможной)



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1779809566789-02c7ee36-cc1e-4e52-82e7-860ae68fe4bb.png" width="662" title="" crop="0,0,1,1" id="u84f8089e" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809683134-a5da8944-f17a-4c76-bf5a-e18eb1ffe579.jpeg" width="223" title="" crop="0,0,1,1" id="ufb18b746" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809699056-1341bf76-a62b-49f6-ae96-d537e2c5d09c.jpeg" width="223" title="" crop="0,0,1,1" id="u774adcc9" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/jpeg/67055297/1779809732125-60f82ecc-dd74-49c0-bee5-c640bd90733b.jpeg" width="222" title="" crop="0,0,1,1" id="u1effb711" class="ne-image">



## Динамическое переключение GPIO (мультиплексирование портов)
В реальной разработке порты могут использоваться повторно, для этого требуется следующий шаблон

**Назначение динамического GPIO:**

+ Некоторые протоколы (например, 1-Wire, программный I2C) требуют переключения направления вывода во время выполнения
+ Мультиплексирование ограниченных выводов

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
    let mut flash = dp.FLASH.constrain(); // Конфигурация циклов ожидания Flash

    // Внешний кварц
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Использование внешнего кварца 8 МГц
            .sysclk(72.MHz()) // PLL умножение до 72 МГц
            .pclk1(36.MHz()) // APB1 деление до 36 МГц
            .pclk2(72.MHz()) // APB2 без деления
            .adcclk(14.MHz()), // ADC 14 МГц
        &mut flash.acr,
    );
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Создание динамического вывода (можно переключать между входом/выходом во время выполнения)
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



## Внешние прерывания EXTI
### Соответствие линий EXTI выводам
Каждая линия EXTI может быть одновременно подключена только к одному выводу, но выводы с одинаковым номером (например, PA0, PB0, PC0) разделяют одну и ту же линию прерывания, поэтому **нельзя одновременно использовать выводы с одинаковым номером из разных портов как источники прерывания**.

| Линия EXTI | Доступные выводы | Имя обработчика прерывания |
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

### Краткое описание ключевых шагов
Пять основных шагов для настройки внешнего прерывания с использованием STM32F1xx-HAL:

| Шаг | Действие | Метод кода | Описание |
| :--- | :--- | :--- | :--- |
| 1 | **Настройка вывода как входа** | `into_pull_up_input()` и т.д. | Выбор подтяжки к питанию/земле или плавающего входа в зависимости от внешней схемы |
| 2 | **Подключение к линии EXTI** | `make_interrupt_source(&mut syscfg)` | Подключение вывода к соответствующей линии прерывания EXTI |
| 3 | **Настройка фронта срабатывания** | `trigger_on_edge(&mut exti, Edge::RISING)` | Можно выбрать срабатывание по нарастающему, спадающему или обоим фронтам |
| 4 | **Разрешение прерывания EXTI** | `enable_interrupt(&mut exti)` | Включение данной линии прерывания в периферии EXTI |
| 5 | **Снятие маски в NVIC** | `NVIC::unmask(pac::Interrupt::EXTI0)` | Включение соответствующего канала прерывания в NVIC |


---


```rust
#![allow(clippy::empty_loop)]
// #![deny(unsafe_code)]
#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use cortex_m::asm::delay; // Импорт инструкции задержки (для антидребезга)
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

// Использование MaybeUninit для хранения неинициализированных глобальных переменных
static mut LED: MaybeUninit<stm32f1xx_hal::gpio::gpioc::PC13<Output>> = MaybeUninit::uninit();
// static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Input>> = MaybeUninit::uninit(); // Плавающий вход
static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Input<PullUp>>> =
    MaybeUninit::uninit();

/// Обработчик прерывания EXTI9_5 (обрабатывает прерывание EXTI7 для PA7, PB7, PC7 и т.д.)
#[interrupt]
fn EXTI9_5() {
    // 2021 -- Rust
    // let led = unsafe { &mut *LED.as_mut_ptr() };
    // let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };

    // 2024 -- Rust
    let led = unsafe {
        &mut *(*(&raw mut LED)).as_mut_ptr()
        // Объяснение автора --- проще говоря, возвращаем права себе
        // &mut заимствование самого себя
        // В 2024 для static mut нельзя напрямую &mut, нужно &raw для разыменования
        // *(&raw mut LED) <==> LED из 2021 года — то есть обратная развязка до самого LED
        // *(*(&raw mut LED)).as_mut_ptr() <==> *LED
        // ---------- Объяснение GPT -------------
        // &raw mut LED
        // Получение raw pointer для static mut LED
        // Тип:
        // *mut MaybeUninit<PC13<Output>>

        // *(&raw mut LED)
        // Разыменование raw pointer
        // Получение места в памяти, соответствующего LED
        // Примечание:
        // Это не "копирование значения"
        // а возврат к месту в памяти этого объекта

        // .as_mut_ptr()
        // Преобразование MaybeUninit<T>
        // в *mut T

        // *ptr
        // Разыменование *mut T
        // Получение места в памяти T (place)

        // &mut *ptr
        // Окончательное создание:
        // &mut T

        // Примечание:
        // По сути это всё равно mutable reference
        // Просто обходится:
        // &mut STATIC
        // прямой записи
    };
    let int_pin = unsafe { &mut *(*(&raw mut INT_PIN)).as_mut_ptr() };

    if int_pin.check_interrupt() {
        rprintln!("Вход в прерывание +1");
        // ====================== Основной код антидребезга ======================
        delay(72_000_000 / 1000 * 40); // Такты 72 МГц → задержка 40 мс
        // ==========================================================
        led.toggle();
        int_pin.clear_interrupt_pending_bit();
    }
}

#[entry]
fn main() -> ! {
    // Инициализация RTT отладочного вывода
    rtt_init_print!();
    rprintln!("Запуск программы...");

    let mut dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain(); // Конфигурация циклов ожидания Flash

    // Внешний кварц
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // Использование внешнего кварца 8 МГц
            .sysclk(72.MHz()) // PLL умножение до 72 МГц
            .pclk1(36.MHz()) // APB1 деление до 36 МГц
            .pclk2(72.MHz()) // APB2 без деления
            .adcclk(14.MHz()), // ADC 14 МГц
        &mut flash.acr,
    );

    rprintln!("Начало настройки");
    // Область видимости -- инициализация конфигурации прерываний
    {
        let mut gpioa = dp.GPIOA.split(&mut rcc);
        let mut gpioc = dp.GPIOC.split(&mut rcc);
        let _afio = dp.AFIO.constrain(&mut rcc);

        // LED
        let led = unsafe { &mut *(*(&raw mut LED)).as_mut_ptr() };
        *led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        // Кнопка
        let int_pin = unsafe { &mut *(*(&raw mut INT_PIN)).as_mut_ptr() };
        // *int_pin = gpioa.pa7.into_floating_input(&mut gpioa.crl); // Плавающий вход, аппаратная схема минимальной платы не поддерживает, требуется внешний конденсатор + подтяжка
        *int_pin = gpioa.pa7.into_pull_up_input(&mut gpioa.crl);

        // Подключение к прерыванию, настройка режима срабатывания: по нарастающему фронту
        int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Rising); // Нажатие — соединение с землёй
        // Разрешение прерывания для этого вывода -- включение
        int_pin.enable_interrupt(&mut dp.EXTI);
    }

    rprintln!("Настройка завершена! Устанавливаем NVIC!");
    // Снятие маски прерывания EXTI9_5 в NVIC
    // Этот шаг можно выполнять только после завершения инициализации!
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }
    rprintln!("Настройка завершена!");
    loop {}
}
```

Сложность заключается в различиях синтаксиса 2021 и 2024 годов, хотя

```rust
#![deny(unsafe_code)] 
```

тоже может решить проблему, но мой подход — это ещё один вариант!



А также автоматическое соответствие обновлениям синтаксиса компилятора

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
        rprintln!("Вход в прерывание +1");
        // ====================== Основной код антидребезга ======================
        delay(72_000_000 / 1000 * 40); // Такты 72 МГц → задержка 40 мс
        // ==========================================================
        led.toggle();
        int_pin.clear_interrupt_pending_bit();
    }
}
```



## Прерывания таймера
### Мигание светодиодом с помощью прерывания таймера
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
            rprintln!("TIM2 прерывание");
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Запуск программы");

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

    rprintln!("GPIO инициализация завершена");

    let mut timer = dp.TIM2.counter_hz(&mut rcc);
    timer.start(6.Hz()).unwrap();
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| {
        *G_TIM.borrow(cs).borrow_mut() = Some(timer);
    });

    rprintln!("TIM2 инициализация завершена");

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    rprintln!("Программа выполняется");

    loop {
        // Wait For Interrupt, CPU переходит в режим низкого энергопотребления -- в production не рекомендуется!
        // wfi(); // После запуска появляется -- rprintln! не может выводить
    }
}
```

Логическая схема

```plain
┌──────────────────────────────────────────────────┐
│                    main()                        │
│  1. Создание LED, Timer                          │
│  2. interrupt::free → перемещение в G_LED, G_TIM │
│  3. NVIC::unmask → разрешение прерывания         │
│  4. wfi() цикл ожидания                          │
└──────────────────────────────────────────────────┘
                           │
                    TIM2 срабатывание прерывания
                           │
┌──────────────────────────────────────────────────┐
│            TIM2() обработка прерывания            │
│  1. Извлечение LED/Timer из глобального хранилища │
│  2. LED.toggle()                                 │
│  3. timer.clear_interrupt()                      │
└──────────────────────────────────────────────────┘
```



### Режим прерывания таймера в RTIC
<font style="color:#DF2A3F;"><<Рекомендуется для разработки>> RTIC</font>

#### RTIC vs голое прерывание — сравнение
##### I. Сравнение архитектур
###### Голое прерывание (Bare Metal)
```rust
// Требуется вручную определить обработчик прерывания и привязать к таблице векторов
#[interrupt]
fn TIM1_UP() {
    // Ручной вход в критическую секцию (отключение прерываний)
    cortex_m::interrupt::free(|_| {
        // Весь код в одной функции, управление ресурсами — на плечах программиста
        // Нет управления приоритетами, всё через ручное включение/отключение прерываний
    });
}
```

###### Прерывание RTIC
```rust
// Декларативная привязка прерывания через #[task(binds = ...)]
#[task(binds = TIM1_UP, priority = 1, local = [led, timer])]
fn tick(cx: tick::Context) {
    // Ресурсами управляет фреймворк RTIC, безопасность гарантируется на этапе компиляции
    // Приоритетами автоматически управляет планировщик RTIC
}
```

---

##### II. Ключевые различия
| Характеристика | Голое прерывание | RTIC |
| --- | --- | --- |
| **Управление ресурсами** | Ручная защита через `critical section` | Автоматическое распределение на этапе компиляции, нулевые накладные расходы |
| **Управление приоритетами** | Ручная работа с регистрами NVIC | Декларативная настройка `priority = N` |
| **Обмен данными** | Требуется `static mut` + `unsafe` | `#[shared]` + `Mutex`, безопасность на этапе компиляции |
| **Критическая секция** | Ручное включение/отключение прерываний | RTIC автоматически генерирует оптимальную критическую секцию |
| **Привязка прерываний** | Изменение `interrupt.rs` или `device.x` | `#[task(binds = TIM1_UP)]` одной строкой |
| **Переключение контекста** | Ручное сохранение/восстановление регистров | Аппаратное сохранение в стек (Cortex-M) |
| **Организация кода** | Вся логика в одной функции прерывания | Каждая задача независима, ресурсы разделены |
| **Защита от взаимоблокировок** | Программист должен быть осторожен | Проверка на этапе компиляции (на основе протокола потолка приоритетов) |


---

##### III. Сравнение управления ресурсами
###### Голый способ: `static mut` + `unsafe`
```rust
// Глобальная изменяемая статическая переменная, требуется unsafe доступ
static mut LED_STATE: bool = false;
static mut COUNT: u8 = 0;

#[interrupt]
fn TIM1_UP() {
    unsafe {
        if LED_STATE {
            // Операции с LED...
            LED_STATE = false;
        } else {
            // Операции с LED...
            LED_STATE = true;
        }
        COUNT += 1;
    }
}
```

**Проблемы:**

+ В блоке `unsafe` компилятор не проверяет ошибки
+ Несколько прерываний, обращающихся к одной переменной, могут вызвать состояние гонки
+ Прерывание с высоким приоритетом может прервать низкоприоритетное, нарушив согласованность данных

###### Способ RTIC: `#[local]` привязка на этапе компиляции
```rust
#[task(binds = TIM1_UP, priority = 1, local = [led, led_state: bool = false, count: u8 = 0])]
fn tick(cx: tick::Context) {
    // Каждый ресурс привязан к этой задаче на этапе компиляции
    // Другие задачи не могут к нему обратиться, естественное предотвращение гонки данных
    if *cx.local.led_state {
        cx.local.led.set_high();
        *cx.local.led_state = false;
    }
    *cx.local.count += 1;
}
```

**Преимущества:**

+ Ноль `unsafe`, компилятор гарантирует корректность
+ Привязка ресурсов на этапе компиляции, нулевые накладные расходы во время выполнения
+ Доступ к ресурсам через `cx.local.xxx`

---

##### IV. Сравнение приоритетов и планирования
###### Голый способ: ручная работа с NVIC
```rust
// Требуется ручная настройка приоритетов
fn setup_timer_interrupt() {
    unsafe {
        // Установка приоритета прерывания TIM1 = 1
        // Необходимо знать адреса регистров NVIC и битовые поля
        let nvic = &*cortex_m::peripheral::NVIC::ptr();
        // Сложные операции с регистрами...
    }
}
```

###### Способ RTIC: декларативный приоритет
```rust
// priority = 1 — и готово
#[task(binds = TIM1_UP, priority = 1)]
fn tick(cx: tick::Context) { ... }

// Задача с высоким приоритетом может вытеснить низкоприоритетную
#[task(binds = USART1, priority = 2)]
fn serial(cx: serial::Context) { ... }
```

**Правила приоритетов RTIC:**

+ Чем больше число, тем выше приоритет
+ Задача с высоким приоритетом может вытеснить задачу с низким приоритетом
+ Задачи с одинаковым приоритетом не вытесняют друг друга

---

##### V. Сравнение общих ресурсов
###### Голый способ: ручная критическая секция
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
    // Забыли критическую секцию? Гонка данных! Компилятор не сообщит об ошибке!
    unsafe { SHARED_DATA += 1; }
}
```

###### Способ RTIC: `#[shared]` + автоматическая блокировка
```rust
#[shared]
struct Shared {
    data: u32,
}

#[task(binds = TIM1_UP, priority = 1, shared = [data])]
fn tick(cx: tick::Context) {
    // lock() автоматически управляет критической секцией, безопасность на этапе компиляции
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}

#[task(binds = USART1, priority = 2, shared = [data])]
fn serial(cx: serial::Context) {
    // Задача с высоким приоритетом обращается к тому же ресурсу, RTIC автоматически генерирует оптимальную блокировку
    cx.shared.data.lock(|d| {
        *d += 1;
    });
}
```

**Механизм блокировки RTIC (протокол потолка приоритетов PCP):**

+ Когда низкоприоритетная задача удерживает блокировку, высокоприоритетная задача ожидает
+ Но RTIC автоматически повышает приоритет задачи, удерживающей блокировку, чтобы предотвратить вмешательство задач со средним приоритетом
+ Всё это определяется на этапе компиляции, нулевые накладные расходы во время выполнения

---

VI. Итоги

| Сценарий | Рекомендуемый способ |
| --- | --- |
| Изучение принципов прерываний | Голый (понимание низкоуровневых механизмов) |
| Разработка реального проекта | RTIC (безопасно, эффективно, поддерживаемо) |
| Одиночное простое прерывание | Голый (объём кода примерно одинаков) |
| Много прерываний + общие ресурсы | RTIC (предотвращение гонки данных) |
| Требуется строгая реальность | RTIC (абстракции с нулевой стоимостью, детерминированное планирование) |


**Ключевые преимущества RTIC:**

+ **Нулевая стоимость абстракций**: всё определяется на этапе компиляции, без дополнительных накладных расходов во время выполнения
+ **Безопасность на этапе компиляции**: гонки данных и взаимоблокировки перехватываются на этапе компиляции
+ **Декларативное программирование**: с помощью атрибутных макросов описывается "что делать", фреймворк генерирует "как сделать"
+ **Протокол потолка приоритетов**: оптимальное управление критическими секциями

<font style="color:#DF2A3F;"></font>

#### Как использовать RTIC
Здесь нужно изменить config.toml

```bash
# Добавление пакета
[dependencies] 
....
rtic = { version = "2", features = ["thumbv7-backend"] }
```

**Причина:**

+ В исходном проекте отсутствует зависимость `rtic`, из-за чего модуль `rtic` не найден
+ RTIC v2 требует указания бэкенда, STM32F103 — архитектура Cortex-M3, используйте `thumbv7-backend`
+ Другие опциональные бэкенды: `thumbv6-backend` (Cortex-M0/M0+), `thumbv8base-backend` (Cortex-M23), `thumbv8main-backend` (Cortex-M33)



```rust
//! Мигание светодиодом с разной частотой с помощью прерываний таймера
//!
//! Предполагается, что светодиод подключён к выводу PC13 (конфигурация по умолчанию на плате Blue Pill)
//!
//! Примечание: без дополнительного оборудования не рекомендуется напрямую управлять светодиодом через PC13 (см. раздел 5.1.2 справочного руководства)
//! Но на плате Blue Pill уже есть встроенный светодиод, так что проблем нет

#![no_std]
#![no_main]

// Импорт обработчика паники, при панике CPU останавливается
// Можно установить точку останова на функции `rust_begin_unwind` для перехвата паники
use panic_halt as _;

// ==================== Точка входа RTIC приложения ====================
// #[rtic::app] — это основной макрос фреймворка RTIC для определения приложения с прерываниями реального времени
// Параметр device указывает используемый PAC (Peripheral Access Crate), здесь используется PAC из stm32f1xx_hal
#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    // Импорт макросов отладочного вывода RTT (Real-Time Transfer)
    // rtt_init_print! инициализирует канал вывода RTT
    // rprintln! выводит отладочную информацию через RTT (требуется J-Link/ST-Link и т.д.)
    use rtt_target::{rprintln, rtt_init_print};

    use stm32f1xx_hal::{
        // Типы GPIO: вывод PC13, режим вывода, состояние вывода, двухтактный выход
        gpio::{gpioc::PC13, Output, PinState, PushPull},
        // PAC (Peripheral Access Crate): низкоуровневый интерфейс для прямого доступа к регистрам
        pac,
        // prelude: предварительно импортированные типажи (например, метод .counter_ms() для таймеров)
        prelude::*,
        // Типы таймеров: CounterMs — таймер с точностью до миллисекунд, Event — перечисление событий таймера
        timer::{CounterMs, Event},
    };

    // ==================== Общие ресурсы ====================
    // Структура, помеченная #[shared], определяет ресурсы, доступные нескольким задачам
    // В этом примере нет общих ресурсов, поэтому структура пуста
    #[shared]
    struct Shared {}

    // ==================== Локальные ресурсы ====================
    // Структура, помеченная #[local], определяет ресурсы, доступные только одной задаче
    // Каждый ресурс привязывается к конкретной задаче на этапе компиляции, исключая накладные расходы на блокировку
    #[local]
    struct Local {
        // Вывод LED (PC13, двухтактный выход)
        led: PC13<Output<PushPull>>,
        // Дескриптор таймера (TIM1, точность до миллисекунд)
        timer_handler: CounterMs<pac::TIM1>,
    }

    // ==================== Функция инициализации ====================
    // Функция, помеченная #[init], выполняется один раз при запуске системы для инициализации оборудования и ресурсов
    // Возвращает кортеж (общие ресурсы, локальные ресурсы)
    // cx — это контекст RTIC, через cx.device можно получить доступ к периферии микросхемы
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Инициализация канала отладочного вывода RTT
        // После этого можно использовать rprintln! для вывода отладочной информации
        rtt_init_print!();
        rprintln!("Запуск программы: выполнение функции init");

        // Получение и ограничение периферии RCC (сброс и тактирование)
        // constrain() настраивает RCC в состояние по умолчанию, возвращает объект конфигурации тактирования
        let mut rcc = cx.device.RCC.constrain();
        rprintln!("Конфигурация тактов RCC завершена");

        // Получение периферии GPIOC и разделение её на отдельные выводы
        // split() инкапсулирует все выводы GPIOC в независимые объекты Pin
        let mut gpioc = cx.device.GPIOC.split(&mut rcc);

        // Настройка PC13 как двухтактного выхода, начальное состояние — высокий уровень (LED выключен)
        // Регистр crh используется для конфигурации выводов 8-15 (выводы 0-7 используют регистр crl)
        // PushPull (двухтактный выход): может активно устанавливать высокий или низкий уровень
        // PinState::High: начальный выход — высокий уровень (на Blue Pill LED загорается низким уровнем)
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High);
        rprintln!("Конфигурация вывода PC13 LED завершена");

        // Настройка TIM1 как счётчика с миллисекундной точностью
        // counter_ms() настраивает TIM1 как таймер с точностью до миллисекунд
        let mut timer = cx.device.TIM1.counter_ms(&mut rcc);
        // Запуск таймера, срабатывание раз в 1 секунду
        timer.start(1.secs()).unwrap();
        // Включение прерывания по обновлению таймера (Update Event)
        // Прерывание срабатывает при переполнении счётчика таймера
        timer.listen(Event::Update);
        rprintln!("Конфигурация таймера TIM1 завершена, прерывание раз в 1 секунду");

        // Возврат инициализированных ресурсов
        // Shared {}: общие ресурсы (в этом примере пусто)
        // Local { led, timer_handler }: локальные ресурсы, привязанные к соответствующей задаче
        (
            Shared {},
            Local {
                led,
                timer_handler: timer,
            },
        )
    }

    // ==================== Функция ожидания (idle) ====================
    // Функция, помеченная #[idle], выполняется постоянно, когда система простаивает (нет задач для выполнения)
    // Возвращаемый тип `!` означает, что функция никогда не завершается (бесконечный цикл)
    // Справка: https://rtic.rs/dev/book/en/by-example/app_idle.html
    // Если не объявить функцию idle, RTIC автоматически установит бит SLEEPONEXIT, переводя CPU в сон
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Вход в цикл ожидания idle, CPU ожидает прерывания...");
        loop {
            // WFI (Wait For Interrupt): перевод CPU в режим ожидания с низким энергопотреблением
            // CPU автоматически просыпается при возникновении прерывания, обрабатывает его и возвращается сюда
            //
            // Примечание: при включении wfi() отладочный вывод RTT может не обновляться нормально
            // Если нужно просматривать вывод rprintln! из idle, закомментируйте wfi()
            // cortex_m::asm::dsb();
            // cortex_m::asm::wfi();
        }
    }

    // ==================== Задача прерывания таймера ====================
    // #[task] определяет задачу, через параметр binds привязывается к конкретному аппаратному прерыванию
    // binds = TIM1_UP: привязка к прерыванию обновления TIM1 (Update Interrupt)
    // priority = 1: приоритет задачи 1 (чем больше число, тем выше приоритет)
    // local = [...]: объявление списка локальных ресурсов, включая:
    //   - led: вывод LED
    //   - timer_handler: дескриптор таймера
    //   - led_state: bool = false: состояние LED (начальное false = LED выключен)
    //   - count: u8 = 0: счётчик прерываний (начальное 0)
    #[task(binds = TIM1_UP, priority = 1, local = [led, timer_handler, led_state: bool = false, count: u8 = 0])]
    fn tick(cx: tick::Context) {
        // Переключение состояния LED
        // Если текущее состояние LED включено (led_state == true), выключить его
        // Если текущее состояние LED выключено (led_state == false), включить его
        if *cx.local.led_state {
            // set_high(): установка высокого уровня (на Blue Pill LED загорается низким уровнем, высокий = выключен)
            cx.local.led.set_high();
            *cx.local.led_state = false;
            rprintln!("[прерывание] LED выключен");
        } else {
            // set_low(): установка низкого уровня (зажигание LED)
            cx.local.led.set_low();
            *cx.local.led_state = true;
            rprintln!("[прерывание] LED включён");
        }

        // Увеличение счётчика прерываний
        // Используется для управления моментом переключения частоты таймера
        *cx.local.count += 1;
        rprintln!("[прерывание] Счёт: {}", *cx.local.count);

        // Динамическое изменение частоты срабатывания таймера
        // На 4-м прерывании: изменение таймера на 500 мс (LED мигает быстрее)
        if *cx.local.count == 4 {
            cx.local.timer_handler.start(500.millis()).unwrap();
            rprintln!("[прерывание] Таймер переключён на 500 мс");
        }
        // На 12-м прерывании: возврат таймера к 1 секунде (LED мигает медленнее)
        // и сброс счётчика для начала нового цикла
        else if *cx.local.count == 12 {
            cx.local.timer_handler.start(1.secs()).unwrap();
            *cx.local.count = 0;
            rprintln!("[прерывание] Таймер переключён на 1 с, счётчик сброшен");
        }

        // Сброс флага прерывания обновления таймера
        // Необходимо сбрасывать вручную, иначе прерывание будет срабатывать постоянно
        cx.local.timer_handler.clear_interrupt(Event::Update);
    }
}
```

Вывод:

```bash
FLASH] Programming via probe-rs...

      Erasing ✔ 100% [####################]   5.00 KiB @   6.69 KiB/s (took 1s)
  Programming ✔ 100% [####################]   5.00 KiB @   4.70 KiB/s (took 1s)                                                   Finished in 1.93s
Запуск программы: выполнение функции init
12:16:00.898: Конфигурация тактов RCC завершена
12:16:00.898: Конфигурация вывода PC13 LED завершена
12:16:00.898: Конфигурация таймера TIM1 завершена, прерывание раз в 1 секунду
12:16:00.898: Вход в цикл ожидания idle, CPU ожидает прерывания...
12:16:01.855: [прерывание] LED включён
12:16:01.855: [прерывание] Счёт: 1
12:16:02.932: [прерывание] LED выключен
12:16:02.932: [прерывание] Счёт: 2
12:16:03.896: [прерывание] LED включён
12:16:03.896: [прерывание] Счёт: 3
12:16:04.849: [прерывание] LED выключен
12:16:04.849: [прерывание] Счёт: 4
12:16:04.849: [прерывание] Таймер переключён на 500 мс
12:16:05.333: [прерывание] LED включён
12:16:05.333: [прерывание] Счёт: 5
12:16:05.934: [прерывание] LED выключен
12:16:05.934: [прерывание] Счёт: 6
12:16:06.426: [прерывание] LED включён
12:16:06.426: [прерывание] Счёт: 7
12:16:06.914: [прерывание] LED выключен
12:16:06.914: [прерывание] Счёт: 8
12:16:07.401: [прерывание] LED включён
12:16:07.401: [прерывание] Счёт: 9
12:16:07.888: [прерывание] LED выключен
12:16:07.888: [прерывание] Счёт: 10
12:16:08.377: [прерывание] LED включён
12:16:08.377: [прерывание] Счёт: 11
12:16:08.874: [прерывание] LED выключен
12:16:08.874: [прерывание] Счёт: 12
12:16:08.874: [прерывание] Таймер переключён на 1 с, счётчик сброшен
12:16:09.830: [прерывание] LED включён
12:16:09.830: [прерывание] Счёт: 1
```

  
<font style="color:#DF2A3F;">Использование hprintln может обойти проблему отсутствия вывода при cortex_m::asm::wfi();</font>

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
//! Мигание светодиодом с разной частотой с помощью прерываний таймера
//!
//! Предполагается, что светодиод подключён к выводу PC13 (конфигурация по умолчанию на плате Blue Pill)
//!
//! Примечание: без дополнительного оборудования не рекомендуется напрямую управлять светодиодом через PC13 (см. раздел 5.1.2 справочного руководства)
//! Но на плате Blue Pill уже есть встроенный светодиод, так что проблем нет

#![no_std]
#![no_main]

// Импорт обработчика паники, при панике CPU останавливается
// Можно установить точку останова на функции `rust_begin_unwind` для перехвата паники
use panic_halt as _;

// ==================== Точка входа RTIC приложения ====================
// #[rtic::app] — это основной макрос фреймворка RTIC для определения приложения с прерываниями реального времени
// Параметр device указывает используемый PAC (Peripheral Access Crate), здесь используется PAC из stm32f1xx_hal
#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    // Импорт макросов отладочного вывода semihosting
    // hprintln! выводит через отладочный интерфейс SWD в терминал OpenOCD/ST-Link
    // По сравнению с RTT не требует дополнительной настройки, но медленнее
    use cortex_m_semihosting::hprintln;

    use stm32f1xx_hal::{
        // Типы GPIO: вывод PC13, режим вывода, состояние вывода, двухтактный выход
        gpio::{gpioc::PC13, Output, PinState, PushPull},
        // PAC (Peripheral Access Crate): низкоуровневый интерфейс для прямого доступа к регистрам
        pac,
        // prelude: предварительно импортированные типажи (например, метод .counter_ms() для таймеров)
        prelude::*,
        // Типы таймеров: CounterMs — таймер с точностью до миллисекунд, Event — перечисление событий таймера
        timer::{CounterMs, Event},
    };

    // ==================== Общие ресурсы ====================
    // Структура, помеченная #[shared], определяет ресурсы, доступные нескольким задачам
    // В этом примере нет общих ресурсов, поэтому структура пуста
    #[shared]
    struct Shared {}

    // ==================== Локальные ресурсы ====================
    // Структура, помеченная #[local], определяет ресурсы, доступные только одной задаче
    // Каждый ресурс привязывается к конкретной задаче на этапе компиляции, исключая накладные расходы на блокировку
    #[local]
    struct Local {
        // Вывод LED (PC13, двухтактный выход)
        led: PC13<Output<PushPull>>,
        // Дескриптор таймера (TIM1, точность до миллисекунд)
        timer_handler: CounterMs<pac::TIM1>,
    }

    // ==================== Функция инициализации ====================
    // Функция, помеченная #[init], выполняется один раз при запуске системы для инициализации оборудования и ресурсов
    // Возвращает кортеж (общие ресурсы, локальные ресурсы)
    // cx — это контекст RTIC, через cx.device можно получить доступ к периферии микросхемы
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        hprintln!("Запуск программы: выполнение функции init");

        // Получение и ограничение периферии RCC (сброс и тактирование)
        // constrain() настраивает RCC в состояние по умолчанию, возвращает объект конфигурации тактирования
        let mut rcc = cx.device.RCC.constrain();
        hprintln!("Конфигурация тактов RCC завершена");

        // Получение периферии GPIOC и разделение её на отдельные выводы
        // split() инкапсулирует все выводы GPIOC в независимые объекты Pin
        let mut gpioc = cx.device.GPIOC.split(&mut rcc);

        // Настройка PC13 как двухтактного выхода, начальное состояние — высокий уровень (LED выключен)
        // Регистр crh используется для конфигурации выводов 8-15 (выводы 0-7 используют регистр crl)
        // PushPull (двухтактный выход): может активно устанавливать высокий или низкий уровень
        // PinState::High: начальный выход — высокий уровень (на Blue Pill LED загорается низким уровнем)
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High);
        hprintln!("Конфигурация вывода PC13 LED завершена");

        // Настройка TIM1 как счётчика с миллисекундной точностью
        // counter_ms() настраивает TIM1 как таймер с точностью до миллисекунд
        let mut timer = cx.device.TIM1.counter_ms(&mut rcc);
        // Запуск таймера, срабатывание раз в 1 секунду
        timer.start(1.secs()).unwrap();
        // Включение прерывания по обновлению таймера (Update Event)
        // Прерывание срабатывает при переполнении счётчика таймера
        timer.listen(Event::Update);
        hprintln!("Конфигурация таймера TIM1 завершена, прерывание раз в 1 секунду");

        // Возврат инициализированных ресурсов
        // Shared {}: общие ресурсы (в этом примере пусто)
        // Local { led, timer_handler }: локальные ресурсы, привязанные к соответствующей задаче
        (
            Shared {},
            Local {
                led,
                timer_handler: timer,
            },
        )
    }

    // ==================== Функция ожидания (idle) ====================
    // Функция, помеченная #[idle], выполняется постоянно, когда система простаивает (нет задач для выполнения)
    // Возвращаемый тип `!` означает, что функция никогда не завершается (бесконечный цикл)
    // Справка: https://rtic.rs/dev/book/en/by-example/app_idle.html
    // Если не объявить функцию idle, RTIC автоматически установит бит SLEEPONEXIT, переводя CPU в сон
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        hprintln!("Вход в цикл ожидания idle, CPU ожидает прерывания...");
        loop {
            // WFI (Wait For Interrupt): перевод CPU в режим ожидания с низким энергопотреблением
            // CPU автоматически просыпается при возникновении прерывания, обрабатывает его и возвращается сюда
            // cortex_m::asm::dsb();
            cortex_m::asm::wfi();
        }
    }

    // ==================== Задача прерывания таймера ====================
    // #[task] определяет задачу, через параметр binds привязывается к конкретному аппаратному прерыванию
    // binds = TIM1_UP: привязка к прерыванию обновления TIM1 (Update Interrupt)
    // priority = 1: приоритет задачи 1 (чем больше число, тем выше приоритет)
    // local = [...]: объявление списка локальных ресурсов, включая:
    //   - led: вывод LED
    //   - timer_handler: дескриптор таймера
    //   - led_state: bool = false: состояние LED (начальное false = LED выключен)
    //   - count: u8 = 0: счётчик прерываний (начальное 0)
    #[task(binds = TIM1_UP, priority = 1, local = [led, timer_handler, led_state: bool = false, count: u8 = 0])]
    fn tick(cx: tick::Context) {
        // Переключение состояния LED
        // Если текущее состояние LED включено (led_state == true), выключить его
        // Если текущее состояние LED выключено (led_state == false), включить его
        if *cx.local.led_state {
            // set_high(): установка высокого уровня (на Blue Pill LED загорается низким уровнем, высокий = выключен)
            cx.local.led.set_high();
            *cx.local.led_state = false;
            hprintln!("[прерывание] LED выключен");
        } else {
            // set_low(): установка низкого уровня (зажигание LED)
            cx.local.led.set_low();
            *cx.local.led_state = true;
            hprintln!("[прерывание] LED включён");
        }

        // Увеличение счётчика прерываний
        // Используется для управления моментом переключения частоты таймера
        *cx.local.count += 1;
        hprintln!("[прерывание] Счёт: {}", *cx.local.count);

        // Динамическое изменение частоты срабатывания таймера
        // На 4-м прерывании: изменение таймера на 500 мс (LED мигает быстрее)
        if *cx.local.count == 4 {
            cx.local.timer_handler.start(500.millis()).unwrap();
            hprintln!("[прерывание] Таймер переключён на 500 мс");
        }
        // На 12-м прерывании: возврат таймера к 1 секунде (LED мигает медленнее)
        // и сброс счётчика для начала нового цикла
        else if *cx.local.count == 12 {
            cx.local.timer_handler.start(1.secs()).unwrap();
            *cx.local.count = 0;
            hprintln!("[прерывание] Таймер переключён на 1 с, счётчик сброшен");
        }

        // Сброс флага прерывания обновления таймера
        // Необходимо сбрасывать вручную, иначе прерывание будет срабатывать постоянно
        cx.local.timer_handler.clear_interrupt(Event::Update);
    }
}
```



Вывод

```rust
     Finished in 1.39s
Запуск программы: выполнение функции init
Конфигурация тактов RCC завершена
Конфигурация вывода PC13 LED завершена
Конфигурация таймера TIM1 завершена, прерывание раз в 1 секунду
Вход в цикл ожидания idle, CPU ожидает прерывания...
[прерывание] LED включён
[прерывание] Счёт: 1
[прерывание] LED выключен
[прерывание] Счёт: 2
[прерывание] LED включён
[прерывание] Счёт: 3
```



## RTIC2 асинхронные задачи
Использование RTIC для планирования 2 задач в асинхронном режиме

```rust
//! Пример асинхронных задач RTIC2 — два async задачи, выполняющиеся параллельно
//!
//! Функциональность:
//!   - задача blink: переключение встроенного LED на PC13 каждые 500 мс
//!   - задача heartbeat: вывод пульса через RTT каждые 2 секунды
//!
//! Две задачи поочередно выполняются без блокировки, демонстрируя ключевую ценность async:
//! При использовании синхронной блокировки (например, `block!(...)`) 2-секундное ожидание heartbeat
//! заблокирует blink. А в async режиме задача автоматически уступает CPU во время ожидания,
//! позволяя планировщику выполнять другие задачи.
//!
//! Оборудование: Blue Pill (STM32F103C8T6), внешний кварц 8 МГц, LED на PC13

// ==================== Базовые атрибуты встраиваемого Rust ====================

// Во встраиваемых программах нет стандартной точки входа main
// Точка входа генерируется макросом #[rtic::app] фреймворка RTIC
#![no_main]

// Во встраиваемой среде нет операционной системы, не используется стандартная библиотека (std)
// Используется только библиотека core: Option, Result, циклы, арифметика и т.д.
#![no_std]

// ==================== Импорт зависимых crate ====================

// panic_halt: обработчик паники
// При невосстановимой ошибке программы просто останавливает CPU (Halt)
// `use ... as _` означает импорт только побочного эффекта (panic handler), без использования конкретного типа
use panic_halt as _;

// Импорт из rtic_monotonics::systick::prelude:
//   - макрос systick_monotonic!: создание типа монотонного таймера на основе SysTick
//   - типаж Monotonic: интерфейс для таймеров
//   - fugit::ExtU32: добавление методов .millis(), .secs() и т.д. для u32
use rtic_monotonics::systick::prelude::*;

// ==================== Конфигурация монотонного таймера (Monotonic) ====================

// Макрос systick_monotonic! разворачивается в структуру с именем Mono
// Внутри он:
//   1. Определяет обработчик прерывания SysTick (extern "C" fn SysTick())
//   2. Реализует типаж Monotonic, предоставляющий методы delay, now и т.д.
//
// Параметр 1_000 означает частоту тиков = 1000 Гц (т.е. одно прерывание SysTick каждые 1 мс)
// Mono::start() также требует системную тактовую частоту для вычисления reload:
//   reload = sysclk / tick_rate - 1 = 72_000_000 / 1_000 - 1 = 71_999
//   SysTick срабатывает каждые 72_000 тактов (1 мс)
//
// Примечание: первый параметр — частота тиков (Гц), а не системная частота!
//   systick_monotonic!(Mono, 1_000)    → 1 кГц, точность 1 мс ✓
//   systick_monotonic!(Mono, 48_000_000) → 48 МГц, 48 миллионов прерываний в секунду ✗ (слишком часто!)
systick_monotonic!(Mono, 1_000);

// ==================== Определение RTIC приложения ====================

// #[rtic::app] — основной макрос фреймворка RTIC, определяет приложение реального времени
//   device = stm32f1xx_hal::pac: указывает PAC микросхемы (Peripheral Access Crate)
//     PAC предоставляет низкоуровневый интерфейс доступа ко всем аппаратным регистрам
//   dispatchers = [USART1]: указывает прерывания для диспетчеризации программных задач
//     RTIC использует вектор прерывания USART1 для запуска планировщика асинхронных задач
//     USART1 выбран только потому, что мы его не используем — подойдёт любое незанятое прерывание
#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [USART1])]
mod app {
    // Отладка через RTT (Real-Time Transfer)
    // Через отладчик J-Link/ST-Link реализует неинвазивный вывод в реальном времени
    //   rtt_init_print!(): инициализация восходящего канала RTT (вызывается один раз)
    //   rprintln!(): вывод строки через RTT (аналогично println!, но без ОС)
    use rtt_target::{rprintln, rtt_init_print};

    // Импорт таймера Mono и panic_halt из родительского модуля (вне mod app)
    use super::*;

    // Типы аппаратной абстракции из stm32f1xx-hal
    use stm32f1xx_hal::{
        gpio::{Output, PC13},  // Тип вывода PC13 в режиме выхода
        prelude::*,             // Предварительно импортированные типажи: .counter_ms(), .MHz() и т.д.
        rcc::Config,            // Структура конфигурации тактирования (выбор HSE/HSI/PLL)
    };

    // ==================== Общие ресурсы ====================
    // #[shared] определяет ресурсы, доступные нескольким задачам
    // RTIC гарантирует безопасность доступа к общим ресурсам на этапе компиляции (мьютекс без блокировки на основе приоритетов)
    // В этом примере нет общих ресурсов, поэтому структура пуста
    #[shared]
    struct Shared {}

    // ==================== Локальные ресурсы ====================
    // #[local] определяет ресурсы, доступные только одной задаче
    // Каждый ресурс привязан к конкретной задаче на этапе компиляции, полностью исключая накладные расходы
    #[local]
    struct Local {
        // Вывод PC13, двухтактный режим (управление LED)
        led: PC13<Output>,
    }

    // ==================== Функция инициализации ====================
    // Функция, помеченная #[init], выполняется один раз при запуске системы (и только один раз)
    // Она выполняется до включения всех прерываний, возвращаемое значение распределяется как shared и local ресурсы
    // ctx — контекстный объект RTIC:
    //   ctx.device → периферия PAC (FLASH, RCC, GPIO и т.д.)
    //   ctx.core   → ядерная периферия Cortex-M (SYST, NVIC и т.д.)
    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        // Инициализация канала RTT, после этого rprintln! будет работать
        rtt_init_print!();
        rprintln!("Start");

        // ==================== Конфигурация системы тактирования ====================
        // Дерево тактирования STM32F103:
        //   HSE (внешний кварц 8 МГц) → PLL ×9 → SYSCLK = 72 МГц
        //                             ├→ AHB  → APB1 (72÷2 = 36 МГц, низкоскоростная периферия)
        //                             └→ AHB  → APB2 (72 МГц, высокоскоростная периферия)
        //
        // constrain() инкапсулирует аппаратные регистры в безопасные типы Rust
        // freeze() фиксирует конфигурацию тактирования, после этого её нельзя изменить
        let mut flash = ctx.device.FLASH.constrain();
        let mut rcc = ctx.device.RCC.freeze(
            Config::hse(8.MHz())  // Использование внешнего кварца 8 МГц (HSE)
                .sysclk(72.MHz()), // PLL умножение до 72 МГц (максимальная частота STM32F103)
            &mut flash.acr,       // Регистр управления доступом к Flash (требуется настройка циклов ожидания)
        );

        // Запуск монотонного таймера SysTick
        // Первый параметр: периферия SysTick (24-битный обратный счётчик, встроенный в ядро Cortex-M)
        // Второй параметр: системная тактовая частота (72 МГц), используется для вычисления времени каждого тика
        // После запуска SysTick срабатывает каждые 1 мс (определяется параметром 1_000 Гц в systick_monotonic!)
        Mono::start(ctx.core.SYST, 72_000_000);

        // ==================== Конфигурация GPIO ====================
        // split() разделяет периферию GPIOC на отдельные объекты выводов
        // into_push_pull_output() настраивает PC13 как двухтактный выход
        //   Двухтактный выход: может активно устанавливать высокий (3,3 В) или низкий (0 В) уровень
        //   Встроенный LED на Blue Pill загорается низким уровнем (active low)
        let mut gpioc = ctx.device.GPIOC.split(&mut rcc);
        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        // ==================== Запуск асинхронных задач ====================
        // spawn() отправляет асинхронную задачу планировщику RTIC
        // Задача не выполняется сразу, а ожидает планирования после возврата из init
        // .ok() игнорирует возможные ошибки (например, переполнение пула задач)
        blink::spawn().ok();
        heartbeat::spawn().ok();

        rprintln!("Запуск завершён");

        // Возврат ресурсов: общие (пусто) и локальные (вывод LED)
        // RTIC назначит led задаче, объявившей его как local
        (Shared {}, Local { led })
    }

    // ==================== Асинхронная задача 1: Мигание LED ====================
    // #[task] определяет асинхронную задачу
    //   local = [led, count: u32 = 0]:
    //     - led: получается из Local, возвращённого init
    //     - count: u32 = 0: объявление приватного счётчика задачи с начальным значением 0
    //          Такой синтаксис встроенной инициализации не требует объявления в struct Local
    //
    // async fn означает, что эта функция может приостанавливаться в .await, уступая CPU
    // Планировщик RTIC автоматически возобновит выполнение после истечения задержки
    #[task(local = [led, count: u32 = 0])]
    async fn blink(ctx: blink::Context) {
        loop {
            // toggle(): переключение уровня вывода (высокий→низкий или низкий→высокий)
            ctx.local.led.toggle();

            // ctx.local.count — приватное состояние задачи, не требует блокировки
            *ctx.local.count += 1;
            rprintln!("[blink] count={}", *ctx.local.count);

            // .delay(500.millis()).await: асинхронное ожидание 500 мс
            // Ключевой момент: не блокирует CPU!
            //   1. Монотонный таймер записывает точку пробуждения
            //   2. Текущая задача уступает CPU (состояние сохраняется в Future задачи)
            //   3. Планировщик RTIC выполняет другие готовые задачи (например, heartbeat)
            //   4. Через 500 мс прерывание SysTick пробуждает эту задачу, выполнение продолжается
            Mono::delay(500.millis()).await;
        }
    }

    // ==================== Асинхронная задача 2: Вывод пульса ====================
    // Другая независимая асинхронная задача, выполняется параллельно с blink
    // local = [beat: u32 = 0]: приватный счётчик пульса задачи
    #[task(local = [beat: u32 = 0])]
    async fn heartbeat(ctx: heartbeat::Context) {
        loop {
            *ctx.local.beat += 1;
            rprintln!("[heartbeat] beat={}", *ctx.local.beat);

            // Ожидание 2 секунды, в это время задача blink выполняется нормально
            // Если бы это была синхронная блокировка (например, пустой цикл for на 2 секунды),
            // вся система не смогла бы реагировать на другие задачи в течение этих 2 секунд.
            // А async .await просто "засыпает" на время, другие задачи не страдают.
            Mono::delay(2.secs()).await;
        }
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045924896-cc601051-f619-4596-87f6-9a2246f5de5c.png" width="581" title="" crop="0,0,1,1" id="ud3c4bf51" class="ne-image">

Как показано на рисунке выше: 4 переключения LED выполняют один пульс! Выполняются одновременно!




## Последовательный интерфейс (UART)
#### Обычный режим
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
    rprintln!("Начало теста UART");
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

    // === Конфигурация выводов USART3 (плата DKX) ===
    // TX: PB10, настроен как альтернативный двухтактный выход
    // Альтернативный двухтактный выход = GPIO управляется аппаратной периферией, а не программно
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // RX: PB11, по умолчанию плавающий вход
    let rx = gpiob.pb11;

    // Создание экземпляра последовательного интерфейса
    // USART3, скорость 115200
    let mut serial = dp
        .USART3
        .serial((tx, rx), Config::default().baudrate(115200.bps()), &mut rcc);

    // === Способ 1: Чтение/запись напрямую через объект serial ===
    // let sent = b'X';
    // block!(serial.tx.write_u8(sent)).unwrap(); // отправить байт
    // let received = block!(serial.rx.read()).unwrap(); // принять байт
    // assert_eq!(received, sent); // проверка
    // rprintln!("{}",received);
    // asm::bkpt(); // точка останова, проверить отладчиком

    // === Способ 2: Разделение на отдельные TX/RX ===
    let sent = b'Y';
    let (mut tx, mut rx) = serial.split();
    block!(tx.write_u8(sent)).unwrap();
    // let received = block!(rx.read()).unwrap();
    // block!(tx.write_u8(received)).unwrap(); // эхо
    // asm::bkpt();

    // === Способ 3: Чтение/запись через отдельные TX/RX после split ===
    // Tx/Rx в stm32f1xx_hal не поддерживает reunite,
    // После разделения можно независимо использовать tx.write_u8() и rx.read()
    // let sent = b'Z';
    // let (mut tx, mut rx) = serial.split();
    // block!(tx.write_u8(sent)).unwrap();


    loop {
        // Способ 1
        // let received = block!(serial.rx.read()).unwrap(); // принять байт
        // rprintln!("{}", received as char);
        // block!(serial.tx.write_u8(received)).unwrap(); // эхо

        // Способ 2
        let received = block!(rx.read()).unwrap();
        block!(tx.write_u8(received)).unwrap(); // эхо
        rprintln!("{}",received as char);

        // Способ 3
        // let received = block!(rx.read()).unwrap();
        // assert_eq!(received, sent);
        // block!(tx.write_u8(received)).unwrap(); // эхо
        // rprintln!("{}",received as char);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045838370-24ec16fe-462e-4267-92b7-5ef5b4c3f955.png" width="1451" title="" crop="0,0,1,1" id="u5f48f617" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780045850995-183a5b47-560b-4c32-96d1-59dd0bc6484e.png" width="372" title="" crop="0,0,1,1" id="ud58583c3" class="ne-image">

**Доступные выводы UART (F103):**

| UART | Вывод TX | Вывод RX | Примечание |
| --- | --- | --- | --- |
| USART1 | PA9 или PB6(remap) | PA10 или PB7(remap) | APB2 |
| USART2 | PA2 | PA3 | APB1 |
| USART3 | PB10 | PB11 | APB1 |


**Ключевые концепции:**

+ `into_alternate_push_pull()` — Альтернативный двухтактный выход, вывод управляется аппаратной периферией
+ `Config::default().baudrate()` — Конфигурация UART (скорость, биты данных, стоп-биты и т.д.)
+ `.split()` — Разделение на отдельные объекты `Tx` и `Rx`
+ `.reunite()` — Объединение `Tx` и `Rx` обратно



#### Режим UART_fmt
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
use core::fmt::Write;  // Импорт трейта Write

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Начало теста UART");
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
    // Форматированный вывод с помощью макроса write!
    writeln!(tx, "Hello formatted string {}", number).unwrap();
    // Перевод строки Windows: write!(tx, "Hello formatted string {}\r\n", number)


    let mut delay = dp.TIM2.delay_us(&mut rcc); // Использование TIM2

    loop {
        writeln!(tx, "Hello formatted string {}", number).unwrap();
        delay.delay_ms(2_000_u16);
        number += 1;
        rprintln!("Отладка:Hello formatted string {}",number);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047558289-b27d36c0-42f4-4c70-b44b-6278e05bc28c.png" width="492" title="" crop="0,0,1,1" id="u784935b0" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780047584627-e3e1ce36-b097-4508-959a-6fa5562f7ca5.png" width="624" title="" crop="0,0,1,1" id="uc9269666" class="ne-image">

Вышеуказанная информация, на мой взгляд, идеальна. Преимущество этого языка в том, что фреймворк готов к использованию AI для выполнения задач!



#### Прерывание UART + детекция простоя (IDLE)
В основном используется режим IDLE

```rust
// Прерывание USART3 + детекция простоя IDLE — прием данных произвольной длины и ретрансляция
//
// Принцип:
//   1. Каждый полученный байт вызывает прерывание RXNE, сохраняется в BUFFER
//   2. При простое на шине (нет новых байтов) вызывается прерывание IDLE, сигнализирующее "конец кадра"
//   3. При IDLE все данные кадра передаются обратно через TX (эхо)
//
// Использование Mutex<RefCell<>> вместо static mut, совместимость с Rust 2024 edition

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

// Глобальное общее состояние: обернуто в Mutex<RefCell<>>, безопасно используется в прерываниях и main
static RX: Mutex<RefCell<Option<Rx<USART3>>>> = Mutex::new(RefCell::new(None));
static TX: Mutex<RefCell<Option<Tx<USART3>>>> = Mutex::new(RefCell::new(None));

const BUFFER_LEN: usize = 4096;
static BUFFER: Mutex<RefCell<[u8; BUFFER_LEN]>> = Mutex::new(RefCell::new([0; BUFFER_LEN]));
static WIDX: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("UART_прерывание_IDLE");

    let p = pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut afio = p.AFIO.constrain(&mut rcc);
    let mut gpiob = p.GPIOB.split(&mut rcc);

    // Выводы USART3: PB10(TX), PB11(RX)
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    // Инициализация USART3, скорость 115200, разделение на TX/RX
    let (mut tx, mut rx) = p
        .USART3
        .remap(&mut afio.mapr)
        .serial((tx, rx), 115_200.bps(), &mut rcc)
        .split();

    // Включение источников прерываний
    tx.listen();       // TXE — прерывание по пустому регистру передачи (не используется, включено по умолчанию)
    rx.listen();       // RXNE — прерывание по непустому регистру приема (срабатывает на каждый байт)
    rx.listen_idle();  // IDLE — прерывание детекции простоя шины (срабатывает при завершении кадра)

    // Сохранение TX/RX в глобальные статические переменные внутри критической секции для использования в обработчике прерываний
    cortex_m::interrupt::free(|cs| {
        TX.borrow(cs).replace(Some(tx));
        RX.borrow(cs).replace(Some(rx));
    });

    // Разрешение прерывания USART3 в NVIC (NVIC::unmask — единственный unsafe вызов в cortex-m)
    #[allow(unsafe_code)]
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART3);
    }

    // Главный цикл: сон WFI, ожидание пробуждения по прерыванию
    loop {
        cortex_m::asm::wfi()
    }
}

/// Отправка всех байтов из buf через TX (блокирующая побайтовая отправка)
fn write(cs: &cortex_m::interrupt::CriticalSection, buf: &[u8]) {
    let mut tx_ref = TX.borrow(cs).borrow_mut();
    if let Some(tx) = tx_ref.as_mut() {
        buf.iter()
            .for_each(|w| if let Err(_err) = nb::block!(tx.write(*w)) {})
    }
}

/// Обработчик прерывания USART3
///
/// Два источника прерываний используют общий вход, различаются по флагам:
///   - RXNE (прием не пуст): побайтовое чтение и сохранение в BUFFER
///   - IDLE (шина свободна): конец кадра, ретрансляция всех полученных данных
#[interrupt]
fn USART3() {
    cortex_m::interrupt::free(|cs| {
        let mut rx_ref = RX.borrow(cs).borrow_mut();
        if let Some(rx) = rx_ref.as_mut() {
            if rx.is_rx_not_empty() {
                // RXNE: получен 1 байт, сохранение в кольцевой буфер
                if let Ok(w) = nb::block!(rx.read()) {
                    let widx = *WIDX.borrow(cs).borrow();
                    BUFFER.borrow(cs).borrow_mut()[widx] = w;
                    let new_widx = widx + 1;
                    if new_widx >= BUFFER_LEN - 1 {
                        // Буфер полон: немедленная ретрансляция блока, сброс указателя записи
                        let buf = BUFFER.borrow(cs).borrow();
                        write(cs, &buf[..new_widx]);
                        drop(buf);
                        *WIDX.borrow(cs).borrow_mut() = 0;
                    } else {
                        *WIDX.borrow(cs).borrow_mut() = new_widx;
                    }
                }
                rx.listen_idle(); // Повторное включение IDLE после каждого RXNE
            } else if rx.is_idle() {
                // IDLE: шина свободна → конец кадра, ретрансляция и очистка буфера
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



**Режим 9 бит данных, 9-й бит отмечает адрес/данные.**

```rust
// Настройка на 9 бит данных
let serial = p.USART3.serial::<PushPull>(
    (tx_pin, rx_pin),
    Config::default()
        .baudrate(9600.bps())
        .wordlength_9bits()    // 9 бит данных
        .parity_none(),        // без контроля чётности
    &mut rcc,
);

// 9-й бит = 1 — байт адреса
// 9-й бит = 0 — байт данных
block!(serial_tx.write(SLAVE_ADDR as u16 | 0x100)).unwrap();  // отправить адрес
block!(serial_tx.write(data_byte)).unwrap();                   // отправить данные
```

**Назначение:** Различение кадров адреса и данных в многоточечной связи.



#### Прием UART через DMA
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
    rprintln!("Начало теста UART DMA");
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

    // Разделение каналов DMA1 (DMA1 имеет 7 каналов)
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

    // Привязка RX UART к каналу DMA 3 (USART3 RX -> DMA1 Ch3)
    let rx_dma = serial.rx.with_dma(channels.3);
    // Привязка TX UART к каналу DMA 2 (USART3 TX -> DMA1 Ch2)
    let tx_dma = serial.tx.with_dma(channels.2);

    // Макрос singleton!: создание уникального экземпляра в статической памяти
    // DMA требует буфер со статическим временем жизни
    let rx_buf = singleton!(: [u8; 8] = [0; 8]).unwrap();

    rprintln!("Ожидание приема 8 байт данных...");
    // Запуск приема DMA (блокировка до получения 8 байт)
    let (buf, _rx) = rx_dma.read(rx_buf).wait();

    rprintln!("Прием DMA завершен!");
    for (i, byte) in buf.iter().enumerate() {
        rprintln!("buf[{}] = 0x{:02X} -> {}", i, byte, *byte as char);
    }

    // Пример отправки DMA
    let tx_buf = singleton!(: [u8; 12] = *b"Hello DMA!\r\n").unwrap();
    let (_buf, _tx) = tx_dma.write(tx_buf).wait();

    rprintln!("Отправка DMA завершена!");

    loop {}
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071793247-2af14a44-7e27-459a-8ed3-05ec1a3abc0a.png" width="342" title="" crop="0,0,1,1" id="u7dbb3119" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071860321-d37bfaff-e264-4099-a590-ef16f984731d.png" width="328" title="" crop="0,0,1,1" id="uabd331a7" class="ne-image"><img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780071685474-f076938b-fd10-4f3b-8796-4be8b770bf0e.png" width="306" title="" crop="0,0,1,1" id="u3ad590c6" class="ne-image">

Объясняет, как срабатывает после сохранения 8 бит данных!



## Сбор данных АЦП
**Ключевые параметры АЦП:**

+ Разрешение: 12 бит (0-4095)
+ Время преобразования: зависит от тактовой частоты АЦП
+ Опорное напряжение: VDDA (обычно 3.3V)
+ Формула: `Напряжение = значение / 4095 * 3.3V`

**Доступные каналы АЦП на плате DKX:**

| Вывод | Канал АЦП |
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


### Измерение внешнего напряжения
Измерение напряжения на выводе PB01

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
    rprintln!("Измерение напряжения АЦП");
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // Настройка тактов: HSE 8MHz, SYSCLK 72MHz, ADCCLK 14MHz
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    rprintln!("adc freq: {}", rcc.clocks.adcclk());

    // Инициализация ADC1
    let mut adc1 = adc::Adc::new(dp.ADC1, &mut rcc);

    // Настройка PB0 как аналогового входа
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut ch0 = gpiob.pb1.into_analog(&mut gpiob.crl);

    // Инициализация таймера SysTick
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, rcc.clocks.sysclk().raw());

    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        // Опорное напряжение 3.3V, 12-битный АЦП (0-4095)
        let voltage_mv = data as u32 * 3300 / 4095;
        let voltage_v = voltage_mv as f32 / 1000.0;
        rprintln!("adc1: {}  |  {}mV  |  {:.3}V", data, voltage_mv, voltage_v);
        delay.delay_ms(600u32);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073349544-41a745c3-faaa-44c9-8ada-70e709ed8da6.png" width="514" title="" crop="0,0,1,1" id="ub9d403d9" class="ne-image">

Земля (GND)

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073451277-a7b6496a-b345-4f55-8089-a2763cb2c721.png" width="536" title="" crop="0,0,1,1" id="u294fb1ca" class="ne-image">

Подключено к 3.3V

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780073526370-be39c8d3-3198-4e0e-81fd-9e58c6035ba3.png" width="529" title="" crop="0,0,1,1" id="u69367ae2" class="ne-image">

Напряжение в средней точке двух последовательных резисторов 1K



### Внутренний АЦП: измерение температуры
**Внутренний датчик температуры:**

+ Подключен к каналу 16 ADC1
+ Невысокая точность (±1.5°C), подходит для приблизительного контроля
+ Время преобразования: не менее 17.1μs

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
    rprintln!("Тест внутреннего датчика температуры АЦП");
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



### Циклический сбор данных АЦП через DMA
**Принцип работы циклического DMA:**

```plain
     Буфер A          Буфер B
┌─────────────┐  ┌─────────────┐
│ [0] [1] ... │  │ [0] [1] ... │
│    [7]      │  │    [7]      │
└─────────────┘  └─────────────┘
       ↑ DMA запись    ↑ DMA запись
       └── чередование ──┘

Half::First  → Буфер A готов к чтению
Half::Second → Буфер B готов к чтению
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
    rprintln!("Тест циклического сбора АЦП DMA");

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

    // Разделение канала 1 DMA1
    let dma_ch1 = dp.DMA1.split(&mut rcc).1;

    let adc1 = adc::Adc::new(dp.ADC1, &mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let adc_ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    // Привязка АЦП к DMA
    let adc_dma = adc1.with_dma(adc_ch0, dma_ch1);

    // Создание двойного буфера (циклический режим требует двух полубуферов)
    // singleton! гарантирует размещение буфера в статической памяти
    let buf = singleton!(: [[u16; 8]; 2] = [[0; 8]; 2]).unwrap();

    // Запуск циклического чтения DMA
    let mut circ_buffer = adc_dma.circ_read(buf);

    // Внимание: в циклическом режиме DMA нельзя вставлять длительные операции между вызовами readable_half()
    // Иначе DMA может завершить полный цикл, вызывая одновременную установку HTIF и TCIF → panic переполнения

    while circ_buffer.readable_half().unwrap() != Half::First {}
    let first_half = circ_buffer.peek(|half, _| *half).unwrap();

    while circ_buffer.readable_half().unwrap() != Half::Second {}
    let second_half = circ_buffer.peek(|half, _| *half).unwrap();

    rprintln!("Первая половина буфера: {:?}", first_half);
    rprintln!("Вторая половина буфера: {:?}", second_half);

    let (_buf, adc_dma) = circ_buffer.stop();
    let (_adc1, _adc_ch0, _dma_ch1) = adc_dma.split();

    rprintln!("Циклический сбор АЦП DMA завершен");
    cortex_m::asm::bkpt();
    loop {}
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780123010814-cbd8950f-c10d-4fd8-b2e0-13887353b25e.png" width="625" title="" crop="0,0,1,1" id="uc37cf9c3" class="ne-image">



## Протокол SPI
**Подробное описание режимов SPI:**

| Режим | CPOL | CPHA | Состояние тактов | Выборка данных |
| --- | --- | --- | --- | --- |
| Mode 0 | 0 | 0 | Низкий | Первый фронт |
| Mode 1 | 0 | 1 | Низкий | Второй фронт |
| Mode 2 | 1 | 0 | Высокий | Первый фронт |
| Mode 3 | 1 | 1 | Высокий | Второй фронт |


**Выводы SPI1 платы JLCPCB STM32F103C8T6:** PA5(SCK), PA6(MISO), PA7(MOSI), PA4(CS)



### Подключение дисплея ST7789
240*240

**Таблица подключения выводов**

| Вывод дисплея | Вывод MCU | Описание |
| :--- | :--- | :--- |
| SCL | PA5 | Линия тактов SPI (SPI1_SCK) |
| SDA | PA7 | Вывод данных SPI (SPI1_MOSI) |
| DC | PA0 | Выбор команда/данные |
| RES | PA1 | Аппаратный сброс |
| CS | GND | CS на землю (всегда выбран) |


> **Примечание:** "SCL" и "SDA" в таблице обычно являются сигналами шины I²C, но здесь подключен интерфейс SPI, фактически соответствующий **SCK** и **MOSI** SPI. Такое именование часто встречается в некоторых LCD-модулях, используйте по назначению согласно выводам. CS подключен к GND, что означает постоянный выбор SPI-устройства без программного управления CS.
>

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780126926273-223244e9-d923-467e-be77-c109bd054a91.png" width="272" title="" crop="0,0,1,1" id="ucea3e5ce" class="ne-image">

Код драйвера ST7789----src/st7789.rs

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

Главная программа

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
    rprintln!("Тест драйвера ST7789 240x240");

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

    rprintln!("SPI инициализирован, 16MHz");

    let mut display = ST7789::new(dc, rst);

    rprintln!("Инициализация ST7789...");
    display.init(&mut spi, &clocks).unwrap();
    rprintln!("ST7789 инициализирован");

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
    let color_names = ["Красный", "Зеленый", "Синий", "Желтый", "Голубой", "Пурпурный", "Белый", "Черный"];

    let mut color_idx: usize = 0;

    loop {
        let c = colors[color_idx % colors.len()];
        rprintln!("Заливка цветом: {} #{:04X}", color_names[color_idx % color_names.len()], c);
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



## Связь по I2C
На примере сканирования адресов!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129573451-695873a0-2895-4ae6-82d9-760b0735e6d3.png" width="547" title="" crop="0,0,1,1" id="uf3139d39" class="ne-image">

**Подключение**

| MCU | Устройство |
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

Подключение как показано на рисунке

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780129746845-ddca9982-5283-4dd1-9def-439f14f69b5e.png" width="645" title="" crop="0,0,1,1" id="u6a6d07a1" class="ne-image">



## ШИМ (PWM)
### Выход
Рассмотрим на примере управления сервоприводом через ШИМ

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
    rprintln!("\r\nТест сервопривода");
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

    // PA0 -> TIM2 CH1, 50Hz (стандартная частота сервопривода)
    let pins = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp.TIM2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 50.Hz(), &mut rcc);
    let max = pwm.get_max_duty();
    pwm.enable(Channel::C1);

    // Диапазон длительности импульса сервопривода: 0.5ms ~ 2.5ms (соответствует 0° ~ 180°)
    // Период 20ms, коэффициент заполнения = длительность импульса / период
    // duty_0   = max * 0.5 / 20  = max / 40
    // duty_180 = max * 2.5 / 20  = max / 8
    let duty_min = max / 40;   // 0.5ms → 0°
    let duty_max = max / 8;    // 2.5ms → 180°
    let step = (duty_max - duty_min) / 180;  // Приращение duty на градус

    let mut current_duty = duty_min;
    let mut direction_up = true;

    // 72MHz, задержка шага ~5ms → сервопривод завершает 0→180 за ~0.7 сек
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



### Вход
Рассмотрим на примере энкодера EC11

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780133888771-3dfd0d4c-f65c-4ded-af72-16faef0fcbed.png" width="394" title="" crop="0,0,1,1" id="ue032c32d" class="ne-image">

EC11 (с кнопкой)

**Подключение**

| MCU | Устройство |
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
    rprintln!("Запуск детекции входа ШИМ");

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

    // Отключение JTAG для освобождения PB4/PB5 (по умолчанию заняты JTAG)
    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    let pb5 = gpiob.pb5;

    // TIM3 настроен в режим входа ШИМ
    // PB4 = IC1 (захват по переднему фронту, измерение периода)
    // PB5 = IC2 (захват по заднему фронту, измерение длительности высокого уровня)
    let pwm_input = dp.TIM3.remap(&mut afio.mapr).pwm_input(
        (pb4, pb5),
        &mut dbg,
        Configuration::Frequency(10.kHz()),
        &mut rcc,
    );

    let timer_clk = pac::TIM3::timer_clock(&rcc.clocks);
    rprintln!("Тактовая частота таймера: {} Hz", timer_clk.raw());

    loop {
        match pwm_input.read_frequency(ReadMode::WaitForNextCapture, timer_clk) {
            Ok(freq) => {
                let freq_hz = freq.raw();
                match pwm_input.read_duty(ReadMode::Instant) {
                    Ok((high, period)) => {
                        let duty_pct = (high as f32 * 100.0) / period as f32;
                        rprintln!(
                            "Частота: {} Hz | Коэф. заполнения: {:.1}% ({}/{})",
                            freq_hz,
                            duty_pct,
                            high,
                            period,
                        );
                    }
                    Err(_) => {
                        rprintln!("Частота: {} Hz | Ошибка чтения коэф.заполнения", freq_hz);
                    }
                }
            }
            Err(Error::FrequencyTooLow) => {
                rprintln!("Частота сигнала слишком низкая или нет сигнала");
            }
        }
    }
}
```



Энкодер обнаруживает данные! Без вращения нет вывода!

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780134186205-bb66094b-f5f1-49a0-8f13-673b832f66e1.png" width="480" title="" crop="0,0,1,1" id="u43c0b949" class="ne-image">

Вывод информации детекции



### Чтение энкодера EC11
**Подключение**

| MCU | Устройство |
| --- | --- |
| PB6 | S1 |
| PB7 | S2 |


**Назначение:** Измерение скорости/положения квадратурных энкодеров (моторные энкодеры, поворотные ручки и т.д.).



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
    rprintln!("Тест QEI энкодера EC11");

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
        rprintln!("Импульсы: {} Направление: {:?}", elapsed, qei.direction());
    }
}
```



<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780136893674-70671b8b-3b27-429a-8a79-12fdca9cbd25.png" width="499" title="" crop="0,0,1,1" id="u76a64ffa" class="ne-image">



## Контрольная сумма CRC
**Назначение:** Проверка целостности данных, CRC в протоколах связи.

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
    rprintln!("Запуск демо CRC");

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
    rprintln!("Одно слово CRC: found={:08x}, expected={:08x}", val, 0xdf8a8a2b_u32);

    crc.reset();
    crc.write(0x00000001);
    crc.write(0x00000002);
    crc.write(0x00000003);
    let val = crc.read();
    rprintln!("Несколько слов CRC: result={:08x}", val);

    crc.reset();
    let val = crc.read();
    rprintln!("Начальное значение после сброса: {:08x} (должно быть ffffffff)", val);

    rprintln!("Демо CRC завершено");

    loop {}
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780138752178-cbb34d1f-7689-4eb2-a05c-651295f45c4d.png" width="590" title="" crop="0,0,1,1" id="u603a4730" class="ne-image">



## ЦАП (DAC)
> Примечание: STM32F103C8T6 (плата DKX) **не имеет ЦАП**, ЦАП доступен только на высокоплотных устройствах (STM32F103xC/D/E).
>

Примечание: C8T6 не поддерживается, поэтому выбираем STM32F103RCT6

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

Изменение конфигурации в config.toml

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780139752540-46050614-f41b-4e59-a165-a384a1b0b9ab.png" width="659" title="" crop="0,0,1,1" id="ud3f9d220" class="ne-image">

```rust
# stm32f1xx-hal: Аппаратный уровень абстракции для серии STM32F1
# Предоставляет высокоуровневый Rust API для периферии RCC, GPIO, TIM, USART и др.
[dependencies.stm32f1xx-hal]
version = "0.11.0"
features = [
    "stm32f103",  # Чип серии STM32F103
    "high",       # Высокая плотность (256KB Flash и выше), RCT6 относится к этому типу
]
```

Код ниже

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
    rprintln!("Тест ЦАП");

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

Результат ниже

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140722259-090abde4-4b38-4c94-b484-0cf40c1bf96a.png" width="396" title="" crop="0,0,1,1" id="u15e94679" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140832880-63b745a7-86fc-43cd-8947-01980100a6dc.png" width="560" title="" crop="0,0,1,1" id="u415533ee" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140776299-cfdce37d-a05f-46cd-a8e9-85e1fe758aa5.png" width="563" title="" crop="0,0,1,1" id="ud77ae8bf" class="ne-image">

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780140863989-0e947147-c83d-4fa5-b09c-59dbb4a05e95.png" width="565" title="" crop="0,0,1,1" id="u57f96e96" class="ne-image">

Результаты измерений в пределах допустимой погрешности



## Шина CAN (без проверки на устройстве — не тестировано)
```markdown
use bxcan::Fifo;
use bxcan::filter::Mask32;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // CAN требует внешнего кварцевого генератора для точности тактов
    let mut rcc = dp.RCC.freeze(rcc::Config::hse(8.MHz()), &mut flash.acr);

    let mut can1 = {
        let gpioa = dp.GPIOA.split(&mut rcc);
        let rx = gpioa.pa11;  // CAN RX
        let tx = gpioa.pa12;  // CAN TX

        let can = dp.CAN.can(dp.USB, (tx, rx), &mut rcc);

        // Настройка битовой синхронизации: 125kBit/s, точка выборки 87.5%
        bxcan::Can::builder(can)
            .set_bit_timing(0x001c_0003)
            .leave_disabled()
    };

    // Настройка фильтра (прием всех кадров)
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters);

    // Включение CAN
    let mut can = can1;
    block!(can.enable_non_blocking()).unwrap();

    // Тест loopback: немедленная отправка принятого кадра обратно
    loop {
        if let Ok(frame) = block!(can.receive()) {
            block!(can.transmit(&frame)).unwrap();
        }
    }
}
```

**Выводы CAN (плата DKX):** PA11(CAN RX), PA12(CAN TX) — обратите внимание, общие с USB



## USB последовательный порт (без проверки на устройстве — не тестировано)
### USB опрос (polling) (без проверки на устройстве — не тестировано)
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

    // USB должен использовать системную частоту 48MHz
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()).sysclk(48.MHz()).pclk1(24.MHz()),
        &mut flash.acr,
    );

    assert!(rcc.clocks.usbclk_valid());  // Проверка валидности тактов USB

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_high();  // Выключить светодиод

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // На линии USB D+ есть подтягивающий резистор
    // При разработке нужно притянуть D+ к GND для USB RESET
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();                           // Притянуть D+ к GND
    delay(rcc.clocks.sysclk().raw() / 100);     // Короткая задержка

    // Настройка периферии USB
    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,                                  // USB DM = PA11
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),  // USB DP = PA12
    };
    let usb_bus = UsbBus::new(usb);

    // Создание устройства CDC-ACM
    let mut serial = SerialPort::new(&usb_bus);

    // Построение USB устройства
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
                led.set_low();  // Включить светодиод
                // Преобразование полученных символов в верхний регистр и отправка обратно
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;  // 'a'~'z' → 'A'~'Z'
                    }
                }
                // Обратная запись (может потребоваться несколько записей)
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
        led.set_high();  // Выключить светодиод
    }
}
```

**Ключевые моменты USB:**

+ **Обязательно 48MHz** (протокол USB требует точной тактовой частоты)
+ PA11 = USB D-, PA12 = USB D+
+ При разработке требуется ручной сброс USB
+ VID/PID `0x16c0:0x27dd` — неофициальный ID для тестирования
+ Требуется компиляция в режиме release (в debug FLASH переполняется)

---

### USB прерывание (без проверки на устройстве — не тестировано)
Обработка USB через прерывания.

```rust
// Глобальный объект USB
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;

#[entry]
fn main() -> ! {
    // ... код инициализации USB ...

    // Включение прерываний USB
    unsafe {
        NVIC::unmask(Interrupt::USB_HP_CAN_TX);   // Высокий приоритет
        NVIC::unmask(Interrupt::USB_LP_CAN_RX0);  // Низкий приоритет
    }

    loop { wfi(); }  // Вся работа выполняется в прерываниях
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
            // Обработка полученных данных
            for c in buf[0..count].iter_mut() {
                if 0x61 <= *c && *c <= 0x7a {
                    *c &= !0x20;  // В верхний регистр
                }
            }
            serial.write(&buf[0..count]).ok();
        }
        _ => {}
    }
}
```

**Опрос (polling) vs прерывание:**

+ Опрос: просто, но CPU постоянно занят ожиданием
+ Прерывание: CPU может спать, экономия энергии



# Реальный проект
## DHT11
```rust
//! # Драйвер датчика температуры и влажности DHT11
//!
//! ## Выводы, совместимые с этим драйвером
//!
//! **Любые выводы GPIO** на STM32F103, включая:
//!
//! | Порт | Вывод       | Описание                       |
//! |------|-------------|-------------------------------|
//! | GPIOA | PA0~PA15   | Наиболее часто используемые, PA6 — по умолчанию в проекте |
//! | GPIOB | PB0~PB15   | Доступны, PB3/PB4 требуют отключения JTAG |
//! | GPIOC | PC13~PC15  | Доступны, но PC13 обычно подключен к светодиоду |
//! | GPIOD | PD0~PD15   | C8T6 выводит только PD0~PD2    |
//!
//! **Единственное ограничение:** PA13/PA14 по умолчанию являются выводами отладки SWD, PA15/PB3/PB4 — выводами JTAG.
//! Для использования этих выводов необходимо отключить мультиплексирование JTAG/SWD (через AFIO).
//!
//! ## Почему код написан так
//!
//! ### 1. Переключение двухтактный выход → плавающий вход
//!
//! DHT11 использует **протокол одиночной шины**, MCU и датчик поочередно управляют одной линией:
//!
//! ```text
//! ┌─────────┐                    ┌─────────┐
//! │   MCU   │───── DATA ────────│  DHT11  │
//! └─────────┘    (Подтягивающий резистор)      └─────────┘
//! ```
//!
//! - **Стартовый сигнал:** MCU должен активно притянуть к GND на 20ms → поднять на 30us (требуется **двухтактный выход**, способный активно управлять высоким/низким уровнем)
//! - **Чтение данных:** DHT11 управляет шиной для передачи данных (MCU должен освободить шину → **плавающий вход**, только чтение)
//!
//! Если использовать открытый сток (OpenDrain), `set_high()` просто освобождает шину (высокоимпедансное состояние),
//! Передний фронт зависит от подтягивающего резистора, медленно, DHT11 может не обнаружить передний фронт стартового сигнала.
//!
//! ### 2. Аппаратный таймер SysTick
//!
//! `cortex_m::asm::delay(n)` — это программный циклический счетчик, подвержен влиянию **flash wait states**:
//! - STM32F103 при 72MHz имеет 2 такта ожидания flash
//! - Каждая инструкция программного цикла фактически требует 2~3 такта
//! - `delay(72)` фактически может занимать 2~3us вместо 1us
//!
//! SysTick — это 24-битный аппаратный счетчик ядра Cortex-M, точно отсчитывающий по системным тактам,
//! **не подвержен влиянию flash wait states**.
//!
//! ### 3. Сохранение высокого уровня после чтения
//!
//! Состояние шины DHT11 в режиме ожидания — высокий уровень (поддерживается внешним подтягивающим резистором).
//! Если после чтения установить низкий уровень на двухтактном выходе, следующий спадающий фронт стартового сигнала не будет распознан DHT11.
//!
//! ### 4. Const Generics
//!
//! `Pin<const P: char, const N: u8, MODE>` использует const generics:
//! - `P` = имя порта ('A', 'B', 'C', 'D')
//! - `N` = номер вывода (0~15)
//! - `MODE` = тип режима (Output<PushPull>, Input<Floating> и т.д.)
//!
//! Компилятор генерирует специализированный код для каждого конкретного вывода, **нулевые накладные расходы во время выполнения**.
//! Выводы 0~7 используют регистр CRL, 8~15 — регистр CRH,
//! автоматически выбирается трейтом `HL` на этапе компиляции.

use cortex_m::peripheral::{syst::SystClkSource, SYST};
use stm32f1xx_hal::gpio::{Floating, HL, Input, Output, Pin, PinState, PushPull};

// ============================================================
//  Задержка аппаратного таймера SysTick
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
        // 72MHz: 1us = 72 тика; SysTick макс. 0xFFFFFF ≈ 233ms
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
//  Типы ошибок DHT11
// ============================================================

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    /// Нет ответа от DHT11 после стартового сигнала (не обнаружен низкий уровень подтверждения)
    NoResponse,
    /// Тайм-аут чтения бита данных (аномальное состояние шины)
    ReadTimeout,
    /// Несовпадение контрольной суммы
    Checksum { calc: u8, recv: u8 },
}

// ============================================================
//  Драйвер DHT11
// ============================================================

pub struct Dht11;

impl Dht11 {
    /// Чтение данных температуры и влажности с DHT11
    ///
    /// # Параметры
    /// - `pin` — вывод в режиме двухтактного выхода (высокий уровень в режиме ожидания)
    /// - `cr`  — ссылка на регистр управления вывода (CRL или CRH, компилятор выводит автоматически)
    /// - `delay` — таймер SysTick
    ///
    /// # Возвращает
    /// - `(Ok((влажность, температура)), pin)` — успех, влажность 0~99%RH, температура 0~50°C
    /// - `(Err(e), pin)` — ошибка
    ///
    /// Независимо от результата, `pin` возвращается вызывающему коду в режиме **двухтактного выхода (высокий уровень)**.
    ///
    /// # Пояснение ограничений типов
    ///
    /// `CR` — ассоциированный тип трейта `HL`, автоматически определяется номером вывода:
    /// - Выводы 0~7 → `Cr<P, false>` = регистр CRL
    /// - Выводы 8~15 → `Cr<P, true>` = регистр CRH
    ///
    /// Два ограничения `where` гарантируют, что тип `CR` одинаков в режимах Output и Input,
    /// так что один и тот же `&mut cr` может использоваться при переключении режимов.
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
        // ---- 1. Стартовый сигнал ведущего (двухтактный выход, активное управление) ----
        pin.set_low();
        delay.ms(20); // Притянуть к GND на 20ms (спецификация 18~30ms)
        pin.set_high();
        delay.us(30); // Поднять на 30us (спецификация 10~35us)

        // ---- 2. Переключение на плавающий вход, чтение данных DHT11 ----
        let in_pin = pin.into_floating_input(cr);

        let result = Self::read_data(&in_pin, delay);

        // ---- 3. Переключение обратно на двухтактный выход, поддержание высокого уровня на шине ----
        let out_pin =
            in_pin.into_push_pull_output_with_state(cr, PinState::High);

        (result, out_pin)
    }

    // ---- Внутренняя: ожидание ответа + чтение 40 бит данных ----

    fn read_data<const P: char, const N: u8>(
        pin: &Pin<P, N, Input<Floating>>,
        delay: &mut Delay,
    ) -> Result<(u8, u8), Error> {
        // Ответ DHT11: сначала притягивает к GND ~80us
        if !Self::wait_level(pin, false, 100, delay) {
            return Err(Error::NoResponse);
        }
        // Ответ DHT11: затем поднимает ~80us
        if !Self::wait_level(pin, true, 100, delay) {
            return Err(Error::NoResponse);
        }

        // Чтение 5 байт (40 бит)
        // Byte0 = целая часть влажности  Byte1 = дробная часть влажности
        // Byte2 = целая часть температуры  Byte3 = дробная часть температуры  Byte4 = контрольная сумма
        let mut buf = [0u8; 5];
        for slot in &mut buf {
            *slot = Self::read_byte(pin, delay).ok_or(Error::ReadTimeout)?;
        }

        // Проверка: младшие 8 бит суммы первых 4 байт == 5-й байт
        let sum = buf[0] as u32 + buf[1] as u32 + buf[2] as u32 + buf[3] as u32;
        if (sum & 0xFF) as u8 != buf[4] {
            return Err(Error::Checksum {
                calc: (sum & 0xFF) as u8,
                recv: buf[4],
            });
        }

        Ok((buf[0], buf[2])) // (влажность, температура)
    }

    // ---- Внутренняя: ожидание достижения целевого уровня на выводе ----

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

    // ---- Внутренняя: чтение одного байта (MSB первым) ----
    //
    // Временная диаграмма каждого бита:
    //   ┌──── 50us ────┐┌── 28us(0) или 70us(1) ───┐
    //   │  Низкий ур.  ││      Высокий ур.        │
    //   └───────────────┘└──────────────────────────┘
    //
    // Стратегия выборки: ожидание фронта низкий→высокий, задержка 40us, затем выборка
    //   "0": высокий уровень ~28us → через 40us уже низкий → читаем 0
    //   "1": высокий уровень ~70us → через 40us все еще высокий → читаем 1

    fn read_byte<const P: char, const N: u8>(
        pin: &Pin<P, N, Input<Floating>>,
        delay: &mut Delay,
    ) -> Option<u8> {
        let mut byte: u8 = 0;
        for _ in 0..8 {
            byte <<= 1;
            // Ожидание низкого уровня (каждый бит начинается с ~50us низкого уровня)
            if !Self::wait_level(pin, false, 70, delay) {
                return None;
            }
            // Ожидание высокого уровня (DHT11 освобождает шину)
            if !Self::wait_level(pin, true, 70, delay) {
                return None;
            }
            // Задержка 40us, затем выборка
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
    rprintln!("Датчик DHT11 температуры и влажности - PA6");

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

    // PA6 двухтактный выход, начальный высокий уровень
    let mut pin = gpioa.pa6.into_push_pull_output_with_state(
        &mut gpioa.crl,
        stm32f1xx_hal::gpio::PinState::High,
    );

    delay.ms(1500);
    rprintln!("DHT11 инициализирован, начало сбора...");

    loop {
        // Dht11::read принимает вывод двухтактного выхода, возвращает вывод двухтактного выхода
        let (result, returned_pin) = Dht11::read(pin, &mut gpioa.crl, &mut delay);
        pin = returned_pin;

        match result {
            Ok((humi, temp)) => {
                rprintln!("Влажность: {}%RH, Температура: {}C", humi, temp);
            }
            Err(e) => {
                rprintln!("Ошибка чтения DHT11: {:?}", e);
            }
        }

        delay.ms(2000);
    }
}
```

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780160254218-22ef0a3f-9b61-4350-b558-f75ed994c24f.png" width="450" title="" crop="0,0,1,1" id="uecb29377" class="ne-image">



## DHT11+ST7789 ЖК-термометр
| Дисплей & DHT11 | Вывод MCU | Описание |
| :--- | :--- | :--- |
| SCL | PA5 | Линия тактов SPI (SPI1_SCK) |
| SDA | PA7 | Вывод данных SPI (SPI1_MOSI) |
| DC | PA0 | Выбор команда/данные |
| RES | PA1 | Аппаратный сброс |
| CS | GND | CS на землю (всегда выбран) |
| DATA | PA6 | Линия данных DHT11 |


Фактический результат

<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780164028402-54ee6f36-3789-4788-b616-7f4f67a666ce.png" width="628" title="" crop="0,0,1,1" id="u132cff35" class="ne-image">

Структура проекта

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
    ('\u{5EA6}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0xC0, 0x1F, 0xF8, 0xFF, 0xFF, 0x03, 0xF8, 0xFF, 0x01, 0x08, 0x00, 0x00, 0x00, 0x10, 0x30, 0x02, 0x00, 0x18, 0x30, 0x02, 0x00, 0x18, 0x10, 0x82, 0x01, 0x08, 0x10, 0x02, 0x07, 0x0C, 0x10, 0x02, 0x1C, 0x0C, 0xD0, 0x7F, 0x32, 0x04, 0x10, 0x12, 0x62, 0x06, 0x10, 0x12, 0xC2, 0x02, 0x17, 0x12, 0x82, 0x03, 0x10, 0x13, 0x02, 0x03, 0x10, 0x13, 0x82, 0x03, 0x10, 0x13, 0xC2, 0x02, 0x10, 0x11, 0x62, 0x04, 0x08, 0x7F, 0x32, 0x04, 0xC8, 0x3F, 0x1E, 0x08, 0x48, 0x01, 0x06, 0x08, 0x08, 0x01, 0x00, 0x08, 0x08, 0x01, 0x00, 0x08, 0x08, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // градус
    ('\u{6E29}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x10, 0x00, 0x0C, 0x00, 0x3C, 0x08, 0x18, 0x00, 0x0F, 0x18, 0x38, 0xC0, 0x03, 0x38, 0x30, 0x7C, 0x00, 0x70, 0xC0, 0x03, 0x00, 0x60, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x07, 0x10, 0x0C, 0x20, 0xFE, 0x17, 0x38, 0x7F, 0x02, 0x10, 0xF0, 0x17, 0x02, 0x10, 0x10, 0x31, 0x02, 0x10, 0x10, 0x31, 0x02, 0x10, 0x10, 0x31, 0xFE, 0x0F, 0x10, 0x11, 0x02, 0x08, 0x90, 0x11, 0x02, 0x08, 0x90, 0x11, 0x02, 0x0A, 0x90, 0x11, 0xFF, 0x0F, 0x90, 0x11, 0xFF, 0x0F, 0x18, 0x10, 0x03, 0x08, 0xF8, 0x7F, 0x03, 0x08, 0x00, 0x70, 0xFF, 0x0F, 0x00, 0x00, 0x0F, 0x0F, 0x00, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00]), // температура
    ('\u{6E7F}', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x18, 0x08, 0x0C, 0x00, 0x1E, 0x18, 0x18, 0x80, 0x07, 0x30, 0x30, 0x78, 0x00, 0x30, 0x20, 0x07, 0x00, 0x60, 0x80, 0x00, 0x10, 0xC0, 0x00, 0x00, 0x10, 0x00, 0x30, 0x06, 0x10, 0x0E, 0x7C, 0x1E, 0x18, 0xF8, 0x1F, 0xF8, 0x18, 0xE0, 0x31, 0xC0, 0x19, 0x08, 0x11, 0x00, 0x09, 0x88, 0xD1, 0xFF, 0x0F, 0x88, 0xD1, 0xFF, 0x0B, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x08, 0x88, 0x18, 0x00, 0x0B, 0x88, 0xD8, 0xFF, 0x0B, 0xC8, 0x18, 0x00, 0x08, 0x88, 0x18, 0x80, 0x09, 0xFC, 0x3F, 0xE0, 0x08, 0xFC, 0x7F, 0x38, 0x08, 0x00, 0x70, 0x0E, 0x08, 0x00, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // влажность
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

Не спрашивайте, почему Python — потому что это быстро!



## DHT20+ST7789 ЖК-термометр
<img src="https://cdn.nlark.com/yuque/0/2026/png/67055297/1780166394105-ec6c2ced-4711-4e04-9992-2d3939656899.png" width="547" title="" crop="0,0,1,1" id="u7f142962" class="ne-image">

| Дисплей & DHT20 | Вывод MCU | Описание |
| :--- | :--- | :--- |
| SCL (дисплей) | PA5 | Линия тактов SPI (SPI1_SCK) |
| SDA (дисплей) | PA7 | Вывод данных SPI (SPI1_MOSI) |
| DC | PA0 | Выбор команда/данные |
| RES | PA1 | Аппаратный сброс |
| CS | GND | CS на землю (всегда выбран) |
| SCL (DHT20) | PB7 | Линия тактов DHT20 |
| SDA (DHT20) | PB6 | Линия данных DHT20 |


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



# Заключение
Email автора: pycx0@qq.com  
На самом деле автор делал этот проект на последнем курсе перед выпуском, потому что давление на рынке труда в Китае слишком велико. Сейчас я ищу работу; если не получится, возможно, пойду на завод работать в две смены!  
Последующие обновления будут по оригинальной ссылке! Надеюсь найти хорошую работу!

Надеюсь, экосистема Rust будет становиться всё лучше! Вперёд, «жители» глобальной деревни!
