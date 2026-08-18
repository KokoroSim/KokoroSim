# Padrões de Código e Nomenclatura

Para manter a sanidade entre o mundo do desenvolvimento web e o meio acadêmico da biologia, adotaremos regras estritas de nomenclatura.

## 1. Variáveis Matemáticas e Biofísicas
Nunca abrevie ou modifique o nome de uma variável proveniente dos artigos científicos, mesmo que fuja do padrão de programação usual. A rastreabilidade é mais importante que a estética do código.
*   **Condutâncias:** Sempre use `g_` seguido do íon e `_max` (ex: `g_Na_max`, `g_CaL_max`).
*   **Correntes:** Sempre use `I_` seguido do íon/canal (ex: `I_Na`, `I_K1`, `I_to`).
*   **Portões (Gating):** Mantenha as letras exatas do paper (`m`, `h`, `j`, `d`, `f`). Não renomeie para `activationGateSodium`.
*   **Concentrações:** Use sufixo `_i` para intracelular e `_o` para extracelular (ex: `K_o`, `Na_i`).

## 2. Convenções do JavaScript
Para todo o resto do código que controla a interface e a mecânica:
*   **Variáveis, métodos e instâncias:** `camelCase` (ex: `drawGhostWave`, `updateCanvas`).
*   **Classes e Componentes:** `PascalCase` (ex: `VentricularCell`, `ActionPotentialMonitor`).
*   **Constantes de UI:** `UPPER_SNAKE_CASE` (ex: `CANVAS_WIDTH`, `REFRESH_RATE_MS`).

## 3. Organização de Arquivos
*   `ui.js` $\rightarrow$ Manipulação do DOM, sliders e layout.
*   `renderer.js` $\rightarrow$ Lógica exclusiva de desenho no Canvas.
*   `engine.worker.js` $\rightarrow$ O ambiente isolado que hospeda as fórmulas.
*   `models/` $\rightarrow$ Diretório contendo as traduções isoladas (`tentusscher.js`, `severi.js`).

