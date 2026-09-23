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

### Fase 10: Osciloscópio Modular, Calibração Eletrocardiográfica (Padrão Livro-Texto) e Grade Isotrópica
* [ ] **Canais Modulares com Dropdown Dupla em Cascata**:
  - Implementação de seletor duplo no cabeçalho de cada um dos 4 osciloscópios (`CH-01` a `CH-04`), tornando todos os canais 100% dinâmicos e intercambiáveis.
  - **1ª Dropdown (Domínio Fisiológico)**: `Potenciais de Ação`, `Eletrocardiograma (ECG)`, `Cinética Iônica e Transportadores`, `Hemodinâmica (Wiggers & Alça PxV)`, `Espirometria e Mecânica Respiratória`.
  - **2ª Dropdown (Visão ou Derivação Contextual)**: Ajusta-se dinamicamente conforme o domínio selecionado na primeira.
* [ ] **Recalibração do ECG (Morfologia Clássica de Livro-Texto)**:
  - Resolução da distorção do traçado da v2 (onde a diferença direta $V_{endo} - V_{epi}$ produzia ruído e ausência de morfologia escarpada realista).
  - Reintrodução da síntese biofísica espaço-temporal calibrada (como no modelo consagrado da v1):
    - **Onda P**: Deflexão atrial suave, arredondada e com duração fisiológica (~80 ms).
    - **Complexo QRS**: Deflexão ventricular afiada, escarpada e bifásica/trifásica (ondas Q, R e S nítidas com largura < 120 ms modulada por bloqueadores de sódio).
    - **Onda T ("Morrinho")**: Repolarização suave, positiva e assimétrica modulada por potássio e bloqueadores de $I_K$.
* [ ] **Grade Biomédica Proporcional do ECG (Isotrópica 40 ms × 0.1 mV)**:
  - Mapeamento matemático em pixels preservando a proporção 1:1 rigorosa do papel milimetrado de ECG:
    - **Quadradinhos pequenos**: $40\text{ ms}$ (horizontal) $\times$ $0.1\text{ mV}$ (vertical).
    - **Quadradões grandes**: $200\text{ ms}$ ($5\text{ quadradinhos}$) $\times$ $0.5\text{ mV}$ ($5\text{ quadradinhos}$).
    - Garantia de que 1 quadradinho no Canvas seja visualmente um quadrado perfeito, permitindo diagnóstico imediato por contagem visual de quadradinhos (PR em 3–5 quadradinhos, QRS < 3 quadradinhos, R com 10 quadradinhos de altura).
    - Toggle de ganho rápido: padrão **$1N$ ($10\text{ mm/mV}$)** e **$2N$ ($20\text{ mm/mV}$)**.
* [ ] **ECG de 12 Derivações Clínicas**:
  - Projeção tridimensional do dipolo cardíaco nas derivações periféricas de Einthoven/Goldberger (**DI, DII, DIII, aVR, aVL, aVF**) e precordiais horizontais (**V1 a V6**).
  - Modo Vetorcardiograma 2D ($DI \times aVF$) traçando o loop elétrico frontal instantâneo e o cálculo do Eixo Elétrico Cardíaco em graus.
* [ ] **Escalas Metrológicas Dinâmicas em Todos os Gráficos**:
  - Indicação numérica visual contínua dos eixos em cada canal: **Topo (Valor Máximo $Y_{max}$)**, **Centro (Valor Médio / Zero $Y_{mid}$)** e **Base (Valor Mínimo $Y_{min}$)** acompanhados das respectivas unidades de engenharia biomédica ($mV$, $mmHg$, $L$, $L/s$, $\mu M$).

### Fase 11: Transparência Iônica Celular e Reestruturação das Sanfonas
* [ ] **Origem e Identificação da Célula na Cinética Iônica**:
  - Discriminação explícita de **qual célula** está sendo inspecionada dentre os 7 subtipos simulados (Nó SA, Músculo Atrial, Nó AV, Fibras de Purkinje, Endocárdio, Célula M, Epicárdio ou Fibroblasto).
  - Seletor de célula e correntes na 2ª dropdown do canal iônico (ex: inspecionar o influxo de cálcio no *Nó SA* via canais T e L vs. *Epicárdio Ventricular* via RyR e SERCA).
* [ ] **Desacoplamento e Reestruturação das Sanfonas do Menu Esquerdo**:
  - **Sanfona 0**: Dedicada exclusivamente aos modos de osciloscópio (rolling, sweep, paged, single-shot) e às ferramentas de captura da Onda Fantasma.
  - **Nova Sanfona (Células & Camadas Ativas)**: Controle independente de quais subtipos celulares estão ativos no CH-01 de potenciais de ação.
  - **Nova Sanfona (Canais Iônicos & Cinética)**: Checkboxes coloridos para habilitar correntes individuais ($I_{Na}$, $I_{Ca,L}$, $I_f$, $I_{Kr}$, $I_{Ks}$, $I_{K1}$, $[Ca^{2+}]_{SR}$).
  - **Nova Sanfona (Mecânica Respiratória & Espirometria)**: Controles ventilatórios e disparos de manobras pulmonares.

### Fase 12: Mecânica Respiratória, Espirometria e Acoplamento Cardiorrespiratório (Pulmo Lab) ✅ CONCLUÍDO
* [x] **Modelagem Matemática de Volumes e Capacidades Pulmonares Estáticos e Dinâmicos**:
  - **Volume Corrente ($V_T$ ~500 mL)**: Volume ventilado na respiração tranquila de repouso.
  - **Volume de Reserva Inspiratório ($VRI$ ~3.000 mL)**: Volume máximo adicional inspirado pós-eupneia.
  - **Volume de Reserva Expiratório ($VRE$ ~1.100 mL)**: Volume máximo expelido pós-expiração normal.
  - **Volume Residual ($VR$ ~1.200 mL — *o ar que fica*)**: Volume de gás que permanece obrigatoriamente retido nos alvéolos mesmo após expiração forçada máxima, impedindo o colapso pulmonar.
  - Cálculo contínuo das capacidades:
    - **Capacidade Vital Forçada ($CVF = V_T + VRI + VRE$ ~4.600 mL)**
    - **Capacidade Residual Funcional ($CRF = VRE + VR$ ~2.300 mL — repouso elástico torácico)**
    - **Capacidade Pulmonar Total ($CPT = CVF + VR$ ~5.800 mL)**
* [x] **Manobra de Espirometria Forçada e Gráficos Respiratórios**:
  - Botão de disparo animado `[ 💨 INICIAR MANOBRA DE ESPIROMETRIA ]` no menu lateral do Pulmo Lab.
  - **Espirograma Dinâmico ($V \times t$)**: Curva contínua de volume no tempo com mensuração automática de **$VEF_1$** (Volume Expiratório Forçado no 1º segundo) e do **Índice de Tiffeneau** ($VEF_1 / CVF$, ~82.6% fisiológico).
  - **Fluxo Aéreo Instantâneo ($\dot{V} \times t$)**: Traçado com identificação do Pico de Fluxo Expiratório (PEF) e padrão diagnóstico morfológico de distúrbios obstrutivos (asma/DPOC com concavidade expiratória) e restritivos.
  - **Laudo Funcional Automatizado**: Diagnóstico em tempo real diferenciando padrões normais, obstrutivos e suspeita de restritivos.
* [x] **Acoplamento Cardiorrespiratório e Pressão Pleural**:
  - Dinâmica da **Pressão Intrapleural ($P_{pl}$)** oscilando entre $-8\text{ cmH}_2\text{O}$ e $-5\text{ cmH}_2\text{O}$ em repouso e até $+30\text{ cmH}_2\text{O}$ na manobra forçada.
  - **Arritmia Sinusal Respiratória (RSA)**: Modulação autonômica do Nó Sinoatrial (Severi) pela respiração (taquicardia transitória na inspiração por inibição vagal; bradicardia na expiração por eferência parassimpática).
  - **Recalibração Fisiológica de Longo Prazo da RSA**: Atenuação do ganho vagal excessivo e balanceamento simpático-vagal ($\Delta FC \approx \pm 3\text{--}5\text{ BPM}$), eliminando a bradicardia progressiva e o colapso pressórico aórtico em repouso prolongado.
  - **Harness de Validação e Baseline de Longo Prazo (300s)**: Teste automatizado sob demanda (`long_baseline_300s.rs`) certificando 5 minutos ininterruptos de estabilidade hemodinâmica eutrófica (PA 122/71 mmHg, 412 batimentos, FE 51–58%, sem colapso por bradicardia).
  - **Controle Didático no Cardio Lab**: Inclusão de toggle independente de RSA na sanfona de monitorização do Cardio Lab para comparação direta imediata.
  - **Monitor Cardíaco no Pulmo Lab**: Telemetria contínua de Frequência Cardíaca ($FC$) e Débito Cardíaco ($DC$) no HUD do Pulmo Lab para feedback cardiovascular durante manobras ventilatórias.
* [x] **Arquitetura de Abas Especializadas (`.lab-tabs`)**:
  - Divisão da interface em dois grandes espaços de trabalho (`💓 CARDIO LAB` e `🫁 PULMO LAB`) sem perda de continuidade do motor biofísico em segundo plano a 60 FPS.

### Fase 13: Hemodinâmica Avançada, Ciclo de Wiggers e Alça Pressão-Volume ($P \times V$) ✅ CONCLUÍDO
* [x] **Diagrama de Wiggers Completo e Alça Pressão-Volume ($P \times V$)**:
  - [x] Cálculo contínuo de volumes ventriculares: Volume Diastólico Final (VDF), Volume Sistólico Final (VSF), Volume Sistólico e Fração de Ejeção ($FE = VS / VDF$).
  - [x] Novo traçado bidimensional em plano de fase exibindo a alça $P \times V$ dinâmica em tempo real (resposta direta à contratilidade via ESPVR, complacência via EDPVR, pré-carga e pós-carga).
  - [x] Refinamento da rotulagem clínica do osciloscópio 2D para `CH-PV [ 圧力-容積ループ // ALÇA PRESSÃO-VOLUME DO VE ]` com discriminação visual de eixos ($mmHg \times mL$) e fases do ciclo.
* [x] **Mecânica e Pressão Atrial**: Ondas *a* (contração atrial ativa pós-onda P), *c* (abaulamento isovolumétrico da mitral) e *v* com descenso *y* (enchimento sistólico passivo e esvaziamento diastólico).
* [x] **Patologias Valvares e Desafios Hemodinâmicos**:
  - Modelagem de Estenose Aórtica (gradiente transvalvar sistólico patológico > 40 mmHg e sobrecarga pressórica com efeito Anrep).
  - Insuficiência Aórtica (regurgitação diastólica com colapso de pressão aórtica diastólica < 50 mmHg e sobrecarga volumétrica).
  - Estenose Mitral (hipertensão atrial esquerda > 18 mmHg por retardo de esvaziamento diastólico).
  - Insuficiência Mitral (regurgitação sistólica ventrículo-atrial gerando onda v patológica gigante > 25 mmHg).
* [x] **Controles Clínicos e Telemetria Sistólica na Interface**:
  - Nova sanfona no Cardio Lab (`8. 弁膜症 // Valvopatias & Dinâmica Valvar`) com sliders contínuos de Estenose Aórtica, Insuficiência Aórtica, Estenose Mitral e Insuficiência Mitral.
  - Telemetria no HUD com Fração de Ejeção ($FE$), Volume Sistólico ($VS$) e Débito Cardíaco ($DC$) atualizados ciclo a ciclo.
* [x] **Validação e Suíte de Testes E2E (Playwright)**:
  - 10 cenários automatizados cobrindo Wiggers, alça $P \times V$, estabilidade hemodinâmica sob valvopatias e persistência em segundo plano.

### Fase 13.1: Acoplamento Sistêmico de Guyton, Retorno Venoso e Resposta à Ortostase (MED-7002 / Issue #2) ✅ CONCLUÍDO
* [x] **Modelo de Retorno Venoso de Arthur Guyton**:
  - Equação fundamental de acoplamento sistêmico: $RV = (PMES - PVC) / R_{rv}$.
  - Dinâmica contínua de conservação de massa para a Pressão Venosa Central: $\frac{d(PVC)}{dt} = \frac{RV - DC}{C_{venous}}$, estabelecendo convergência estável no ponto de equilíbrio $RV = DC$.
  - Acoplamento biofísico de pré-carga atrial esquerda ($p_{la\_base}$) derivado de $PVC$ e volume circulante efetivo.
* [x] **Desafio Postural e Resposta à Ortostase (Tilt)**:
  - Represamento venoso gravitacional esplâncnico e de membros inferiores (~$32\%$ de redução de $PMES$ e aumento de resistência ao retorno venoso $R_{rv}$).
  - Gradiente de coluna hidrostática carotídea ($\Delta P \approx 18\text{ mmHg}$) descarregando imediatamente os barorreceptores do seio carotídeo antes da queda aórtica.
  - Resposta compensatória fisiológica de malha fechada via barorreflexo (taquicardia reflexa, vasoconstrição venosa e arteriolar via $\alpha_1$, manutenção de $PAM$).
* [x] **Controles Clínicos e Telemetria no Simulador Web**:
  - Nova sanfona no Cardio Lab (`9. 循環平衡 // Retorno Venoso & Guyton`) com checkbox `🚶 Ortostase (Em Pé / Tilt)` e slider contínuo de volemia `Volemia (PMES)` ($1.0$ a $15.0\text{ mmHg}$).
  - Novos pods de telemetria no HUD do Cardio Lab: Pressão Venosa Central (`PVC`) e Pressão Média de Enchimento Sistêmico (`PMES`).
* [x] **Harness de Testes Automatizados**:
  - 4 testes de integração em Rust (`guyton_and_orthostasis.rs`): equilíbrio normovolêmico, desafio ortostático com taquicardia reflexa, choque hipovolêmico/hemorragia e expansão volêmica.
  - Suíte Playwright E2E expandida para 11 testes, validando controles de sanfona, telemetria no HUD e persistência.


### Fase 14: Expansão Espacial (Monodomínio 2D / 3D)
* [ ] Substituição do modelo 0D acoplado por malha bidimensional de diferenças finitas (matriz de 100x100 a 200x200 miócitos).
* [ ] Difusão tecidual contínua com tensor de condutividade anisotrópica.
* [ ] Visualização topográfica de frentes de onda, espirais arritmogênicas (*rotor waves*), fibrilação ventricular e despolarizações fracionadas.

### Fase 15: Modelação Farmacológica Avançada e Novos Protocolos Clínicos
* [ ] Implementação de novos fármacos e toxinas (ex.: digitalina, bloqueadores específicos de $I_{Kr}$, agentes parassimpaticomiméticos).
* [ ] Painel interativo de casos clínicos e cenários patológicos pré-configurados para auxílio diagnóstico e ensino médico.

### Fase 16: Arquitetura de Interface Mobile-First e Responsividade Nativa
* [ ] **Layout Responsivo Adaptativo para Smartphones (Modo Retrato / Mobile UI)**:
  - Divisão vertical da tela (*Split-Screen Mobile*): osciloscópio multicanal dinâmico no topo (40–45% da altura de visualização).
  - Gaveta de controles inferior colapsável (*Bottom Sheet Drawer*) operável por gestos de deslize (*swipe up/down*).
  - Sistema de abas horizontais de acesso rápido por domínio de controle.
  - Otimização ergonômica de sliders, seletores e botões para zonas de alcance do polegar (*hit targets* >= 48px).

---

## 🏛️ KokoroSim v1.0 (Protótipo Legado em TypeScript/Vite) ✅ CONCLUÍDO
* Marco inicial com simulação zero-dimensional em Web Worker JavaScript, interface estática e modelos celulares iniciais (disponível online em [kokorosim.github.io/v1](https://kokorosim.github.io/v1/)).
