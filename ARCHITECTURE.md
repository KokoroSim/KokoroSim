# Arquitetura do Sistema e Decisões de Engenharia

Este documento detalha o "porquê" das abordagens técnicas adotadas no projeto, contrapondo-as às alternativas descartadas.

## 1. Separação de Threads (Web Workers vs. Main Thread)
*   **Decisão Adotada:** O cálculo das equações diferenciais roda em um Web Worker isolado. A UI (Canvas e Sliders) roda na Main Thread.
*   **Alternativa Recusada:** Rodar o laço matemático na mesma thread do Canvas.
*   **Justificativa:** O laço de integração precisa rodar milhares de vezes por segundo para evitar rigidez numérica. Se colocado na Main Thread, a interface travaria completamente sempre que o usuário arrastasse um *slider*. O Worker garante que o cálculo flua sem gargalos de UI.

## 2. Abordagem Numérica (Método de Euler vs. Máquina de Estados)
*   **Decisão Adotada:** Resolução contínua das equações de Hodgkin-Huxley/Luo-Rudy via integração de Euler com passo de tempo micro (0.05 ms).
*   **Alternativa Recusada:** Simulação por fases e gatilhos de voltagem hardcoded ("Máquina de Estados").
*   **Justificativa:** A máquina de estados não permite a emergência sistêmica de patologias combinadas (ex: Hipercalemia + Hipocalcemia). O modelo rígido mascara distorções de canais. A integração contínua reflete a biofísica real, onde a voltagem é apenas consequência da soma das correntes iônicas instantâneas.

## 3. Motor da Lógica (JavaScript Puro vs. WebAssembly)
*   **Decisão Adotada:** Tradução manual das constantes algébricas de C para Vanilla JS.
*   **Alternativa Recusada:** Compilar os modelos via WebAssembly (WASM).
*   **Justificativa:** WASM tem alta performance de cálculo, mas impõe um "pedágio" (overhead) severo de memória ao trafegar arrays contínuos de coordenadas (X, Y) a 60 FPS de volta para o JavaScript a fim de desenhar no Canvas. Para simulações isoladas (0D), o motor JIT do V8 processa JS puro com velocidade suficiente, eliminando a complexidade da ponte JS-WASM.

## 4. Renderização do Gráfico (Canvas 2D LineTo vs. Interpolação Spline)
*   **Decisão Adotada:** Alta densidade de pontos plotados com comandos retos do Canvas (`lineTo`) + *Anti-aliasing* nativo do navegador.
*   **Alternativa Recusada:** Computar equações de interpolação curva (Bézier) entre pontos distantes (ex: a cada 10 ms).
*   **Justificativa:** A interpolação gera "alucinações" matemáticas em transições rápidas (como a Fase 0), distorcendo a leitura clínica. A alta amostragem do modelo (calculada no Worker) fornece pontos suficientes para que o `lineTo` de 1 pixel de largura pareça perfeitamente curvo ao olho humano.

