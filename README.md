# SimCardio: Simulador Eletrofisiológico Cardíaco em Tempo Real

O **SimCardio** é um projeto de código aberto dedicado à simulação matemática da eletrofisiologia celular cardíaca em tempo real no navegador. Desenvolvido inteiramente em **Rust** e compilado para **WebAssembly (WASM)** com interface reativa em **Dioxus**, o sistema resolve mais de 160 equações diferenciais ordinárias (EDOs) e variáveis de estado simultaneamente a 16.000 passos por frame (passo de integração $dt = 0.001\text{ ms}$), reproduzindo com rigor biofísico a gênese do potencial de ação e a condução elétrica através de todo o sincício cardíaco humano.

---

## 🧬 Modelos Biofísicos Integrados

O simulador implementa 5 modelos eletrofisiológicos padrão-ouro validados pela literatura acadêmica internacional e pelo consórcio [Physiome Project / CellML](https://models.physiomeproject.org/):

1. **Nó Sinoatrial (SA) — Marcapasso Primário:**
   - **Modelo:** Severi et al. (2012)
   - **Referência:** Severi S, Fantini M, Charawi LA, DiFrancesco D. *An updated computational model of rabbit sinoatrial action potential to investigate the mechanisms of heart rate modulation.* J Physiol. 2012.
   - **Função:** Gera o automatismo elétrico biológico (Fase 4 despolarizante) impulsionado pelo "relógio de membrana" (corrente *funny* $I_f$) e "relógio de cálcio" intracelular ($I_{Ca,L}$, $I_{Ca,T}$ e NCX). Modulado por receptores $\beta_1$-adrenérgicos e colinérgicos muscarínicos ($M_2$).

2. **Músculo Atrial Humano:**
   - **Modelo:** Courtemanche, Ramirez, Nattel (1998)
   - **Referência:** Courtemanche M, Ramirez RJ, Nattel S. *Ionic mechanisms underlying human atrial action potential properties: insights from a mathematical model.* Am J Physiol. 1998.
   - **Função:** Representa os miócitos atriais de resposta rápida, caracterizados por rápida ascensão dependente de canais de sódio ($I_{Na}$), platô intermediário e repolarização dependente de canais ultrarrápidos de potássio ($I_{Kur}$). Gera a deflexão mecânica e a **Onda P** do eletrocardiograma.

3. **Nó Atrioventricular (AV) — Filtro e Retardo Fisiológico:**
   - **Modelo:** Inada et al. (2009)
   - **Referência:** Inada S, et al. *One-dimensional mathematical model of the atrioventricular node...* Biophys J. 2009.
   - **Função:** Executa o retardo atrioventricular essencial para permitir o enchimento ventricular prévio à sístole (intervalo PR). Implementa resposta lenta dependente de canais de cálcio do tipo L ($I_{Ca,L}$) com condução decremental.

4. **Fibras de Purkinje e Feixe de His — Rede de Condução Rápida:**
   - **Modelo:** Stewart et al. (2009)
   - **Referência:** Stewart P, Aslanidi OV, Noble D, Noble PJ, Boyett MR, Zhang H. *Mathematical model of the electrical action potential of the human Purkinje cell.* Biophys J. 2009.
   - **Função:** Modela a rede hisiana e as fibras subendocárdicas com 20 variáveis de estado. Caracteriza-se por velocidade de ascensão extremamente rápida ($dV/dt > 400\text{ V/s}$), entalhe precoce acentuado ($I_{to}$ e $I_{sus}$) e potencial diastólico com corrente marcapasso $I_f$ residual, atuando como centro terciário de ritmo de escape idioventricular (~25–35 BPM).

5. **Músculo Ventricular com Heterogeneidade Transmural:**
   - **Modelo:** ten Tusscher & Panfilov (2006)
   - **Referência:** ten Tusscher KHWJ, Panfilov AV. *Alternans and spiral breakup in a human ventricular tissue model.* Am J Physiol Heart Circ Physiol. 2006.
   - **Função:** Modela as três camadas da parede livre ventricular humana:
     - **Endocárdio (Subendocárdio):** Densidade de $I_{to}$ baixa ($G_{to} = 0.073\text{ nS/pF}$), platô arredondado e duração do potencial de ação (APD) intermediária. É ativado primeiro pelas fibras de Purkinje.
     - **Célula M (Mid-miocárdio):** Densidade reduzida de $I_{Ks}$ ($G_{Ks} = 0.098\text{ nS/pF}$), conferindo o platô mais longo de todas as camadas. Principal determinante do intervalo QT e substrato para arritmias de reentrada.
     - **Epicárdio (Subepicárdico):** Densidade de $I_{to}$ robusta ($G_{to} = 0.294\text{ nS/pF}$), entalhe proeminente na Fase 1 e **APD mais curto**. É a última camada a ser despolarizada, mas a **primeira a repolarizar**.

6. **Fibroblastos Cardíacos e Miofibroblastos:**
   - **Modelo:** MacCannell et al. (2007)
   - **Referência:** MacCannell KA, Bazzazi H, Chilton L, Shibukawa Y, Clark RB, Giles WR. *A mathematical model of electrotonic interactions between ventricular myocytes and fibroblasts.* Biophys J. 2007.
   - **Função:** Modela células não-excitáveis com potencial de repouso despolarizado ($-35\text{ a }-45\text{ mV}$) e correntes $I_{Kv}$, $I_{K1}$, $I_b$ e $I_{NaK}$. Acopla-se eletrotonicamente aos miócitos por junções comunicantes (*gap junctions*), atuando como dreno capacitivo, despolarizando o repouso do miócito e diminuindo a velocidade de condução intramiocárdica.

7. **Acoplamento Eletromecânico e Hemodinâmica (Diagrama de Wiggers):**
   - **Modelos:** Elastância Ventricular Variável no Tempo (Suga & Sagawa, 1974) e Circulação Arterial Windkessel de 3 elementos (Westerhof et al., 2009).
   - **Função:** Converte o transiente real de $[Ca^{2+}]_i$ em desenvolvimento de força isométrica de troponina. Calcula em tempo real a Pressão Ventricular Esquerda ($LVP$, 0-140 mmHg), Pressão Aórtica ($AoP$, 80-120 mmHg) com incisura dicrótica, volume ventricular ($LVV$) e dinâmica valvar das cúspides mitral e aórtica.

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
    - **B1 ("Tum" / *Lub*):** Disparada na contração isovolumétrica no fechamento da valva mitral ($LVP \ge LAP$), com frequência grave (65 Hz), ressonância muscular e filtro passa-baixa.
    - **B2 ("Tá" / *Dub*):** Disparada no relaxamento isovolumétrico no fechamento da valva aórtica ($LVP \le AoP$), com frequência mais alta (130 Hz) e estalido seco.
  - As duas opções vêm **desativadas por padrão** no Accordion "5. Monitorização & Áudio", sendo ativadas pelo clique do usuário em conformidade com as diretrizes de autoplay dos navegadores.

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

## 📜 Licença

Distribuído sob a licença **GNU General Public License v3.0 (GPLv3)**. Consulte o arquivo `LICENSE` para mais detalhes.
