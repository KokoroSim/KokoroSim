# Roadmap de Implementação

A construção do simulador seguirá passos modulares focados em isolar a matemática da interface gráfica.

*   **Fase 1: Estrutura Visual (Mockup)**
    *   Implementar o layout estático em HTML/CSS (Cockpit, HUD de UTI e Sliders).
    *   Testar a responsividade e o motor de renderização do Canvas com dados falsos.
*   **Fase 2: Motor Matemático Base (Web Worker)**
    *   Criar o *Worker* isolado.
    *   Implementar o integrador numérico (Método de Euler) com passo variável ($\Delta t = 0.05\text{ ms}$).
*   **Fase 3: Transcrição dos Modelos Biofísicos**
    *   Traduzir o código C do Ten Tusscher (2004) para JavaScript no Worker.
    *   Traduzir o código C do Severi (2012).
    *   Traduzir o código C do Inada (2009).
*   **Fase 4: Acoplamento e Condução**
    *   Estabelecer o repasse do impulso: SA $\rightarrow$ AV $\rightarrow$ Ventrículo.
    *   Garantir a latência do Nó AV para formação do Intervalo PR.
*   **Fase 5: Renderização e Modos de Tela**
    *   Sincronizar a saída do Worker com o Canvas.
    *   Implementar os três modos de visualização (Varredura Contínua, Gatilho Único, Paginação Longa).
    *   Implementar o carimbo dinâmico da "Onda Fantasma".
*   **Fase 6: Interface de Farmacologia e Eletrólitos**
    *   Conectar os *sliders* da UI às variáveis basais do Worker via `postMessage`.
    *   Aplicar o modelo de inibição fracional para os fármacos.
*   **Fase 7: Áudio e UX Final**
    *   Sintetizar o *Bip* (Pico da Fase 0) e os sons B1/B2 (*Tum-Tá* nas Fases 2 e 3).

