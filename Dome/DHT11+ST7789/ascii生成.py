#!/usr/bin/env python3
"""
ASCII 字模生成器
支持多种尺寸: 5x7, 6x8, 8x16, 12x24, 16x32 等
输出 Rust 格式的 const 数组

用法:
    python ascii生成.py                        # 默认 8x16
    python ascii生成.py --size 8x16            # 指定尺寸
    python ascii生成.py --size 16x32 --out font_16x32.rs
    python ascii生成.py --preview 65           # 预览字符 'A' (ASCII 65)
"""

import argparse
import sys
import os

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    print("需要安装 Pillow: conda activate Py311 && pip install Pillow")
    sys.exit(1)


def find_mono_font(size: int) -> ImageFont.FreeTypeFont:
    """查找可用的等宽字体"""
    font_paths = [
        "/System/Library/Fonts/Menlo.ttc",
        "/System/Library/Fonts/Courier.dfont",
        "/System/Library/Fonts/SFMono-Regular.otf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
        "C:/Windows/Fonts/consola.ttf",
        "C:/Windows/Fonts/cour.ttf",
    ]
    for path in font_paths:
        if os.path.exists(path):
            try:
                return ImageFont.truetype(path, size)
            except Exception:
                continue
    return ImageFont.load_default()


def generate_ascii_bitmap(width: int, height: int, font_size: int,
                          start: int = 32, end: int = 127,
                          lsb_first: bool = True, column_major: bool = True) -> list:
    """
    生成 ASCII 字符位图数据

    Args:
        width: 字符宽度(像素)
        height: 字符高度(像素)
        font_size: 字体大小(pt)
        start: 起始 ASCII 码
        end: 结束 ASCII 码(不含)
        lsb_first: True=LSB在上(嵌入式常用), False=MSB在上
        column_major: True=列优先(每字节一列), False=行优先(每字节一行)

    Returns:
        list of (char, [bytes]) 元组
    """
    font = find_mono_font(font_size)
    results = []

    for code in range(start, end):
        ch = chr(code)
        img = Image.new('1', (width, height), 0)
        draw = ImageDraw.Draw(img)

        bbox = draw.textbbox((0, 0), ch, font=font)
        tw = bbox[2] - bbox[0]
        th = bbox[3] - bbox[1]
        x = (width - tw) // 2 - bbox[0]
        y = (height - th) // 2 - bbox[1]
        draw.text((x, y), ch, fill=1, font=font)

        pixels = list(img.getdata())

        byte_data = []
        bytes_per_col = (height + 7) // 8
        if column_major:
            for col in range(width):
                for bi in range(bytes_per_col):
                    byte_val = 0
                    for bit in range(8):
                        row = bi * 8 + bit
                        if row < height:
                            px = pixels[row * width + col]
                            if px:
                                byte_val |= (1 << bit)
                    byte_data.append(byte_val)
        else:
            bytes_per_row = (width + 7) // 8
            for row in range(height):
                for bi in range(bytes_per_row):
                    byte_val = 0
                    for bit in range(8):
                        col = bi * 8 + bit
                        if col < width:
                            px = pixels[row * width + col]
                            if px:
                                if lsb_first:
                                    byte_val |= (1 << bit)
                                else:
                                    byte_val |= (1 << (7 - bit))
                    byte_data.append(byte_val)

        results.append((code, byte_data))
    return results


def format_rust_array(data: list, width: int, height: int, name: str = "FONT") -> str:
    """格式化为 Rust const 数组"""
    bytes_per_char = len(data[0][1])
    lines = []
    lines.append(f"#![allow(dead_code)]")
    lines.append(f"")
    lines.append(f"/// {width}x{height} ASCII 字模 (ASCII {data[0][0]}-{data[-1][0]})")
    lines.append(f"/// 每字符 {bytes_per_char} 字节, 列优先, LSB在上")
    lines.append(f"pub const {name}_{width}X{height}: [[u8; {bytes_per_char}]; {len(data)}] = [")

    for code, byte_data in data:
        ch = chr(code) if 32 <= code < 127 else '?'
        hex_str = ", ".join(f"0x{b:02X}" for b in byte_data)
        lines.append(f"    [{hex_str}], // {code:3d} '{ch}'")

    lines.append("];")
    return "\n".join(lines)


def preview_char(code: int, width: int, height: int, font_size: int):
    """在终端预览单个字符的位图"""
    data = generate_ascii_bitmap(width, height, font_size, code, code + 1)
    _, byte_data = data[0]

    print(f"字符 '{chr(code)}' (ASCII {code}), {width}x{height}:")
    print(f"十六进制: {', '.join(f'0x{b:02X}' for b in byte_data)}")
    print(f"二进制位图:")

    bytes_per_col = (height + 7) // 8
    for row in range(height):
        line = ""
        for col in range(width):
            byte_idx = col * bytes_per_col + row // 8
            bit_idx = row % 8
            if byte_idx < len(byte_data):
                if byte_data[byte_idx] & (1 << bit_idx):
                    line += "██"
                else:
                    line += "  "
            else:
                line += "  "
        print(f"  {line}")


def main():
    parser = argparse.ArgumentParser(description="ASCII 字模生成器")
    parser.add_argument("--size", default="8x16",
                        help="字符尺寸 WxH (如 5x7, 8x16, 16x32)")
    parser.add_argument("--font-size", type=int, default=None,
                        help="字体大小(pt), 默认自动计算")
    parser.add_argument("--out", default=None,
                        help="输出文件路径 (默认打印到终端)")
    parser.add_argument("--start", type=int, default=32,
                        help="起始 ASCII 码 (默认 32)")
    parser.add_argument("--end", type=int, default=127,
                        help="结束 ASCII 码, 不含 (默认 127)")
    parser.add_argument("--msb-top", action="store_true",
                        help="MSB在上 (默认 LSB在上)")
    parser.add_argument("--row-major", action="store_true",
                        help="行优先 (默认列优先)")
    parser.add_argument("--preview", type=int, default=None,
                        help="预览指定 ASCII 码的字符位图")
    parser.add_argument("--name", default="FONT",
                        help="Rust 常量名前缀 (默认 FONT)")

    args = parser.parse_args()

    parts = args.size.lower().split("x")
    if len(parts) != 2:
        print(f"错误: 尺寸格式应为 WxH, 如 8x16, 收到: {args.size}")
        sys.exit(1)

    width, height = int(parts[0]), int(parts[1])
    font_size = args.font_size or max(width, height)

    if args.preview is not None:
        preview_char(args.preview, width, height, font_size)
        return

    data = generate_ascii_bitmap(
        width, height, font_size,
        start=args.start, end=args.end,
        lsb_first=not args.msb_top,
        column_major=not args.row_major,
    )

    rust_code = format_rust_array(data, width, height, args.name)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(rust_code)
        print(f"已写入 {args.out} ({len(data)} 字符, {len(data) * len(data[0][1])} 字节)")
    else:
        print(rust_code)


if __name__ == "__main__":
    main()
