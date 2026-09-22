# Modelos Biofísicos e Referências Científicas

O **KokoroSim** implementa 7 modelos biofísicos padrão-ouro validados pela literatura científica internacional e pelo consórcio [Physiome Model Repository (CellML)](https://models.physiomeproject.org/). Este documento reúne a fundamentação biofísica, artigos seminais, identificadores científicos e repositórios computacionais de cada componente simulado.

---

## 📋 Sumário dos Modelos

1. [Nó Sinoatrial (SA) — Severi et al. (2012)](#1-nó-sinoatrial-sa--marcapasso-primário)
2. [Músculo Atrial Humano — Courtemanche et al. (1998)](#2-músculo-atrial-humano)
3. [Nó Atrioventricular (AV) — Inada et al. (2009)](#3-nó-atrioventricular-av--filtro-e-retardo-fisiológico)
4. [Fibras de Purkinje e Feixe de His — Stewart et al. (2009)](#4-fibras-de-purkinje-e-feixe-de-his--condução-rápida-e-marcapasso-terciário)
5. [Músculo Ventricular com Heterogeneidade Transmural — ten Tusscher & Panfilov (2006)](#5-músculo-ventricular-com-heterogeneidade-transmural)
6. [Fibroblastos Cardíacos e Fibrose — MacCannell et al. (2007)](#6-fibroblastos-cardíacos-e-miofibroblastos)
7. [Acoplamento Eletromecânico e Hemodinâmica — Suga-Sagawa & Westerhof](#7-acoplamento-eletromecânico-e-hemodinâmica-diagrama-de-wiggers)
8. [Quimerismo Biofísico e Calibrações Computacionais (Coelho → Humano)](#8-quimerismo-biofísico-e-calibrações-computacionais-coelho--humano)
9. [Referências Científicas Complementares (Além do CellML)](#9-referências-científicas-complementares-além-do-cellml)

---

## 1. Nó Sinoatrial (SA) — Marcapasso Primário

* **Modelo:** Severi et al. (2012)
* **Artigo Original:** *An updated computational model of rabbit sinoatrial action potential to investigate the mechanisms of heart rate modulation.* J Physiol. 2012;590(18):4483-4499.
* **Repositório CellML:** [PMR Model e/144](https://models.physiomeproject.org/e/144/)
* **Identificadores Científicos:** [PubMed 22711956](https://pubmed.ncbi.nlm.nih.gov/22711956/) | [DOI 10.1113/jphysiol.2012.234385](https://doi.org/10.1113/jphysiol.2012.234385)

### Biofísica e Papel Fisiológico
O nó sinoatrial gera o automatismo elétrico intrínseco (Fase 4 despolarizante espontânea) através da interação sinérgica entre:
- **Relógio de Membrana (*Voltage Clock*):** Ativação hiperpolarização-dependente da corrente *funny* ($I_f$), despolarizando a célula a partir de $-65\text{ mV}$.
- **Relógio de Cálcio (*Calcium Clock*):** Liberações diastólicas submembranares espontâneas de cálcio pelo retículo sarcoplasmático via receptores de rianodina (RyR), ativando o trocador eletrogênico sódio-cálcio ($I_{NCX}$) que acelera a despolarização até o limiar de disparo dos canais de cálcio do tipo T ($I_{Ca,T}$) e do tipo L ($I_{Ca,L}$).
- **Modulação Autonômica:** Sensibilidade direta à estimulação de receptores adrenérgicos $\beta_1$ (aceleração cronotrópica) e colinérgicos muscarínicos $M_2$ via corrente de potássio dependente de acetilcolina ($I_{K,ACh}$).

> [!NOTE] Adaptação Computacional de Estabilidade e Escala Temporal (KokoroSim v2.1+)
> * **Origem Experimental:** Modelo formulado originalmente para cardiomiócitos marcapasso isolados de coelho (*Oryctolagus cuniculus*), cujas taxas cinéticas e equações diferenciais operavam no domínio de **Segundos ($s$)**.
> * **Conversão Estrutural de Tempo:** O motor do KokoroSim aplica a transformação de escala temporal ($dt / 1000$) no loop de Euler/Rush-Larsen, sincronizando o modelo aos tecidos humanos calculados em **Milissegundos ($ms$)** a $dt = 0.01\text{ ms}$.
> * **Piso Eletrolítico Anti-Bifurcação ($[K^+]_o \ge 5.4\text{ mM}$):** O modelo original de Severi foi calibrado exclusivamente na concentração de banho de Tyrode a $5.4\text{ mM}$. Em modelos unicelulares isolados desprovidos de sincício e homeostase tecidual, qualquer redução para $[K^+]_o < 5.2\text{ mM}$ causa bifurcação matemática de Hopf e parada de oscilação artificial. Na fisiologia humana real *in vivo*, o nó SA mantém automatismo contínuo em normocalemia e hipocalemia ($3.5 - 4.5\text{ mM}$). O KokoroSim estabelece o piso de $5.4\text{ mM}$ no cálculo do potencial de Nernst intrínseco, assegurando ritmo sinusal contínuo em qualquer faixa clínica de potássio ($2.0\text{ a }10.0\text{ mM}$).

---

## 2. Músculo Atrial Humano

* **Modelo:** Courtemanche, Ramirez, Nattel (1998)
* **Artigo Original:** *Ionic mechanisms underlying human atrial action potential properties: insights from a mathematical model.* Am J Physiol. 1998;275(1):H301-H321.
* **Repositório CellML:** [PMR Model e/286](https://models.physiomeproject.org/e/286/courtemanche_ramirez_nattel_1998.cellml)
* **Identificadores Científicos:** [PubMed 9688927](https://pubmed.ncbi.nlm.nih.gov/9688927/) | [DOI 10.1152/ajpheart.1998.275.1.H301](https://doi.org/10.1152/ajpheart.1998.275.1.H301)

### Biofísica e Papel Fisiológico
Representa os miócitos atriais contráteis humanos de resposta rápida:
- **Fase 0:** Ascensão ultraveloz mediada por densa corrente rápida de canais de sódio ($I_{Na}$).
- **Platô Curto:** Platô intermediário mantido por $I_{Ca,L}$ e balanceado pelas correntes transitórias para fora ($I_{to}$).
- **Repolarização Atrial Específica:** Dependente da corrente de retificação tardia ultrarrápida ($I_{Kur}$), alvo preferencial de novos fármacos atriais seletivos para fibrilação atrial.
- **Eletrocardiograma:** A despolarização da câmara atrial sincronizada gera a **Onda P** do traçado de ECG.

---

## 3. Nó Atrioventricular (AV) — Filtro e Retardo Fisiológico

* **Modelo:** Inada et al. (2009)
* **Artigo Original:** *One-dimensional mathematical model of the atrioventricular node including the atrioventricular ring and bundle of His.* Biophys J. 2009;97(8):2117-2127.
* **Repositório CellML:** [PMR Model e/55](https://models.physiomeproject.org/e/55/inada_hancox_zhang_boyett_2009.cellml)
* **Identificadores Científicos:** [PubMed 19843444](https://pubmed.ncbi.nlm.nih.gov/19843444/) | [DOI 10.1016/j.bpj.2009.06.056](https://doi.org/10.1016/j.bpj.2009.06.056)

### Biofísica e Papel Fisiológico
Atua como filtro de segurança hemodinâmica e retardo cronometrado:
- **Retardo AV:** Permite que a sístole atrial preencha completamente os ventrículos antes da contração ventricular (intervalo PR do ECG).
- **Condução Lenta de Resposta Lenta:** Ausência de canais rápidos de sódio funcionais; a subida de potencial depende exclusivamente de $I_{Ca,L}$, conferindo baixa velocidade de condução.
- **Condução Decremental e Refratariedade:** Em frequências elevadas, os canais de cálcio não se recuperam completamente da inativação, provocando lentificação progressiva ou bloqueio de condução (proteção contra taquicardias supraventriculares com resposta ventricular descontrolada).

> [!NOTE] Adaptações Biofísicas do Quimerismo de Espécies e Condução (KokoroSim v2.1+)
> * **Origem Experimental em Coelho (*Oryctolagus cuniculus*):** O modelo original de Inada et al. (2009) reflete o nó atrioventricular de coelho, dotado de automatismo espontâneo muito acelerado (~150 a 180 BPM). Se transposto sem intervenção para um coração humano, o nó AV competiria ativamente com o nó sinoatrial, deflagrando taquicardia juncional ininterrupta e suprimindo o ritmo sinusal.
> * **Inibição do Automatismo e Conversão em Filtro Condutor:**
>   Para rebaixar a excitabilidade espontânea e forçar o nó AV a atuar estritamente como via de retardo e condução fisiológica, as condutâncias de marcapasso foram atenuadas:
>   - **Corrente *Funny* ($I_f$):** Reduzida para **15%** do valor original (`cell.g_f = 0.001 * 0.15`).
>   - **Corrente de Fuga Basal ($I_b$):** Reduzida para **45%** (`cell.g_b = 0.0012 * 0.45`), ponto ótimo (*sweet-spot*) que mantém a rampa de despolarização lenta pronta para condução decremental sem disparar ritmos ectópicos inadvertidos, preservando o ritmo de escape idioventricular/juncional apenas em bloqueios atrioventriculares totais prolongados.
> * **Conversão de Unidade Temporal:** As taxas diferenciais de Inada (2009) formuladas em **Segundos ($s$)** são integradas dinamicamente com passo corrigido ($dt / 1000$) para coexistir harmoniosamente com a malha ventricular em **Milissegundos ($ms$)**.
> * **Piso Eletrolítico ($[K^+]_o \ge 5.4\text{ mM}$) e Limiares Funcionais:** Aplicação de piso de potássio e recalibração dos limiares de refratariedade funcional nos pingers de condução de $-60\text{ mV}$ para $-45\text{ mV}$ (com disparo de ativação em $\ge -20\text{ mV}$), prevenindo bloqueio AV artificial sob hipocalemia ou isquemia.

---

## 4. Fibras de Purkinje e Feixe de His — Condução Rápida e Marcapasso Terciário

* **Modelo:** Stewart et al. (2009)
* **Artigo Original:** *Mathematical model of the electrical action potential of the human Purkinje cell.* Biophys J. 2009;96(9):3493-3507.
* **Repositório CellML:** [PMR Model e/7e](https://models.physiomeproject.org/e/7e/stewart_aslanidi_noble_noble_boyett_zhang_2009.cellml)
* **Identificadores Científicos:** [PubMed 19413956](https://pubmed.ncbi.nlm.nih.gov/19413956/) | [DOI 10.1016/j.bpj.2009.01.047](https://doi.org/10.1016/j.bpj.2009.01.047)

### Biofísica e Papel Fisiológico
Constitui a rede condutora especializada do sistema His-Purkinje:
- **Alta Velocidade de Propagação:** $dV/dt_{\max} > 400\text{ V/s}$, garantindo sincronia biventricular.
- **Morfologia Característica:** Entalhe precoce acentuado (*notch* na Fase 1) impulsionado por $I_{to1}$ e corrente sustentada para fora ($I_{sus}$).
- **Automatismo de Escape Terciário:** Presença de corrente marcapasso $I_f$ residual, permitindo ritmo de escape idioventricular fisiológico (~25 a 35 BPM) em casos de bloqueio atrioventricular total (BAV de 3º grau).

---

## 5. Músculo Ventricular com Heterogeneidade Transmural

* **Modelo:** ten Tusscher & Panfilov (2006)
* **Artigo Original:** *Alternans and spiral breakup in a human ventricular tissue model.* Am J Physiol Heart Circ Physiol. 2006;291(3):H1088-H1100.
* **Repositório CellML:** [PMR Model e/210](https://models.physiomeproject.org/e/210/tentusscher_panfilov_2006_m.cellml)
* **Identificadores Científicos:** [PubMed 16565318](https://pubmed.ncbi.nlm.nih.gov/16565318/) | [DOI 10.1152/ajpheart.00109.2006](https://doi.org/10.1152/ajpheart.00109.2006)

### Biofísica e Papel Fisiológico
Modela a parede livre ventricular humana diferenciada em 3 camadas eletrofisiologicamente distintas:
1. **Endocárdio:** Baixa densidade de corrente de saída transitória ($I_{to}$), ativado primeiro pela rede subendocárdica de Purkinje.
2. **Célula M (Mesomiocárdio):** Densidade reduzida da corrente lenta de potássio ($I_{Ks}$), gerando o potencial de ação mais longo de toda a parede miocárdica.
3. **Epicárdio:** Densidade elevada de $I_{to}$ e **duração do potencial de ação (APD) significativamente mais curta**.

#### A Gênese Biofísica da Onda T
Como a despolarização avança do **Endocárdio $\to$ Epicárdio**, mas a repolarização ocorre na direção inversa (**Epicárdio $\to$ Endocárdio** devido ao APD mais curto do epicárdio), o vetor elétrico transmural mantém a mesma orientação em ambas as fases. Isso explica por que a **Onda T normal é positiva e concordante com o complexo QRS**.

---

## 6. Fibroblastos Cardíacos e Miofibroblastos

* **Modelo:** MacCannell et al. (2007)
* **Artigo Original:** *A mathematical model of electrotonic interactions between ventricular myocytes and fibroblasts.* Biophys J. 2007;92(11):4121-4132.
* **Repositório CellML:** [PMR Model e/98](https://models.physiomeproject.org/e/98/maccannell_bazzazi_chilton_shibukawa_clark_giles_2007.cellml)
* **Identificadores Científicos:** [PubMed 17351008](https://pubmed.ncbi.nlm.nih.gov/17351008/) | [DOI 10.1529/biophysj.106.101410](https://doi.org/10.1529/biophysj.106.101410)

### Biofísica e Papel Fisiológico
Modela o acoplamento eletrotônico heterocelular via junções comunicantes (*gap junctions* de conexina-43 e conexina-45):
- **Células Não-Excitáveis de Repouso Alto:** Fibroblastos possuem potencial de repouso despolarizado característico entre $-35\text{ mV}$ e $-50\text{ mV}$.
- **Dreno Capacitivo:** Ao se conectarem aos miócitos, drenam corrente durante a Fase 0 (reduzindo a amplitude e a velocidade de ascensão $dV/dt$) e injetam corrente durante a diástole (despolarizando parcialmente o potencial de repouso miocitário).
- **Substrato Arritmogênico:** Na presença de fibrose miocárdica extensa pós-infarto ou senil, o aumento do número de fibroblastos acoplados ($G_{gap}$) causa dispersão espacial da repolarização, alargamento do QRS e predisposição a arritmias ventriculares por reentrada.

> [!NOTE] Adaptação Computacional de Estabilidade (KokoroSim v2.1+)
> **O que foi feito:** Calibração da condutância retificadora de entrada $G_{K1} = 0.04822\text{ nS}$ e potencial inicial $V = -49.6\text{ mV}$, de acordo com a formulação canônica de MacCannell et al. (2007).
> **Por que foi feito:** A condutância anterior superestimada ($0.4822\text{ nS}$) forçava o repouso para $-71.3\text{ mV}$, colando o traçado no rodapé da escala do osciloscópio ($-90\text{ a }+50\text{ mV}$) e impedindo a visualização nítida do dreno capacitivo.
> **Resultado esperado:** O traçado do fibroblasto repousa visivelmente centralizado no osciloscópio em $\approx -50\text{ mV}$, desenvolvendo deflexões eletrotônicas acopladas límpidas ($> 50\text{ mV}$ de amplitude) em sincronia com o miócito ventricular conforme o slider de fibrose é elevado.

---

## 7. Acoplamento Eletromecânico e Hemodinâmica (Diagrama de Wiggers)

* **Modelos:** 
  - Elastância Ventricular Variável no Tempo: Suga & Sagawa (1974)
  - Circulação Arterial Windkessel de 3 Elementos: Westerhof et al. (2009)
* **Identificadores Científicos:** 
  - Suga H, Sagawa K. *Instantaneous pressure-volume relationships and their ratio in the excised, supported canine left ventricle.* Circ Res. 1974;35(1):117-126. [PubMed 4841253](https://pubmed.ncbi.nlm.nih.gov/4841253/)
  - Westerhof N, Lankhaar JW, Westerhof BE. *The arterial Windkessel.* Med Biol Eng Comput. 2009;47(2):131-141. [PubMed 19194725](https://pubmed.ncbi.nlm.nih.gov/19194725/) | [DOI 10.1007/s11517-008-0359-2](https://doi.org/10.1007/s11517-008-0359-2)

### Biofísica e Dinâmica Ventricular
1. **Acoplamento Excitação-Contração:** A concentração intracelular transitória de cálcio livre ($[Ca^{2+}]_i$) é mapeada dinamicamente na ativação miofilamentar e na função de elastância $E(t)$.
2. **Pressão Ventricular Esquerda ($LVP$):** Calculada em tempo real pela relação pressão-volume dependente da elastância instantânea:
   $$P_{lv}(t) = E(t) \cdot (V_{lv}(t) - V_0) + P_{edpv}(V_{lv})$$
3. **Pressão Aórtica ($AoP$) e Incisura Dicrótica:** Resolvida pelo modelo arterial de Windkessel de 3 elementos (resistência periférica total $R_p$, complacência arterial $C_a$ e impedância característica da aorta $Z_c$). O fechamento abrupto das cúspides aórticas gera a incisura dicrótica realista.
4. **Pressão Atrial Esquerda ($LAP$) e Ondas de Wiggers:**
   - **Onda $a$:** Contração atrial ativa pré-sistólica sincronizada com o final da onda P do ECG.
   - **Onda $c$:** Abaulamento isovolumétrico da valva mitral em direção ao átrio no início da contração ventricular.
   - **Descenso $x$:** Tração do assoalho atrial pela sístole ventricular gerando sucção venosa pulmonar.
   - **Onda $v$:** Enchimento atrial passivo com valva mitral fechada durante a ejeção ventricular.
   - **Descenso $y$:** Abertura da mitral e esvaziamento diastólico rápido para o ventrículo.
5. **Volumes Ventriculares e Métricas Sistólicas:**
   - **Volume Diastólico Final ($VDF$):** Volume ventricular no instante do fechamento da mitral (B1, ~120 mL).
   - **Volume Sistólico Final ($VSF$):** Volume ventricular mínimo ao término da ejeção aórtica (B2, ~50 mL).
   - **Volume Sistólico ($VS$):** $VS = VDF - VSF$ (~70 mL).
   - **Fração de Ejeção ($FE$):** $FE = \frac{VS}{VDF} \times 100\%$ (~58%).
   - **Débito Cardíaco ($DC$):** $DC = \frac{FC \times VS}{1000}$ (~5.0 L/min).
6. **Alça Pressão-Volume 2D ($P \times V$):**
   - **Fase I (Enchimento Diastólico):** O ventrículo se expande de $VSF$ para $VDF$ ao longo da curva de complacência passiva ($EDPVR$).
   - **Fase II (Contração Isovolumétrica):** Pressão sobe verticalmente de $LAP$ até a pressão diastólica aórtica com ambas as valvas fechadas ($V = VDF$).
   - **Fase III (Ejeção Ventricular):** Valva aórtica aberta; o sangue é ejetado na aorta, traçando a trajetória superior até atingir a reta elastância de fim de sístole ($ESPVR$).
   - **Fase IV (Relaxamento Isovolumétrico):** Valva aórtica fecha; a pressão desaba bruscamente com ambas as valvas fechadas ($V = VSF$).
   - **Área da Alça:** Representa o Trabalho Sistólico Efetivo (*Stroke Work* $\approx \oint P \, dV$).
7. **Bioacústica das Bulhas Cardíacas:**
   - **Primeira Bulha (B1):** Disparada acusticamente pelo fechamento de alta energia da valva mitral no início da sístole isovolumétrica ($LVP > LAP$).
   - **Segunda Bulha (B2):** Disparada pelo fechamento das cúspides da valva aórtica no início da diástole isovolumétrica ($AoP > LVP$).
8. **Fisiopatologia das Valvopatias:**
   - **Estenose Aórtica:** Obstrução da via de saída gerando gradiente sistólico patológico ($LVP \gg AoP$, podendo superar 180 mmHg) com estreitamento da amplitude aórtica e sobrecarga concêntrica (efeito Anrep).
   - **Insuficiência Aórtica:** Refluxo diastólico patológico aorta $\to$ VE, gerando colapso da pressão diastólica aórtica ($< 50\text{ mmHg}$), pressão de pulso alargada (pulso em martelo d'água / Corrigan) e sobrecarga volumétrica ventricular ($VDF > 140\text{ mL}$).
   - **Estenose Mitral:** Resistência aumentada ao fluxo átrio $\to$ ventrículo na diástole, gerando hipertensão atrial esquerda sustentada ($LAP > 18\text{ mmHg}$) e atraso no enchimento ventricular.
   - **Insuficiência Mitral:** Regurgitação sistólica do VE para o átrio esquerdo através da valva incompetente, deformando a curva de LAP com uma onda $v$ patológica gigante ($> 25\text{ mmHg}$).

---

## 8. Quimerismo Biofísico e Calibrações Computacionais (Coelho → Humano)

A construção de um simulador cardiovascular multi-escala totalmente interativo impôs a superação de um dos maiores desafios da modelagem biofísica computacional: a **integração coerente de modelos padrão-ouro oriundos de espécies e escalas experimentais distintas**.

Na literatura eletrofisiológica internacional, a maior parte dos dados experimentais detalhados de pinçamento de membrana (*patch-clamp*) unicelular para tecidos marcapasso isolados advém de preparações em coelho (*Oryctolagus cuniculus*), como os modelos de Severi et al. (2012) para o Nó SA e Inada et al. (2009) para o Nó AV. Por outro lado, os miócitos contráteis (Courtemanche 1998, ten Tusscher 2006) e as fibras de Purkinje (Stewart 2009) são de origem humana.

Para harmonizar esses modelos em um coração virtual funcional e fidedigno à clínica humana, o KokoroSim implementou as seguintes adaptações:

### 1. Compatibilização de Domínios Temporais (Segundos vs. Milissegundos)
* **O Problema:** Os modelos matemáticos CellML de Severi (2012) e Inada (2009) foram formulados com equações diferenciais e taxas cinéticas expressas em **Segundos ($s$) e $s^{-1}$**. Já Courtemanche (1998), Stewart (2009) e ten Tusscher (2006) operavam em **Milissegundos ($ms$)**.
* **A Solução:** O motor interno do KokoroSim aplica uma conversão dimensional estrita da variável temporal ($dt / 1000$) no loop de integração para Severi e Inada, assegurando que todas as equações evoluam com exatidão sincronizada a um passo temporal unificado de $dt = 0.01\text{ ms}$ (60 FPS contínuos).

### 2. Inibição do Automatismo e Eliminação da Competição Juncional do Nó AV
* **O Problema:** O nó AV de coelho (Inada 2009) possui automatismo basal nativo extremamente rápido (~150 a 180 BPM). Em um coração humano simulado, essa alta frequência intrínseca competiria ativamente com o nó sinoatrial, causando taquicardia juncional ininterrupta e impedindo o marcapasso sinusal de assumir o comando cronotrópico.
* **A Solução:**
  - Redução da condutância da corrente *funny* do Nó AV para **15%** do valor original (`cell.g_f = 0.001 * 0.15`).
  - Redução da corrente de fuga basal para **45%** (`cell.g_b = 0.0012 * 0.45`).
* **Resultado:** O Nó AV foi transformado estritamente em uma **via de condução lenta com retardo fisiológico e filtro decremental**, respondendo fielmente aos disparos atriais e expressando ritmo de escape idioventricular/juncional tardio apenas quando ocorrem bloqueios AV totais prolongados.

### 3. Blindagem Eletrolítica contra Bifurcações Matemáticas de Hopf ($[K^+]_o \ge 5.4\text{ mM}$)
* **O Problema:** Os modelos unicelulares de Severi e Inada foram calibrados estritamente na concentração de banho de Tyrode de $5.4\text{ mM}$. Em modelos unicelulares isolados desprovidos de sincício tecidual, qualquer redução para $[K^+]_o < 5.2\text{ mM}$ provoca uma bifurcação matemática de Hopf, travando abruptamente as oscilações e gerando assistolia artificial.
* **A Solução:** Estabeleceu-se um piso fisiológico de potássio ($[K^+]_o \ge 5.4\text{ mM}$) especificamente para os cálculos de reversão de Nernst dos canais intrínsecos nodais, permitindo ao usuário manipular hipocalemia severa ($2.0\text{ a }3.5\text{ mM}$) na interface sem paralisar o automatismo cardíaco.

### 4. Integração Exponencial Híbrida de Rush-Larsen e Micro-stepping em Canais $I_{Na}$
* **O Problema:** Células rápidas humanas (átrio, Purkinje e ventrículo) apresentam velocidade de ascensão da Fase 0 que ultrapassa $dV/dt > 400\text{ V/s}$ com constantes de tempo dos canais de sódio extremamente curtas ($\tau_m \approx 0.0008\text{ ms}$). Métodos explícitos tradicionais (como Euler simples) divergem para infinito (`NaN`) caso o passo não seja microscopicamente pequeno.
* **A Solução:** Aplicação do método numérico híbrido de **Rush-Larsen (1978)**, que integra analiticamente as comportas iônicas na forma exponencial e desacopla a rigidez numérica (*stiffness*), garantindo estabilidade matemática absoluta com consumo de CPU inferior a 5-10%.

### 5. Calibração do Potencial de Repouso do Fibroblasto (MacCannell 2007)
* **O Problema:** Na importação direta da formulação, uma superestimativa da condutância retificadora de entrada $G_{K1}$ forçava o repouso para $-71.3\text{ mV}$, colando o traçado no rodapé da escala do osciloscópio.
* **A Solução:** Ajuste para o valor canônico $G_{K1} = 0.04822\text{ nS}$, estabelecendo o potencial de repouso fisiológico despolarizado característico em $-49.6\text{ mV}$ e permitindo a visualização nítida das deflexões eletrotônicas acopladas aos cardiomiócitos.

---

## 9. Referências Científicas Complementares (Além do CellML)

Além dos artigos seminais dos modelos iônicos celulares registrados no consórcio Physiome/CellML, o KokoroSim fundamenta-se nas seguintes obras da biofísica, hemodinâmica, métodos numéricos e bioeletromagnetismo:

### Métodos Numéricos e Computacionais
* **Rush S, Larsen H.** *A practical algorithm for solving dynamic membrane equations.* **IEEE Transactions on Biomedical Engineering.** 1978;BME-25(4):389-392. [DOI: 10.1109/TBME.1978.326270](https://doi.org/10.1109/TBME.1978.326270)
* **Press WH, Teukolsky SA, Vetterling WT, Flannery BP.** *Numerical Recipes: The Art of Scientific Computing.* 3rd ed. Cambridge University Press; 2007.

### Hemodinâmica, Elastância Ventricular e Dinâmica Cardiovascular
* **Suga H, Sagawa K.** *Instantaneous pressure-volume relationships and their ratio in the excised, supported canine left ventricle.* **Circulation Research.** 1974;35(1):117-126. [PubMed: 4841253](https://pubmed.ncbi.nlm.nih.gov/4841253/)
* **Sagawa K, Maughan L, Suga H, Sunagawa K.** *Cardiac Contraction and the Left Ventricle: A Natural Approach to Left Ventricular Function.* Oxford University Press; 1988.
* **Westerhof N, Lankhaar JW, Westerhof BE.** *The arterial Windkessel.* **Medical & Biological Engineering & Computing.** 2009;47(2):131-141. [PubMed: 19194725](https://pubmed.ncbi.nlm.nih.gov/19194725/) | [DOI: 10.1007/s11517-008-0359-2](https://doi.org/10.1007/s11517-008-0359-2)
* **Wiggers CJ.** *The Pressure Pulses in the Cardiovascular System.* Longmans, Green and Co.; 1928.
* **Sunagawa K, Maughan WL, Burkhoff D, Sagawa K.** *Left ventricular interaction with arterial system in dogs: conceptual framework.* **American Journal of Physiology.** 1983;245(5):H773-H780. [PubMed: 6638195](https://pubmed.ncbi.nlm.nih.gov/6638195/)

### Bioeletromagnetismo, Eletrocardiografia e Teoria Dipolar
* **Wilson FN, Johnston FD, Macleod AG, Barker PS.** *Electrocardiograms that represent the potential variations of a single electrode.* **American Heart Journal.** 1934;9(4):447-458. [DOI: 10.1016/S0002-8703(34)90382-7](https://doi.org/10.1016/S0002-8703(34)90382-7)
* **Malmivuo J, Plonsey R.** *Bioelectromagnetism: Principles and Applications of Bioelectric and Biomagnetic Fields.* Oxford University Press; 1995.
* **Plonsey R, Barr RC.** *Bioelectricity: A Quantitative Approach.* 3rd ed. Springer; 2007.

### Eletrofisiologia Celular, Acoplamento Miofibroblástico e Tecidual
* **Kohl P, Camelliti P, Burton FL, Smith GL.** *Electrical coupling of fibroblasts and myocytes: relevance for cardiac conduction.* **Journal of Electrocardiology.** 2005;38(4 Suppl):45-50. [PubMed: 16226075](https://pubmed.ncbi.nlm.nih.gov/16226075/)
* **Kohl P, Gourdie RG.** *Fibroblast-myocyte electrotonic coupling: Does it occur in native mammalian heart?* **Circulation Research.** 2014;114(4):596-598. [PubMed: 24526670](https://pubmed.ncbi.nlm.nih.gov/24526670/)

### Fisiologia Autonômica e Farmacologia Cardíaca
* **Levy MN.** *Sympathetic-parasympathetic interactions in the heart.* **Circulation Research.** 1971;29(5):437-445. [PubMed: 4940562](https://pubmed.ncbi.nlm.nih.gov/4940562/)
* **Katz AM.** *Physiology of the Heart.* 5th ed. Lippincott Williams & Wilkins; 2011.
