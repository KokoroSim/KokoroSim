# SimCardio: Simulador Eletrofisiológico Cardíaco em Tempo Real

[![Deploy to GitHub Pages](https://github.com/lumenpink/simcardio/actions/workflows/deploy.yml/badge.svg)](https://lumenpink.github.io/simcardio/)
[![Licença: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-purple.svg?logo=webassembly)](https://webassembly.org/)
[![Dioxus](https://img.shields.io/badge/UI-Dioxus_0.6-00cec9.svg)](https://dioxuslabs.com/)
[![Mobile Optimized](https://img.shields.io/badge/Hardware-Mobile_%26_Low--Power-success.svg)](#-arquitetura-de-software)
[![Acessibilidade](https://img.shields.io/badge/Acessibilidade-Neurodivergente--Friendly-brightgreen.svg)](#-modos-de-visualização-do-osciloscópio-e-onda-fantasma)

O **SimCardio** é um projeto acadêmico de código aberto dedicado à simulação biofísica da eletrofisiologia celular cardíaca e hemodinâmica ventricular em tempo real diretamente no navegador web. Desenvolvido para servir prioritariamente a universidades e centros de pesquisa no Brasil, o software é construído em **Rust** e compilado para **WebAssembly (WASM)** com interface declarativa reativa em **Dioxus**. O motor numérico resolve mais de 160 equações diferenciais ordinárias (EDOs) e variáveis de estado simultaneamente utilizando o método numérico híbrido de **Rush-Larsen** a $dt = 0.01\text{ ms}$ (1.600 passos por quadro de 16 ms a 60 FPS), assegurando execução em **1x tempo real** com baixíssimo consumo de CPU (**< 5-10%**) em smartphones, tablets, notebooks e computadores de laboratórios acadêmicos, sem depender de placas gráficas dedicadas.

---

## 🧭 Navegação e Documentação do Projeto
* 🚀 **[Acessar Simulador em Execução (Web App)](https://lumenpink.github.io/simcardio/)**
* 🎓 **[Roteiro de Aulas Práticas para Universidades](docs/roteiro_aulas_praticas.md)**: 4 experimentos completos para Fisiologia e Farmacologia.
* 🗺️ **[Roadmap de Desenvolvimento (ROADMAP.md)](ROADMAP.md)**: Fases concluídas e planejamento de expansão 2D/3D.
* 🏛️ **[Decisões de Arquitetura (ARCHITECTURE.md)](ARCHITECTURE.md)**: Justificativas biofísicas, matemáticas e de engenharia.
* 📜 **[Como Citar o SimCardio (CITATION.cff)](CITATION.cff)**: Metadados formais para TCCs, dissertações e artigos científicos.

---

## 🧬 Modelos Biofísicos Integrados

O simulador implementa 7 modelos biofísicos padrão-ouro validados pela literatura científica internacional e pelo consórcio [Physiome Model Repository (CellML)](https://models.physiomeproject.org/):

1. **Nó Sinoatrial (SA) — Marcapasso Primário:**
   - **Modelo:** Severi et al. (2012)
   - **Artigo:** *An updated computational model of rabbit sinoatrial action potential to investigate the mechanisms of heart rate modulation.* J Physiol. 2012;590(18):4483-4499.
   - **Repositório CellML:** [PMR Model e/144](https://models.physiomeproject.org/e/144/)
   - **Identificadores:** [PubMed 22711956](https://pubmed.ncbi.nlm.nih.gov/22711956/) | [DOI 10.1113/jphysiol.2012.234385](https://doi.org/10.1113/jphysiol.2012.234385)
   - **Função:** Gera o automatismo elétrico biológico (Fase 4 despolarizante espontânea) impulsionado pelo "relógio de membrana" (corrente *funny* $I_f$) e "relógio de cálcio" intracelular ($I_{Ca,L}$, $I_{Ca,T}$ e NCX). Modulado por receptores autonômicos $\beta_1$ e $M_2$.

2. **Músculo Atrial Humano:**
   - **Modelo:** Courtemanche, Ramirez, Nattel (1998)
   - **Artigo:** *Ionic mechanisms underlying human atrial action potential properties: insights from a mathematical model.* Am J Physiol. 1998;275(1):H301-H321.
   - **Repositório CellML:** [PMR Model e/286](https://models.physiomeproject.org/e/286/courtemanche_ramirez_nattel_1998.cellml)
   - **Identificadores:** [PubMed 9688927](https://pubmed.ncbi.nlm.nih.gov/9688927/) | [DOI 10.1152/ajpheart.1998.275.1.H301](https://doi.org/10.1152/ajpheart.1998.275.1.H301)
   - **Função:** Miócitos atriais de resposta rápida, caracterizados por ascensão rápida via canais rápidos de sódio ($I_{Na}$), platô intermediário e repolarização dependente de canais ultrarrápidos de potássio ($I_{Kur}$). Gera a **Onda P** do ECG.

3. **Nó Atrioventricular (AV) — Filtro e Retardo Fisiológico:**
   - **Modelo:** Inada et al. (2009)
   - **Artigo:** *One-dimensional mathematical model of the atrioventricular node including the atrioventricular ring and bundle of His.* Biophys J. 2009;97(8):2117-2127.
   - **Repositório CellML:** [PMR Model e/55](https://models.physiomeproject.org/e/55/inada_hancox_zhang_boyett_2009.cellml)
   - **Identificadores:** [PubMed 19843444](https://pubmed.ncbi.nlm.nih.gov/19843444/) | [DOI 10.1016/j.bpj.2009.06.056](https://doi.org/10.1016/j.bpj.2009.06.056)
   - **Função:** Retardo nodal essencial para o enchimento ventricular diastólico (intervalo PR). Resposta lenta dependente de $I_{Ca,L}$ com condução decremental frequência-dependente.

4. **Fibras de Purkinje e Feixe de His — Condução Rápida e Marcapasso Terciário:**
   - **Modelo:** Stewart et al. (2009)
   - **Artigo:** *Mathematical model of the electrical action potential of the human Purkinje cell.* Biophys J. 2009;96(9):3493-3507.
   - **Repositório CellML:** [PMR Model e/7e](https://models.physiomeproject.org/e/7e/stewart_aslanidi_noble_noble_boyett_zhang_2009.cellml)
   - **Identificadores:** [PubMed 19413956](https://pubmed.ncbi.nlm.nih.gov/19413956/) | [DOI 10.1016/j.bpj.2009.01.047](https://doi.org/10.1016/j.bpj.2009.01.047)
   - **Função:** Condução ultrarrápida hisiana ($dV/dt > 400\text{ V/s}$), entalhe precoce acentuado ($I_{to}$, $I_{sus}$) e corrente marcapasso $I_f$ residual de escape idioventricular (~25–35 BPM).

5. **Músculo Ventricular com Heterogeneidade Transmural:**
   - **Modelo:** ten Tusscher & Panfilov (2006)
   - **Artigo:** *Alternans and spiral breakup in a human ventricular tissue model.* Am J Physiol Heart Circ Physiol. 2006;291(3):H1088-H1100.
   - **Repositório CellML:** [PMR Model e/210](https://models.physiomeproject.org/e/210/tentusscher_panfilov_2006_m.cellml)
   - **Identificadores:** [PubMed 16565318](https://pubmed.ncbi.nlm.nih.gov/16565318/) | [DOI 10.1152/ajpheart.00109.2006](https://doi.org/10.1152/ajpheart.00109.2006)
   - **Função:** Modela as três camadas da parede livre ventricular:
     - **Endocárdio:** Densidade de $I_{to}$ baixa, ativado primeiro via Purkinje.
     - **Célula M:** Densidade de $I_{Ks}$ reduzida e platô mais longo de todas as camadas.
     - **Epicárdio:** Densidade de $I_{to}$ proeminente e **APD mais curto**. É o último a despolarizar e o **primeiro a repolarizar**, gerando a **Onda T positiva concordante** no ECG.

6. **Fibroblastos Cardíacos e Miofibroblastos:**
   - **Modelo:** MacCannell et al. (2007)
   - **Artigo:** *A mathematical model of electrotonic interactions between ventricular myocytes and fibroblasts.* Biophys J. 2007;92(11):4121-4132.
   - **Repositório CellML:** [PMR Model e/98](https://models.physiomeproject.org/e/98/maccannell_bazzazi_chilton_shibukawa_clark_giles_2007.cellml)
   - **Identificadores:** [PubMed 17351008](https://pubmed.ncbi.nlm.nih.gov/17351008/) | [DOI 10.1529/biophysj.106.101410](https://doi.org/10.1529/biophysj.106.101410)
   - **Função:** Células não-excitáveis com repouso alto ($-38\text{ mV}$) que se acoplam eletrotonicamente aos miócitos por conexinas ($G_{gap}$ até $4.0\text{ nS}$), exercendo efeito de dreno capacitivo e lentificação na condução proporcional à fibrose.

7. **Acoplamento Eletromecânico e Hemodinâmica (Diagrama de Wiggers):**
   - **Modelos:** Elastância Ventricular Variável no Tempo (Suga & Sagawa, 1974) e Circulação Arterial Windkessel de 3 elementos (Westerhof et al., 2009).
   - **Identificadores:** [PubMed 4841253](https://pubmed.ncbi.nlm.nih.gov/4841253/) (Suga & Sagawa) | [PubMed 19194725](https://pubmed.ncbi.nlm.nih.gov/19194725/) (Westerhof)
   - **Função:** Transiente de $[Ca^{2+}]_i$ acoplado à elastância ventricular para calcular a Pressão Ventricular Esquerda ($LVP$, 0-140 mmHg), Pressão Aórtica ($AoP$) com incisura dicrótica e dinâmica valvar das cúspides mitral e aórtica.

---

## ⚡ Sistema de Condução e Dromotropismo Dinâmico

Diferente de simuladores convencionais que utilizam grampeamentos de voltagem ou temporizadores arbitrários estáticos, o SimCardio v2.0 implementa uma cadeia acoplada com **estímulos de corrente fisiológica** e **latências dependentes de estado (dromotropismo dinâmico)**:

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

O SimCardio calcula a derivação eletrocardiográfica (equivalente a DII / precordiais) a partir de primeiros princípios biofísicos, resolvendo o gradiente elétrico do campo distante dipolar gerado pela despolarização e repolarização da parede cardíaca:

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

## 💻 Arquitetura de Software

- **Linguagem Principal:** Rust 2021 Edition.
- **Compilação WebAssembly:** `wasm-bindgen` e `wasm-pack` com otimizações em release (`-O3`).
- **Interface Gráfica:** [Dioxus](https://dioxuslabs.com/) 0.6 (declarativo e reativo nativo em Rust).
- **Mecanismo de Renderização:** Pipeline `Canvas 2D` de alto desempenho com buffers circulares de 500 pontos (janela horizontal de 2.5s, comportando > 3 ciclos cardíacos completos) e processamento em lote via `HeartSystem::run_batch()`.
- **Módulo de Áudio:** Sintetizador biofísico nativo via Web Audio API (`AudioContext`, `OscillatorNode`, `GainNode`, `BiquadFilterNode`) sem dependência de assets de áudio externos.
- **Ambiente de Desenvolvimento:** Script `./scripts/run_local.sh` com hot-reloading inteligente, fila concorrente assíncrona, debounce e injeção automática de versão e build-time no rodapé.

---

## 📖 Citação Acadêmica

Se você utilizar o SimCardio em pesquisas científicas, aulas práticas, monografias, dissertações, teses ou publicações acadêmicas, por favor cite conforme as diretrizes do arquivo [`CITATION.cff`](CITATION.cff):

```bibtex
@software{pink2026simcardio,
  author       = {Pink, Lumen},
  title        = {SimCardio: Simulador Eletrofisiológico Cardíaco Celular e Hemodinâmico em Tempo Real},
  year         = {2026},
  publisher    = {GitHub},
  journal      = {GitHub repository},
  howpublished = {\url{https://github.com/lumenpink/simcardio}},
  url          = {https://lumenpink.github.io/simcardio/}
}
```

---

## 📜 Licença

Distribuído sob a licença **GNU General Public License v3.0 (GPLv3)**. Consulte o arquivo `LICENSE` para mais detalhes.

