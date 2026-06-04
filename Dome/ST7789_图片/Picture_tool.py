#!/usr/bin/env python3
"""
Picture Bitmap Generator — PyQt5 GUI
Converts images (including GIF frames) to RGB565 byte arrays for ST7789.

Usage:
    python Picture_tool.py
"""

import sys
import os
import struct
from PIL import Image
from PyQt5.QtWidgets import (
    QApplication, QMainWindow, QWidget, QVBoxLayout, QHBoxLayout,
    QGridLayout, QLabel, QPushButton, QComboBox, QLineEdit,
    QSpinBox, QTextEdit, QFileDialog, QGroupBox,
    QMessageBox, QSizePolicy, QDialog, QCheckBox, QScrollArea,
)
from PyQt5.QtCore import Qt, pyqtSignal, QSize
from PyQt5.QtGui import QPainter, QColor, QFont, QPalette, QPixmap, QImage

# ============================================================
#  i18n
# ============================================================

TRANS = {
    "zh_CN": {
        "app_title": "图片取模工具 — RGB565",
        "lang_label": "语言",
        "group_image": "图片设置",
        "image_file": "图片文件",
        "browse": "浏览...",
        "output_size": "输出尺寸 (WxH)",
        "keep_ratio": "保持比例",
        "bg_color": "背景颜色",
        "bg_white": "白色",
        "bg_black": "黑色",
        "group_options": "生成选项",
        "byte_order": "字节序",
        "big_endian": "大端 (Big-Endian)",
        "little_endian": "小端 (Little-Endian)",
        "color_format": "颜色格式",
        "rgb565": "RGB565 (16bit)",
        "rgb565_swapped": "RGB565 字节交换",
        "var_name": "变量名",
        "generate": "生成取模",
        "group_preview": "预览",
        "group_code": "Rust 代码",
        "copy": "复制代码",
        "save": "保存文件",
        "clear": "清空",
        "status_ready": "就绪",
        "status_generated": "已生成 {w}x{h} 图片, 共 {b} 字节 ({kb} KB)",
        "status_generated_gif": "已生成 {n} 帧 ({w}x{h}), 共 {b} 字节 ({kb} KB)",
        "status_copied": "代码已复制到剪贴板",
        "status_saved": "已保存到 {path}",
        "err_no_image": "请先选择图片文件",
        "err_image_not_found": "图片文件不存在: {path}",
        "err_image_load": "无法加载图片: {path}",
        "preview_orig": "原始图片",
        "preview_result": "转换结果",
        "info_size": "尺寸",
        "info_bytes": "字节数",
        "info_format": "格式",
        "dither": "抖动 (Dithering)",
        "dither_none": "无抖动",
        "dither_floyd": "Floyd-Steinberg 抖动",
        "resize_mode": "缩放模式",
        "resize_fit": "适应 (留背景色)",
        "resize_fill": "拉伸填充",
        "resize_crop": "裁剪居中",
        "gif_frames": "GIF 帧",
        "gif_total": "共 {n} 帧",
        "gif_mode": "导出模式",
        "gif_single": "单帧",
        "gif_range": "范围",
        "gif_all": "全部帧",
        "gif_frame": "帧号",
        "gif_from": "起始帧",
        "gif_to": "结束帧",
        "gif_preview_frame": "预览帧",
    },
    "en": {
        "app_title": "Picture Tool — RGB565",
        "lang_label": "Language",
        "group_image": "Image Settings",
        "image_file": "Image File",
        "browse": "Browse...",
        "output_size": "Output Size (WxH)",
        "keep_ratio": "Keep Ratio",
        "bg_color": "Background Color",
        "bg_white": "White",
        "bg_black": "Black",
        "group_options": "Generation Options",
        "byte_order": "Byte Order",
        "big_endian": "Big-Endian",
        "little_endian": "Little-Endian",
        "color_format": "Color Format",
        "rgb565": "RGB565 (16bit)",
        "rgb565_swapped": "RGB565 Byte-Swapped",
        "var_name": "Variable Name",
        "generate": "Generate",
        "group_preview": "Preview",
        "group_code": "Rust Code",
        "copy": "Copy Code",
        "save": "Save File",
        "clear": "Clear",
        "status_ready": "Ready",
        "status_generated": "Generated {w}x{h} image, {b} bytes ({kb} KB)",
        "status_generated_gif": "Generated {n} frames ({w}x{h}), {b} bytes ({kb} KB)",
        "status_copied": "Code copied to clipboard",
        "status_saved": "Saved to {path}",
        "err_no_image": "Please select an image file first",
        "err_image_not_found": "Image file not found: {path}",
        "err_image_load": "Cannot load image: {path}",
        "preview_orig": "Original",
        "preview_result": "Result",
        "info_size": "Size",
        "info_bytes": "Bytes",
        "info_format": "Format",
        "dither": "Dithering",
        "dither_none": "None",
        "dither_floyd": "Floyd-Steinberg",
        "resize_mode": "Resize Mode",
        "resize_fit": "Fit (bg fill)",
        "resize_fill": "Stretch",
        "resize_crop": "Crop Center",
        "gif_frames": "GIF Frames",
        "gif_total": "{n} frames total",
        "gif_mode": "Export Mode",
        "gif_single": "Single Frame",
        "gif_range": "Range",
        "gif_all": "All Frames",
        "gif_frame": "Frame #",
        "gif_from": "From",
        "gif_to": "To",
        "gif_preview_frame": "Preview Frame",
    },
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
#  Image conversion core
# ============================================================


def rgb_to_rgb565(r, g, b):
    return ((r & 0xF8) << 8) | ((g & 0xFC) << 3) | (b >> 3)


def resize_image(img, out_w, out_h, mode, bg_color):
    in_w, in_h = img.size
    if mode == "fill":
        return img.resize((out_w, out_h), Image.LANCZOS)

    if mode == "crop":
        ratio_in = in_w / in_h
        ratio_out = out_w / out_h
        if ratio_in > ratio_out:
            new_h = in_h
            new_w = int(in_h * ratio_out)
            left = (in_w - new_w) // 2
            img = img.crop((left, 0, left + new_w, new_h))
        else:
            new_w = in_w
            new_h = int(in_w / ratio_out)
            top = (in_h - new_h) // 2
            img = img.crop((0, top, new_w, top + new_h))
        return img.resize((out_w, out_h), Image.LANCZOS)

    # fit mode
    img.thumbnail((out_w, out_h), Image.LANCZOS)
    canvas = Image.new("RGB", (out_w, out_h), bg_color)
    offset_x = (out_w - img.width) // 2
    offset_y = (out_h - img.height) // 2
    canvas.paste(img, (offset_x, offset_y))
    return canvas


def floyd_steinberg_dither(img):
    pixels = img.load()
    w, h = img.size
    for y in range(h):
        for x in range(w):
            old_r, old_g, old_b = pixels[x, y]
            new_r = round(old_r / 31) * 31
            new_g = round(old_g / 63) * 63
            new_b = round(old_b / 31) * 31
            new_r = max(0, min(255, new_r))
            new_g = max(0, min(255, new_g))
            new_b = max(0, min(255, new_b))
            pixels[x, y] = (new_r, new_g, new_b)
            err_r = old_r - new_r
            err_g = old_g - new_g
            err_b = old_b - new_b
            for dx, dy, w_factor in [(1, 0, 7/16), (-1, 1, 3/16), (0, 1, 5/16), (1, 1, 1/16)]:
                nx, ny = x + dx, y + dy
                if 0 <= nx < w and 0 <= ny < h:
                    nr, ng, nb = pixels[nx, ny]
                    nr = max(0, min(255, int(nr + err_r * w_factor)))
                    ng = max(0, min(255, int(ng + err_g * w_factor)))
                    nb = max(0, min(255, int(nb + err_b * w_factor)))
                    pixels[nx, ny] = (nr, ng, nb)
    return img


def convert_image(img, out_w, out_h, resize_mode, bg_color, dither, big_endian, swapped):
    img = img.convert("RGB")
    img = resize_image(img, out_w, out_h, resize_mode, bg_color)

    if dither:
        img = floyd_steinberg_dither(img)

    pixels = list(img.getdata())
    byte_data = []
    for r, g, b in pixels:
        val = rgb_to_rgb565(r, g, b)
        if swapped:
            val = ((val & 0xFF) << 8) | ((val >> 8) & 0xFF)
        if big_endian:
            byte_data.append((val >> 8) & 0xFF)
            byte_data.append(val & 0xFF)
        else:
            byte_data.append(val & 0xFF)
            byte_data.append((val >> 8) & 0xFF)

    return img, byte_data


def load_gif_frames(path):
    img = Image.open(path)
    frames = []
    try:
        while True:
            frame = img.copy().convert("RGBA")
            bg = Image.new("RGBA", frame.size, (255, 255, 255, 255))
            if frame.info.get("transparency") is not None:
                bg.paste(frame, mask=frame.split()[3])
            else:
                bg.paste(frame)
            frames.append(bg.convert("RGB"))
            img.seek(img.tell() + 1)
    except EOFError:
        pass
    return frames


def get_gif_frame_count(path):
    img = Image.open(path)
    count = 0
    try:
        while True:
            count += 1
            img.seek(img.tell() + 1)
    except EOFError:
        pass
    return count


def get_gif_frame(path, index):
    img = Image.open(path)
    img.seek(index)
    frame = img.copy().convert("RGBA")
    bg = Image.new("RGBA", frame.size, (255, 255, 255, 255))
    if frame.info.get("transparency") is not None:
        bg.paste(frame, mask=frame.split()[3])
    else:
        bg.paste(frame)
    return bg.convert("RGB")


def format_rust_code(byte_data, out_w, out_h, var_name, big_endian):
    if not byte_data:
        return "// No data"

    lines = []
    lines.append("#![allow(dead_code)]")
    lines.append("")
    be_desc = "big-endian" if big_endian else "little-endian"
    lines.append(f"/// {out_w}x{out_h} RGB565 image data ({be_desc})")
    lines.append(f"/// {len(byte_data)} bytes ({len(byte_data) / 1024:.1f} KB)")
    lines.append("")

    arr_name = f"{var_name}_{out_w}X{out_h}"
    lines.append(f"pub const {arr_name}_WIDTH: u16 = {out_w};")
    lines.append(f"pub const {arr_name}_HEIGHT: u16 = {out_h};")
    lines.append(f"pub const {arr_name}: [u8; {len(byte_data)}] = [")

    for i in range(0, len(byte_data), 16):
        chunk = byte_data[i:i+16]
        hex_str = ", ".join(f"0x{b:02X}" for b in chunk)
        lines.append(f"    {hex_str},")

    lines.append("];")
    return "\n".join(lines)


def format_rust_code_gif(frames_data, out_w, out_h, var_name, big_endian):
    if not frames_data:
        return "// No data"

    lines = []
    lines.append("#![allow(dead_code)]")
    lines.append("")
    be_desc = "big-endian" if big_endian else "little-endian"
    frame_bytes = len(frames_data[0])
    lines.append(f"/// {out_w}x{out_h} RGB565 GIF animation ({be_desc})")
    lines.append(f"/// {len(frames_data)} frames, {frame_bytes} bytes/frame ({frame_bytes / 1024:.1f} KB)")
    lines.append(f"/// Total: {len(frames_data) * frame_bytes} bytes ({len(frames_data) * frame_bytes / 1024:.1f} KB)")
    lines.append("")

    arr_name = f"{var_name}_{out_w}X{out_h}"
    lines.append(f"pub const {arr_name}_WIDTH: u16 = {out_w};")
    lines.append(f"pub const {arr_name}_HEIGHT: u16 = {out_h};")
    lines.append(f"pub const {arr_name}_FRAMES: usize = {len(frames_data)};")
    lines.append(f"pub const {arr_name}_FRAME_BYTES: usize = {frame_bytes};")
    lines.append(f"pub const {arr_name}: [[u8; {frame_bytes}]; {len(frames_data)}] = [")

    for fi, byte_data in enumerate(frames_data):
        lines.append(f"    [ // frame {fi}")
        for i in range(0, len(byte_data), 16):
            chunk = byte_data[i:i+16]
            hex_str = ", ".join(f"0x{b:02X}" for b in chunk)
            lines.append(f"        {hex_str},")
        lines.append("    ],")

    lines.append("];")

    lines.append("")
    lines.append(f"pub fn {arr_name.lower()}_frame(idx: usize) -> Option<&'static [u8; {frame_bytes}]>")
    lines.append("{")
    lines.append(f"    if idx < {arr_name}_FRAMES {{")
    lines.append(f"        Some(&{arr_name}[idx])")
    lines.append("    } else {")
    lines.append("        None")
    lines.append("    }")
    lines.append("}")

    return "\n".join(lines)


# ============================================================
#  Image preview widget
# ============================================================


class ImagePreviewWidget(QWidget):
    def __init__(self, label="", parent=None):
        super().__init__(parent)
        self.pixmap = None
        self.label = label
        self.setMinimumSize(160, 120)
        self.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Expanding)

    def set_pixmap(self, pixmap):
        self.pixmap = pixmap
        self.update()

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.SmoothPixmapTransform)
        painter.fillRect(self.rect(), QColor(30, 30, 30))

        if self.pixmap and not self.pixmap.isNull():
            scaled = self.pixmap.scaled(
                self.size(), Qt.KeepAspectRatio, Qt.SmoothTransformation
            )
            x = (self.width() - scaled.width()) // 2
            y = (self.height() - scaled.height()) // 2
            painter.drawPixmap(x, y, scaled)

        if self.label:
            painter.setPen(QColor(180, 180, 180))
            painter.setFont(QFont("sans-serif", 10))
            painter.drawText(6, 16, self.label)

        painter.end()


class ImageInfoWidget(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.info_lines = []
        self.setFixedWidth(220)
        self.setMinimumHeight(80)

    def set_info(self, lines):
        self.info_lines = lines
        self.update()

    def clear_info(self):
        self.info_lines = []
        self.update()

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.fillRect(self.rect(), QColor(35, 35, 35))
        painter.setPen(QColor(200, 200, 200))
        painter.setFont(QFont("Menlo", 10))
        y = 18
        for line in self.info_lines:
            painter.drawText(8, y, line)
            y += 18
        painter.end()


# ============================================================
#  Main window
# ============================================================


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.generated_data = []
        self.generated_frames = []
        self.rust_code = ""
        self.original_pixmap = None
        self.result_pixmap = None
        self.is_gif = False
        self.gif_frame_count = 0
        self.gif_path = ""
        self._build_ui()
        self._retranslate()

    def _build_ui(self):
        self.setMinimumSize(900, 720)
        self.resize(1000, 780)

        central = QWidget()
        self.setCentralWidget(central)
        main_layout = QVBoxLayout(central)
        main_layout.setSpacing(6)

        # --- Top bar: language ---
        top_bar = QHBoxLayout()
        top_bar.addStretch()
        self.lbl_lang = QLabel()
        top_bar.addWidget(self.lbl_lang)
        self.cmb_lang = QComboBox()
        self.cmb_lang.addItem("简体中文", "zh_CN")
        self.cmb_lang.addItem("English", "en")
        self.cmb_lang.currentIndexChanged.connect(self._on_lang_changed)
        top_bar.addWidget(self.cmb_lang)
        main_layout.addLayout(top_bar)

        # --- Image settings ---
        grp_image = QGroupBox()
        self.grp_image = grp_image
        img_grid = QGridLayout(grp_image)

        self.lbl_image_file = QLabel()
        img_grid.addWidget(self.lbl_image_file, 0, 0)
        self.txt_image = QLineEdit()
        img_grid.addWidget(self.txt_image, 0, 1)
        self.btn_browse = QPushButton()
        self.btn_browse.clicked.connect(self._browse_image)
        img_grid.addWidget(self.btn_browse, 0, 2)

        self.lbl_size = QLabel()
        img_grid.addWidget(self.lbl_size, 1, 0)
        size_layout = QHBoxLayout()
        self.spn_w = QSpinBox()
        self.spn_w.setRange(1, 1024)
        self.spn_w.setValue(240)
        size_layout.addWidget(self.spn_w)
        size_layout.addWidget(QLabel("x"))
        self.spn_h = QSpinBox()
        self.spn_h.setRange(1, 1024)
        self.spn_h.setValue(240)
        size_layout.addWidget(self.spn_h)
        self.chk_ratio = QCheckBox()
        self.chk_ratio.setChecked(True)
        size_layout.addWidget(self.chk_ratio)
        size_layout.addStretch()
        img_grid.addLayout(size_layout, 1, 1)

        self.lbl_resize = QLabel()
        img_grid.addWidget(self.lbl_resize, 2, 0)
        self.cmb_resize = QComboBox()
        img_grid.addWidget(self.cmb_resize, 2, 1)

        self.lbl_bg = QLabel()
        img_grid.addWidget(self.lbl_bg, 3, 0)
        self.cmb_bg = QComboBox()
        img_grid.addWidget(self.cmb_bg, 3, 1)

        main_layout.addWidget(grp_image)

        # --- GIF frame settings (hidden by default) ---
        grp_gif = QGroupBox()
        self.grp_gif = grp_gif
        gif_grid = QGridLayout(grp_gif)

        self.lbl_gif_frames = QLabel()
        gif_grid.addWidget(self.lbl_gif_frames, 0, 0)
        self.lbl_gif_total = QLabel()
        gif_grid.addWidget(self.lbl_gif_total, 0, 1)

        self.lbl_gif_mode = QLabel()
        gif_grid.addWidget(self.lbl_gif_mode, 1, 0)
        self.cmb_gif_mode = QComboBox()
        self.cmb_gif_mode.currentIndexChanged.connect(self._on_gif_mode_changed)
        gif_grid.addWidget(self.cmb_gif_mode, 1, 1)

        self.lbl_gif_frame = QLabel()
        gif_grid.addWidget(self.lbl_gif_frame, 2, 0)
        self.spn_gif_frame = QSpinBox()
        self.spn_gif_frame.setRange(0, 0)
        self.spn_gif_frame.valueChanged.connect(self._on_gif_frame_changed)
        gif_grid.addWidget(self.spn_gif_frame, 2, 1)

        self.lbl_gif_from = QLabel()
        gif_grid.addWidget(self.lbl_gif_from, 3, 0)
        range_layout = QHBoxLayout()
        self.spn_gif_from = QSpinBox()
        self.spn_gif_from.setRange(0, 0)
        range_layout.addWidget(self.spn_gif_from)
        self.lbl_gif_to = QLabel()
        range_layout.addWidget(self.lbl_gif_to)
        self.spn_gif_to = QSpinBox()
        self.spn_gif_to.setRange(0, 0)
        range_layout.addWidget(self.spn_gif_to)
        range_layout.addStretch()
        gif_grid.addLayout(range_layout, 3, 1)

        self.lbl_gif_preview = QLabel()
        gif_grid.addWidget(self.lbl_gif_preview, 4, 0)
        self.spn_gif_preview = QSpinBox()
        self.spn_gif_preview.setRange(0, 0)
        self.spn_gif_preview.valueChanged.connect(self._on_gif_preview_changed)
        gif_grid.addWidget(self.spn_gif_preview, 4, 1)

        self.grp_gif.setVisible(False)
        main_layout.addWidget(grp_gif)

        # --- Options ---
        grp_opts = QGroupBox()
        self.grp_opts = grp_opts
        opts_grid = QGridLayout(grp_opts)

        self.lbl_byte_order = QLabel()
        opts_grid.addWidget(self.lbl_byte_order, 0, 0)
        self.cmb_byte_order = QComboBox()
        opts_grid.addWidget(self.cmb_byte_order, 0, 1)

        self.lbl_color_fmt = QLabel()
        opts_grid.addWidget(self.lbl_color_fmt, 1, 0)
        self.cmb_color_fmt = QComboBox()
        opts_grid.addWidget(self.cmb_color_fmt, 1, 1)

        self.lbl_dither = QLabel()
        opts_grid.addWidget(self.lbl_dither, 2, 0)
        self.cmb_dither = QComboBox()
        opts_grid.addWidget(self.cmb_dither, 2, 1)

        self.lbl_var = QLabel()
        opts_grid.addWidget(self.lbl_var, 3, 0)
        self.txt_var = QLineEdit("PICTURE")
        opts_grid.addWidget(self.txt_var, 3, 1)

        main_layout.addWidget(grp_opts)

        # --- Action buttons ---
        btn_layout = QHBoxLayout()
        btn_layout.addStretch()
        self.btn_gen = QPushButton()
        self.btn_gen.setMinimumSize(120, 36)
        self.btn_gen.clicked.connect(self._generate)
        btn_layout.addWidget(self.btn_gen)
        btn_layout.addStretch()
        main_layout.addLayout(btn_layout)

        # --- Preview area ---
        preview_group = QGroupBox()
        self.grp_preview = preview_group
        preview_layout = QHBoxLayout(preview_group)

        orig_col = QVBoxLayout()
        self.lbl_orig = QLabel()
        self.lbl_orig.setAlignment(Qt.AlignCenter)
        orig_col.addWidget(self.lbl_orig)
        self.preview_orig = ImagePreviewWidget()
        orig_col.addWidget(self.preview_orig, 1)
        preview_layout.addLayout(orig_col, 1)

        self.info_widget = ImageInfoWidget()
        preview_layout.addWidget(self.info_widget)

        result_col = QVBoxLayout()
        self.lbl_result = QLabel()
        self.lbl_result.setAlignment(Qt.AlignCenter)
        result_col.addWidget(self.lbl_result)
        self.preview_result = ImagePreviewWidget()
        result_col.addWidget(self.preview_result, 1)
        preview_layout.addLayout(result_col, 1)

        main_layout.addWidget(preview_group, 1)

        # --- Code area ---
        code_group = QGroupBox()
        self.grp_code = code_group
        code_layout = QVBoxLayout(code_group)

        self.txt_code = QTextEdit()
        self.txt_code.setReadOnly(True)
        self.txt_code.setFont(QFont("Menlo", 10))
        self.txt_code.setMaximumHeight(180)
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

        self.statusBar().showMessage("")

    def _browse_image(self):
        path, _ = QFileDialog.getOpenFileName(
            self,
            i18n.t("browse"),
            "",
            "Images (*.png *.jpg *.jpeg *.bmp *.gif *.tga *.webp);;All (*)",
        )
        if path:
            self.txt_image.setText(path)
            self._load_preview(path)

    def _load_preview(self, path):
        if not os.path.exists(path):
            return

        is_gif = path.lower().endswith(".gif")
        self.is_gif = is_gif
        self.gif_path = path

        if is_gif:
            self.gif_frame_count = get_gif_frame_count(path)
            self._setup_gif_ui()
            img = get_gif_frame(path, 0)
            w, h = img.size
        else:
            self.gif_frame_count = 0
            self.gif_path = ""
            img = Image.open(path)
            w, h = img.size

        self.grp_gif.setVisible(is_gif)

        pixmap = QPixmap(path)
        if pixmap.isNull():
            qimg = QImage(
                img.tobytes(), img.width, img.height,
                img.width * 3, QImage.Format_RGB888
            )
            pixmap = QPixmap.fromImage(qimg)
        self.original_pixmap = pixmap
        self.preview_orig.set_pixmap(pixmap)

        if self.chk_ratio.isChecked():
            self.spn_w.blockSignals(True)
            self.spn_h.blockSignals(True)
            self.spn_w.setValue(w)
            self.spn_h.setValue(h)
            self.spn_w.blockSignals(False)
            self.spn_h.blockSignals(False)

        info = [
            f"{i18n.t('info_size')}: {w} x {h}",
            f"{i18n.t('info_format')}: {'GIF' if is_gif else 'N/A'}",
            f"Mode: {img.mode}",
        ]
        if is_gif:
            info.append("")
            info.append(i18n.t("gif_total", n=self.gif_frame_count))
        self.info_widget.set_info(info)

    def _setup_gif_ui(self):
        n = self.gif_frame_count
        self.lbl_gif_total.setText(i18n.t("gif_total", n=n))
        self.spn_gif_frame.setRange(0, max(0, n - 1))
        self.spn_gif_frame.setValue(0)
        self.spn_gif_from.setRange(0, max(0, n - 1))
        self.spn_gif_from.setValue(0)
        self.spn_gif_to.setRange(0, max(0, n - 1))
        self.spn_gif_to.setValue(max(0, n - 1))
        self.spn_gif_preview.setRange(0, max(0, n - 1))
        self.spn_gif_preview.setValue(0)

    def _on_gif_mode_changed(self, idx):
        is_single = idx == 0
        is_range = idx == 1
        self.lbl_gif_frame.setVisible(is_single)
        self.spn_gif_frame.setVisible(is_single)
        self.lbl_gif_from.setVisible(is_range)
        self.lbl_gif_to.setVisible(is_range)
        self.spn_gif_from.setVisible(is_range)
        self.spn_gif_to.setVisible(is_range)

    def _on_gif_frame_changed(self, val):
        self.spn_gif_preview.setValue(val)

    def _on_gif_preview_changed(self, val):
        if not self.is_gif or not self.gif_path:
            return
        img = get_gif_frame(self.gif_path, val)
        qimg = QImage(
            img.tobytes(), img.width, img.height,
            img.width * 3, QImage.Format_RGB888
        )
        self.original_pixmap = QPixmap.fromImage(qimg)
        self.preview_orig.set_pixmap(self.original_pixmap)

    def _on_lang_changed(self, idx):
        lang = self.cmb_lang.currentData()
        i18n.set_lang(lang)
        self._retranslate()

    def _generate(self):
        path = self.txt_image.text().strip()
        if not path:
            QMessageBox.warning(self, "", i18n.t("err_no_image"))
            return
        if not os.path.exists(path):
            QMessageBox.warning(self, "", i18n.t("err_image_not_found", path=path))
            return

        out_w = self.spn_w.value()
        out_h = self.spn_h.value()
        resize_mode_data = self.cmb_resize.currentData()
        bg_idx = self.cmb_bg.currentIndex()
        bg_color = (255, 255, 255) if bg_idx == 0 else (0, 0, 0)
        big_endian = self.cmb_byte_order.currentIndex() == 0
        swapped = self.cmb_color_fmt.currentIndex() == 1
        dither = self.cmb_dither.currentIndex() == 1
        var = self.txt_var.text().strip() or "PICTURE"

        if self.is_gif:
            self._generate_gif(path, out_w, out_h, resize_mode_data, bg_color, dither, big_endian, swapped, var)
        else:
            self._generate_single(path, out_w, out_h, resize_mode_data, bg_color, dither, big_endian, swapped, var)

    def _generate_single(self, path, out_w, out_h, resize_mode, bg_color, dither, big_endian, swapped, var):
        try:
            img = Image.open(path)
        except Exception:
            QMessageBox.warning(self, "", i18n.t("err_image_load", path=path))
            return

        result_img, byte_data = convert_image(
            img, out_w, out_h, resize_mode, bg_color, dither, big_endian, swapped
        )

        self.generated_data = byte_data
        self.generated_frames = []
        self.rust_code = format_rust_code(byte_data, out_w, out_h, var, big_endian)

        qimg = QImage(
            result_img.tobytes(), result_img.width, result_img.height,
            result_img.width * 3, QImage.Format_RGB888
        )
        self.result_pixmap = QPixmap.fromImage(qimg)
        self.preview_result.set_pixmap(self.result_pixmap)

        self.info_widget.set_info([
            f"{i18n.t('info_size')}: {img.width} x {img.height}",
            f"{i18n.t('info_format')}: {img.format or 'N/A'}",
            f"Mode: {img.mode}",
            "",
            f"-> {out_w} x {out_h}",
            f"{i18n.t('info_bytes')}: {len(byte_data)}",
            f"  ({len(byte_data) / 1024:.1f} KB)",
        ])

        self.txt_code.setPlainText(self.rust_code)
        self.statusBar().showMessage(
            i18n.t(
                "status_generated",
                w=out_w, h=out_h,
                b=len(byte_data),
                kb=f"{len(byte_data) / 1024:.1f}",
            )
        )

    def _generate_gif(self, path, out_w, out_h, resize_mode, bg_color, dither, big_endian, swapped, var):
        gif_mode = self.cmb_gif_mode.currentIndex()

        if gif_mode == 0:  # single frame
            frame_idx = self.spn_gif_frame.value()
            frame_indices = [frame_idx]
        elif gif_mode == 1:  # range
            fr = self.spn_gif_from.value()
            to = self.spn_gif_to.value()
            if fr > to:
                fr, to = to, fr
            frame_indices = list(range(fr, to + 1))
        else:  # all
            frame_indices = list(range(self.gif_frame_count))

        frames_data = []
        last_result_img = None
        for idx in frame_indices:
            img = get_gif_frame(path, idx)
            result_img, byte_data = convert_image(
                img, out_w, out_h, resize_mode, bg_color, dither, big_endian, swapped
            )
            frames_data.append(byte_data)
            last_result_img = result_img

        self.generated_data = frames_data[0] if len(frames_data) == 1 else []
        self.generated_frames = frames_data

        if len(frames_data) == 1:
            self.rust_code = format_rust_code(frames_data[0], out_w, out_h, var, big_endian)
        else:
            self.rust_code = format_rust_code_gif(frames_data, out_w, out_h, var, big_endian)

        if last_result_img:
            qimg = QImage(
                last_result_img.tobytes(), last_result_img.width, last_result_img.height,
                last_result_img.width * 3, QImage.Format_RGB888
            )
            self.result_pixmap = QPixmap.fromImage(qimg)
            self.preview_result.set_pixmap(self.result_pixmap)

        total_bytes = sum(len(fd) for fd in frames_data)
        self.info_widget.set_info([
            f"GIF: {self.gif_frame_count} frames",
            f"{i18n.t('info_size')}: {out_w} x {out_h}",
            f"Exported: {len(frames_data)} frames",
            "",
            f"{i18n.t('info_bytes')}: {total_bytes}",
            f"  ({total_bytes / 1024:.1f} KB)",
            f"Per frame: {len(frames_data[0])} B",
        ])

        self.txt_code.setPlainText(self.rust_code)
        self.statusBar().showMessage(
            i18n.t(
                "status_generated_gif",
                n=len(frames_data), w=out_w, h=out_h,
                b=total_bytes,
                kb=f"{total_bytes / 1024:.1f}",
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
            self, i18n.t("save"), "picture_data.rs", "Rust (*.rs);;All (*)"
        )
        if path:
            with open(path, "w", encoding="utf-8") as f:
                f.write(self.rust_code)
            self.statusBar().showMessage(i18n.t("status_saved", path=path))

    def _clear_all(self):
        self.generated_data = []
        self.generated_frames = []
        self.rust_code = ""
        self.original_pixmap = None
        self.result_pixmap = None
        self.is_gif = False
        self.gif_frame_count = 0
        self.gif_path = ""
        self.grp_gif.setVisible(False)
        self.preview_orig.set_pixmap(QPixmap())
        self.preview_result.set_pixmap(QPixmap())
        self.txt_code.clear()
        self.info_widget.clear_info()
        self.statusBar().showMessage(i18n.t("status_ready"))

    def _retranslate(self):
        self.setWindowTitle(i18n.t("app_title"))
        self.lbl_lang.setText(i18n.t("lang_label"))
        self.grp_image.setTitle(i18n.t("group_image"))
        self.lbl_image_file.setText(i18n.t("image_file"))
        self.btn_browse.setText(i18n.t("browse"))
        self.lbl_size.setText(i18n.t("output_size"))
        self.chk_ratio.setText(i18n.t("keep_ratio"))
        self.lbl_resize.setText(i18n.t("resize_mode"))
        self.lbl_bg.setText(i18n.t("bg_color"))
        self.grp_opts.setTitle(i18n.t("group_options"))
        self.lbl_byte_order.setText(i18n.t("byte_order"))
        self.lbl_color_fmt.setText(i18n.t("color_format"))
        self.lbl_dither.setText(i18n.t("dither"))
        self.lbl_var.setText(i18n.t("var_name"))
        self.btn_gen.setText(i18n.t("generate"))
        self.grp_preview.setTitle(i18n.t("group_preview"))
        self.lbl_orig.setText(i18n.t("preview_orig"))
        self.lbl_result.setText(i18n.t("preview_result"))
        self.grp_code.setTitle(i18n.t("group_code"))
        self.btn_copy.setText(i18n.t("copy"))
        self.btn_save.setText(i18n.t("save"))
        self.btn_clear.setText(i18n.t("clear"))

        self.grp_gif.setTitle(i18n.t("gif_frames"))
        self.lbl_gif_frames.setText(i18n.t("gif_frames") + ":")
        self.lbl_gif_mode.setText(i18n.t("gif_mode"))
        self.lbl_gif_frame.setText(i18n.t("gif_frame"))
        self.lbl_gif_from.setText(i18n.t("gif_from"))
        self.lbl_gif_to.setText(i18n.t("gif_to"))
        self.lbl_gif_preview.setText(i18n.t("gif_preview_frame"))

        cur_gif_mode = self.cmb_gif_mode.currentIndex()
        self.cmb_gif_mode.blockSignals(True)
        self.cmb_gif_mode.clear()
        self.cmb_gif_mode.addItem(i18n.t("gif_single"))
        self.cmb_gif_mode.addItem(i18n.t("gif_range"))
        self.cmb_gif_mode.addItem(i18n.t("gif_all"))
        self.cmb_gif_mode.setCurrentIndex(max(0, cur_gif_mode))
        self.cmb_gif_mode.blockSignals(False)
        self._on_gif_mode_changed(self.cmb_gif_mode.currentIndex())

        self.cmb_byte_order.blockSignals(True)
        self.cmb_byte_order.clear()
        self.cmb_byte_order.addItem(i18n.t("big_endian"))
        self.cmb_byte_order.addItem(i18n.t("little_endian"))
        self.cmb_byte_order.blockSignals(False)

        self.cmb_color_fmt.blockSignals(True)
        self.cmb_color_fmt.clear()
        self.cmb_color_fmt.addItem(i18n.t("rgb565"))
        self.cmb_color_fmt.addItem(i18n.t("rgb565_swapped"))
        self.cmb_color_fmt.blockSignals(False)

        self.cmb_dither.blockSignals(True)
        self.cmb_dither.clear()
        self.cmb_dither.addItem(i18n.t("dither_none"))
        self.cmb_dither.addItem(i18n.t("dither_floyd"))
        self.cmb_dither.blockSignals(False)

        cur_resize = self.cmb_resize.currentIndex()
        self.cmb_resize.blockSignals(True)
        self.cmb_resize.clear()
        self.cmb_resize.addItem(i18n.t("resize_fit"), "fit")
        self.cmb_resize.addItem(i18n.t("resize_fill"), "fill")
        self.cmb_resize.addItem(i18n.t("resize_crop"), "crop")
        self.cmb_resize.setCurrentIndex(max(0, cur_resize))
        self.cmb_resize.blockSignals(False)

        self.cmb_bg.blockSignals(True)
        self.cmb_bg.clear()
        self.cmb_bg.addItem(i18n.t("bg_white"))
        self.cmb_bg.addItem(i18n.t("bg_black"))
        self.cmb_bg.blockSignals(False)


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
        QCheckBox {
            spacing: 4px;
        }
    """)

    win = MainWindow()
    win.show()
    sys.exit(app.exec_())


if __name__ == "__main__":
    main()
