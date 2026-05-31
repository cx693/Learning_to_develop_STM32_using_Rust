//! # cargo-ocd
//!
//! Cargo 子命令，用于通过 OpenOCD 一键烧录和调试嵌入式固件。
//!
//! 支持**任何 OpenOCD 兼容的目标设备**（STM32、NXP、AVR、RISC-V 等）
//! 和**任何 OpenOCD 支持的下载器**（CMSIS-DAP、ST-Link、J-Link 等）。
//!
//! ## 安装
//!
//! ```bash
//! cargo install cargo-ocd
//! ```
//!
//! ## 使用
//!
//! ```bash
//! # Debug 模式编译 + 烧录
//! cargo ocd
//!
//! # Release 模式编译 + 烧录
//! cargo ocd --release
//!
//! # Debug 编译 + 烧录 + GDB 调试
//! cargo ocd d
//! ```
//!
//! ## 配置
//!
//! 在项目的 `Cargo.toml` 中添加 `[package.metadata.ocd]` 段：
//!
//! ```toml
//! [package.metadata.ocd]
//! interface = "interface/cmsis-dap.cfg"   # 下载器配置
//! target = "target/stm32f1x.cfg"          # 芯片配置
//! target-triple = "thumbv7m-none-eabi"    # Rust 编译目标
//! ```
//!
//! 支持 CMSIS-DAP / ST-Link / J-Link 等所有 OpenOCD 支持的下载器。
//! 支持任何 OpenOCD 兼容的目标芯片（通过 target 和 target-triple 配置）。

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io::Read};

fn main() {
    // cargo-ocd 作为 cargo 子命令运行时，cargo 会把子命令参数传过来
    // 例如: cargo ocd --release → args: ["cargo-ocd", "ocd", "--release"]
    // 注意：args[1] 是子命令名 "ocd"，需要跳过
    // 工作目录是用户运行命令的目录（即工作区根目录）
    let args: Vec<String> = std::env::args().collect();

    // 检测子命令
    let subcommand = if args.len() > 2 {
        let first = &args[2];
        if first == "debug" || first == "d" {
            Some("debug")
        } else {
            None
        }
    } else {
        None
    };

    // 处理 --help / -h
    if args.len() > 2 {
        let sub_args: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
        if sub_args.contains(&"--help") || sub_args.contains(&"-h") {
            print_help();
            return;
        }
    }

    // 读取 Cargo.toml 配置
    let config = load_config("Cargo.toml");
    let pkg_name = config.pkg_name.clone();
    let target_triple = config.target_triple.clone();

    // 确定编译参数（跳过子命令本身）
    let mut cargo_args = vec!["build".to_string()];
    let mut release = false;
    let mut user_gdb: Option<String> = None; // 用户通过 --gdb / --rust-gdb 指定的 GDB
    let mut gdb_port: u16 = 3333; // GDB 端口，默认 3333

    let mut i = 2;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--release" {
            release = true;
            cargo_args.push("--release".to_string());
        } else if arg == "--help" || arg == "-h" {
            // 已在上方处理
        } else if arg == "debug" || arg == "d" {
            // 子命令，跳过
        } else if arg == "--gdb" || arg == "--rust-gdb" {
            // 用户指定 GDB，提取名称
            let gdb_name = if arg == "--gdb" {
                "gdb"
            } else {
                "rust-gdb"
            };
            // 检查指定的 GDB 是否存在
            if !check_gdb(gdb_name) {
                eprintln!("[ERROR] 指定的 GDB '{}' 未找到或不可执行", gdb_name);
                eprintln!("  请确保已安装后再使用 --{}", if arg == "--gdb" { "gdb" } else { "rust-gdb" });
                std::process::exit(1);
            }
            user_gdb = Some(gdb_name.to_string());
        } else if arg == "--port" {
            // 用户指定 GDB 端口
            i += 1;
            if i >= args.len() {
                eprintln!("[ERROR] --port 参数需要指定端口号");
                eprintln!("  用法: cargo ocd d --port 3333");
                std::process::exit(1);
            }
            match args[i].parse::<u16>() {
                Ok(port) => gdb_port = port,
                Err(_) => {
                    eprintln!("[ERROR] 无效的端口号: '{}'", args[i]);
                    eprintln!("  端口号应为 1-65535 之间的数字");
                    std::process::exit(1);
                }
            }
        } else {
            cargo_args.push(arg.clone());
        }
        i += 1;
    }

    // 步骤 1: 编译固件
    println!("[BUILD] Compiling firmware...");
    let status = Command::new("cargo")
        .env("RUSTFLAGS", "-C link-arg=-Tlink.x")
        .args(&cargo_args)
        .arg("--target")
        .arg(&target_triple)
        .status()
        .expect("编译失败");

    if !status.success() {
        eprintln!("[ERROR] Build failed");
        std::process::exit(1);
    }

    // 步骤 2: 确定 ELF 文件路径
    let target_dir = if release {
        format!("target/{}/release", target_triple)
    } else {
        format!("target/{}/debug", target_triple)
    };

    let elf_path = PathBuf::from(&target_dir).join(&pkg_name);
    if !elf_path.exists() {
        eprintln!("[ERROR] Firmware not found: {:?}", elf_path);
        std::process::exit(1);
    }

    // 步骤 3: 显示固件大小
    println!();
    show_firmware_size(&elf_path);

    match subcommand {
        Some("debug") => {
            if release {
                eprintln!("[ERROR] Release 模式不支持调试，请使用 Debug 模式");
                eprintln!("  cargo ocd d    # Debug 编译 + 烧录 + GDB 调试");
                std::process::exit(1);
            }
            run_debug(&config, &elf_path, user_gdb, gdb_port);
        }
        _ => run_flash(&config, &elf_path),
    }
}

/// 烧录模式：编译 → 烧录 → 退出
fn run_flash(config: &OcdConfig, elf_path: &Path) {
    println!();
    let elf_str = elf_path.to_string_lossy().replace('\\', "/");
    println!("[FLASH] Firmware: {}", elf_str);
    println!("[FLASH] Programming via OpenOCD...");

    let status = Command::new("openocd")
        .args(&[
            "-f",
            &config.interface,
            "-f",
            &config.target_chip,
            "-c",
            &format!("program {} verify reset exit", elf_str),
        ])
        .status()
        .expect("无法执行 openocd，请确保已安装");

    if !status.success() {
        eprintln!("[ERROR] Flash failed");
        std::process::exit(1);
    }

    println!();
    println!("[DONE] Flash complete!");
}

/// 调试模式：编译 → 烧录 → 启动 OpenOCD GDB 服务器 → 启动 GDB
fn run_debug(config: &OcdConfig, elf_path: &Path, user_gdb: Option<String>, gdb_port: u16) {
    // 根据 target-triple 判断目标架构，用于选择合适的 GDB
    let target_arch = detect_target_arch(&config.target_triple);
    println!();
    let elf_str = elf_path.to_string_lossy().replace('\\', "/");
    println!("[DEBUG] Firmware: {}", elf_str);
    println!("[DEBUG] Programming & starting GDB server...");

    // 先烧录固件
    let flash_status = Command::new("openocd")
        .args(&[
            "-f",
            &config.interface,
            "-f",
            &config.target_chip,
            "-c",
            &format!("program {} verify reset exit", elf_str),
        ])
        .status()
        .expect("无法执行 openocd，请确保已安装");

    if !flash_status.success() {
        eprintln!("[ERROR] Flash failed");
        std::process::exit(1);
    }

    println!();
    println!("[DEBUG] Starting OpenOCD GDB server on port {}...", gdb_port);
    println!("[DEBUG] Connect GDB with: target remote :{}", gdb_port);
    println!("[DEBUG] ELF file: {}", elf_str);
    println!();

    // 启动 OpenOCD GDB 服务器（保持运行）
    // 将 gdb_port 放在最前面，确保 GDB 服务器在正确的端口上启动
    // 然后再执行 program 和 reset halt
    let mut openocd = Command::new("openocd")
        .args(&[
            "-f",
            &config.interface,
            "-f",
            &config.target_chip,
            "-c",
            &format!("gdb_port {}", gdb_port),
            "-c",
            &format!("program {}", elf_str),
            "-c",
            "reset halt",
        ])
        .spawn()
        .expect("无法执行 openocd，请确保已安装");

    // 等待 OpenOCD 启动（给足时间，特别是 Linux 上某些 USB 设备需要更长时间初始化）
    // 先等待 2 秒让 OpenOCD 初始化
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 轮询等待 GDB 端口就绪，最多等待 10 秒
    eprintln!("[DEBUG] 等待 GDB 服务器端口 {} 就绪...", gdb_port);
    let max_wait = std::time::Duration::from_secs(10);
    let poll_interval = std::time::Duration::from_millis(200);
    let start = std::time::Instant::now();
    let mut port_ready = false;

    while start.elapsed() < max_wait {
        if std::net::TcpStream::connect(("127.0.0.1", gdb_port)).is_ok() {
            port_ready = true;
            break;
        }
        std::thread::sleep(poll_interval);
    }

    if port_ready {
        eprintln!("[DEBUG] GDB 服务器端口 {} 已就绪（耗时 {:.1}s）", gdb_port, start.elapsed().as_secs_f64());
    } else {
        eprintln!("[WARN] GDB 服务器端口 {} 在 {:.0}s 内未就绪，仍尝试连接...", gdb_port, max_wait.as_secs_f64());
    }

    // 查找可用的 GDB（如果用户指定了则直接使用，否则根据架构和 OS 自动检测）
    let gdb = match user_gdb {
        Some(name) => name,
        None => find_gdb(&target_arch),
    };

    println!("[DEBUG] Starting GDB: {}", gdb);
    println!("[DEBUG] Auto-executing: target remote :{}, break main, continue", gdb_port);
    println!("[DEBUG] 已自动在 main() 设置断点，程序将在 main 入口处暂停");
    println!("[DEBUG] 之后可使用 GDB 命令单步调试（见文档 9.3 节）");
    println!();

    // 构建 GDB 启动参数
    // 添加 GDB 命令：连接到 OpenOCD → 设置断点 → 运行到 main
    let gdb_args: Vec<String> = vec![
        "-ex".to_string(),
        format!("target remote :{}", gdb_port),
        "-ex".to_string(),
        "break main".to_string(),
        "-ex".to_string(),
        "continue".to_string(),
        elf_str.to_string(),
    ];

    // 启动 GDB，自动执行：
    //   1. target remote :{port}  - 连接到 OpenOCD
    //   2. break main            - 在 main() 设置断点
    //   3. continue              - 运行到 main() 断点处暂停
    let gdb_status = Command::new(&gdb)
        .args(&gdb_args)
        .status()
        .expect("无法启动 GDB，请确保已安装 gdb 或 rust-gdb 最后考虑 arm-none-eabi-gdb");

    // GDB 退出后，关闭 OpenOCD
    let _ = openocd.kill();
    println!("[DEBUG] Debug session ended.");

    if !gdb_status.success() {
        eprintln!();
        eprintln!("[ERROR] GDB exited with error.");
        eprintln!("提示: 请确保已安装 GDB 调试器");
        let os = std::env::consts::OS;
        match os {
            "macos" => {
                eprintln!("  macOS 推荐: brew install gdb");
            }
            "windows" => {
                eprintln!("  Windows 推荐: arm-none-eabi-gdb（ARM GCC 工具链）");
                eprintln!("  下载: https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads");
            }
            "linux" => {
                eprintln!("  Linux 推荐: sudo apt install gdb-arm-none-eabi");
            }
            _ => {
                eprintln!("  请安装 GDB（如 gdb-multiarch、rust-gdb 或 arm-none-eabi-gdb）");
            }
        }
        eprintln!();
        std::process::exit(1);
    }
}

/// 根据 target-triple 检测目标架构类型
///
/// 返回 "arm"（ARM Cortex-M/R/A）、"riscv"（RISC-V）或 "unknown"
fn detect_target_arch(target_triple: &str) -> &str {
    if target_triple.starts_with("thumbv")
        || target_triple.starts_with("armv")
        || target_triple.starts_with("arm")
    {
        "arm"
    } else if target_triple.starts_with("riscv") {
        "riscv"
    } else {
        "unknown"
    }
}

/// 检测 GDB 是否可执行
fn check_gdb(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
}

/// 根据目标架构和操作系统自动选择并查找可用的 GDB
///
/// ARM 架构各系统 GDB 优先级：
/// - macOS:   rust-gdb > gdb > arm-none-eabi-gdb（仅这 3 个）
/// - Windows: arm-none-eabi-gdb（仅这 1 个）
/// - Linux:   gdb-multiarch > arm-none-eabi-gdb（仅这 2 个）
///
/// RISC-V 架构各系统 GDB 优先级：
/// - macOS:   riscv64-unknown-elf-gdb > rust-gdb > gdb（仅这 3 个）
/// - Windows: riscv64-unknown-elf-gdb（仅这 1 个）
/// - Linux:   riscv64-unknown-elf-gdb > gdb-multiarch（仅这 2 个）
///
/// 用户可通过 `--gdb` 或 `--rust-gdb` 参数手动指定。
/// 如果所有候选 GDB 都不可用，给出明确的安装提示并退出。
fn find_gdb(target_arch: &str) -> String {
    let os = std::env::consts::OS;

    // 根据目标架构和操作系统确定 GDB 候选列表（按优先级排列）
    let candidates: &[&str] = match (target_arch, os) {
        // ARM 架构
        ("arm", "macos") => &["rust-gdb", "gdb", "arm-none-eabi-gdb"],
        ("arm", "windows") => &["arm-none-eabi-gdb"],
        ("arm", "linux") => &["arm-none-eabi-gdb"],
        // RISC-V 架构
        ("riscv", "macos") => &["riscv64-unknown-elf-gdb", "rust-gdb", "gdb"],
        ("riscv", "windows") => &["riscv64-unknown-elf-gdb"],
        ("riscv", "linux") => &["riscv64-unknown-elf-gdb"],
        // 未知架构，回退到通用列表
        (_, "macos") => &["rust-gdb", "gdb", "arm-none-eabi-gdb"],
        (_, "windows") => &["arm-none-eabi-gdb"],
        (_, "linux") => &["arm-none-eabi-gdb"],
        _ => &["rust-gdb", "gdb", "arm-none-eabi-gdb"],
    };

    // 按优先级依次检查
    for name in candidates {
        if check_gdb(name) {
            return name.to_string();
        }
    }

    // 所有 GDB 都不可用，给出明确的安装提示
    eprintln!();
    eprintln!("[ERROR] 未找到任何可用的 GDB 调试器！");
    eprintln!();
    match (target_arch, os) {
        ("arm", "macos") => {
            eprintln!("  ARM 目标 | macOS 系统推荐使用系统 GDB：");
            eprintln!("    brew install gdb");
            eprintln!();
            eprintln!("  或使用 arm-none-eabi-gdb（ARM 官方工具链）");
        }
        ("arm", "windows") => {
            eprintln!("  ARM 目标 | Windows 系统请安装 arm-none-eabi-gdb：");
            eprintln!("  从 ARM 官网下载 ARM GCC 工具链：");
            eprintln!("    https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads");
            eprintln!();
            eprintln!("  或通过 MSYS2 安装：");
            eprintln!("    pacman -S mingw-w64-x86_64-arm-none-eabi-gdb");
        }
        ("arm", "linux") => {
            eprintln!("  ARM 目标 | Linux 系统请安装 arm-none-eabi-gdb：");
            eprintln!("    sudo apt install gdb-arm-none-eabi");
            eprintln!();
            eprintln!("  或从 ARM 官网下载 ARM GCC 工具链：");
            eprintln!("    https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads");
        }
        ("riscv", "macos") => {
            eprintln!("  RISC-V 目标 | macOS 系统推荐使用 riscv64-unknown-elf-gdb：");
            eprintln!("  通过 Homebrew 安装 RISC-V 工具链：");
            eprintln!("    brew install riscv64-elf-gdb");
            eprintln!();
            eprintln!("  或使用 xPack 发布的 RISC-V GDB：");
            eprintln!("    https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases");
            eprintln!();
            eprintln!("  也可尝试系统 GDB（功能可能受限）：");
            eprintln!("    brew install gdb");
        }
        ("riscv", "windows") => {
            eprintln!("  RISC-V 目标 | Windows 系统请安装 riscv64-unknown-elf-gdb：");
            eprintln!("  从 xPack 下载 RISC-V 工具链：");
            eprintln!("    https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases");
            eprintln!();
            eprintln!("  或通过 MSYS2 安装：");
            eprintln!("    pacman -S mingw-w64-x86_64-riscv64-unknown-elf-gdb");
        }
        ("riscv", "linux") => {
            eprintln!("  RISC-V 目标 | Linux 系统请安装 riscv64-unknown-elf-gdb：");
            eprintln!("    sudo apt install gdb-riscv64-unknown-elf");
            eprintln!();
            eprintln!("  或从 xPack 下载 RISC-V 工具链：");
            eprintln!("    https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases");
        }
        (_, "macos") => {
            eprintln!("  macOS 系统推荐使用系统 GDB：");
            eprintln!("    brew install gdb");
        }
        (_, "windows") => {
            eprintln!("  Windows 系统请安装 arm-none-eabi-gdb：");
            eprintln!("  从 ARM 官网下载 ARM GCC 工具链：");
            eprintln!("    https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads");
        }
        (_, "linux") => {
            eprintln!("  Linux 系统请安装 arm-none-eabi-gdb：");
            eprintln!("    sudo apt install gdb-arm-none-eabi");
        }
        _ => {
            eprintln!("  请安装 GDB 调试器（如 rust-gdb、gdb-multiarch 或 arm-none-eabi-gdb）");
        }
    }
    eprintln!();
    eprintln!("  或使用 --gdb / --rust-gdb 参数手动指定已安装的 GDB：");
    eprintln!("    cargo ocd d --gdb");
    eprintln!("    cargo ocd d --rust-gdb");
    eprintln!();
    std::process::exit(1);
}

// ============================================================
// 配置解析
// ============================================================

/// OpenOCD 配置
struct OcdConfig {
    pkg_name: String,
    interface: String,
    target_chip: String,
    target_triple: String,
}

/// 从 Cargo.toml 读取配置
fn load_config(cargo_toml_path: &str) -> OcdConfig {
    let content = fs::read_to_string(cargo_toml_path).unwrap_or_else(|_| {
        eprintln!("[ERROR] Cargo.toml not found. Run this command from the project root.");
        std::process::exit(1);
    });

    let lines: Vec<&str> = content.lines().collect();

    // 解析包名
    let pkg_name = parse_toml_value(&lines, "name")
        .unwrap_or_else(|| "firmware".to_string());

    // 解析 [package.metadata.ocd] 段
    let ocd_section = extract_section(&lines, "[package.metadata.ocd]");

    let interface = parse_section_value(&ocd_section, "interface")
        .unwrap_or_else(|| "interface/cmsis-dap.cfg".to_string());

    let target_chip = parse_section_value(&ocd_section, "target")
        .unwrap_or_else(|| "target/stm32f1x.cfg".to_string());

    let target_triple = parse_section_value(&ocd_section, "target-triple")
        .unwrap_or_else(|| "thumbv7m-none-eabi".to_string());

    OcdConfig {
        pkg_name,
        interface,
        target_chip,
        target_triple,
    }
}

/// 从所有行中解析 TOML 键值对（用于 [package] 段）
fn parse_toml_value(lines: &[&str], key: &str) -> Option<String> {
    for line in lines {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = line.find('=') {
            let k = line[..eq_pos].trim();
            if k == key {
                let v = line[eq_pos + 1..].trim();
                let v = v.trim_matches('"').trim_matches('\'');
                return Some(v.to_string());
            }
        }
    }
    None
}

/// 提取 TOML 中指定 section 的内容（不含 section 标题行）
fn extract_section(lines: &[&str], section_name: &str) -> Vec<String> {
    let mut in_section = false;
    let mut result = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            if in_section {
                break;
            }
            if trimmed == section_name {
                in_section = true;
            }
            continue;
        }
        if in_section {
            result.push(line.to_string());
        }
    }

    result
}

/// 从 section 行中解析键值对
fn parse_section_value(lines: &[String], key: &str) -> Option<String> {
    for line in lines {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = line.find('=') {
            let k = line[..eq_pos].trim();
            if k == key {
                let v = line[eq_pos + 1..].trim();
                let v = v.trim_matches('"').trim_matches('\'');
                return Some(v.to_string());
            }
        }
    }
    None
}

// ============================================================
// ELF 解析 & 进度条显示
// ============================================================

/// 显示固件大小进度条
fn show_firmware_size(path: &Path) {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("  [WARN] Cannot read firmware file");
            return;
        }
    };

    let mut data = Vec::new();
    if file.read_to_end(&mut data).is_err() || data.len() < 52 {
        return;
    }

    // 解析 32-bit ELF header
    let e_shoff = u32::from_le_bytes(data[0x20..0x24].try_into().unwrap()) as usize;
    let e_shentsize = u16::from_le_bytes(data[0x2E..0x30].try_into().unwrap()) as usize;
    let e_shnum = u16::from_le_bytes(data[0x30..0x32].try_into().unwrap()) as usize;

    let mut flash_used: u64 = 0;
    let mut ram_used: u64 = 0;

    // 从 memory.x 读取 FLASH 和 RAM 的地址范围
    let (flash_origin, flash_len, ram_origin, ram_len) = parse_memory_x_addrs("memory.x");
    let flash_origin = flash_origin as u32;
    let flash_end = (flash_origin as u64 + flash_len) as u32;
    let ram_origin = ram_origin as u32;
    let ram_end = (ram_origin as u64 + ram_len) as u32;

    for i in 0..e_shnum {
        let sh_off = e_shoff + i * e_shentsize;
        if sh_off + 24 > data.len() {
            break;
        }

        let sh_flags = u32::from_le_bytes(data[sh_off + 8..sh_off + 12].try_into().unwrap());
        let sh_addr = u32::from_le_bytes(data[sh_off + 12..sh_off + 16].try_into().unwrap());
        let sh_size = u32::from_le_bytes(data[sh_off + 20..sh_off + 24].try_into().unwrap());

        // SHF_ALLOC = 0x2
        if sh_flags & 0x2 != 0 {
            if sh_addr >= ram_origin && sh_addr < ram_end {
                ram_used += sh_size as u64;
            } else if sh_addr >= flash_origin && sh_addr < flash_end {
                flash_used += sh_size as u64;
            }
        }
    }

    // 从 memory.x 读取总大小
    let (flash_total, ram_total) = parse_memory_x("memory.x");

    let flash_pct = flash_used as f64 * 100.0 / flash_total as f64;
    let ram_pct = ram_used as f64 * 100.0 / ram_total as f64;

    // 格式化大小显示
    let flash_used_str = format_bytes(flash_used);
    let flash_total_str = format_bytes(flash_total);
    let ram_used_str = format_bytes(ram_used);
    let ram_total_str = format_bytes(ram_total);

    // 进度条（30 格）
    let flash_bar = progress_bar(flash_pct, 30);
    let ram_bar = progress_bar(ram_pct, 30);

    // 固定边框宽度，内容用空格补齐对齐
    let bar_width = 32; // [ + 30格 + ]
    let total_width = 2 + 5 + 1 + bar_width + 2 + 6 + 2 + 8 + 3 + 8 + 1;
    //                sp LABEL sp [bar]   sp  xx.x% sp  xxxx / xxxx sp

    let line_flash = format!(
        " {:<5} {} {:>5.1}%  {:>6} / {:<6} ",
        "FLASH", flash_bar, flash_pct, flash_used_str, flash_total_str
    );
    let line_ram = format!(
        " {:<5} {} {:>5.1}%  {:>6} / {:<6} ",
        "RAM", ram_bar, ram_pct, ram_used_str, ram_total_str
    );

    // 补齐到固定宽度（按字符数，避免 UTF-8 多字节截断）
    let pad = |s: &str, w: usize| {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() >= w {
            chars[..w].iter().collect()
        } else {
            format!("{}{}", s, " ".repeat(w - chars.len()))
        }
    };

    let border = format!("+{}+", "-".repeat(total_width));

    println!("  [FIRMWARE SIZE]");
    println!("  {}", border);
    println!("  |{}|", pad(&line_flash, total_width));
    println!("  |{}|", pad(&line_ram, total_width));
    println!("  {}", border);
}

/// 生成进度条字符串
fn progress_bar(pct: f64, width: usize) -> String {
    let fill = if pct <= 0.0 {
        0
    } else {
        let f = (pct / 100.0 * width as f64) as usize;
        if f < 1 { 1 } else { f }.min(width)
    };
    let empty = width - fill;
    format!("[{}{}]", "█".repeat(fill), "░".repeat(empty))
}

/// 解析 memory.x 文件，提取 FLASH 和 RAM 大小（字节）
fn parse_memory_x(path: &str) -> (u64, u64) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return (65536, 20480),
    };

    let mut flash_size: u64 = 0;
    let mut ram_size: u64 = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty()
            || line.starts_with("/*")
            || line.starts_with('*')
            || line.starts_with("//")
        {
            continue;
        }

        if line.starts_with("FLASH") || line.starts_with("RAM") {
            if let Some(len_str) = line.split(',').nth(1) {
                if let Some(eq) = len_str.find('=') {
                    let val_str = len_str[eq + 1..].trim();
                    let size = parse_size(val_str);
                    if line.starts_with("FLASH") {
                        flash_size = size;
                    } else {
                        ram_size = size;
                    }
                }
            }
        }
    }

    if flash_size == 0 {
        flash_size = 65536;
    }
    if ram_size == 0 {
        ram_size = 20480;
    }

    (flash_size, ram_size)
}

/// 解析 memory.x 文件，提取 FLASH 和 RAM 的起始地址和大小
/// 返回 (flash_origin, flash_length, ram_origin, ram_length)
fn parse_memory_x_addrs(path: &str) -> (u64, u64, u64, u64) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return (0x0800_0000, 65536, 0x2000_0000, 20480),
    };

    let mut flash_origin: u64 = 0x0800_0000;
    let mut flash_len: u64 = 65536;
    let mut ram_origin: u64 = 0x2000_0000;
    let mut ram_len: u64 = 20480;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty()
            || line.starts_with("/*")
            || line.starts_with('*')
            || line.starts_with("//")
        {
            continue;
        }

        if line.starts_with("FLASH") || line.starts_with("RAM") {
            let is_flash = line.starts_with("FLASH");
            // 格式: FLASH : ORIGIN = 0x08000000, LENGTH = 64K
            // 或:   FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
            if let Some(origin_str) = line.split(',').nth(0) {
                if let Some(eq) = origin_str.find("ORIGIN") {
                    let after_eq = origin_str[eq + "ORIGIN".len()..].trim();
                    let after_assign = after_eq.trim_start_matches('=').trim();
                    let val_str = after_assign.trim_matches(' ').trim();
                    // 解析 0x 十六进制或十进制数
                    let addr = if val_str.starts_with("0x") || val_str.starts_with("0X") {
                        u64::from_str_radix(&val_str[2..], 16).unwrap_or(0)
                    } else {
                        val_str.parse().unwrap_or(0)
                    };
                    if is_flash {
                        flash_origin = addr;
                    } else {
                        ram_origin = addr;
                    }
                }
            }
            if let Some(len_str) = line.split(',').nth(1) {
                if let Some(eq) = len_str.find("LENGTH") {
                    let val_str = len_str[eq + "LENGTH".len()..].trim();
                    let val_str = val_str.trim_start_matches('=').trim();
                    let size = parse_size(val_str);
                    if is_flash {
                        flash_len = size;
                    } else {
                        ram_len = size;
                    }
                }
            }
        }
    }

    (flash_origin, flash_len, ram_origin, ram_len)
}

/// 打印帮助信息
fn print_help() {
    println!("cargo-ocd — 一键编译并通过 OpenOCD 烧录/调试嵌入式固件");
    println!();
    println!("用法: cargo ocd [SUBCOMMAND] [OPTIONS]");
    println!();
    println!("子命令:");
    println!("  debug, d       编译、烧录并启动 GDB 调试会话（仅 Debug 模式）");
    println!();
    println!("选项:");
    println!("  --release      使用 Release 模式编译（默认 debug 模式）");
    println!("  --gdb          调试时使用系统 GDB（macOS 可用）");
    println!("  --rust-gdb     调试时使用 rust-gdb（macOS/Linux 可用）");
    println!("  --port <PORT>  指定 GDB 服务器端口（默认 3333）");
    println!("  --help, -h     显示此帮助信息");
    println!();
    println!("GDB 自动选择策略（按优先级）：");
    println!("  ARM 架构：");
    println!("    macOS:   rust-gdb > gdb > arm-none-eabi-gdb");
    println!("    Windows: arm-none-eabi-gdb");
    println!("    Linux:   arm-none-eabi-gdb");
    println!("  RISC-V 架构：");
    println!("    macOS:   riscv64-unknown-elf-gdb > rust-gdb > gdb");
    println!("    Windows: riscv64-unknown-elf-gdb");
    println!("    Linux:   riscv64-unknown-elf-gdb");
    println!();
    println!("配置方式（在项目的 Cargo.toml 中）：");
    println!();
    println!("  [package.metadata.ocd]");
    println!("  interface = \"interface/cmsis-dap.cfg\"   # 下载器配置");
    println!("  target = \"target/stm32f1x.cfg\"          # 芯片配置");
    println!("  target-triple = \"thumbv7m-none-eabi\"    # Rust 编译目标");
    println!();
    println!("示例:");
    println!("  cargo ocd                    # Debug 模式编译 + 烧录");
    println!("  cargo ocd --release          # Release 模式编译 + 烧录");
    println!("  cargo ocd d                  # Debug 编译 + 烧录 + GDB 调试");
    println!("  cargo ocd d --gdb            # 使用系统 GDB 调试");
    println!("  cargo ocd d --rust-gdb       # 使用 rust-gdb 调试");
    println!("  cargo ocd d --port 3334      # 指定 GDB 服务器端口");
    println!();
    println!("GDB 调试提示:");
    println!("  进入 GDB 后，依次执行:");
    println!("    (gdb) break main           # 在 main 函数设断点");
    println!("    (gdb) continue             # 运行到断点");
    println!("    (gdb) step                 # 单步执行");
    println!("    (gdb) print variable       # 查看变量");
    println!("     --其余指令见GDB调试协议--             ");
    println!();
    println!("支持的下载器: CMSIS-DAP / ST-Link / J-Link（通过 interface 配置）");
    println!("支持的芯片:   任何 OpenOCD 兼容的目标（通过 target 和 target-triple 配置）");
    println!();
    println!("详细文档: 基础环境配置与使用.md");
}

fn parse_size(s: &str) -> u64 {
    let s = s.trim().to_uppercase();
    if s.ends_with("K") {
        let num: f64 = s[..s.len() - 1].trim().parse().unwrap_or(0.0);
        (num * 1024.0) as u64
    } else if s.ends_with("M") {
        let num: f64 = s[..s.len() - 1].trim().parse().unwrap_or(0.0);
        (num * 1024.0 * 1024.0) as u64
    } else {
        s.parse().unwrap_or(0)
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{} MB", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{} KB", bytes / 1024)
    } else {
        format!("{} B", bytes)
    }
}