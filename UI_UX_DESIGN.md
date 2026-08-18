# Especificação de Interface (Mockup de Cockpit Clínico)

A interface abandona a estética de pixel-art em favor de uma resolução vetorial lisa (*Dark Mode / Glassmorphism*). O objetivo é fornecer precisão diagnóstica em nível de subpixel, remetendo a um monitor moderno de UTI atrelado a um osciloscópio de bancada.

## 1. Topologia (Grid Layout)
*   **Barra Superior (HUD):** Ocupa 10% da altura vertical. Texto monoespaçado neon. Contém os dados dinâmicos processados pelo simulador (BPM, PR, QRS, QT).
*   **Painel Esquerdo (Controles):** Ocupa 25% da largura. Estética translúcida (Glassmorphism). Contém menus expansíveis (*accordions*) para Eletrólitos, Sistema Autônomo e Fármacos. Usa sliders (*range inputs*) com botão de reset lateral.
*   **Palco Central (Gráficos):** Ocupa o restante do espaço. Dividido horizontalmente em duas telas pretas:
    1.  **Monitor Superior:** Eletrofisiologia celular (Nó SA, Nó AV, Ventrículo sobrepostos com opacidades variadas).
    2.  **Monitor Inferior:** ECG de superfície gerado pela cascata elétrica.
    *Ambos possuem eixo X (tempo) estritamente sincronizados.*

## 2. Paleta de Cores (Cenário Escuro)
*   **Fundo:** `#0a0b10` (Cinza muito escuro, quase preto).
*   **Glassmorphism:** Fundo rgba translúcido (`rgba(255,255,255,0.05)`) com desfoque (`backdrop-filter: blur(10px)`).
*   **Traçados (Neon):**
    *   HUD Valores: Branco `#ffffff` ou Ciano `#00ffff`
    *   Alerta Crítico: Vermelho Sangue `#ff3333`
    *   Nó SA: Amarelo `#f1c40f`
    *   Nó AV: Laranja `#e67e22`
    *   Ventrículo: Verde Clínico `#2ecc71`
    *   Onda Fantasma: Cinza Transparente `rgba(255,255,255,0.2)`

