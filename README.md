<p align="center">
  <img src="ui/assets/logo.png" width="220" alt="KokoroSim">
</p>

<h1 align="center" style="font-size: 2.8rem; letter-spacing: 2px; margin-top: 4px; margin-bottom: 4px;">
  kokor<span style="color: #ff1754;">心</span><span style="color: #00cec9;">sim</span>
</h1>

<p align="center" style="font-size: 1.25rem; font-weight: 500; color: #a4b0be; margin-top: 0; margin-bottom: 18px;">
  Simulador Eletrofisiológico Cardíaco em Tempo Real
</p>

<p align="center">
  <a href="https://kokorosim.github.io/"><img alt="Deploy to GitHub Pages" src="https://github.com/KokoroSim/KokoroSim/actions/workflows/deploy.yml/badge.svg"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0"><img alt="Licença: GPL v3" src="https://img.shields.io/badge/License-GPLv3-blue.svg"></a>
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-2021_Edition-orange.svg?logo=rust"></a>
  <a href="https://webassembly.org/"><img alt="WebAssembly" src="https://img.shields.io/badge/WebAssembly-WASM-purple.svg?logo=webassembly"></a>
  <a href="https://dioxuslabs.com/"><img alt="Dioxus" src="https://img.shields.io/badge/UI-Dioxus_0.6-00cec9.svg"></a>
  <a href="#-arquitetura-de-software"><img alt="Mobile Optimized" src="https://img.shields.io/badge/Hardware-Mobile_%26_Low--Power-success.svg"></a>
  <a href="#-modos-de-visualização-do-osciloscópio-e-onda-fantasma"><img alt="Acessibilidade" src="https://img.shields.io/badge/Acessibilidade-Neurodivergente--Friendly-brightgreen.svg"></a>
</p>

O **kokor<span style="color: #ff1754;">心</span><span style="color: #00cec9;">sim</span>** é um projeto de código aberto dedicado à simulação biofísica da eletrofisiologia celular cardíaca e hemodinâmica ventricular em tempo real diretamente no navegador web. Desenvolvido inicialmente como projeto pessoal, evoluiu para uma ferramenta que pode auxiliar o ensino e a pesquisa no Brasil, sendo construído em **Rust** e compilado para **WebAssembly (WASM)** com interface declarativa reativa em **Dioxus**. O motor numérico resolve mais de 160 equações diferenciais ordinárias (EDOs) e variáveis de estado simultaneamente utilizando o método numérico híbrido de **Rush-Larsen** a $dt = 0.01\text{ ms}$ (1.600 passos por quadro de 16 ms a 60 FPS), assegurando execução em **1x tempo real** com baixíssimo consumo de CPU (**< 5-10%**) em smartphones, tablets, notebooks e computadores de laboratórios acadêmicos, sem depender de placas gráficas dedicadas.

---

## 🧭 Navegação e Documentação do Projeto
* 🚀 **[Abrir Simulador Interativo (Web App)](app.html)**
* 🧬 **[Modelos Biofísicos e Referências Científicas](docs/modelos.md)**: Equações completas, formulações de gating e modelos CellML validados.
* 🎓 **[Roteiro de Aulas Práticas para Universidades](docs/roteiro_aulas_praticas.md)**: 4 experimentos completos para Fisiologia e Farmacologia.
* 🗺️ **[Roadmap de Desenvolvimento](ROADMAP.md)**: Fases concluídas da v2.0 e expansão espacial para a v2.1+.
* 🏛️ **[Decisões de Arquitetura e Engenharia](ARCHITECTURE.md)**: Justificativas biofísicas, matemáticas e de engenharia de software.
* 🎨 **[Identidade Visual e Conceito do Logotipo](docs/identidade_visual.md)**: O Rotor, o Magatama (勾玉) e a convergência biofísica.
* 📜 **[Como Citar o KokoroSim](CITATION.cff)**: Normas ABNT, Vancouver, BibTeX e metadados formais.

---

## 🧬 Modelos Biofísicos Integrados

O simulador implementa 7 modelos biofísicos padrão-ouro validados pela literatura científica internacional e pelo consórcio [Physiome Model Repository (CellML)](https://models.physiomeproject.org/):

1. **Nó Sinoatrial (SA) — Marcapasso Primário:** Severi et al. (2012) — Automatismo biológico acoplado aos relógios de membrana ($I_f$) e cálcio ($I_{Ca,L}$, $I_{Ca,T}$, $I_{NCX}$) com modulação autonômica cronotrópica ($\beta_1$ e $M_2$).
2. **Músculo Atrial Humano:** Courtemanche, Ramirez, Nattel (1998) — Miócitos atriais de resposta rápida e repolarização dependente de $I_{Kur}$, gerando a **Onda P** do ECG.
3. **Nó Atrioventricular (AV) — Filtro e Retardo:** Inada et al. (2009) — Retardo nodal fisiológico dependente de cálcio (intervalo PR) e condução decremental protetora.
4. **Fibras de Purkinje e Feixe de His:** Stewart et al. (2009) — Condução hisiana ultrarrápida ($dV/dt > 400\text{ V/s}$) e automatismo terciário de escape idioventricular (~25–35 BPM).
5. **Músculo Ventricular com Heterogeneidade Transmural:** ten Tusscher & Panfilov (2006) — Subtipos Endocárdio, Célula M e Epicárdio reproduzindo o complexo QRS e a **Onda T positiva concordante** do ECG.
6. **Fibroblastos Cardíacos e Fibrose:** MacCannell et al. (2007) — Acoplamento eletrotônico miócito-fibroblasto via *gap junctions*, dreno capacitivo e lentificação na condução.
7. **Acoplamento Eletromecânico e Hemodinâmica:** Elastância de Suga & Sagawa (1974) e Circulação Arterial Windkessel de Westerhof et al. (2009) — Curvas dinâmicas de pressão ventricular ($LVP$) e aórtica ($AoP$) com incisura dicrótica e bulhas cardíacas (B1/B2).

> 🔬 **Documentação Científica Completa:** Para consultar as equações diferenciais detalhadas, formulações de correntes iônicas, links para o PubMed/DOI e repositórios computacionais CellML, acesse a página de **[Modelos Biofísicos e Referências Científicas](docs/modelos.md)**.

---

## ⚡ Sistema de Condução e Dromotropismo Dinâmico

Diferente de simuladores convencionais que utilizam grampeamentos de voltagem ou temporizadores arbitrários estáticos, o KokoroSim v2.0 implementa uma cadeia acoplada com **estímulos de corrente fisiológica** e **latências dependentes de estado (dromotropismo dinâmico)**:

### 1. Cadeia de Propagação Fisiológica
$$\text{Nó SA} \xrightarrow{\Delta t_{SA \to Atr}} \text{Átrio} \xrightarrow{\Delta t_{Atr \to AV}} \text{Nó AV} \xrightarrow{\Delta t_{AV \to His}} \text{Purkinje} \xrightarrow{\Delta t_{Purk \to Endo}} \text{Endocárdio} \xrightarrow{\Delta t_{trans}} \text{Célula M} \to \text{Epicárdio}$$

- Cada transição despolariza o tecido subsequente através de um **pulso transitório de injeção de corrente** ($I_{stim}$ ou $I_{st}$ por 1.5 a 2.0 ms), respeitando o limiar elétrico da membrana e o período refratário natural dos canais de sódio e cálcio.

### 2. Dromotropismo Dinâmico e Condução Decremental no Nó AV
O Nó Atrioventricular não possui um atraso fixo; sua velocidade de condução varia continuamente em resposta a fatores cronotrópicos e farmacológicos:
- **Condução Decremental Frequência-Dependente:** Conforme a frequência cardíaca se eleva (taquicardia sinusal ou batimentos ectópicos), os canais de cálcio $I_{Ca,L}$ do nó AV têm menos tempo para se recuperar da inativação. Como consequência, o atraso nodal alarga progressivamente (simulando a curva de recuperação do nó AV e o fenômeno de Wenckebach).
- **Tônus Autonômico:**
  - *Estimulação Simpática ($\beta_1$):* Aumenta a corrente $I_{Ca,L}$, acelerando a condução AV (dromotropismo positivo) e encurtando o intervalo PR.
  - *Estimulação Vagal / Parassimpática ($M_2$):* Ativa a corrente hiperpolarizante de potássio $I_{K,ACh}$, desacelerando a condução AV (dromotropismo negativo) e alargando o PR.
- **Verapamil e Bloqueadores de Cálcio:** Como o nó AV depende do cálcio para a Fase 0, a inibição de $I_{Ca,L}$ induz alargamento acentuado da condução (BAV de 1º grau) ou bloqueio completo (BAV de 2º e 3º grau), momento no qual as fibras de Purkinje assumem o marcapasso por automatismo terciário.
- **Isquemia Tecidual:** Lentifica a condução nodal e prolonga o intervalo PR.

### 3. Acoplamento Eletrotônico Miócito-Fibroblasto e Fibrose Miocárdica
A fibrose cardíaca pós-isquêmica ou senescente é simulada através da proliferação e acoplamento de fibroblastos não-excitáveis aos miócitos ventriculares via junções comunicantes de conexina-43/45:
- **Efeito Dreno Capacitivo:** Como os fibroblastos repousam a potenciais menos negativos ($-35\text{ a }-45\text{ mV}$), o acoplamento eletrotônico drena corrente dos miócitos vizinhos durante a despolarização e injeta corrente durante o repouso.
- **Despolarização Parcial Diastólica:** O potencial de repouso ventricular é elevado de $-86\text{ mV}$ para até $-76\text{ mV}$, promovendo inativação em estado estacionário dos canais rápidos de sódio $I_{Na}$.
- **Retardo Conducional e Bloqueios:** O $dV/dt_{\max}$ da Fase 0 ventricular é atenuado e a condução transmural sofre lentificação progressiva, criando o substrato clássico de arritmias por reentrada e dispersão tecidual.

---

## 📈 Eletrocardiograma (ECG) Baseado em Dipolo Transmural

O KokoroSim calcula a derivação eletrocardiográfica (equivalente a DII / precordiais) a partir de primeiros princípios biofísicos, resolvendo o gradiente elétrico do campo distante dipolar gerado pela despolarização e repolarização da parede cardíaca:

$$\text{ECG}(t) = 0.15 \cdot (V_{atrio}(t) + 80) + 0.55 \cdot (V_{endo}(t) - V_{epi}(t)) + 0.25 \cdot (V_M(t) - V_{epi}(t))$$

### Como a Morfologia P-QRS-T Real Emerge:
1. **Onda P:** Gerada pela despolarização inicial do tecido atrial ($V_{atrio}$).
2. **Intervalo PR:** Reflete o tempo de trânsito através do átrio e retardo nodal AV até o feixe de His.
3. **Complexo QRS (Onda R escarpada):** A despolarização chega primeiro ao subendocárdio via Purkinje ($t = 0\text{ ms}$), enquanto o epicárdio permanece em repouso ($-86\text{ mV}$). O gradiente $V_{endo} - V_{epi} \approx +25 - (-86) = +111\text{ mV}$ projeta uma deflexão positiva rápida e estreita.
4. **Segmento ST:** Durante a fase de platô, todas as camadas ventriculares estão simultaneamente despolarizadas ($V_{endo} \approx V_M \approx V_{epi} \approx +15\text{ a }+25\text{ mV}$), anulando a diferença de potencial e mantendo o traçado na linha isoelétrica.
5. **Onda T Positiva e Fisiológica:** O epicárdio possui densidade superior de canais de potássio de efluxo rápido e APD mais curto, repolarizando para $-86\text{ mV}$ **antes** do endocárdio e da célula M. Durante a repolarização:
   $$V_{endo}(t) - V_{epi}(t) \approx (-30\text{ mV}) - (-86\text{ mV}) = +56\text{ mV} > 0$$
   Esse gradiente elétrico positivo gera a clássica **Onda T positiva e concordante com o QRS**, resolvendo o paradoxo histórico dos modelos unicelulares!

---

## 💓 Hemodinâmica Ventricular, Diagrama de Wiggers e Síntese de Áudio

O quarto canal do osciloscópio exibe em tempo real o núcleo mecânico do **Diagrama de Wiggers**:
- **Pressão Ventricular Esquerda ($LVP$, ciano `#00cec9`):** Eleva-se de ~7 mmHg na diástole até ~120 mmHg na sístole, cruzando a curva aórtica nos pontos de abertura e fechamento valvar.
- **Pressão Aórtica ($AoP$, coral `#ff7675`):** Decai exponencialmente na diástole até ~70-80 mmHg e atinge ~120 mmHg no pico da ejeção. O fechamento da valva aórtica gera a clássica **incisura dicrótica**.
- **Síntese Acústica em Tempo Real (Web Audio API):**
  - **🔊 Bip de Monitor (UTI - Onda R):** Disparado na despolarização ventricular rápida ($dV/dt > 0$), gerando uma onda senoidal pura de **880 Hz** com decaimento exponencial de 75 ms.
  - **🩺 Bulhas Cardíacas (Ausculta B1/B2):**
    - **B1 ("Tum" / *Lub*):** Disparada na contração isovolumétrica no fechamento da valva mitral ($LVP \ge LAP$), sintetizada com *pitch sweep* descendente (140 $\to$ 85 Hz), ressonância muscular calibrada e ganho reforçado para clara audibilidade em transdutores de smartphones e notebooks.
    - **B2 ("Tá" / *Dub*):** Disparada no relaxamento isovolumétrico no fechamento da valva aórtica ($LVP \le AoP$), sintetizada com estalido de alta frequência (240 $\to$ 160 Hz), filtro passa-banda e decaimento rápido.
  - **Marcadores Visuais Verticais Multicanal:** Quando as opções de áudio estão ativas, linhas verticais correspondentes são desenhadas simultaneamente em todos os 4 canais (amarelo tracejado para Onda R/UTI, verde esmeralda para B1 e coral para B2), estabelecendo correlação áudio-visual em tempo real.
  - As duas opções de som vêm **desativadas por padrão** no Accordion "5. Monitorização & Áudio", sendo ativadas pelo clique do usuário em conformidade com as diretrizes de autoplay dos navegadores.

### 📺 Modos de Visualização do Osciloscópio e Onda Fantasma
- **5 Modos de Varredura Temporal:**
  1. *Fita Deslizante (Rolling Strip-Chart — Padrão):* Fluxo contínuo da direita para a esquerda.
  2. *Varredura Contínua (Continuous Sweep — Monitor UTI):* Varredura da esquerda para a direita com barra apagadora à frente da caneta e wrap-around.
  3. *Paginação Sincronizada (Triggered Paged):* Sincronizada pelo início da Fase 0 do Nó SA ($X = 0$); desenha uma página completa (~1.5 a 2 ciclos), congela a imagem para estudo minucioso e recomeça a nova página no próximo marco sinusal.
  4. *Gatilho Automático (Auto-Trigger):* Redesenha automaticamente a cada batimento a partir do Nó SA com barra apagadora, sobrepondo os ciclos no mesmo eixo temporal.
  5. *Gatilho Único (Single-Shot / Congelado):* Traça exatamente 1 ciclo completo e congela indefinidamente, aguardando o comando `⚡ DISPARAR / ARMAR PRÓXIMO CICLO`.
- **Controle de Execução e Congelamento:**
  - Botão `[ ⏸ CONGELAR ]` / `[ ▶ CONTINUAR ]` no topo da barra de controles para suspender instantaneamente a simulação, congelar o traçado e silenciar o áudio para análise pedagógica detalhada.
- **Onda Fantasma Bio-Sincronizada (Referência Basal em Memória):**
  - Gravação bio-disparada na **Fase 0 do Nó SA**: ao clicar em `📸 Capturar`, o sistema aguarda o marcapasso sinusal e grava com precisão 1 ciclo $RR$ completo fechado ($t_0 \to t_{RR}$).
  - **Dinâmica de Espaçamento e Encurtamento:** Em bradicardia, repousa no potencial basal ($V_{rest}$) destacando o atraso; em taquicardia, reinicia sincronizada a cada novo batimento.
  - **Acessibilidade Sensorial (Zero Formiguinhas):** Renderizada como linha sólida contínua translúcida e suave (`rgba(..., 0.35)`, espessura 1.2px), sem cintilações ou vibrações visuais, propiciando conforto máximo a usuários neurodivergentes.
  - **Custo de CPU Zero:** Armazenada em vetor estático sem exigir instâncias secundárias da simulação.
- **Reset Limpo de Memória nos Modos de Varredura e Gatilho:**
  - Comutação entre modos com higienização instantânea de memória por sentinelas `NaN`, eliminando quaisquer rastros ou artefatos de ciclos anteriores.

---

## 🎛️ Modulação Farmacológica, Eletrolítica e Patológica

- **Potássio $[K^+]_o$ (2.0 a 8.5 mEq/L):** Modula o potencial de repouso ($V_{rest}$) pela equação de Nernst. Hipocalemia causa hiperexcitabilidade e pós-despolarizações; hipercalemia severa causa inativação permanente dos canais de sódio e parada diastólica.
- **Cálcio $[Ca^{2+}]_o$ (1.0 a 3.5 mmol/L):** Regula a corrente $I_{Ca,L}$, modulando a duração do platô ventricular e o intervalo QT.
- **Sódio $[Na^+]_o$ (125 a 155 mEq/L):** Determina a amplitude e velocidade da Fase 0 nas células rápidas (Átrio, Purkinje e Ventrículo).
- **Fibrose Miocárdica (0% a 100%):** Aumenta a densidade e o acoplamento de junções comunicantes miócito-fibroblasto ($G_{gap}$ até $4.0\text{ nS}$), deprimindo $dV/dt$, prolongando a condução e gerando dispersão da repolarização.
- **Antiarrítmicos (Classes I a IV de Vaughan Williams):**
  - *Classe I (Lidocaína):* Bloqueio fracionário de canais de $Na^+$, alargando o complexo QRS e reduzindo a velocidade de condução transmural.
  - *Classe III (Amiodarona):* Bloqueio de canais de $K^+$ ($I_{Kr}$, $I_{Ks}$), prolongando o platô e o intervalo QT.
  - *Classe IV (Verapamil):* Inibição seletiva de $I_{Ca,L}$, deprimindo a rampa nodal AV e simulando bloqueios atrioventriculares.
  - *Glicosídeo Cardíaco (Digoxina):* Inibição da bomba $Na^+/K^+$ ATPase, elevando o cálcio intracelular e a força de contração ativa.
- **Isquemia Miocárdica:** Ativação de canais $K_{ATP}$, encurtando precocemente o potencial de ação e provocando alterações morfológicas do segmento ST e da onda T.

---

## 🎨 Identidade Visual e Conceito do Logotipo: O Rotor / Magatama (勾玉)

O isotipo do **KokoroSim** foge dos clichês gráficos tradicionais da cardiologia (como corações estilizados ou traçados simplistas de ECG). Em vez disso, estabelece uma ponte geométrica direta entre a **eletrofisiologia não-linear**, a **hemodinâmica ventricular** e a **iconografia tradicional japonesa**:

### 1. A Convergência Multifísica (O Choque e o Fluxo)
A silhueta baseia-se no **magatama** (勾玉) e no elemento dinâmico unitário do *mitsudomoe* (a gota espiralada clássica). Essa topologia resolve em um único símbolo as duas forças que regem o simulador:
- **Na Eletrofisiologia (O Choque):** Rotores são **ondas espirais auto-sustentadas** que giram em torno de uma singularidade de fase em meio excitável (reentrância funcional), representando a base matemática primordial das taquiarritmias e da fibrilação ventricular.
- **Na Hemodinâmica (O Fluxo):** A mesma espiral descreve a formação dos **anéis de vórtice transmitrais** (*vortex rings*) durante o enchimento diastólico ventricular rápido, canalizando a inércia do sangue para a via de ejeção aórtica com máxima eficiência energética.

### 2. Eficiência de Silhueta e Traço Dinâmico
- **Assimetria Funcional:** A terminação afiada e agressiva da cauda do magatama evoca o disparo elétrico da despolarização rápida da membrana (Fase 0 mediada por $I_{Na}$ / pico da Onda R). O corpo arredondado fecha-se em espiral logarítmica de vórtice, sugerindo o turbilhonamento fluido ventricular.
- **Resolução em Baixa Altura (1-bit / 24px):** Projetado para manter reconhecimento visual imediato tanto em alta resolução quanto em ícones microscópicos de 24 pixels (avatares de repositório, favicons de navegador e executáveis de desktop), superando o ruído visual de diagramas densos e o peso estático de arabescos têxteis.

### 3. O Kanji e a Tipografia Integrada: `kokor心sim`
- O caractere **心** (*kokoro* — coração, mente, espírito) é fundido diretamente na tipografia geométrica, substituindo o segundo "o" da palavra. Essa fusão sutil preserva a legibilidade fonética e ancora a alma do software.
- A paleta de cores adota o **Carmesim Neon** (`#ff1754` — representando o sangue arterial oxigenado, a pressão aórtica e o calor metabólico) em contraste com o **Ciano Elétrico** (`#00f2fe` — representando a bioeletricidade transmembrana, a polaridade iônica e a convenção anatômica do sangue venoso), montados sobre um fundo escuro com estética de interface tática Mecha/HUD de ficção científica.

> 📖 **Documento Conceitual Completo:** Leia o ensaio ilustrado sobre a semiologia, geometria e fundamentação visual em **[Identidade Visual e Conceito do Logotipo](docs/identidade_visual.md)**.

---

## 💻 Arquitetura de Software

- **Linguagem Principal:** Rust 2021 Edition.
- **Compilação WebAssembly:** `wasm-bindgen` e `wasm-pack` com otimizações em release (`-O3`).
- **Interface Gráfica:** [Dioxus](https://dioxuslabs.com/) 0.6 (declarativo e reativo nativo em Rust).
- **Mecanismo de Renderização:** Pipeline `Canvas 2D` de alto desempenho com buffers circulares de 500 pontos (janela horizontal de 2.5s, comportando > 3 ciclos cardíacos completos) e processamento em lote via `HeartSystem::run_batch()`.
- **Módulo de Áudio:** Sintetizador biofísico nativo via Web Audio API (`AudioContext`, `OscillatorNode`, `GainNode`, `BiquadFilterNode`) sem dependência de assets de áudio externos.
- **Ambiente de Desenvolvimento:** Script `./scripts/run_local.sh` com hot-reloading inteligente, fila concorrente assíncrona, debounce e injeção automática de versão e build-time no rodapé.

---

## 📖 Citação Acadêmica

Se você utilizar o KokoroSim em pesquisas científicas, aulas práticas, monografias, dissertações, teses ou publicações acadêmicas, por favor cite conforme os formatos abaixo ou utilize os metadados do arquivo [`CITATION.cff`](CITATION.cff):

### Formato ABNT (NBR 6023:2018)
> FREITAS, Lumen Muller Lohn. **KokoroSim: Simulador Eletrofisiológico Cardíaco Celular e Hemodinâmico em Tempo Real**. Versão 2.0.0. Florianópolis: Universidade Federal de Santa Catarina (UFSC), 2026. Disponível em: <https://kokorosim.github.io/>. Acesso em: [data de acesso].

### Formato Vancouver
> Freitas LML. KokoroSim: Simulador Eletrofisiológico Cardíaco Celular e Hemodinâmico em Tempo Real [software na Internet]. Versão 2.0.0. Florianópolis: Universidade Federal de Santa Catarina; 2026 [citado em ano mês dia]. Disponível em: https://kokorosim.github.io/

### Formato BibTeX
```bibtex
@software{freitas2026kokorosim,
  author       = {Freitas, Lumen Muller Lohn},
  title        = {KokoroSim: Simulador Eletrofisiológico Cardíaco Celular e Hemodinâmico em Tempo Real},
  year         = {2026},
  publisher    = {GitHub},
  journal      = {GitHub repository},
  howpublished = {\url{https://github.com/KokoroSim/KokoroSim}},
  url          = {https://kokorosim.github.io/}
}
```

---

## 🕰️ Versão Histórica 1.0 (Protótipo Legado em TypeScript/Vite)

Para fins históricos, didáticos e de comparação de desempenho e evolução da engenharia de software, a primeira versão do simulador permanece disponível online:

* 🌐 **[Acessar KokoroSim v1.0 (Protótipo Legado)](https://kokorosim.github.io/v1/)**

### Como foi concebida e construída a v1.0
A versão 1.0 foi o protótipo inicial do projeto, desenvolvida em Vanilla TypeScript, empacotada com Vite e executada em um Web Worker que integrava as equações diferenciais pelo método explícito de *Forward Euler* em JavaScript puro para os modelos atrioventriculares iniciais, desenhando os traçados em um elemento Canvas 2D.

### Por que o projeto foi completamente reescrito na v2.0 (Rust + WebAssembly + Dioxus)?
1. **Rigidez Numérica (*Stiffness*) e a Transição de Forward Euler para Rush-Larsen:** À medida que a simulação incorporou novos modelos acoplados (Purkinje, as 3 camadas ventriculares transmurais e fibroblastos), a rigidez matemática dos canais rápidos de sódio ($\tau_m \approx 0.0008\text{ ms}$) impôs passos microscópicos de integração ($dt = 0.001\text{ ms}$). O navegador precisava processar mais de 8 milhões de avaliações de EDOs por segundo, saturando CPUs modestas e tornando a execução lenta e inviável em celulares. Na v2.0, a reescrita em Rust e a adoção do método de integração exponencial de **Rush-Larsen (1978)** permitiram multiplicar o passo em 10x ($dt = 0.01\text{ ms}$), reduzindo o consumo de CPU para **< 5-10%** com estabilidade numérica incondicional.
2. **Eliminação de *Stuttering* por Garbage Collection:** O motor V8 do JavaScript sofria com pausas intermitentes de desalocação de memória (*Garbage Collection*) e desotimizações JIT causadas pelo volume constante de arrays transitórios, provocando micro-travamentos no osciloscópio. O WebAssembly em Rust roda com determinismo estrito, latência zero de GC e taxa contínua de 60 FPS.
3. **Memória Linear Compartilhada (*Zero-Copy*):** Na arquitetura antiga, os buffers de pontos precisavam ser serializados e copiados via mensagens entre o Web Worker e a thread principal de renderização. Na v2.0, a interface reativa em Dioxus e o motor biofísico operam no mesmo espaço de memória linear contígua do Wasm sem nenhuma cópia de dados.

---

## 📜 Licença

Distribuído sob a licença **GNU General Public License v3.0 (GPLv3)**. Consulte o arquivo `LICENSE` para mais detalhes.

