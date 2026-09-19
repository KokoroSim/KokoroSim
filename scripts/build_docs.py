#!/usr/bin/env python3
"""
Gera páginas HTML estáticas a partir dos arquivos Markdown do SimCardio
com tema escuro médico, suporte a MathJax para fórmulas LaTeX e navegação integrada.
"""

import os
import sys
import markdown

HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} — KokoroSim 🌸</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;600&display=swap" rel="stylesheet">
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <style>
        :root {{
            --bg-main: #0a0a0c;
            --bg-card: #121216;
            --border: #22222a;
            --text-main: #e2e8f0;
            --text-muted: #94a3b8;
            --neon-cyan: #00cec9;
            --neon-yellow: #f1c40f;
            --neon-coral: #ff7675;
            --neon-green: #2ecc71;
            --code-bg: #181820;
        }}

        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}

        body {{
            background-color: var(--bg-main);
            color: var(--text-main);
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.68;
            font-size: 16px;
            padding-bottom: 60px;
        }}

        nav.top-nav {{
            position: sticky;
            top: 0;
            z-index: 1000;
            background: rgba(10, 10, 12, 0.88);
            backdrop-filter: blur(10px);
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 24px;
            max-width: 100%;
        }}

        .brand {{
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 1.15rem;
            font-weight: 700;
            color: #fff;
            text-decoration: none;
        }}

        .brand span.pulse {{
            color: var(--neon-cyan);
        }}

        .nav-links {{
            display: flex;
            gap: 16px;
            align-items: center;
            flex-wrap: wrap;
        }}

        .nav-links a {{
            color: var(--text-muted);
            text-decoration: none;
            font-size: 0.9rem;
            font-weight: 500;
            transition: color 0.2s;
            padding: 4px 8px;
            border-radius: 4px;
        }}

        .nav-links a:hover, .nav-links a.active {{
            color: var(--neon-cyan);
            background: rgba(0, 206, 201, 0.08);
        }}

        .nav-links a.btn-app {{
            background: var(--neon-cyan);
            color: #000;
            font-weight: 700;
            padding: 6px 14px;
            border-radius: 6px;
        }}

        .nav-links a.btn-app:hover {{
            background: #81ecec;
            color: #000;
        }}

        main.doc-container {{
            max-width: 920px;
            margin: 36px auto;
            padding: 0 20px;
        }}

        h1, h2, h3, h4, h5, h6 {{
            color: #ffffff;
            font-weight: 700;
            margin-top: 1.8em;
            margin-bottom: 0.6em;
            letter-spacing: -0.02em;
        }}

        h1 {{
            font-size: 2.2rem;
            border-bottom: 1px solid var(--border);
            padding-bottom: 12px;
            margin-top: 0.5em;
            color: var(--neon-cyan);
        }}

        h2 {{
            font-size: 1.5rem;
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
            padding-bottom: 8px;
        }}

        h3 {{
            font-size: 1.25rem;
            color: var(--neon-yellow);
        }}

        p {{
            margin-bottom: 1.1em;
            color: var(--text-main);
        }}

        a {{
            color: var(--neon-cyan);
            text-decoration: none;
            transition: 0.2s;
        }}

        a:hover {{
            text-decoration: underline;
        }}

        ul, ol {{
            margin-bottom: 1.2em;
            padding-left: 28px;
        }}

        li {{
            margin-bottom: 0.4em;
        }}

        hr {{
            border: none;
            border-top: 1px solid var(--border);
            margin: 2em 0;
        }}

        pre {{
            background: var(--code-bg);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 16px;
            overflow-x: auto;
            margin-bottom: 1.4em;
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.88rem;
        }}

        code {{
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.9em;
            background: rgba(255, 255, 255, 0.07);
            padding: 2px 6px;
            border-radius: 4px;
            color: #ffd166;
        }}

        pre code {{
            background: none;
            padding: 0;
            color: inherit;
        }}

        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1.5em 0;
            background: var(--bg-card);
            border-radius: 8px;
            overflow: hidden;
            border: 1px solid var(--border);
        }}

        th, td {{
            padding: 10px 14px;
            text-align: left;
            border-bottom: 1px solid var(--border);
            font-size: 0.92rem;
        }}

        th {{
            background: rgba(255, 255, 255, 0.04);
            color: var(--neon-cyan);
            font-weight: 600;
        }}

        tr:hover {{
            background: rgba(255, 255, 255, 0.02);
        }}

        blockquote {{
            border-left: 3px solid var(--neon-cyan);
            background: rgba(0, 206, 201, 0.05);
            padding: 12px 16px;
            margin: 1.2em 0;
            border-radius: 0 6px 6px 0;
            color: var(--text-muted);
        }}

        footer {{
            max-width: 920px;
            margin: 60px auto 0 auto;
            padding-top: 20px;
            border-top: 1px solid var(--border);
            text-align: center;
            font-size: 0.85rem;
            color: var(--text-muted);
        }}

        @media (max-width: 768px) {{
            body {{
                font-size: 15px;
            }}
            h1 {{
                font-size: 1.8rem;
            }}
            .nav-links {{
                gap: 8px;
            }}
        }}
    </style>
</head>
<body>
    <nav class="top-nav">
        <a href="./index.html" class="brand">
            🌸 <span class="pulse">KokoroSim</span>
        </a>
        <div class="nav-links">
            <a href="./index.html" class="btn-app">▶ Simulador Web</a>
            <a href="./sobre.html" class="{active_sobre}">📖 Sobre</a>
            <a href="./roteiro.html" class="{active_roteiro}">🎓 Roteiro Prático</a>
            <a href="./roadmap.html" class="{active_roadmap}">🗺️ Roadmap</a>
            <a href="./arquitetura.html" class="{active_arquitetura}">🏛️ Arquitetura</a>
            <a href="https://github.com/KokoroSim/kokorosim" target="_blank">💻 GitHub</a>
        </div>
    </nav>
    <main class="doc-container">
        {content}
    </main>
    <footer>
        <p><b>KokoroSim 🌸</b> — Simulador Eletrofisiológico Cardíaco Celular e Hemodinâmico em Tempo Real</p>
        <p>Software Livre distribuído sob a licença GNU GPLv3. Desenvolvido para universidades e centros de pesquisa.</p>
    </footer>
</body>
</html>
"""

DOCS_MAP = [
    {
        "src": "README.md",
        "dest": "ui/sobre.html",
        "title": "Sobre e Documentação",
        "active": "sobre"
    },
    {
        "src": "ROADMAP.md",
        "dest": "ui/roadmap.html",
        "title": "Roadmap de Desenvolvimento",
        "active": "roadmap"
    },
    {
        "src": "ARCHITECTURE.md",
        "dest": "ui/arquitetura.html",
        "title": "Decisões de Arquitetura",
        "active": "arquitetura"
    },
    {
        "src": "docs/roteiro_aulas_praticas.md",
        "dest": "ui/roteiro.html",
        "title": "Guia Didático e Roteiro de Aulas Práticas",
        "active": "roteiro"
    }
]

def main():
    root_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(root_dir)

    md = markdown.Markdown(extensions=[
        'tables',
        'fenced_code',
        'toc',
        'attr_list',
        'def_list'
    ])

    for doc in DOCS_MAP:
        src_path = os.path.join(root_dir, doc["src"])
        dest_path = os.path.join(root_dir, doc["dest"])

        if not os.path.exists(src_path):
            print(f"Aviso: {src_path} não encontrado, pulando...")
            continue

        with open(src_path, "r", encoding="utf-8") as f:
            raw_text = f.read()

        html_body = md.convert(raw_text)
        md.reset()

        page_html = HTML_TEMPLATE.format(
            title=doc["title"],
            content=html_body,
            active_sobre="active" if doc["active"] == "sobre" else "",
            active_roteiro="active" if doc["active"] == "roteiro" else "",
            active_roadmap="active" if doc["active"] == "roadmap" else "",
            active_arquitetura="active" if doc["active"] == "arquitetura" else "",
        )

        os.makedirs(os.path.dirname(dest_path), exist_ok=True)
        with open(dest_path, "w", encoding="utf-8") as f:
            f.write(page_html)

        print(f"✅ Gerado: {doc['dest']} a partir de {doc['src']}")

if __name__ == "__main__":
    main()
