#!/usr/bin/env python3
"""
Gera o banner de compartilhamento Open Graph / Twitter Card (1200x630 pixels)
para o KokoroSim com Safe Zone ampla (protecao contra cortes de bordas),
fontes ampliadas de alto impacto visual e eliminacao de textos redundantes.
"""

import os
from PIL import Image, ImageDraw, ImageFont, ImageFilter

def create_og_image(output_path="ui/assets/og_preview.png"):
    width, height = 1200, 630
    
    # 1. Base com fundo profundo e grade técnica sutil
    base = Image.new("RGBA", (width, height), (9, 10, 15, 255))
    draw = ImageDraw.Draw(base)

    grid_spacing = 32
    grid_color = (28, 32, 45, 100)
    for x in range(0, width, grid_spacing):
        draw.line([(x, 0), (x, height)], fill=grid_color, width=1)
    for y in range(0, height, grid_spacing):
        draw.line([(0, y), (width, y)], fill=grid_color, width=1)

    # 2. Luz difusa de fundo (Glows em Carmesim e Ciano)
    glow_layer = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow_layer)
    
    # Halo carmesim suave atrás do isotipo
    glow_draw.ellipse([40, 90, 480, 520], fill=(255, 23, 84, 55))
    # Halo ciano suave sob o título
    glow_draw.ellipse([630, 120, 1150, 530], fill=(0, 242, 254, 35))
    glow_layer = glow_layer.filter(ImageFilter.GaussianBlur(75))
    base = Image.alpha_composite(base, glow_layer)

    # 3. Carregar e recortar o logotipo completo (sem corte no raio)
    logo_path = "ui/assets/logo.png"
    if os.path.exists(logo_path):
        logo = Image.open(logo_path).convert("RGBA")
        bbox = logo.getbbox()
        if bbox:
            logo = logo.crop(bbox)
        
        # Redimensiona para altura de 300px mantendo proporção (largura ~438px)
        target_h = 300
        target_w = int(logo.width * (target_h / logo.height))
        logo = logo.resize((target_w, target_h), Image.Resampling.LANCZOS)
        
        # Sombra suave sob o logotipo
        shadow = Image.new("RGBA", (target_w + 40, target_h + 40), (0, 0, 0, 0))
        shadow_draw = ImageDraw.Draw(shadow)
        shadow_draw.ellipse([20, 20, target_w + 20, target_h + 20], fill=(0, 0, 0, 210))
        shadow = shadow.filter(ImageFilter.GaussianBlur(22))
        
        # Margem esquerda confortável (Safe Zone)
        logo_x = 48
        logo_y = (height - target_h) // 2 - 8
        base.paste(shadow, (logo_x - 20, logo_y - 10), shadow)
        base.paste(logo, (logo_x, logo_y), logo)

    # 4. Tipografia de Alto Impacto (Sem redundâncias, fontes grandes e legíveis)
    text_draw = ImageDraw.Draw(base)

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

    # Escala tipográfica generosa
    font_brand = ImageFont.truetype(font_bold_path, 78) if font_bold_path else ImageFont.load_default()
    font_kanji = ImageFont.truetype(font_cjk_path, 82) if font_cjk_path else font_brand
    font_badge = ImageFont.truetype(font_bold_path, 16) if font_bold_path else ImageFont.load_default()
    font_title = ImageFont.truetype(font_bold_path, 38) if font_bold_path else ImageFont.load_default()
    font_url = ImageFont.truetype(font_bold_path, 23) if font_bold_path else ImageFont.load_default()
    font_author = ImageFont.truetype(font_reg_path, 20) if font_reg_path else ImageFont.load_default()

    # Coluna de texto com Safe Zone à direita
    start_x = 512
    curr_y = 100

    # Badge superior tecnológico
    badge_text = " RUST + WEBASSEMBLY // 60 FPS "
    badge_bbox = text_draw.textbbox((start_x, curr_y), badge_text, font=font_badge)
    badge_pad_x, badge_pad_y = 12, 6
    text_draw.rectangle(
        [badge_bbox[0] - badge_pad_x, badge_bbox[1] - badge_pad_y, badge_bbox[2] + badge_pad_x, badge_bbox[3] + badge_pad_y],
        fill=(14, 20, 32, 230),
        outline=(0, 242, 254, 210),
        width=1
    )
    text_draw.text((start_x, curr_y), badge_text, font=font_badge, fill=(0, 242, 254, 255))
    curr_y += 54

    # Marca Gigante: kokor + 心 + sim
    text_draw.text((start_x, curr_y), "kokor", font=font_brand, fill=(255, 255, 255, 255))
    kokor_bbox = text_draw.textbbox((start_x, curr_y), "kokor", font=font_brand)
    kokor_w = kokor_bbox[2] - kokor_bbox[0]
    
    kanji_x = start_x + kokor_w + 4
    # Halo luminoso no kanji
    for ox, oy in [(-2, 0), (2, 0), (0, -2), (0, 2), (-1, -1), (1, 1)]:
        text_draw.text((kanji_x + ox, curr_y - 2 + oy), "心", font=font_kanji, fill=(255, 23, 84, 110))
    text_draw.text((kanji_x, curr_y - 2), "心", font=font_kanji, fill=(255, 23, 84, 255))
    kanji_bbox = text_draw.textbbox((kanji_x, curr_y - 2), "心", font=font_kanji)
    kanji_w = kanji_bbox[2] - kanji_bbox[0]
    
    sim_x = kanji_x + kanji_w + 6
    text_draw.text((sim_x, curr_y), "sim", font=font_brand, fill=(0, 242, 254, 255))

    curr_y += 114

    # Título do Projeto em 2 linhas de altíssimo impacto (38pt)
    text_draw.text((start_x, curr_y), "Simulador Eletrofisiológico", font=font_title, fill=(255, 255, 255, 250))
    curr_y += 50
    text_draw.text((start_x, curr_y), "Cardíaco & Hemodinâmico", font=font_title, fill=(241, 196, 15, 250))

    # 5. Barra inferior tecnológica com Safe Zone (dentro dos limites seguros)
    foot_y = height - 70
    right_margin = width - 75
    text_draw.line([(start_x, foot_y - 18), (right_margin, foot_y - 18)], fill=(42, 49, 68, 190), width=1)
    
    # Marcador de status online
    text_draw.ellipse([start_x, foot_y + 4, start_x + 14, foot_y + 18], fill=(46, 204, 113, 255))
    text_draw.text((start_x + 26, foot_y), "kokorosim.github.io", font=font_url, fill=(0, 242, 254, 255))
    
    author_text = "UFSC // Código Aberto"
    author_bbox = text_draw.textbbox((0, 0), author_text, font=font_author)
    author_w = author_bbox[2] - author_bbox[0]
    text_draw.text((right_margin - author_w, foot_y + 2), author_text, font=font_author, fill=(132, 147, 168, 220))

    # Borda decorativa superior neon
    text_draw.line([(0, 0), (width, 0)], fill=(255, 23, 84, 255), width=3)
    text_draw.line([(0, height - 1), (width, height - 1)], fill=(0, 242, 254, 200), width=2)

    # 6. Salvar imagem otimizada em PNG
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    base = base.convert("RGB")
    base.save(output_path, format="PNG", optimize=True)
    file_size_kb = os.path.getsize(output_path) / 1024
    print(f"✅ Banner OG com Safe Zone e fontes ampliadas gerado: {output_path} ({width}x{height}, {file_size_kb:.1f} KB)")

if __name__ == "__main__":
    create_og_image()
