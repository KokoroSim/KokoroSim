# Arquitetura do Sistema e Decisões de Engenharia — KokoroSim v2.0

Este documento detalha o "porquê" das decisões de engenharia, arquitetura de software e modelagem biofísica adotadas no KokoroSim v2.0, contrapondo-as às alternativas descartadas e documentando a evolução histórica do projeto.

---

## 1. Migração do Motor Matemático: De JavaScript Puro para Rust + WebAssembly (WASM)

* **Decisão Adotada:** Implementação completa do motor eletrofisiológico em **Rust nativo** e compilação para **WebAssembly (WASM)** via `wasm-bindgen` e `wasm-pack`.
* **Alternativa Recusada (v1.0):** Execução do laço de cálculo em JavaScript puro dentro de um Web Worker.
* **Justificativa Técnica:**
  - **Sobrecarga de Ponto Flutuante e JIT Thrashing:** O cálculo simultâneo de 5 modelos acoplados com 160 variáveis de estado a um passo de integração de $dt = 0.001\text{ ms}$ requer 16.000 passos por quadro de 16 ms (1 milhão de passos/segundo). O motor V8 do JavaScript, embora veloz, sofre com deotimizações JIT frequentes causadas por desvios condicionais e pausas de *Garbage Collection* (GC), gerando *stuttering* no traçado do osciloscópio.
  - **Determinismo e Segurança de Tipos:** O compilador Rust com LLVM gera código SIMD vetorizado e otimizado (`-O3`), com overhead zero em abstrações e garantia formal de integridade de memória.
  - **Zero-Copy e FFI sem Serialização:** No modelo v1.0, o Web Worker precisava serializar mensagens via `postMessage` (ou SharedArrayBuffer), introduzindo latência de desserialização JSON/binária. No v2.0, o Dioxus e o `HeartSystem` operam no mesmo espaço de memória linear WASM. A função `run_batch()` preenche um vetor contíguo `Vec<f64>` sem necessidade de cópias ou pontes de thread lentas.

---

## 2. Interface de Usuário: Dioxus 0.6 (Rust-First) vs. Frameworks JS (React/Vue/Svelte)

* **Decisão Adotada:** Interface reativa declarativa escrita em Rust puro utilizando **Dioxus 0.6**.
* **Alternativa Recusada:** Frontend em React, Svelte ou Vanilla TypeScript integrado a Vite.
* **Justificativa Técnica:**
  - **Eliminação do Ecossistema Node/npm:** A compilação e execução passam a depender unicamente da *toolchain* de Rust (`cargo` e `wasm-pack`), eliminando centenas de dependências frágeis do npm e problemas de compatibilidade de versões.
  - **Granularidade Reativa via Signals:** O Dioxus 0.6 utiliza reatividade fina (`use_signal`) que recompila em código nativo sem passar por uma *Virtual DOM* pesada em JavaScript.
  - **Consistência de Tipagem:** Os parâmetros da simulação (`Pharmaco`, `HudMetrics`, `HeartSystem`) são compartilhados de forma transparente entre o motor e a interface como structs nativas de Rust, sem duplicação de interfaces TypeScript ou risco de incompatibilidade de campos.

---

## 3. Condução Elétrica: Injeção Transitória de Corrente vs. Grampeamento Abrupto de Voltagem

* **Decisão Adotada:** Propagação de estímulo por **injeção calibrada de corrente despolarizante transitória** ($I_{stim} = -40\text{ a }-52\text{ pA/pF}$ por 1.5 a 2.0 ms).
* **Alternativa Recusada:** Grampeamento forçado de voltagem de membrana ($V = -15\text{ mV}$ ou $V = -30\text{ mV}$) a cada ciclo.
* **Justificativa Técnica:**
  - O grampeamento abrupto viola a conservação de carga e "reinicia" artificialmente os portões de inativação das correntes iônicas (especialmente os portões $h$ e $j$ dos canais rápidos de sódio e $f$ de cálcio). Isso gerava assincronia e dissociação ventricular artificial.
  - A injeção transitória de corrente reproduz exatamente o estímulo eletrotônico local que uma célula transmite à outra pelas junções comunicantes (*gap junctions*). Se a célula receptora estiver em seu período refratário absoluto, o pulso não causará disparo, simulando fisiologicamente fenômenos reais como extrassístoles bloqueadas e condução oculta.

---

## 4. Dromotropismo Dinâmico do Nó AV vs. Latências Estáticas

* **Decisão Adotada:** Atraso de condução atrioventricular dinâmico sensível à frequência cardíaca (condução decremental), tônus autonômico simpático/vagal, bloqueadores de cálcio e isquemia.
* **Alternativa Recusada:** Cronômetros estáticos fixos (`timer = time + 40.0 ms`).
* **Justificativa Técnica:**
  - O Nó AV é a estrutura mais plástica do sistema de condução cardíaco. Canais de cálcio $I_{Ca,L}$ requerem tempo para se recuperar da inativação. Em taquicardias, a condução decremental alarga o intervalo PR naturalmente, prevenindo arritmias ventriculares descontroladas.
  - A inclusão da modulação por Verapamil e tônus autonômico permite demonstrar em aula e bancada virtual a emergência de Bloqueios AV de 1º, 2º e 3º grau com precisão farmacológica.

---

## 5. Eletrocardiograma Sintético: Gradiente Dipolar Transmural vs. Combinações Lineares Arbitrárias

* **Decisão Adotada:** Gênese do traçado de ECG a partir da diferença de potencial transmural instantânea entre as camadas ventriculares:
  $$\text{ECG}(t) \propto V_{endo}(t) - V_{epi}(t)$$
* **Alternativa Recusada:** Combinação linear arbitrária de uma única célula ventricular com o átrio ($0.1 \cdot V_{atrio} + 0.9 \cdot V_{vent}$).
* **Justificativa Técnica:**
  - Uma única célula ventricular repolariza de cima para baixo ($+20 \to -86\text{ mV}$), o que inevitavelmente produz uma deflexão negativa (Onda T invertida no traçado clássico).
  - No miocárdio humano real, o epicárdio repolariza antes do endocárdio devido à maior densidade de canais $I_{to}$. Portanto, o vetor elétrico de repolarização aponta do endocárdio para o epicárdio, na mesma direção do vetor de despolarização (QRS).
  - A heterogeneidade transmural das 3 camadas (Endocárdio, Célula M, Epicárdio) permite que o complexo **P - Q - R - S - T** com **Onda T positiva e concordante** emerja de forma 100% orgânica a partir das leis de Maxwell e da biofísica celular.

---

## 6. Acoplamento Eletrotônico Miócito-Fibroblasto: Dinâmica Biofísica vs. Fronteiras Rígidas Isoladas

* **Decisão Adotada:** Implementação do modelo biofísico de **MacCannell et al. (2007)** com acoplamento eletrotônico bidirecional contínuo via condutância de junção comunicante ($G_{gap} = \text{fibrose} \times 4.0\text{ nS}$):
  $$I_{gap,myo} = \frac{G_{gap} \cdot (V_{myo} - V_{fib})}{C_{m,myo}}$$
  $$I_{gap,fib} = \frac{G_{gap} \cdot (V_{fib} - V_{myo})}{C_{m,fib}}$$
* **Alternativa Recusada:** Modelação de fibrose puramente como barreira não-condutora estática ou redução paramétrica arbitrária de $G_{Na}$ no miócito.
* **Justificativa Técnica:**
  - Fibroblastos e miofibroblastos cardíacos in vivo não são meros isolantes dielétricos; eles expressam conexinas (Cx43 e Cx45) e formam acoplamento elétrico direto com cardiomiócitos vizinhos.
  - Como os fibroblastos possuem capacitância menor ($C_{m,fib} = 6.3\text{ pF}$) e potencial de repouso elevado ($-35\text{ a }-45\text{ mV}$), o acoplamento eletrotônico atua como um **dreno capacitivo** durante a fase rápida de despolarização (diminuindo $dV/dt_{\max}$) e como uma **fonte de corrente despolarizante diastólica** em repouso (elevando $V_{rest}$ do miócito para $-78\text{ mV}$).
  - Essa despolarização parcial diastólica induz a inativação dependente de voltagem dos canais de sódio ($h_{\infty}$ e $j_{\infty}$ de ten Tusscher), explicando mecanisticamente a lentificação da condução intramiocárdica e o bloqueio unidirecional observados em miocárdios infartados e senescentes.
  - O buffer linear de amostragem no WASM foi expandido de 9 para 10 canais contíguos (`[sa, av, atrium, purkinje, vent_endo, vent_epi, fibroblast, cai, force, ecg]`), preservando custo de memória zero-copy e permitindo inspecionar o potencial de ação do fibroblasto e a corrente de acoplamento em tempo real.

---

## 7. Acoplamento Eletromecânico e Hemodinâmica: Elastância Variável e Windkessel vs. Curvas Paramétricas

* **Decisão Adotada:** Cálculo da pressão ventricular esquerda ($LVP$) via **Elastância Variável no Tempo** de Suga & Sagawa ($P_{LV} = E(t) \cdot (V - V_0)$) modulada pelo transiente real de $[Ca^{2+}]_i$, e da pressão aórtica ($AoP$) via modelo arterial **Windkessel de 3 elementos** com dinâmica de abertura/fechamento valvar baseada em gradientes físicos de pressão.
* **Alternativa Recusada:** Geração de curvas senoidais pré-gravadas ou interpolação spline puramente cosmética.
* **Justificativa Técnica:**
  - Em simuladores puramente visuais, curvas de pressão são geradas como animações decorativas desconectadas da célula. No KokoroSim v2.0, se um fármaco (como Verapamil ou Digoxina) alterar o influxo de cálcio ou a frequência cardíaca, a força ativa e a curva de elastância $E(t)$ mudam organicamente.
  - A abertura da valva aórtica ocorre estritamente quando $P_{LV} > P_{ao}$, e o fechamento abrupto ocorre quando $P_{LV} \le P_{ao}$, gerando de forma determinística a **incisura dicrótica** na curva aórtica sem nenhuma aproximação artificial.

---

## 8. Síntese Acústica em Tempo Real: Web Audio API Nativa vs. Arquivos de Áudio Estáticos

* **Decisão Adotada:** Síntese aditiva/subtrativa puramente matemática em tempo real via **Web Audio API** (`OscillatorNode`, `GainNode`, `BiquadFilterNode`) acoplada aos nós de processamento do navegador.
* **Alternativa Recusada:** Carregamento e reprodução de arquivos de som pré-gravados (`.mp3` ou `.wav`).
* **Justificativa Técnica:**
  - **Latência Zero e Confiabilidade Offline:** Arquivos externos dependem de requisições de rede, sofrem latência de decodificação e podem falhar em ambientes offline. A síntese Web Audio é instanciada diretamente pela memória do WebAssembly com latência inferior a 5 milissegundos.
  - **Sincronia Hemodinâmica Perfeita:** As bulhas B1 e B2 são disparadas no exato nanossegundo em que o integrador numérico detecta a transição de estado da valva mitral e aórtica, enquanto o bip de UTI é sincronizado com o pico da onda R.
  - **Autoplay Compliance:** O estado inicial desativado das duas opções respeita integralmente a política de autoplay de navegadores modernos (Chrome, Firefox, Safari), ativando o contexto de áudio unicamente sob gesto explícito do usuário.

---

## 9. Acessibilidade Universal, Otimização para Hardware de Baixo Consumo/Mobile e Integração Híbrida Rush-Larsen

* **Decisão Adotada:** Implementação do método de integração híbrido **Rush-Larsen (1978)** para todas as variáveis de portão de condutância iônica nos modelos ventriculares (ten Tusscher 2006), de Purkinje (Stewart 2009) e atriais (Courtemanche 1998), aliado à calibração eletroacústica para microtransdutores de smartphones e notebooks.
* **Alternativa Recusada:** Método explícito de Forward Euler puro com passo rígido microscópico ($dt = 0.001\text{ ms}$) ou integradores implícitos de passo adaptativo (Runge-Kutta / CVODE / BDF).
* **Justificativa Técnica e Diretriz Arquitetural:**
  - **Requisito Não-Funcional Inegociável: Acessibilidade e Inclusão Tecnológica:** O KokoroSim foi concebido como ferramenta de ensino e pesquisa para alcançar o maior número de estudantes, médicos e pesquisadores em escala global, especialmente em regiões com restrição orçamentária e países em desenvolvimento. O sistema **não pode pressupor computadores potentes com GPUs dedicadas ou CPUs de alto desempenho**. Ele deve obrigatoriamente rodar com fluidez a **60 FPS estáveis** e consumo de CPU mínimo (**< 5-10% de uso de CPU**) em **smartphones de entrada (Android/iOS)**, **tablets**, **Chromebooks** e notebooks antigos com baixo TDP (Intel Celeron, Atom, processadores ARM eficientes).
  - **O Gargalo do Forward Euler e a Rigidez Numérica (*Stiffness*):** No esquema anterior com Forward Euler a $dt = 0.001\text{ ms}$, cada quadro de 16 ms demandava 16.000 passos por modelo celular. Com 8 modelos simultâneos acoplados (SA, AV, Átrio, Purkinje, 3 camadas ventriculares e Fibroblasto), o navegador precisava computar **128.000 avaliações completas de EDOs por quadro (8 milhões por segundo)**. Em computadores e dispositivos móveis modestos, isso saturava a CPU em 100%, gerando lentidão de até 10x em relação ao tempo real.
  - Ao tentar aumentar o passo de integração para $dt = 0.01\text{ ms}$ no Forward Euler clássico, o sistema explodia numericamente para `NaN`. A causa física é a **rigidez numérica** dos canais rápidos de sódio: a constante de tempo do portão $m$ durante a Fase 0 atinge $\tau_m \approx 0.0008\text{ ms}$. No Forward Euler, qualquer passo $dt > 2\tau$ viola a estabilidade assintótica local ($|1 - dt/\tau| > 1$), amplificando o erro exponencialmente até o colapso.
  - **A Solução Analítica Exata de Rush-Larsen:** Todas as variáveis de abertura e inativação de canais iônicos seguem a forma canônica de relaxamento linear de 1ª ordem:
    $$\frac{dx}{dt} = \frac{x_\infty(V) - x}{\tau_x(V)}$$
    Sob a premissa de potencial $V$ constante no intervalo infinitesimal $[t, t + \Delta t]$, a EDO admite solução analítica exata incondicionalmente estável para qualquer tamanho de passo $\Delta t > 0$:
    $$x(t + \Delta t) = x_\infty + (x(t) - x_\infty) \cdot e^{-\Delta t / \tau_x}$$
  - **Ganhos Obtidos:**
    1. O passo de integração foi multiplicado por 10 ($dt = 0.01\text{ ms} = 10\,\mu\text{s}$), reduzindo o orçamento de cálculo de 16.000 para **1.600 passos por frame a 60 FPS**.
    2. Em benchmark nativo de release, **3.0 segundos de biologia cardíaca completa executam em apenas 1.38 segundos** (~2.2x mais rápido que o tempo real).
    3. No navegador, o uso médio de CPU despenca para **< 5-10%**, permitindo sessões prolongadas sem aquecimento térmico nem estrangulamento de bateria (*thermal throttling*) em dispositivos portáteis.
  - **Calibração Eletroacústica para Transdutores de Baixo Diâmetro:** Alto-falantes de celulares e laptops sofrem atenuação acústica acentuada abaixo de 120-150 Hz. Para evitar que as bulhas ficassem inaudíveis (como ocorria com a B1 em 65 Hz puro), foram introduzidos *pitch sweeps* descendentes (B1 de 140 para 85 Hz; B2 de 240 para 160 Hz) e filtros ressonantes ajustados com envelope percussivo, garantindo ausculta nítida e percussiva em qualquer dispositivo sem fone de ouvido.

---

## 10. Modos de Exibição do Osciloscópio, Sincronização Biofísica de Varredura e Onda Fantasma em Memória (Zero-Cost)

* **Decisão Adotada:** Implementação de 5 modos de visualização temporal comutáveis no osciloscópio (`Rolling`, `Sweep`, `Paged`, `TriggeredAuto`, `TriggeredSingle`), sincronização biofísica da varredura temporal pela **Fase 0 do Nó Sinoatrial** (marcapasso primário), armazenamento em memória da **Onda Fantasma (Snapshot Buffer)** e projeção vertical simultânea dos eventos acústicos em todos os canais.
* **Alternativa Recusada:** Execução de uma segunda instância paralela de simulação para gerar a curva fantasma e varredura temporal cega (sem alinhamento de fase cardíaca).
* **Justificativa Técnica e Decisões de Design:**
  - **Os 5 Modos de Visualização:**
    1. *Fita Deslizante (Rolling Strip-Chart — Padrão):* Buffer FIFO contínuo onde os pontos mais antigos deslizam suavemente para a esquerda. Ideal para monitorização hemodinâmica e telemetria contínua.
    2. *Varredura Contínua (Continuous Sweep / Monitor de UTI):* A caneta avança da esquerda para a direita com uma barra preta apagadora de 20 amostras à frente, mantendo o traço do ciclo anterior visível à direita até ser sobrescrito (estética clássica dos monitores hospitalares de leito).
    3. *Paginação Sincronizada (Triggered Paged):* Cada página se inicia estritamente no início da **Fase 0 do Nó SA** ($X = 0$). O traçado avança até preencher a tela (~1.5 a 2 ciclos), congela a página estática para leitura minuciosa, e apenas quando o próximo marco do Nó SA dispara, a tela inteira é renovada.
    4. *Gatilho Automático (Auto-Trigger):* A cada novo batimento (Nó SA), a caneta reseta para a margem esquerda com barra apagadora, sobrepondo os batimentos no mesmo eixo temporal (excelente para detectar alternância de onda T, extrassístoles e variações ciclo a ciclo).
    5. *Gatilho Único (Single-Shot / Congelado):* Captura exatamente 1 ciclo cardíaco completo e congela indefinidamente, aguardando o clique do usuário no botão `⚡ DISPARAR / ARMAR PRÓXIMO CICLO`.
  - **A Escolha da Fase 0 do Nó Sinoatrial como Marco de Sincronização:** O Nó SA é o ponto zero anatômico e temporal da despolarização cardíaca. Ao ancorar a margem esquerda ($X = 0$) na despolarização sinusal, a tela exibe a sequência cronológica real da anatomia cardíaca da esquerda para a direita:
    $$\text{Margem Esquerda } (X=0) \to \text{Nó SA (Fase 0)} \to \text{Onda P (Átrio)} \to \text{Intervalo PR (Nó AV)} \to \text{Complexo QRS (Ventrículo)} \to \text{Onda T (Repolarização)}$$
    Se a sincronização ocorresse na onda R ventricular, a onda P do mesmo batimento ficaria amputada à esquerda e a onda P visível pertenceria ao ciclo posterior.
  - **Onda Fantasma Bio-Sincronizada (Ciclo $RR$ Fechado na Fase 0 do Nó SA):** A captura da referência basal não é uma foto estática cega. Ao acionar o botão, o sistema entra em espera armada (`Armed`), aguarda o disparo da **Fase 0 do Nó Sinoatrial** ($t = 0$), grava as amostras até o disparo da Fase 0 do ciclo subsequente (fechando rigorosamente 1 ciclo $RR$ completo) e armazena o potencial diastólico basal de repouso ($V_{rest}$).
    - *Preservação da Variabilidade Natural:* Em repouso sem fármacos, a onda fantasma e a ativa compartilham a mesma morfologia com a leve e sutil variabilidade estocástica ciclo a ciclo da simulação biológica.
    - *Bradicardia (Batimentos Espaçados):* A onda fantasma completa seu ciclo e permanece repousando na linha de base ($V_{rest}$) até o ciclo atual atingir o próximo disparo do Nó SA.
    - *Taquicardia (Batimentos Encurtados):* O novo disparo do Nó SA interrompe antecipadamente a projeção anterior e re-dispara a onda fantasma imediatamente no ponto zero, mantendo o alinhamento estrito.
    - *Modo Fita Deslizante (`Rolling`):* A injeção da onda fantasma é re-sincronizada batimento a batimento a cada disparo do Nó SA, eliminando completamente defasagens de fase ou frações de ciclo.
  - **Acessibilidade Sensorial e Eliminação do Efeito "Formiguinhas" (*Marching Ants*):** O pontilhado dinâmico que gerava efeito estroboscópico de esteira rolante foi substituído por uma **Linha Sólida Contínua Esmaecida (*Dimmed Ghostline*)** de espessura fina (1.2px) e cor translúcida suave (`rgba(..., 0.35)`). Estável pixel a pixel, sem trepidação nem cintilação, garantindo conforto visual prolongado para pessoas neurodivergentes (TEA, TDAH e hipersensibilidade sensorial).
  - **Reset Limpo e Eliminação de Rastros na Comutação de Modos:** Na função `set_mode`, todo o buffer é inicializado com valores sentinela `f64::NAN`. Durante a varredura e nos modos de gatilho (`TriggeredAuto`), os pontos à frente da barra apagadora também são preenchidos com `f64::NAN`. O renderizador do Canvas 2D ignora valores inválidos quebrando o caminho vetorial (`first_point = true`), o que impede completamente o surgimento de diagonais espúrias ou rastros remanescentes de ciclos anteriores.
  - **Controle de Pausa / Congelamento da Simulação (`⏸ CONGELAR` / `▶ CONTINUAR`):** Botão localizado no cabeçalho superior da barra de comandos (ao lado de Reset e Sobre). Quando acionado, suspende instantaneamente a integração numérica de equações diferenciais ordinárias (ODE), congela as métricas do HUD e silencia o sintetizador de áudio, mantendo os laços de redesenho ativos no Canvas 2D para que o usuário possa alternar camadas celulares, inspecionar traçados estáticos e capturar telas com máxima fidelidade.
  - **Marcadores Verticais Acústicos Multicanal:** Quando as opções de som estão ativadas, linhas verticais sutis cortam **todos os 4 osciloscópios simultaneamente** (Potenciais de Ação, ECG, Cálcio e Hemodinâmica):
    - *Amarelo Dourado Tracejado:* instante do pico da **Onda R (Bip da UTI)**.
    - *Verde Esmeralda Sólido:* instante do fechamento da valva mitral / início da sístole isovolumétrica **(Bulha B1 - "Tum")**.
    - *Coral Sólido:* instante do fechamento da valva aórtica / incisura dicrótica **(Bulha B2 - "Tá")**.
    Isso estabelece uma correlação áudio-visual em tempo real: o usuário vê a linha passar na tela no mesmo milissegundo em que escuta a bulha ou o bip.

---

## 11. Governança do Repositório, Hooks Git e Versionamento Semântico com `cccp.sh`

* **Decisão Adotada:** Integração do [**`cccp.sh` (Conventional Commits Compliance Program)**](https://github.com/lumenpink/cccp.sh) como guardião nativo de fluxo de trabalho Git no repositório.
* **Mecanismos e Hooks Ativos:**
  - **`commit-msg`:** Valida rigorosamente todas as mensagens de commit conforme o padrão Conventional Commits em Português-BR (`feat`, `docs`, `fix`, `chore`, `refactor`, `style`, `ci`, etc.).
  - **`post-commit`:** Sincroniza automaticamente a versão de desenvolvimento SemVer no arquivo `VERSION` a cada commit (`X.Y.Z-dev.N+data.hash`).
  - **`reference-transaction` & `pre-push` (Gosplan Quality Bureau):** Interceptam a criação de tags locais (`git tag`) e o envio de branches/tags para o repositório remoto (`git push`). O sistema bloqueia sumariamente tags apontando para commits que não possuam o arquivo `VERSION` devidamente registrado ou versões com sufixo de desenvolvimento (`-dev`), forçando o uso do comando `cccp tag` para geração oficial e sincronizada de releases com o `CHANGELOG.md`.
* **Diretriz para Manutenção de Baixo Nível e Histórico:**
  - Em rotinas excepcionais de reescrita histórica profunda (ex.: `git-filter-repo` ou `git filter-branch`), comandos internos como `git fast-import` acionam os hooks de transação do Git. Nesses cenários específicos de expurgo ou retrocompatibilidade pré-`VERSION`, os hooks locais em `.git/hooks` devem ser contornados temporariamente durante o processamento em lote da ferramenta de filtro e prontamente reativados ao término, garantindo que o histórico legado possa ser saneado sem falsos-positivos das regras de release em vigor.
