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

> [!NOTE] Adaptação Computacional de Estabilidade (KokoroSim v2.1+)
> **O que foi feito:** Definição de piso fisiológico $[K^+]_o \ge 5.4\text{ mM}$ para a formulação nodal de Severi (2012).
> **Por que foi feito:** O modelo original de Severi foi calibrado exclusivamente em banho experimental de Tyrode a $5.4\text{ mM}$. Em modelos unicelulares isolados sem sincício, reduções para $K^+ < 5.2\text{ mM}$ geram bifurcação matemática de Hopf e parada de oscilação artificial. Na fisiologia humana real *in vivo*, o nó SA não expressa $I_{K1}$ e mantém automatismo marcapasso contínuo mesmo em normocalemia baixa ($3.5 - 4.5\text{ mM}$).
> **Resultado esperado:** Automatismo sinusal preservado em toda a faixa clínica de potássio ($2.0\text{ a }10.0\text{ mM}$) sem interrupção do ritmo cardíaco.

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

> [!NOTE] Adaptação Computacional de Estabilidade (KokoroSim v2.1+)
> **O que foi feito:** Aplicação de piso $[K^+]_o \ge 5.4\text{ mM}$ nas correntes nodais de Inada (2009) e transição dos limiares de refratariedade funcional nos pingers de condução de $-60\text{ mV}$ para $-45\text{ mV}$ (e disparo AV em $\ge -20\text{ mV}$).
> **Por que foi feito:** Previne bloqueio artificial de condução por hiperpolarização em hipocalemia e impede o aprisionamento de refratariedade quando hipercalemia ou isquemia mantêm o repouso despolarizado em $-55\text{ mV}$.
> **Resultado esperado:** Condução fisiológica atrioventricular íntegra e contínua sob qualquer intervenção farmacológica ou eletrolítica na interface.

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
- **Células Não-Excitáveis de Repouso Alto:** Fibroblastos possuem potencial de repouso entre $-35\text{ mV}$ e $-45\text{ mV}$.
- **Dreno Capacitivo:** Ao se conectarem aos miócitos, drenam corrente durante a Fase 0 (reduzindo a amplitude e a velocidade de ascensão $dV/dt$) e injetam corrente durante a diástole (despolarizando parcialmente o potencial de repouso miocitário).
- **Substrato Arritmogênico:** Na presença de fibrose miocárdica extensa pós-infarto ou senil, o aumento do número de fibroblastos acoplados ($G_{gap}$) causa dispersão espacial da repolarização, alargamento do QRS e predisposição a arritmias ventriculares por reentrada.

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
2. **Pressão Ventricular Esquerda ($LVP$):** Calculada em tempo real pela relação pressão-volume dependente da elastância instantânea.
3. **Pressão Aórtica ($AoP$) e Incisura Dicrótica:** Resolvida pelo modelo arterial de Windkessel de 3 elementos (resistência periférica total $R_p$, complacência arterial $C_a$ e impedância característica da aorta $Z_c$). O fechamento abrupto das cúspides aórticas gera a incisura dicrótica realista.
4. **Bioacústica das Bulhas Cardíacas:**
   - **Primeira Bulha (B1):** Disparada acusticamente pelo fechamento de alta energia da valva mitral no início da sístole isovolumétrica ($LVP > LAP$).
   - **Segunda Bulha (B2):** Disparada pelo fechamento das cúspides da valva aórtica no início da diástole isovolumétrica ($AoP > LVP$).
