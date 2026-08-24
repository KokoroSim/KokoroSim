# SimCardio: Simulador Eletrofisiológico Cardíaco em Tempo Real

O **SimCardio** é um projeto de código aberto dedicado à simulação matemática da eletrofisiologia celular cardíaca em ambiente web. O sistema resolve equações diferenciais ordinárias (EDOs) em tempo real — executando mais de 100 mil passos de integração por segundo — para simular o comportamento de íons, correntes e canais de quatro tecidos fundamentais do coração humano e de modelos experimentais estabelecidos.

## 🧬 Modelos Matemáticos Empregados

O projeto integra quatro modelos matemáticos amplamente validados na literatura de eletrofisiologia cardíaca (disponíveis no repositório do [Physiome Project / CellML](https://models.physiomeproject.org/)):

1. **Nó Sinoatrial (SA) - Marca-passo Natural (Coelho):**
   - **Modelo:** Severi et al. (2012)
   - **Referência:** Severi S, Fantini M, Charawi LA, DiFrancesco D. *An updated computational model of rabbit sinoatrial action potential to investigate the mechanisms of heart rate modulation.* J Physiol. 2012.
   - **Função na Simulação:** Responsável por gerar o automatismo do sistema cardíaco. O modelo é baseado na eletrofisiologia nodal de coelhos, um padrão-ouro em estudos experimentais de ritmologia.

2. **Músculo Atrial (Humano):**
   - **Modelo:** Courtemanche, Ramirez, Nattel (1998)
   - **Referência:** Courtemanche M, Ramirez RJ, Nattel S. *Ionic mechanisms underlying human atrial action potential properties: insights from a mathematical model.* Am J Physiol. 1998.
   - **Função na Simulação:** Representa as células de resposta rápida responsáveis pela contração atrial. O modelo é estritamente baseado em dados de tecido atrial humano.

3. **Nó Atrioventricular (AV) - Estação de Retardo (Coelho):**
   - **Modelo:** Inada et al. (2009)
   - **Referência:** Inada S, et al. *One-dimensional mathematical model of the atrioventricular node...* Biophys J. 2009.
   - **Função na Simulação:** Executa o retardo fisiológico do sinal elétrico. Por conta da dificuldade de isolamento in vitro de células nodais humanas funcionais, o modelo utiliza a base eletrofisiológica do coelho, tendo sido matematicamente calibrado para integração com o tecido humano adjacente.

4. **Músculo Ventricular (Humano):**
   - **Modelo:** ten Tusscher & Panfilov (2006)
   - **Referência:** ten Tusscher KHWJ, Panfilov AV. *Alternans and spiral breakup in a human ventricular tissue model.* Am J Physiol Heart Circ Physiol. 2006.
   - **Função na Simulação:** Atua como o principal motor hemodinâmico, sendo caracterizado por um potencial de ação com platô prolongado e alta dependência dos transientes de cálcio.

## ⚙️ Arquitetura do Motor de Integração e Calibrações

Para viabilizar a execução simultânea e estável de quatro modelos de alta complexidade em ambiente de navegador web, a arquitetura da simulação baseou-se em técnicas específicas de cálculo numérico e modelagem sistêmica:

* **Integração por Forward Euler com Domínios Híbridos de Tempo:**
  O cálculo das EDOs é isolado em uma thread dedicada (`Web Worker`) para garantir alta taxa de atualização gráfica (60 FPS) na renderização do cliente. O método de Forward Euler é empregado com um passo base fixo ($dt$) de **0.01 milissegundos**.
  Um desafio inerente à integração bibliográfica foi a divergência das unidades de tempo:
  * **Nó SA (Severi) e Nó AV (Inada)** foram formulados computacionalmente em **Segundos**. O motor aplica a conversão estrutural da variável tempo ($dt / 1000$) internamente a essas funções.
  * **Átrio (Courtemanche) e Ventrículo (ten Tusscher)** operam fundamentalmente em **Milissegundos**.
  
* **Micro-stepping Discreto em Células Rápidas:**
  Os modelos de trabalho contrátil (Átrio e Ventrículo) possuem canais rápidos de Sódio ($I_{Na}$) nos quais a derivada de voltagem ($\frac{dV}{dt}$) ultrapassa 400 V/s durante a Fase 0 do potencial de ação. Para evitar a divergência numérica inerente a essas altas taxas de variação, implementou-se um algoritmo de **micro-stepping interno (10 passos por ciclo)**. Isso reduz o passo de integração efetivo para **0.001 milissegundos** apenas nesses tecidos, mantendo a estabilidade global do integrador sem comprometer a eficiência computacional.

* **Condução Sistêmica Simplificada (Modelo 0D Acoplado):**
  Para dispensar simulações espaciais (modelos bidomínio) excessivamente custosas, adotou-se uma aproximação de propagação temporal. O sistema avalia quatro células isoladas. Quando o limiar de despolarização de um tecido superior é atingido (ex: SA $> -30mV$), cronômetros de latência de condução são iniciados, injetando uma corrente de estímulo ($I_{stim}$) no tecido adjacente após o tempo fisiológico determinado (como o intervalo PR).

* **Calibração do Quimerismo Biológico:**
  A integração de modelos advindos de espécies distintas exigiu parametrização estrita:
  - **Inibição do Automastismo Nodal AV:** Como o ritmo fisiológico do modelo Inada (coelho) é elevado, ele competiria diretamente com o Nó SA no sistema humano simulado. As correntes intrínsecas de marca-passo foram matematicamente reduzidas ($I_f$ a 15% e a Corrente de Fuga a 45%). Consequentemente, o nódulo AV opera estritamente como via de condução, expressando automatismo (Ritmo de Escape Juncional) apenas em resposta a falhas prolongadas na condução sinoatrial.
  - **Potenciais de Nernst Dinâmicos:** As concentrações eletrolíticas externas foram dissociadas de constantes estáticas, permitindo que a variação de $K^+_o$, $Ca^{2+}_o$ e $Na^+_o$ pelo usuário provoque recálculo imediato dos respectivos gradientes eletroquímicos em todos os modelos.

## 🎛️ Modulação Farmacológica e Autonômica

A interface permite intervenção dinâmica, traduzindo parâmetros macroscópicos em efeitos celulares e correntes iônicas. As relações implementadas são:

### 1. Íons e Eletrólitos
Estes controles modificam diretamente a força motriz das equações de Nernst para cada íon:
* **Potássio [K+]_o (2.0 a 8.5 mEq/L):** Determinante central do potencial de repouso transmembrana ($V_{rest}$). A hipocalemia causa hiperpolarização sistêmica, predispondo o tecido a fenômenos arritmogênicos. A hipercalemia extrema despolariza as membranas ao ponto de inativar permanentemente os canais de sódio, resultando em assistolia (Parada Cardíaca em Diástole).
* **Cálcio [Ca2+]_o (1.0 a 3.5 mmol/L):** Modula primariamente o trânsito da corrente de cálcio tipo L ($I_{CaL}$). Hipocalcemia resulta no aumento do platô e prolongamento do intervalo QT virtual; a hipercalcemia reduz o tempo de repolarização sistólica.
* **Sódio [Na+]_o (125 a 155 mEq/L):** Responsável pela corrente de pico em células de resposta rápida. Concentrações baixas atenuam severamente a amplitude da Fase 0. No limiar inferior parametrizado, a hiponatremia desencadeia Bloqueio Sinoatrial (SA Block), onde a geração do potencial SA é mantida pelo NCX (Sodium-Calcium Exchanger), mas o estímulo propagado é insuficiente para deflagrar o tecido atrial.

### 2. Sistema Nervoso Autônomo
* **Tônus Simpático (0-100%):** O efeito catecolaminérgico é reproduzido pelo escalonamento direto das condutâncias das correntes $I_f$ e $I_{CaL}$. Como resultado fisiológico, observa-se efeito cronotrópico positivo severo (taquicardia) e encurtamento do tempo de duração do potencial de ação.
* **Tônus Parassimpático (0-100%):** O tônus vagal é reproduzido pela modulação da corrente mediada por acetilcolina ($I_{K,ACh}$). O consequente efluxo de potássio acentua a fase 4 hiperpolarizante do tecido nodal, resultando em bradicardia sinusal.

### 3. Fármacos Antiarrítmicos
Os fármacos atuam como frações inibitórias na condutância de canais específicos:
* **Lidocaína (Bloqueador de Canais de Na+):** Fármaco Classe I. Atua multiplicando a condutância dos canais rápidos ($g_{Na}$) por uma razão residual. O efeito é a perda de inclinação da Fase 0 e o alargamento compensatório da despolarização (QRS alargado).
* **Amiodarona (Bloqueador de Canais de K+):** Fármaco Classe III. Interfere na saída de Potássio ($I_{Kr}$, $I_{Ks}$), correntes fundamentais para a Fase 3 da repolarização. O retardo da repolarização culmina na manifestação visual de platôs extensos (QT longo).
* **Verapamil (Bloqueador de Canais de Ca2+):** Fármaco Classe IV. Suprime as correntes L-type ($I_{CaL}$). Os modelos respondem com o achatamento agudo do platô ventricular e com a supressão significativa da condução na rampa nodal AV, simulando graus variáveis de bloqueio atrioventricular.
* **Digoxina (Inibidor Na+/K+):** Diminui a condutância da bomba Sódio-Potássio ATP-ase ($I_{NaK}$). Isso inibe a manutenção da estabilidade iônica transmembrana a longo prazo, predispondo os tecidos à sobrecarga de cálcio intracelular.

### 4. Isquemia Tecidual
* **Nível de Isquemia:** Modelo patológico que ativa sumariamente a corrente de escape $I_{K,ATP}$ induzida pela deficiência de ATP. O aumento do efluxo simula uma hipercalemia local. Consequentemente, observa-se atenuação imediata e perda precoce do platô do potencial de ação ventricular.

---

O SimCardio busca alinhar a responsividade visual de interfaces modernas ao rigor acadêmico da biologia sistêmica computacional, fornecendo uma base experimental contínua para estudos in silico das dinâmicas cardíacas.

## Licença (GPLv3)
Este projeto está sob a licença **GNU General Public License v3.0**. Você é livre para:
* **Usar** o software para qualquer fim (incluindo uso acadêmico e comercial).
* **Modificar** o código e adaptar as equações para os seus estudos.
* **Distribuir** cópias do software ou das modificações.
Entretanto, ao distribuir qualquer trabalho derivado deste projeto, o código-fonte deve permanecer aberto e sob a mesma licença GPLv3, assegurando que o conhecimento biológico permaneça livre e acessível à comunidade.
