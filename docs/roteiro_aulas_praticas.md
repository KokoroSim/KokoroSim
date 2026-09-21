# Guia Didático e Roteiro de Aulas Práticas
### Manual de Experimentos Virtuais para Cursos de Medicina, Biomedicina e Ciências da Saúde

O **KokoroSim** foi planejado para ser utilizado diretamente em computadores, tablets ou smartphones em laboratórios de ensino e salas de aula das universidades brasileiras. Este guia apresenta **4 experimentos práticos completos**, estruturados com objetivos de aprendizagem, fundamentação teórica, passo a passo procedural no simulador e questões norteadoras para discussão em grupo ou elaboração de relatório.

---

## 📋 Sumário dos Experimentos
1. [Experimento 1: Distúrbios Eletrolíticos e Parada Cardíaca por Hipercalemia](#experimento-1-distúrbios-eletrolíticos-e-parada-cardíaca-por-hipercalemia)
2. [Experimento 2: Bloqueio Atrioventricular e Escape Idioventricular por Verapamil](#experimento-2-bloqueio-atrioventricular-e-escape-idioventricular-por-verapamil)
3. [Experimento 3: Modulação Autonômica (Simpático vs. Parassimpático) e Hemodinâmica](#experimento-3-modulação-autonômica-simpático-vs-parassimpático-e-hemodinâmica)
4. [Experimento 4: Fibrose Miocárdica, Acoplamento Miócito-Fibroblasto e Arritmogênese](#experimento-4-fibrose-miocárdica-acoplamento-miócito-fibroblasto-e-arritmogênese)

---

## Experimento 1: Distúrbios Eletrolíticos e Parada Cardíaca por Hipercalemia

### 🎯 Objetivos de Aprendizagem
* Compreender a dependência do potencial de repouso ($V_{rest}$) em relação à concentração extracelular de potássio ($[K^+]_o$) pela Equação de Nernst.
* Identificar as repercussões eletrocardiográficas da hipercalemia progressiva (ondas T apiculadas, alargamento de QRS, achatamento da onda P e assistolia diastólica).
* Comparar as curvas basais e patológicas através do recurso **Onda Fantasma**.

### 🔬 Procedimento Prático
1. Clique no botão **`☢ RESETAR`** para restaurar as condições basais fisiológicas ($[K^+]_o = 5.4\text{ mEq/L}$, FC $\approx 75\text{ BPM}$).
2. Abra o Accordion *0. Visualização* e clique no botão **`📸 Capturar`** para congelar a curva basal de referência. Acompanhe a transição de status (`⏳ Aguardando Nó SA...` $\to$ `🔴 Gravando Ciclo...` $\to$ `📸 Capturar`).
3. Verifique que a **Onda Fantasma** translúcida se alinha perfeitamente ao traçado ativo em repouso.
4. Abra o Accordion *1. Íons e Eletrólitos* e altere o slider de **Potássio $[K^+]_o$**:
   * **Etapa A (Normocalemia de transição):** Reduza para $4.0\text{ mEq/L}$. Observe o valor de $V_{rest}$ no HUD superior.
   * **Etapa B (Hipercalemia Moderada):** Eleve $[K^+]_o$ gradativamente para $6.8\text{ mEq/L}$. Observe a elevação da voltagem de repouso, a alteração de amplitude da onda T no canal de ECG e a perda de amplitude da Fase 0 no potencial de ação ventricular.
   * **Etapa C (Hipercalemia Grave):** Eleve $[K^+]_o$ para $8.5\text{ mEq/L}$. Observe a inativação dos canais de sódio por despolarização sustentada e a parada cardíaca em diástole.
5. Utilize o botão **`[ ⏸ CONGELAR ]`** para paralisar a imagem estática e analisar os traçados com calma.

### ❓ Questões para Discussão
1. Por que uma elevação extracelular de potássio despolariza a membrana celular em vez de hiperpolarizá-la?
2. Explique o mecanismo eletrofisiológico da inativação em estado estacionário dos canais rápidos de sódio ($I_{Na}$) e por que isso leva à perda de excitabilidade ventricular.
3. Compare o traçado ativo com a onda fantasma basal: o que aconteceu com a morfologia da onda T no ECG?

---

## Experimento 2: Bloqueio Atrioventricular e Escape Idioventricular por Verapamil

### 🎯 Objetivos de Aprendizagem
* Analisar a farmacologia dos bloqueadores de canais de cálcio (Classe IV de Vaughan Williams) no tecido nodal lento.
* Diferenciar Bloqueio AV de 1º grau (prolongamento do intervalo PR), 2º grau e 3º grau (dissociação AV total).
* Observar o automatismo terciário de escape assumido pelas fibras de Purkinje na vigência de BAV total.

### 🔬 Procedimento Prático
1. Clique em **`☢ RESETAR`**.
2. No canal de Potenciais de Ação (Canal 1), marque as caixas de seleção: **Nó SA (vermelho)**, **Átrio (azul)**, **Nó AV (amarelo)** e **Purkinje (laranja)**.
3. No Accordion *5. Monitorização & Áudio*, ative o **Bip de Monitor (UTI)** e a **Ausculta de Bulhas (B1/B2)** para correlacionar o ritmo auditivo com o traçado visual.
4. Capture a referência clicando em **`📸 Capturar`**.
5. Abra o Accordion *3. Fármacos Antiarrítmicos* e aumente o slider **Bloq. Ca2+ (Verapamil)**:
   * **Etapa A (BAV de 1º grau):** Eleve para $35\%$. Acompanhe o valor de **PR** no HUD superior alargando de ~160 ms para > 220 ms. Note o atraso entre o disparo atrial e a despolarização do feixe de Purkinje.
   * **Etapa B (BAV de 2º grau):** Eleve para $65\%$. Observe falhas intermitentes de condução ventricular.
   * **Etapa C (BAV de 3º grau / Bloqueio Total):** Eleve para $90\%-100\%$. O nó AV deixa de conduzir os estímulos sinusais. Observe que o nó SA continua batendo em frequência fisiológica, enquanto as fibras de Purkinje assumem o comando ventricular em ritmo de escape idioventricular (~28 a 35 BPM).

### ❓ Questões para Discussão
1. Qual a diferença iônica fundamental entre a Fase 0 do Nó Sinoatrial/Atrioventricular e a Fase 0 do Músculo Ventricular?
2. Por que o Verapamil afeta drasticamente a condução no Nó AV, mas tem efeito muito menor sobre a velocidade de ascensão da Fase 0 no ventrículo?
3. Como as bulhas B1 e B2 se comportam durante a dissociação atrioventricular completa?

---

## Experimento 3: Modulação Autonômica e Hemodinâmica (Wiggers)

### 🎯 Objetivos de Aprendizagem
* Estudar os efeitos cronotrópico, dromotrópico e inotrópico da estimulação simpática ($\beta_1$) e parassimpática ($M_2$).
* Analisar as curvas de Pressão Ventricular Esquerda ($LVP$) e Pressão Aórtica ($AoP$) no Diagrama de Wiggers.
* Compreender a correlação acústica do fechamento da valva mitral (B1) e valva aórtica (B2).

### 🔬 Procedimento Prático
1. Clique em **`☢ RESETAR`**.
2. No Accordion *5. Monitorização & Áudio*, ative a **Ausculta de Bulhas (B1/B2)**. Note as linhas verticais verdes (B1) e corais (B2) cortando simultaneamente os 4 osciloscópios.
3. Observe o Canal 4 (Hemodinâmica):
   * Identifique o pico da sístole ventricular ($LVP$, curva ciano).
   * Identifique a incisura dicrótica na pressão aórtica ($AoP$, curva coral).
4. No Accordion *0. Visualização*, mude o modo para **Gatilho Automático (Auto a cada Batimento)** para sobrepor os ciclos no mesmo eixo temporal de Fase 0.
5. Capture a referência basal com **`📸 Capturar`**.
6. Abra o Accordion *2. Sistema Nervoso Autônomo*:
   * **Estimulação Simpática:** Eleve o **Tônus Simpático** para $70\%$. Observe o aumento expressivo de BPM, o encurtamento do intervalo PR e QT, a elevação da pressão sistólica ventricular ($LVP$) para > 130 mmHg e o aumento da amplitude do transiente de cálcio (Canal 3).
   * **Estimulação Parassimpática:** Zere o simpático e eleve o **Tônus Parassimpático** para $60\%$. Observe a bradicardia sinusal acentuada, a queda da força contrátil e o prolongamento do ciclo de repouso, com a onda fantasma basal destacando com nitidez o atraso da onda atual.

### ❓ Questões para Discussão
1. Quais correntes iônicas do nó sinoatrial são moduladas pela estimulação dos receptores $\beta_1$ adrenérgicos?
2. O que determina fisicamente a ocorrência da incisura dicrótica na raiz da aorta e em que momento do ciclo cardíaco a segunda bulha (B2) é auscultada?
3. Por que o aumento do influxo de cálcio ($I_{Ca,L}$) eleva a pressão intraventricular máxima?

---

## Experimento 4: Fibrose Miocárdica, Acoplamento Miócito-Fibroblasto e Arritmogênese

### 🎯 Objetivos de Aprendizagem
* Compreender o papel dos fibroblastos não-excitáveis na eletrofisiologia ventricular humana.
* Analisar o efeito de dreno capacitivo exercido pelas junções comunicantes (*gap junctions*) miócito-fibroblasto.
* Correlacionar o grau de fibrose tecidual com o alargamento do QRS e lentificação da condução transmural.

### 🔬 Procedimento Prático
1. Clique em **`☢ RESETAR`**.
2. No Canal 1, ative a exibição do **Endocárdio (verde)**, **Epicárdio (verde-menta)** e **Fibroblasto (lilás)**.
3. Observe que em condições basais (Fibrose = 0%), o fibroblasto permanece com traçado reto em seu potencial de repouso característico ($-38\text{ mV}$), sem acoplamento eletrotônico.
4. Capture o ciclo basal com **`📸 Capturar`**.
5. Abra o Accordion *4. Condições Patológicas* e eleve gradualmente o slider **Fibrose Miocárdica**:
   * **Fibrose Leve ($25\%$):** Observe que o potencial do fibroblasto começa a oscilar sincronizado com a despolarização dos miócitos vizinhos.
   * **Fibrose Moderada ($50\%$):** Note que o potencial de repouso dos miócitos é puxado para cima (despolarização parcial diastólica), o complexo QRS alarga no ECG e a repolarização ventricular sofre dispersão.
   * **Fibrose Severa ($80\%-100\%$):** Observe a profunda deformação do potencial de ação transmural, simulação de áreas cicatriciais pós-infarto do miocárdio e vulnerabilidade arritmogênica.

### ❓ Questões para Discussão
1. Por que o acoplamento eletrotônico com células não-excitáveis de repouso alto (como fibroblastos) diminui a velocidade de condução do impulso elétrico?
2. Como a fibrose miocárdica extensa atua como substrato para taquicardias ventriculares por reentrada?
3. Qual é a utilidade do eletrocardiograma de alta resolução e da duração do QRS na estratificação de risco de pacientes com miocardiopatia fibrótica?

---

## 📄 Recomendações para Elaboração de Relatórios Acadêmicos
Para submissão de relatórios práticos em disciplinas universitárias, recomenda-se:
1. Incluir capturas de tela comparativas utilizando o botão **`[ ⏸ CONGELAR ]`** e a **Onda Fantasma**.
2. Tabular os valores medidos no HUD superior (**BPM, PR, QRS, QT, V.Rep, PR/RR**) em cada etapa do experimento.
3. Citar formalmente o software utilizando as referências padronizadas da seção [Citação Acadêmica](index.html#citacao-academica).
