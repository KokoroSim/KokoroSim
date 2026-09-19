#!/usr/bin/env python3
"""
Gera o banner de compartilhamento Open Graph / Twitter Card (1200x630 pixels)
para o KokoroSim com Safe Zone 4:3 no miolo centralizado (blindado contra recortes
do WhatsApp e redes móveis), tipografia nítida e ausência de redundâncias.
"""

import os
from PIL import Image, ImageDraw, ImageFont, ImageFilter

def create_og_image(output_path="ui/assets/og_preview.png"):
    width, height = 1200, 630

    # 1. Base com fundo profundo e grade cibernética sutil
    base = Image.new("RGBA", (width, height), (9, 10, 15, 255))
    draw = ImageDraw.Draw(base)

    grid_spacing = 32
    grid_color = (28, 32, 45, 100)
    for x in range(0, width, grid_spacing):
        draw.line([(x, 0), (x, height)], fill=grid_color, width=1)
    for y in range(0, height, grid_spacing):
        draw.line([(0, y), (width, y)], fill=grid_color, width=1)

    # 2. Luz difusa central (Glows carmesim e ciano no miolo)
    glow_layer = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow_layer)
    glow_draw.ellipse([340, 60, 860, 520], fill=(255, 23, 84, 50))
    glow_draw.ellipse([280, 120, 920, 550], fill=(0, 242, 254, 35))
    glow_layer = glow_layer.filter(ImageFilter.GaussianBlur(85))
    base = Image.alpha_composite(base, glow_layer)

    # 3. Carregar e recortar o logotipo oficial
    logo_path = "ui/assets/logo.png"
    if not os.path.exists(logo_path):
        logo_path = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "ui", "assets", "logo.png")

    logo = Image.open(logo_path).convert("RGBA")
    bbox = logo.getbbox()
    if bbox:
        logo = logo.crop(bbox)

    # 4. Fontes
    font_bold_candidates = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
    ]
    font_reg_candidates = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    ]
    font_cjk_candidates = [
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
    ]

    font_bold_path = next((f for f in font_bold_candidates if os.path.exists(f)), None)
    font_reg_path = next((f for f in font_reg_candidates if os.path.exists(f)), None)
    font_cjk_path = next((f for f in font_cjk_candidates if os.path.exists(f)), None)

    f_badge = ImageFont.truetype(font_bold_path, 16) if font_bold_path else ImageFont.load_default()
    f_brand = ImageFont.truetype(font_bold_path, 80) if font_bold_path else ImageFont.load_default()
    f_kanji = ImageFont.truetype(font_cjk_path, 84) if font_cjk_path else f_brand
    f_title1 = ImageFont.truetype(font_bold_path, 36) if font_bold_path else ImageFont.load_default()
    f_title2 = ImageFont.truetype(font_bold_path, 28) if font_bold_path else ImageFont.load_default()
    f_url = ImageFont.truetype(font_bold_path, 22) if font_bold_path else ImageFont.load_default()
    f_author = ImageFont.truetype(font_reg_path, 19) if font_reg_path else ImageFont.load_default()

    tdraw = ImageDraw.Draw(base)

    # 5. Badge superior centralizado
    b_text = " OPEN SOURCE // RUST + WEBASSEMBLY "
    b_box = tdraw.textbbox((0, 0), b_text, font=f_badge)
    bw = b_box[2] - b_box[0]
    bx = (width - bw) // 2
    by = 52
    tdraw.rectangle([bx - 14, by - 6, bx + bw + 14, by + 26], fill=(14, 20, 32, 230), outline=(0, 242, 254, 210), width=1)
    tdraw.text((bx, by), b_text, font=f_badge, fill=(0, 242, 254, 255))

    # 6. Cabeçalho Centralizado: Logo + Marca kokor心sim integrados
    target_h = 160
    target_w = int(logo.width * (target_h / logo.height))
    lg = logo.resize((target_w, target_h), Image.Resampling.LANCZOS)

    k_box = tdraw.textbbox((0, 0), "kokor", font=f_brand)
    kj_box = tdraw.textbbox((0, 0), "心", font=f_kanji)
    s_box = tdraw.textbbox((0, 0), "sim", font=f_brand)
    brand_w = (k_box[2] - k_box[0]) + (kj_box[2] - kj_box[0]) + (s_box[2] - s_box[0]) + 10

    header_w = target_w + 32 + brand_w
    header_x = (width - header_w) // 2
    header_y = 110

    # Sombra suave sob o logo
    shadow = Image.new("RGBA", (target_w + 40, target_h + 40), (0, 0, 0, 0))
    sdraw = ImageDraw.Draw(shadow)
    sdraw.ellipse([20, 20, target_w + 20, target_h + 20], fill=(0, 0, 0, 210))
    shadow = shadow.filter(ImageFilter.GaussianBlur(18))
    base.paste(shadow, (header_x - 20, header_y - 15), shadow)
    base.paste(lg, (header_x, header_y - 12), lg)

    # Marca kokor心sim com halo no kanji
    brand_x = header_x + target_w + 28
    brand_y = header_y + 22
    tdraw.text((brand_x, brand_y), "kokor", font=f_brand, fill=(255, 255, 255, 255))
    kanji_x = brand_x + (k_box[2] - k_box[0]) + 4
    for ox, oy in [(-2, 0), (2, 0), (0, -2), (0, 2)]:
        tdraw.text((kanji_x + ox, brand_y - 2 + oy), "心", font=f_kanji, fill=(255, 23, 84, 110))
    tdraw.text((kanji_x, brand_y - 2), "心", font=f_kanji, fill=(255, 23, 84, 255))
    sim_x = kanji_x + (kj_box[2] - kj_box[0]) + 6
    tdraw.text((sim_x, brand_y), "sim", font=f_brand, fill=(0, 242, 254, 255))

    # 7. Títulos de alto impacto no Miolo Central
    t1_text = "Simulador Cardíaco Open Source"
    t1_box = tdraw.textbbox((0, 0), t1_text, font=f_title1)
    t1_x = (width - (t1_box[2] - t1_box[0])) // 2
    t1_y = 300
    tdraw.text((t1_x, t1_y), t1_text, font=f_title1, fill=(255, 255, 255, 250))

    t2_text = "Eletrofisiologia, Dromotropismo & Hemodinâmica"
    t2_box = tdraw.textbbox((0, 0), t2_text, font=f_title2)
    t2_x = (width - (t2_box[2] - t2_box[0])) // 2
    t2_y = 362
    tdraw.text((t2_x, t2_y), t2_text, font=f_title2, fill=(241, 196, 15, 250))

    # 8. Rodapé no miolo (largura de 720px)
    foot_y = height - 85
    f_bar_w = 720
    f_bar_x = (width - f_bar_w) // 2
    tdraw.line([(f_bar_x, foot_y - 18), (f_bar_x + f_bar_w, foot_y - 18)], fill=(42, 49, 68, 190), width=1)

    tdraw.ellipse([f_bar_x, foot_y + 4, f_bar_x + 14, foot_y + 18], fill=(46, 204, 113, 255))
    tdraw.text((f_bar_x + 24, foot_y), "kokorosim.github.io", font=f_url, fill=(0, 242, 254, 255))

    auth_text = "UFSC // Código Aberto // GPL-3.0"
    abox = tdraw.textbbox((0, 0), auth_text, font=f_author)
    tdraw.text((f_bar_x + f_bar_w - (abox[2] - abox[0]), foot_y + 2), auth_text, font=f_author, fill=(132, 147, 168, 220))

    # Borda decorativa superior neon
    tdraw.line([(0, 0), (width, 0)], fill=(255, 23, 84, 255), width=3)
    tdraw.line([(0, height - 1), (width, height - 1)], fill=(0, 242, 254, 200), width=2)

    # 9. Salvar imagem otimizada em PNG
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    base = base.convert("RGB")
    base.save(output_path, format="PNG", optimize=True)
    file_size_kb = os.path.getsize(output_path) / 1024
    print(f"✅ Banner OG centrado com Safe Zone 4:3 gerado: {output_path} ({width}x{height}, {file_size_kb:.1f} KB)")

if __name__ == "__main__":
    create_og_image()
