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
