#!/usr/bin/env python3
"""
汉字字模生成器
支持多种尺寸: 16x16, 24x24, 32x32, 48x48, 64x64, 128x128
输出 Rust 格式的 const 数组

用法:
    python 汉字.py --text "温湿度"                       # 默认 16x16
    python 汉字.py --text "温度 湿度" --size 32x32       # 32x32
    python 汉字.py --text "你好世界" --size 48x48 --out font_cn.rs
    python 汉字.py --file input.txt --size 16x16         # 从文件读取
    python 汉字.py --range 0x4E00-0x4E50                 # Unicode 范围
    python 汉字.py --preview "温" --size 16x16           # 预览单字
"""

import argparse
import sys
import os

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    print("需要安装 Pillow: conda activate Py311 && pip install Pillow")
    sys.exit(1)


def find_cjk_font(size: int) -> ImageFont.FreeTypeFont:
    """查找可用的中文字体"""
    font_paths = [
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/STHeiti Medium.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        "/System/Library/Fonts/Supplemental/Songti.ttc",
        "/System/Library/Fonts/Supplemental/STHeiti Light.ttc",
        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        "C:/Windows/Fonts/msyh.ttc",
        "C:/Windows/Fonts/simhei.ttf",
        "C:/Windows/Fonts/simsun.ttc",
    ]
    for path in font_paths:
        if os.path.exists(path):
            try:
                return ImageFont.truetype(path, size)
            except Exception:
                continue
    print("警告: 未找到中文字体, 使用默认字体(可能无法显示中文)")
    return ImageFont.load_default()


def generate_char_bitmap(ch: str, font: ImageFont.FreeTypeFont,
                         width: int, height: int,
                         lsb_first: bool = True, column_major: bool = True) -> list:
    """
    生成单个字符的位图数据

    Args:
        ch: 字符
        font: PIL 字体对象
        width: 宽度
        height: 高度
        lsb_first: LSB在上
        column_major: 列优先

    Returns:
        字节列表
    """
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

    return byte_data


def collect_chars(text: str = None, file: str = None,
                  range_str: str = None) -> list:
    """收集需要生成字模的字符(去重排序)"""
    chars = set()

    if text:
        chars.update(text)

    if file:
        with open(file, "r", encoding="utf-8") as f:
            chars.update(f.read())

    if range_str:
        parts = range_str.split("-")
        if len(parts) == 2:
            start = int(parts[0], 0)
            end = int(parts[1], 0)
            for code in range(start, end + 1):
                try:
                    chars.add(chr(code))
                except (ValueError, OverflowError):
                    pass

    chars.discard('\n')
    chars.discard('\r')
    chars.discard('\t')
    return sorted(chars)


def format_rust_map(data: dict, width: int, height: int, name: str = "CN_FONT") -> str:
    """格式化为 Rust HashMap 或 const 数组"""
    bytes_per_char = len(next(iter(data.values())))
    sorted_items = sorted(data.items(), key=lambda x: ord(x[0]))

    lines = []
    lines.append(f"#![allow(dead_code)]")
    lines.append(f"")
    lines.append(f"/// {width}x{height} 汉字字模")
    lines.append(f"/// 每字符 {bytes_per_char} 字节, 列优先, LSB在上")
    lines.append(f"/// 共 {len(sorted_items)} 个字符")
    lines.append(f"")
    lines.append(f"/// 字符 -> 字模数据的映射表")
    lines.append(f"/// (字符Unicode码, 字模数据)")
    lines.append(f"pub const {name}_{width}X{height}: &[(char, [u8; {bytes_per_char}])] = &[")

    for ch, byte_data in sorted_items:
        code = ord(ch)
        hex_str = ", ".join(f"0x{b:02X}" for b in byte_data)
        lines.append(f"    ('\\u{{{code:X}}}', [{hex_str}]), // {ch}")

    lines.append("];")
    lines.append("")

    lines.append(f"pub fn lookup_{width}x{height}(c: char) -> Option<&'static [u8; {bytes_per_char}]> {{")
    lines.append(f"    {name}_{width}X{height}")
    lines.append(f"        .iter()")
    lines.append(f"        .find(|(ch, _)| *ch == c)")
    lines.append(f"        .map(|(_, data)| data)")
    lines.append("}")

    return "\n".join(lines)


def preview_char(ch: str, width: int, height: int, font_size: int):
    """在终端预览单个字符的位图"""
    font = find_cjk_font(font_size)
    byte_data = generate_char_bitmap(ch, font, width, height)

    print(f"字符 '{ch}' (U+{ord(ch):04X}), {width}x{height}:")
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
    parser = argparse.ArgumentParser(description="汉字字模生成器")
    parser.add_argument("--size", default="16x16",
                        help="字符尺寸 WxH (如 16x16, 32x32, 64x64)")
    parser.add_argument("--font-size", type=int, default=None,
                        help="字体大小(pt), 默认自动计算")
    parser.add_argument("--text", default=None,
                        help="要生成字模的汉字文本")
    parser.add_argument("--file", default=None,
                        help="从文件读取汉字")
    parser.add_argument("--range", default=None,
                        help="Unicode 范围, 如 0x4E00-0x9FFF")
    parser.add_argument("--out", default=None,
                        help="输出文件路径")
    parser.add_argument("--msb-top", action="store_true",
                        help="MSB在上 (默认 LSB在上)")
    parser.add_argument("--row-major", action="store_true",
                        help="行优先 (默认列优先)")
    parser.add_argument("--preview", default=None,
                        help="预览单个汉字的位图")
    parser.add_argument("--name", default="CN_FONT",
                        help="Rust 常量名前缀 (默认 CN_FONT)")

    args = parser.parse_args()

    parts = args.size.lower().split("x")
    if len(parts) != 2:
        print(f"错误: 尺寸格式应为 WxH, 如 16x16, 收到: {args.size}")
        sys.exit(1)

    width, height = int(parts[0]), int(parts[1])
    font_size = args.font_size or int(width * 0.85)

    if args.preview:
        font = find_cjk_font(font_size)
        preview_char(args.preview, width, height, font_size)
        return

    chars = collect_chars(text=args.text, file=args.file, range_str=args.range)
    if not chars:
        print("错误: 未指定任何汉字, 使用 --text, --file 或 --range 参数")
        sys.exit(1)

    print(f"正在生成 {len(chars)} 个字符的 {width}x{height} 字模...", file=sys.stderr)

    font = find_cjk_font(font_size)
    data = {}
    for i, ch in enumerate(chars):
        byte_data = generate_char_bitmap(
            ch, font, width, height,
            lsb_first=not args.msb_top,
            column_major=not args.row_major,
        )
        data[ch] = byte_data

    rust_code = format_rust_map(data, width, height, args.name)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(rust_code)
        total_bytes = len(chars) * len(next(iter(data.values())))
        print(f"已写入 {args.out} ({len(chars)} 字符, {total_bytes} 字节)")
    else:
        print(rust_code)


if __name__ == "__main__":
    main()
