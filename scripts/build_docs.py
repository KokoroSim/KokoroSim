#!/usr/bin/env python3
"""
Gera páginas HTML estáticas a partir dos arquivos Markdown do KokoroSim
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
    <title>__PAGE_TITLE__</title>
    <link rel="icon" type="image/png" href="assets/icon.png">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Chakra+Petch:ital,wght@0,400;0,600;0,700;1,400&family=JetBrains+Mono:wght@400;600&family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
    <script>
        window.MathJax = {
            tex: {
                inlineMath: [['$', '$'], ['\\\\(', '\\\\)']],
                displayMath: [['$$', '$$'], ['\\\\[', '\\\\]']],
                processEscapes: true
            },
            options: {
                ignoreHtmlClass: 'tex2jax_ignore',
                processHtmlClass: 'tex2jax_process'
            }
        };
    </script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <style>
        :root {
            --bg-main: #090a0f;
            --bg-card: #10121a;
            --border: #1c202d;
            --border-tech: #2a3144;
            --text-main: #e2e8f0;
            --text-muted: #8493a8;
            --neon-carmine: #ff1754;
            --carmine-glow: rgba(255, 23, 84, 0.45);
            --neon-cyan: #00f2fe;
            --cyan-glow: rgba(0, 242, 254, 0.4);
            --neon-amber: #ffd32a;
            --neon-green: #05c46b;
            --code-bg: #141724;
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            background-color: var(--bg-main);
            color: var(--text-main);
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.7;
            font-size: 16px;
            padding-bottom: 60px;
            background-image: 
                linear-gradient(rgba(28, 32, 45, 0.15) 1px, transparent 1px),
                linear-gradient(90deg, rgba(28, 32, 45, 0.15) 1px, transparent 1px);
            background-size: 32px 32px;
        }

        nav.top-nav {
            position: sticky;
            top: 0;
            z-index: 1000;
            background: rgba(9, 10, 15, 0.94);
            backdrop-filter: blur(12px);
            border-bottom: 2px solid var(--border-tech);
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.6);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 10px 24px;
            max-width: 100%;
        }

        .brand {
            display: flex;
            align-items: center;
            gap: 10px;
            font-family: 'Chakra Petch', sans-serif;
            font-size: 1.25rem;
            font-weight: 700;
            color: #fff;
            text-decoration: none;
            letter-spacing: 0.05em;
            white-space: nowrap;
            flex-shrink: 0;
            min-width: max-content;
        }

        .brand-logo {
            width: 28px;
            height: 28px;
            min-width: 28px;
            min-height: 28px;
            flex-shrink: 0;
            object-fit: contain;
            filter: drop-shadow(0 0 8px var(--carmine-glow));
        }

        .brand-kanji {
            color: var(--neon-carmine);
            text-shadow: 0 0 10px var(--carmine-glow);
            font-weight: 900;
            margin: 0 1px;
        }

        .brand-sim {
            color: var(--neon-cyan);
            text-shadow: 0 0 8px var(--cyan-glow);
            font-weight: 700;
        }

        .nav-links {
            display: flex;
            gap: 14px;
            align-items: center;
            flex-wrap: wrap;
            font-family: 'Chakra Petch', sans-serif;
        }

        .nav-links a {
            color: var(--text-muted);
            text-decoration: none;
            font-size: 0.92rem;
            font-weight: 600;
            transition: all 0.2s ease;
            padding: 6px 12px;
            border: 1px solid transparent;
            clip-path: polygon(6px 0%, 100% 0%, calc(100% - 6px) 100%, 0% 100%);
        }

        .nav-links a:hover, .nav-links a.active {
            color: #ffffff;
            border-color: var(--neon-cyan);
            background: rgba(0, 242, 254, 0.12);
            box-shadow: 0 0 12px var(--cyan-glow);
            text-shadow: 0 0 6px var(--cyan-glow);
        }

        .nav-links a.btn-app {
            background: linear-gradient(135deg, var(--neon-carmine), #d61346);
            color: #ffffff;
            font-weight: 700;
            padding: 7px 18px;
            border: 1px solid #ff4d79;
            box-shadow: 0 0 15px var(--carmine-glow);
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }

        .nav-links a.btn-app:hover {
            background: linear-gradient(135deg, #ff4375, var(--neon-carmine));
            box-shadow: 0 0 24px var(--neon-carmine);
            transform: translateY(-1px);
        }

        main.doc-container {
            max-width: 960px;
            margin: 36px auto;
            padding: 0 24px;
        }

        h1, h2, h3, h4, h5, h6 {
            font-family: 'Chakra Petch', sans-serif;
            color: #ffffff;
            font-weight: 700;
            margin-top: 1.8em;
            margin-bottom: 0.6em;
            letter-spacing: 0.02em;
        }

        h1 {
            font-size: 2.3rem;
            border-bottom: 2px solid var(--border-tech);
            padding-bottom: 12px;
            margin-top: 0.5em;
            color: #ffffff;
            text-shadow: 0 0 12px rgba(255, 255, 255, 0.2);
            position: relative;
        }

        h1::after {
            content: "";
            position: absolute;
            bottom: -2px;
            left: 0;
            width: 80px;
            height: 2px;
            background: var(--neon-carmine);
            box-shadow: 0 0 8px var(--neon-carmine);
        }

        h1[align="center"] {
            border-bottom: none;
            padding-bottom: 0;
            text-shadow: 0 0 20px rgba(0, 242, 254, 0.3);
        }

        h1[align="center"]::after {
            display: none;
        }

        h2 {
            font-size: 1.55rem;
            border-bottom: 1px solid var(--border);
            padding-bottom: 8px;
            color: var(--neon-cyan);
            text-shadow: 0 0 8px rgba(0, 242, 254, 0.25);
        }

        h3 {
            font-size: 1.25rem;
            color: var(--neon-amber);
        }

        p {
            margin-bottom: 1.1em;
            color: var(--text-main);
        }

        a {
            color: var(--neon-cyan);
            text-decoration: none;
            transition: 0.2s;
            border-bottom: 1px dotted rgba(0, 242, 254, 0.4);
        }

        a:hover {
            color: #80fbff;
            text-shadow: 0 0 8px var(--cyan-glow);
            border-bottom-color: var(--neon-cyan);
        }

        ul, ol {
            margin-bottom: 1.2em;
            padding-left: 28px;
        }

        li {
            margin-bottom: 0.4em;
        }

        hr {
            border: none;
            border-top: 1px solid var(--border-tech);
            margin: 2.2em 0;
            position: relative;
        }

        pre {
            background: var(--code-bg);
            border: 1px solid var(--border-tech);
            border-left: 3px solid var(--neon-cyan);
            border-radius: 4px;
            padding: 16px;
            overflow-x: auto;
            margin-bottom: 1.4em;
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.88rem;
            box-shadow: inset 0 2px 8px rgba(0, 0, 0, 0.5);
        }

        code {
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.9em;
            background: rgba(255, 255, 255, 0.08);
            padding: 2px 6px;
            border-radius: 3px;
            color: #ffd32a;
            border: 1px solid rgba(255, 255, 255, 0.05);
        }

        pre code {
            background: none;
            border: none;
            padding: 0;
            color: inherit;
        }

        table {
            width: 100%;
            border-collapse: collapse;
            margin: 1.8em 0;
            background: var(--bg-card);
            border-radius: 4px;
            overflow: hidden;
            border: 1px solid var(--border-tech);
            box-shadow: 0 4px 14px rgba(0, 0, 0, 0.4);
        }

        th, td {
            padding: 11px 16px;
            text-align: left;
            border-bottom: 1px solid var(--border);
            font-size: 0.92rem;
        }

        th {
            font-family: 'Chakra Petch', sans-serif;
            background: rgba(28, 32, 45, 0.6);
            color: var(--neon-cyan);
            font-weight: 700;
            letter-spacing: 0.04em;
            text-transform: uppercase;
            font-size: 0.85rem;
        }

        tr:hover {
            background: rgba(0, 242, 254, 0.03);
        }

        blockquote {
            border-left: 3px solid var(--neon-carmine);
            background: rgba(255, 23, 84, 0.06);
            padding: 14px 18px;
            margin: 1.4em 0;
            border-radius: 0 4px 4px 0;
            color: var(--text-main);
            box-shadow: inset 0 0 12px rgba(255, 23, 84, 0.04);
        }

        footer {
            max-width: 960px;
            margin: 60px auto 0 auto;
            padding: 24px 20px 0 20px;
            border-top: 1px solid var(--border-tech);
            text-align: center;
            font-size: 0.85rem;
            color: var(--text-muted);
            font-family: 'Chakra Petch', sans-serif;
        }

        footer p {
            margin-bottom: 6px;
        }

        .nav-toggle {
            display: none;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid var(--border-tech);
            border-radius: 4px;
            width: 40px;
            height: 40px;
            cursor: pointer;
            padding: 0;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            gap: 5px;
            transition: all 0.25s ease;
            z-index: 102;
            flex-shrink: 0;
        }

        .nav-toggle:hover {
            border-color: var(--neon-cyan);
            background: rgba(0, 242, 254, 0.1);
            box-shadow: 0 0 10px var(--cyan-glow);
        }

        .hamburger-bar {
            width: 22px;
            height: 2px;
            background: var(--neon-cyan);
            transition: all 0.25s ease-in-out;
            box-shadow: 0 0 4px var(--cyan-glow);
            border-radius: 1px;
            display: block;
        }

        .nav-toggle.open .hamburger-bar:nth-child(1) {
            transform: translateY(7px) rotate(45deg);
        }

        .nav-toggle.open .hamburger-bar:nth-child(2) {
            opacity: 0;
            transform: scaleX(0);
        }

        .nav-toggle.open .hamburger-bar:nth-child(3) {
            transform: translateY(-7px) rotate(-45deg);
        }

        @media (max-width: 1024px) {
            .nav-toggle {
                display: flex;
            }

            .nav-links {
                display: none;
                position: absolute;
                top: 100%;
                left: 0;
                right: 0;
                background: rgba(9, 11, 18, 0.98);
                backdrop-filter: blur(20px);
                -webkit-backdrop-filter: blur(20px);
                border-bottom: 2px solid var(--border-tech);
                box-shadow: 0 16px 32px rgba(0, 0, 0, 0.85);
                padding: 16px 24px 24px 24px;
                flex-direction: column;
                align-items: stretch;
                gap: 8px;
                max-height: calc(100vh - 65px);
                overflow-y: auto;
                z-index: 101;
            }

            .nav-links.open {
                display: flex;
            }

            .nav-links a {
                padding: 12px 16px;
                font-size: 1rem;
                clip-path: none;
                border-radius: 4px;
                background: rgba(255, 255, 255, 0.03);
                border: 1px solid rgba(255, 255, 255, 0.05);
            }

            .nav-links a.btn-app {
                clip-path: none;
                border-radius: 4px;
                margin-bottom: 4px;
            }
        }

        @media (max-width: 768px) {
            body {
                font-size: 15px;
            }
            h1 {
                font-size: 1.8rem;
            }
            .top-nav {
                padding: 10px 16px;
            }
            .brand {
                font-size: 1.15rem;
            }
        }
    </style>
</head>
<body>
    <nav class="top-nav">
        <a href="./index.html" class="brand">
            <img src="assets/icon.png" alt="KokoroSim" class="brand-logo">
            <span>kokor<span class="brand-kanji">心</span><span class="brand-sim">sim</span></span>
        </a>
        <button class="nav-toggle" id="navToggle" aria-label="Abrir menu de navegação" aria-expanded="false">
            <span class="hamburger-bar"></span>
            <span class="hamburger-bar"></span>
            <span class="hamburger-bar"></span>
        </button>
        <div class="nav-links" id="navLinks">
            <a href="./app.html" class="btn-app">🚀 Simulador Web</a>
            <a href="./index.html" class="__ACTIVE_SOBRE__">📖 Sobre</a>
            <a href="./roteiro.html" class="__ACTIVE_ROTEIRO__">🎓 Roteiro Prático</a>
            <a href="https://github.com/KokoroSim/KokoroSim" target="_blank">💻 GitHub</a>
        </div>
    </nav>
    <main class="doc-container">
        __PAGE_CONTENT__
    </main>
    <footer>
        <p><b>kokor<span style="color: var(--neon-carmine); font-weight: 900;">心</span><span style="color: var(--neon-cyan); font-weight: bold;">sim</span></b> — Simulador Eletrofisiológico Cardíaco Celular e Hemodinâmico em Tempo Real</p>
        <p style="font-size: 0.85rem; color: #7f8c8d; margin-top: 4px;">Desenvolvido com Rust, WebAssembly & Dioxus // Licença GPLv3</p>
    </footer>
    <script>
        const toggle = document.getElementById('navToggle');
        const navLinks = document.getElementById('navLinks');
        if (toggle && navLinks) {
            toggle.addEventListener('click', (e) => {
                e.stopPropagation();
                const isOpen = navLinks.classList.toggle('open');
                toggle.classList.toggle('open', isOpen);
                toggle.setAttribute('aria-expanded', isOpen ? 'true' : 'false');
            });
            document.addEventListener('click', (e) => {
                if (!navLinks.contains(e.target) && !toggle.contains(e.target)) {
                    navLinks.classList.remove('open');
                    toggle.classList.remove('open');
                    toggle.setAttribute('aria-expanded', 'false');
                }
            });
            navLinks.querySelectorAll('a').forEach(a => {
                a.addEventListener('click', () => {
                    navLinks.classList.remove('open');
                    toggle.classList.remove('open');
                    toggle.setAttribute('aria-expanded', 'false');
                });
            });
        }
    </script>
</body>
</html>
"""

DOCS_MAP = [
    {
        "src": "README.md",
        "dest": "index.html",
        "title": "KokoroSim — Simulador Eletrofisiológico Cardíaco em Tempo Real",
        "active": "sobre"
    },
    {
        "src": "README.md",
        "dest": "sobre.html",
        "title": "Sobre o Projeto — KokoroSim",
        "active": "sobre"
    },
    {
        "src": "docs/modelos.md",
        "dest": "modelos.html",
        "title": "Modelos Biofísicos e Referências Científicas",
        "active": "modelos"
    },
    {
        "src": "docs/identidade_visual.md",
        "dest": "identidade.html",
        "title": "Identidade Visual e Conceito do Logotipo",
        "active": "identidade"
    },
    {
        "src": "ROADMAP.md",
        "dest": "roadmap.html",
        "title": "Roadmap de Desenvolvimento",
        "active": "roadmap"
    },
    {
        "src": "ARCHITECTURE.md",
        "dest": "arquitetura.html",
        "title": "Decisões de Arquitetura e Engenharia",
        "active": "arquitetura"
    },
    {
        "src": "docs/roteiro_aulas_praticas.md",
        "dest": "roteiro.html",
        "title": "Guia Didático e Roteiro de Aulas Práticas",
        "active": "roteiro"
    }
]

def main():
    root_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(root_dir)

    target_dir_name = sys.argv[1] if len(sys.argv) > 1 else os.environ.get("DOCS_OUT_DIR", "dist")
    out_dir = target_dir_name if os.path.isabs(target_dir_name) else os.path.join(root_dir, target_dir_name)
    os.makedirs(out_dir, exist_ok=True)

    # Sincroniza assets para o diretório de destino
    src_assets = os.path.join(root_dir, "ui", "assets")
    dest_assets = os.path.join(out_dir, "assets")
    if os.path.exists(src_assets):
        import shutil
        if os.path.exists(dest_assets):
            shutil.rmtree(dest_assets)
        shutil.copytree(src_assets, dest_assets)

    # Copia templates HTML base se existirem
    for html_name in ["app.html", "simulador.html"]:
        src_html = os.path.join(root_dir, "ui", html_name)
        if os.path.exists(src_html):
            import shutil
            shutil.copy2(src_html, os.path.join(out_dir, html_name))

    md = markdown.Markdown(extensions=[
        'tables',
        'fenced_code',
        'toc',
        'attr_list',
        'def_list'
    ])

    for doc in DOCS_MAP:
        src_path = os.path.join(root_dir, doc["src"])
        dest_path = os.path.join(out_dir, doc["dest"])

        if not os.path.exists(src_path):
            print(f"⚠️ Aviso: Arquivo fonte não encontrado: {src_path}")
            continue

        with open(src_path, "r", encoding="utf-8") as f:
            raw_text = f.read()

        # Ajusta links internos entre documentos markdown para as páginas HTML geradas
        raw_text = raw_text.replace("docs/modelos.md", "./modelos.html")
        raw_text = raw_text.replace("docs/roteiro_aulas_praticas.md", "./roteiro.html")
        raw_text = raw_text.replace("docs/identidade_visual.md", "./identidade.html")
        raw_text = raw_text.replace("ROADMAP.md", "./roadmap.html")
        raw_text = raw_text.replace("ARCHITECTURE.md", "./arquitetura.html")
        raw_text = raw_text.replace("README.md", "./index.html")
        raw_text = raw_text.replace("../ui/assets/", "assets/")
        raw_text = raw_text.replace("ui/assets/", "assets/")

        html_body = md.convert(raw_text)
        md.reset()

        page_html = (
            HTML_TEMPLATE
            .replace("__PAGE_TITLE__", doc["title"])
            .replace("__PAGE_CONTENT__", html_body)
            .replace("__ACTIVE_SOBRE__", "active" if doc["active"] == "sobre" else "")
            .replace("__ACTIVE_MODELOS__", "active" if doc["active"] == "modelos" else "")
            .replace("__ACTIVE_IDENTIDADE__", "active" if doc["active"] == "identidade" else "")
            .replace("__ACTIVE_ROTEIRO__", "active" if doc["active"] == "roteiro" else "")
            .replace("__ACTIVE_ROADMAP__", "active" if doc["active"] == "roadmap" else "")
            .replace("__ACTIVE_ARQUITETURA__", "active" if doc["active"] == "arquitetura" else "")
        )

        with open(dest_path, "w", encoding="utf-8") as f:
            f.write(page_html)

        print(f"✅ Gerado: {dest_path} a partir de {doc['src']}")

if __name__ == "__main__":
    main()
