# SimCardio: Simulador Eletrofisiológico Cardíaco em Tempo Real

O **SimCardio** é um projeto open-source que traz simulações matemáticas complexas da eletrofisiologia celular cardíaca diretamente para o navegador web. Ao invés de usar animações pré-programadas, o SimCardio resolve equações diferenciais ordinárias (EDOs) em tempo real (mais de 100 mil passos de integração por segundo) para simular o comportamento exato dos íons, correntes e canais de quatro tecidos vitais do coração humano.

## 🧬 Os Modelos Matemáticos

O projeto unifica quatro dos mais respeitados modelos matemáticos da eletrofisiologia cardíaca (disponíveis publicamente através do repositório [Physiome Project / CellML](https://models.physiomeproject.org/)):

1. **Nó Sinoatrial (SA) - O Marca-passo (Coelho):**
   - **Modelo:** Severi et al. (2012)
   - **Referência:** Severi S, Fantini M, Charawi LA, DiFrancesco D. *An updated computational model of rabbit sinoatrial action potential to investigate the mechanisms of heart rate modulation.* J Physiol. 2012.
   - **Papel:** Gera o ritmo automático do coração. Baseado na eletrofisiologia de coelhos (comum em estudos nodais).

2. **Músculo Atrial (Humano):**
   - **Modelo:** Courtemanche, Ramirez, Nattel (1998)
   - **Referência:** Courtemanche M, Ramirez RJ, Nattel S. *Ionic mechanisms underlying human atrial action potential properties: insights from a mathematical model.* Am J Physiol. 1998.
   - **Papel:** Células de resposta rápida responsáveis pela contração atrial, perfeitamente validadas para humanos.

3. **Nó Atrioventricular (AV) - A Estação de Retardo (Coelho):**
   - **Modelo:** Inada et al. (2009)
   - **Referência:** Inada S, et al. *One-dimensional mathematical model of the atrioventricular node...* Biophys J. 2009.
   - **Papel:** Retarda o sinal elétrico. Como é dificílimo isolar células nodais humanas, usa-se a base do coelho, que foi cuidadosamente "domada" ("Tuning") por nós para escalar com o coração humano.

4. **Músculo Ventricular (Humano):**
   - **Modelo:** ten Tusscher & Panfilov (2006)
   - **Referência:** ten Tusscher KHWJ, Panfilov AV. *Alternans and spiral breakup in a human ventricular tissue model.* Am J Physiol Heart Circ Physiol. 2006.
   - **Papel:** Motor principal de bombeamento (humano). Caracterizado por um longo platô dependente de Cálcio.

## ⚙️ Adaptações e Engenharia do Motor

Para permitir que esses quatro modelos colossais rodassem de forma estável, simultânea e responsiva em um navegador web, realizamos profundas alterações de engenharia:

* **Integração por Forward Euler com Domínios Híbridos de Tempo:**
  Os modelos originais foram convertidos de arquivos `.cellml` para TypeScript bruto. Para garantir 60 FPS na renderização gráfica sem travar a interface, o cálculo matemático roda isolado em uma thread de `Web Worker`. Utilizamos o método de Forward Euler com um passo base ($dt$) hiper-rígido de **0.01 milissegundos**.
  Um dos maiores desafios resolvidos foi o conflito de unidades de tempo na literatura original:
  * **Nó SA (Severi) e Nó AV (Inada)** foram matematicamente construídos assumindo que a variável tempo e as taxas estivessem em **Segundos**. O motor converte o tempo local (`dt / 1000`) para integrá-los de forma estável.
  * **Átrio (Courtemanche) e Ventrículo (ten Tusscher)** foram construídos em **Milissegundos**.
  
* **Micro-stepping para Células Rápidas:**
  Modelos de trabalho (Átrio e Ventrículo) possuem canais rápidos de Sódio ($I_{Na}$) cuja derivada ($\frac{dV}{dt}$) cresce violentamente na Fase 0 (mais de 400V/s). Mesmo um $dt$ de 0.01ms causa divergência matemática (NaN). Para resolver isso, implementamos um loop de **micro-stepping interno de 10 passos** por ciclo apenas para Courtemanche e ten Tusscher. Isso reduz o sub-DT efetivo para **0.001 milissegundos**, prevenindo explosões da equação diferencial durante a despolarização sem penalizar o Nó SA e AV.

* **Sistema de Condução por Gatilhos ("Pingers"):**
  Simular o coração em 3D exigiria supercomputadores. Nossa abordagem foca no *Zero-Dimensional (0D) acoplado no tempo*. Simulamos 4 células isoladas. Quando a voltagem do Nó SA passa de -30mV, um temporizador é iniciado e, 40ms depois, um choque (corrente de estímulo de +50mV) é injetado artificialmente na equação do Átrio, e assim sucessivamente. Isso recria a fidedignidade temporal da condução cardíaca (intervalos PR e QRS) de forma muito barata computacionalmente.

* **Tuning ("Domando" a Quimera Coelho-Humano):**
  Como estamos conectando corações de espécies diferentes (SA/AV de coelho com Átrio/Ventriculo humano), tivemos que realizar uma calibração cruzada:
  - **Supressão do Ritmo do Nó AV:** Para que o Nó AV Inada (que em coelhos bate bem mais rápido) funcionasse obedientemente como estação de repasse e não competisse com o SA, as suas correntes intrínsecas de marca-passo foram suprimidas em inicialização matemática cirúrgica (`I_f` mantida a 15% e Corrente de Fuga a 45%). Ele só dispara se for acordado, ou se o Nó SA falhar (Ritmo de Escape Juncional).
  - **Potenciais de Nernst Dinâmicos:** Os modelos originais usam concentrações externas fixas de íons. Nós sequestramos as variáveis de Nernst (ex: $E_{Na}$, $E_{K}$) e as recalculamos a cada frame baseando-se nos sliders do usuário.

## 🎛️ Controles e Farmacologia Dinâmica

Diferente de simulações em vídeo, o SimCardio permite que o usuário brinque de "Deus" (ou de médico intervencionista), alterando a farmacologia celular em tempo real. Veja o que cada slider faz por debaixo dos panos:

### 1. Íons e Eletrólitos
A base da eletrofisiologia. Estes sliders alteram o gradiente eletroquímico celular modificando a Força Motriz das correntes de íons.
* **Potássio [K+]_o (2.0 a 8.5 mEq/L):** O principal determinante do potencial de repouso ($V_{rest}$).
  * *Efeito:* Valores baixos (hipocalemia) causam hiperpolarização severa e risco de arritmias. Valores muito altos (8.5) tornam as células tão despolarizadas no repouso que as comportas de inativação dos canais de Sódio travam fechadas. O resultado é a **Parada Cardíaca em Diástole** absoluta (todas as ondas se tornam linhas retas).
* **Cálcio [Ca2+]_o (1.0 a 3.5 mmol/L):** Afeta a corrente de cálcio L-type ($I_{CaL}$).
  * *Efeito:* Hipocalcemia prolonga o platô (aumenta o intervalo QT), enquanto a hipercalcemia encurta o platô (encurta o QT e enrijece o batimento).
* **Sódio [Na+]_o (125 a 155 mEq/L):** O combustível das células rápidas (Átrio e Ventrículo).
  * *Efeito:* Reduzir o Sódio diminui drasticamente a amplitude do Potencial de Ação, alargando as ondas. Nos limites mínimos, causa um **Bloqueio Sinoatrial (SA Block)**: o Nó SA continua tentando comandar o ritmo, mas o átrio "falha" em gerar o potencial de ação e fica silente.

### 2. Sistema Nervoso Autônomo
* **Tônus Simpático (0-100%):** Simula a ação da Adrenalina/Noradrenalina. Nós multiplicamos a condutância dos canais $I_f$ (relógio natural) e $I_{CaL}$. Resultado visual: Taquicardia severa e potenciais de ação muito altos e curtos (pela aceleração da bomba SERCA).
* **Tônus Parassimpático (0-100%):** Simula o nervo Vago e a Acetilcolina. Ativa a corrente inibidora $I_{K,ACh}$, puxando a célula nodal para baixo (hiperpolarização). Resultado: Bradicardia imediata, ou até parada temporária do SA.

### 3. Fármacos Antiarrítmicos
* **Lidocaína (Bloqueador de Na+):** Anestésico Classe I. Em nosso código, injetado como um multiplicador `(1.0 - porcentagem)` reduzindo fisicamente a condutância máxima dos canais $g_{Na}$. Achata a Fase 0 de despolarização, alargando muito os complexos sem alterar o repouso.
* **Amiodarona (Bloqueador de K+):** Antiarrítmico Classe III. Inibe a saída de Potássio ($I_{Kr}$, $I_{Ks}$), essencial para a Fase 3 da repolarização. O resultado visível é um platô colossalmente arrastado e intervalo QT gigante.
* **Verapamil (Bloqueador de Ca2+):** Classe IV. Corta as comportas do Cálcio L-type ($I_{CaL}$). Achata o platô do ventrículo imediatamente e inibe profundamente a rampa do Nó AV, causando bloqueios de condução atrioventriculares de variados graus.
* **Digoxina (Inibidor Na+/K+):** Um veneno terapêutico. Interrompe a Bomba de Sódio/Potássio ATP-ase ($I_{NaK}$). Impede o reequilíbrio final da célula, causando acúmulo tóxico progressivo de sódio e cálcio intracelular.

### 4. Isquemia (Infarto)
* **Nível de Isquemia:** Simula o temido fecho de uma artéria. Na matemática, isquemia aciona um canal específico que só abre sem oxigênio ($I_{K,ATP}$), sugando Potássio para fora da célula de forma caótica. Ao mesmo tempo, "vaza" potássio externo, subindo o $K^+_o$ local em até +6 mEq/L. Visualmente, você verá a destruição aguda e antecipada do platô ventricular e um colapso brutal na eficácia do tecido.

---

O SimCardio une Front-End visual de ponta com Biofísica Médica estrita, resultando numa plataforma incrivelmente robusta e responsiva. Divirta-se brincando de cardiologista molecular!
