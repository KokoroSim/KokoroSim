# Roadmap de Desenvolvimento

O desenvolvimento do **KokoroSim** está organizado em grandes marcos arquiteturais e eletrofisiológicos. 

---

## 🚀 KokoroSim v2.0 (Marco Eletrofisiológico e Hemodinâmico em Rust + WASM) ✅ CONCLUÍDO

### Fase 1: Transição Arquitetural (Rust + WASM + Dioxus) ✅ CONCLUÍDO
* [x] Migração de todo o motor matemático de equações diferenciais para **Rust nativo**.
* [x] Compilação do motor para **WebAssembly (WASM)** com otimizações de alto rendimento.
* [x] Reescrever a interface de usuário reativa utilizando **Dioxus 0.6** e Signals.
* [x] Pipeline de execução em lote (`run_batch`) com amostragem direta em memória compartilhada WASM.
* [x] Ambiente de desenvolvimento local (`run_local.sh`) com monitoramento de arquivos, livereload, debounce e compilação concorrente assíncrona.

### Fase 2: Condução Fisiológica por Corrente e Fibras de Purkinje ✅ CONCLUÍDO
* [x] Eliminação da assincronia e marcapasso parasita ventricular (remoção do timer rígido de 1000 ms).
* [x] Substituição de grampeamentos de voltagem por **injeção transitória de corrente despolarizante** ($I_{stim}$).
* [x] Transposição do modelo biofísico de **Stewart et al. (2009)** para a célula de Purkinje humana (20 variáveis de estado, integração em ms, canais $I_f$, $I_{to}$, $I_{sus}$).
* [x] Acoplamento da cadeia completa: Nó SA $\to$ Átrio $\to$ Nó AV $\to$ Purkinje / His $\to$ Ventrículo.
* [x] Automatismo terciário de escape idioventricular (~30 BPM) em caso de bloqueio atrioventricular.

### Fase 3: Heterogeneidade Transmural Ventricular e ECG Dipolar ✅ CONCLUÍDO
* [x] Implementação dos 3 subtipos ventriculares de **ten Tusscher & Panfilov (2006)**:
  - **Endocárdio (Subendocárdio):** $G_{to}$ baixo ($0.073$), platô arredondado, ativado primeiro via Purkinje.
  - **Célula M (Mid-miocárdio):** $G_{Ks}$ reduzido ($0.098$), platô estendido, substrato para dispersão do QT.
  - **Epicárdio (Subepicárdico):** $G_{to}$ robusto ($0.294$), entalhe proeminente na Fase 1, APD curto, repolariza primeiro.
* [x] Condução transmural intramiocárdica com velocidade finita (Endo $\to$ M-cell em ~6ms $\to$ Epicárdio em ~12ms).
* [x] Gênese biofísica do **Eletrocardiograma (ECG)** a partir do gradiente dipolar de campo distante:
  $$\text{ECG}(t) = 0.15 \cdot V_{atrio} + 0.55 \cdot (V_{endo} - V_{epi}) + 0.25 \cdot (V_M - V_{epi})$$
  gerando organicamente a **Onda P**, o **Complexo QRS escarpado** e a **Onda T POSITIVA e fisiológica**.

### Fase 4: Dromotropismo Dinâmico e Condução Decremental no Nó AV ✅ CONCLUÍDO
* [x] Eliminação de atrasos nodais estáticos.
* [x] Implementação da **condução decremental frequência-dependente** (aumento do retardo sob taquicardia devido à cinética de recuperação do $I_{Ca,L}$).
* [x] Modulação autonômica do atraso nodal: simpático encurta o PR (dromotropismo +); vagal alarga o PR (dromotropismo -).
* [x] Sensibilidade a bloqueadores de cálcio (Verapamil) e isquemia, induzindo Bloqueios AV de 1º, 2º e 3º grau com escape terciário de Purkinje.

### Fase 5: Fibroblastos Cardíacos e Acoplamento Eletrotônico (MacCannell et al., 2007) ✅ CONCLUÍDO
* [x] Implementação do modelo biofísico do **Fibroblasto Cardíaco Humano** (MacCannell et al., 2007) com $C_f = 6.3\text{ pF}$, canais $I_{Kv}$, $I_{K1}$, $I_b$ e bomba eletrogênica $I_{NaK}$.
* [x] Acoplamento eletrotônico miócito-fibroblasto via junções comunicantes ($I_{gap}$ bidirecional) com condutância $G_{gap} = \text{fibrose} \times 4.0\text{ nS}$.
* [x] Simulação de dreno capacitivo, despolarização parcial diastólica do miócito (inativação de $I_{Na}$ em repouso) e lentificação da condução intramiocárdica proporcional à fibrose.
* [x] Visualização no osciloscópio de Potencial de Ação com canal lilás (`#a29bfe`) e slider de controle de Fibrose Miocárdica (0% a 100%).

### Fase 6: Acoplamento Eletromecânico, Hemodinâmica e Áudio (Wiggers) ✅ CONCLUÍDO
* [x] Acoplamento Excitação-Contração via cinemática cooperativa de ligação de Cálcio na Troponina C.
* [x] Modelo de **Elastância Ventricular Variável no Tempo ($E(t)$)** de Suga & Sagawa para calcular a Pressão Ventricular Esquerda ($LVP$).
* [x] Acoplamento hidráulico com modelo arterial **Windkessel de 3 elementos** para calcular a Pressão Aórtica ($AoP$) com incisura dicrótica.
* [x] Dinâmica valvar das cúspides mitral e aórtica acionadas por gradientes pressóricos instantâneos.
* [x] **Síntese de Áudio Biofísico via Web Audio API**:
  - `🔊 Bip de Monitor (UTI)` acoplado à detecção de onda R (880 Hz).
  - `🩺 Ausculta de Bulhas (B1 / B2)` acoplada ao fechamento das valvas mitral e aórtica.
* [x] Compressão horizontal do traçado (janela de 2.5s com capacidade de 500 amostras, comportando > 3 ciclos completos).

### Fase 7: Otimização Numérica Híbrida (Rush-Larsen) e Acessibilidade Universal (Mobile/Low-End) ✅ CONCLUÍDO
* [x] Diagnóstico e resolução do gargalo de rigidez numérica (*stiffness*) dos canais rápidos de sódio ($m$).
* [x] Integração analítica exata **Rush-Larsen** para todos os portões de condutância iônica em Ventrículo (ten Tusscher 2006), Purkinje (Stewart 2009) e Átrio (Courtemanche 1998).
* [x] Elevação do passo de integração para $dt = 0.01\text{ ms}$ com 100% de estabilidade e redução de 10x no orçamento de cálculo (1.600 passos por quadro de 16 ms a 60 FPS).
* [x] Execução em **1x tempo real garantida** com consumo de CPU mínimo (**< 5-10%**) em smartphones, tablets, Chromebooks e computadores de baixo custo.
* [x] Recalibração acústica das bulhas B1 e B2 com *pitch sweeps* (140 $\to$ 85 Hz e 240 $\to$ 160 Hz) e ganho otimizado para microtransdutores de celulares e laptops.
* [x] Formalização do requisito não-funcional de acessibilidade e computação inclusiva no `ARCHITECTURE.md`.

### Fase 8: Modos de Exibição do Osciloscópio, Gatilho Biofísico e Onda Fantasma ✅ CONCLUÍDO
* [x] Implementação dos **5 modos de exibição temporal**: Fita Deslizante (Padrão), Varredura Contínua (Monitor UTI com barra apagadora), Paginação Sincronizada (Triggered Paged), Gatilho Automático (Auto-Trigger por batimento) e Gatilho Único (Single-Shot congelado).
* [x] Detecção no motor e sincronização biofísica da varredura temporal pela **Fase 0 do Nó Sinoatrial** (marcapasso primário), preservando a sequência anatômica estrita (SA $\to$ P $\to$ PR $\to$ QRS $\to$ T).
* [x] Botão `⚡ DISPARAR / ARMAR PRÓXIMO CICLO` para captura e congelamento de 1 batimento no modo Single-Shot.
* [x] **Onda Fantasma Bio-Sincronizada (Ciclo Fechado $RR$ na Fase 0 do Nó SA)**: captura precisa de 1 batimento basal completo, suporte fisiológico a espaçamento (bradicardia com repouso em $V_{rest}$) e encurtamento (taquicardia com reinício imediato), re-ancoragem em tempo real na fita deslizante e renderização sólida esmaecida (1.2px, `rgba(..., 0.35)`) anti-fadiga visual para neurodivergentes (zero *marching ants*).
* [x] **Higienização de Memória e Reset Limpo**: inicialização e apagamento de buffers com sentinelas `NaN` nos modos de varredura e gatilho automático, eliminando qualquer rastro residual de ciclos anteriores.
* [x] **Controle de Pausa / Congelamento da Simulação**: botão `[ ⏸ CONGELAR ]` / `[ ▶ CONTINUAR ]` no cabeçalho da barra lateral, permitindo suspender o cálculo biológico e o áudio instantaneamente para inspeção estática.
* [x] **Marcadores Visuais Acústicos Multicanal**: linhas verticais correspondentes desenhadas simultaneamente em todos os 4 canais (amarelo tracejado para Onda R/UTI, verde para B1 e coral para B2).

---

## 🔬 Próximos Passos (KokoroSim v2.1+)

### Fase 9: Modernização de Infraestrutura e Qualidade de Software (Rust 2024 & Dioxus 0.7)
* [ ] **Migração para Rust Edition 2024**: Adoção das novas convenções de compilação, closures assíncronas otimizadas e lifetimes estritos para WebAssembly.
* [ ] **Atualização para Dioxus 0.7**: Transição para o novo reconciliador de Virtual DOM e reatividade avançada de Signals, reduzindo o overhead de renderização do osciloscópio.
* [x] **Harness de Testes Automatizados E2E e Wasm**:
  - [x] Testes unitários do motor numérico em Rust nativo e Wasm (validação de invariantes matemáticas, estabilidade de $V_m$, ausência de `NaN`, resiliência e não-congelamento sob espectro extremo de parâmetros).
  - [x] Suíte de testes de interface end-to-end com Playwright em modo headless e worker único: validação de inicialização WASM, 4 canais de osciloscópio, congelamento/pausa, captura da Onda Fantasma bio-sincronizada, disparo Single-Shot, reset de parâmetros, 3 camadas padrão ativas e legendas humanizadas.
  - [x] Testes de regressão biofísica e roteiro prático universitário: verificação programática dos 4 experimentos clínicos (Nernst e hipercalemia, dromotropismo e BAVT com Verapamil, modulação simpático/vagal com Wiggers, e acoplamento heterocelular com fibroblastos).

### Fase 10: Hemodinâmica Avançada, Ciclo de Wiggers e Alça Pressão-Volume
* [ ] **Diagrama de Wiggers Completo e Alça Pressão-Volume ($P \times V$)**:
  - Cálculo contínuo de volumes ventriculares: Volume Diastólico Final (VDF), Volume Sistólico Final (VSF), Volume Sistólico e Fração de Ejeção ($FE = VS / VDF$).
  - Novo traçado ou painel gráfico bidimensional exibindo a alça $P \times V$ dinâmica em tempo real (resposta direta à contratilidade e pré/pós-carga).
* [ ] **Mecânica e Pressão Atrial**: Ondas *a* (contração atrial ativa), *c* (protrusão valvar) e *v* (enchimento passivo ventricular).
* [ ] **Patologias Valvares e Desafios Hemodinâmicos**:
  - Modelagem de Estenose Aórtica (gradiente transvalvar patológico e sobrecarga pressórica).
  - Insuficiência Aórtica (regurgitação diastólica, colapso de pressão de pulso e ausência de incisura dicrótica).
  - Estenose e Insuficiência Mitral com repercussão volumétrica retrógrada.

### Fase 11: Acoplamento Cardiorrespiratório e Mecânica Torácica
* [ ] **Dinâmica da Pressão Intrapleural ($P_{pl}$)**: Oscilações cíclicas respiratórias ($-5\text{ cmH}_2\text{O}$ a $-8\text{ cmH}_2\text{O}$) acopladas ao retorno venoso e à pré-carga das câmaras direitas.
* [ ] **Arritmia Sinusal Respiratória (RSA)**: Modulação autonômica cronotrópica do Nó Sinoatrial vinculada ao ciclo respiratório (aumento fisiológico de FC na inspiração e desaceleração vagal na expiração).
* [ ] **Desdobramento Fisiológico da Segunda Bulha ($B_2$)**: Separação acústica temporal entre o fechamento da valva aórtica ($A_2$) e pulmonar ($P_2$) durante a inspiração profunda.

### Fase 12: Expansão Espacial (Monodomínio 2D / 3D)
* [ ] Substituição do modelo 0D acoplado por malha bidimensional de diferenças finitas (matriz de 100x100 a 200x200 miócitos).
* [ ] Difusão tecidual contínua com tensor de condutividade anisotrópica.
* [ ] Visualização topográfica de frentes de onda, espirais arritmogênicas (*rotor waves*), fibrilação ventricular e despolarizações fracionadas.

### Fase 13: Modelação Farmacológica Avançada e Novos Protocolos Clínicos
* [ ] Implementação de novos fármacos e toxinas (ex.: digitalina, bloqueadores específicos de $I_{Kr}$, agentes parassimpaticomiméticos).
* [ ] Painel interativo de casos clínicos e cenários patológicos pré-configurados para auxílio diagnóstico e ensino médico.

### Fase 14: Arquitetura de Interface Mobile-First e Responsividade Nativa (Caminho 2)
* [ ] **Layout Responsivo Adaptativo para Smartphones (Modo Retrato / Mobile UI)**:
  - Divisão vertical da tela (*Split-Screen Mobile*): osciloscópio multicanal dinâmico no topo (40–45% da altura de visualização).
  - Gaveta de controles inferior colapsável (*Bottom Sheet Drawer*) operável por gestos de deslize (*swipe up/down*).
  - Sistema de abas horizontais de acesso rápido por domínio de controle: *1. Modos de Tela / 2. Eletrofisiologia Celular / 3. Farmacologia / 4. Hemodinâmica*.
  - Otimização ergonômica de sliders, seletores e botões para zonas de alcance do polegar (*thumb-friendly hit targets* >= 48px).
  - Detecção inteligente e transição fluida entre modo retrato (bottom sheet) e modo paisagem (painel de instrumentação estendido).

---

## 🏛️ KokoroSim v1.0 (Protótipo Legado em TypeScript/Vite) ✅ CONCLUÍDO
* Marco inicial com simulação zero-dimensional em Web Worker JavaScript, interface estática e modelos celulares iniciais (disponível online em [kokorosim.github.io/v1](https://kokorosim.github.io/v1/)).
