# Arquitetura do Sistema e Decisões de Engenharia — SimCardio v2.0

Este documento detalha o "porquê" das decisões de engenharia, arquitetura de software e modelagem biofísica adotadas no SimCardio v2.0, contrapondo-as às alternativas descartadas e documentando a evolução histórica do projeto.

---

## 1. Migração do Motor Matemático: De JavaScript Puro para Rust + WebAssembly (WASM)

* **Decisão Adotada:** Implementação completa do motor eletrofisiológico em **Rust nativo** e compilação para **WebAssembly (WASM)** via `wasm-bindgen` e `wasm-pack`.
* **Alternativa Recusada (v1.0):** Execução do laço de cálculo em JavaScript puro dentro de um Web Worker.
* **Justificativa Técnica:**
  - **Sobrecarga de Ponto Flutuante e JIT Thrashing:** O cálculo simultâneo de 5 modelos acoplados com 160 variáveis de estado a um passo de integração de $dt = 0.001\text{ ms}$ requer 16.000 passos por quadro de 16 ms (1 milhão de passos/segundo). O motor V8 do JavaScript, embora veloz, sofre com deotimizações JIT frequentes causadas por desvios condicionais e pausas de *Garbage Collection* (GC), gerando *stuttering* no traçado do osciloscópio.
  - **Determinismo e Segurança de Tipos:** O compilador Rust com LLVM gera código SIMD vetorizado e otimizado (`-O3`), com overhead zero em abstrações e garantia formal de integridade de memória.
  - **Zero-Copy e FFI sem Serialização:** No modelo v1.0, o Web Worker precisava serializar mensagens via `postMessage` (ou SharedArrayBuffer), introduzindo latência de desserialização JSON/binária. No v2.0, o Dioxus e o `HeartSystem` operam no mesmo espaço de memória linear WASM. A função `run_batch()` preenche um vetor contíguo `Vec<f64>` sem necessidade de cópias ou pontes de thread lentas.

---

## 2. Interface de Usuário: Dioxus 0.6 (Rust-First) vs. Frameworks JS (React/Vue/Svelte)

* **Decisão Adotada:** Interface reativa declarativa escrita em Rust puro utilizando **Dioxus 0.6**.
* **Alternativa Recusada:** Frontend em React, Svelte ou Vanilla TypeScript integrado a Vite.
* **Justificativa Técnica:**
  - **Eliminação do Ecossistema Node/npm:** A compilação e execução passam a depender unicamente da *toolchain* de Rust (`cargo` e `wasm-pack`), eliminando centenas de dependências frágeis do npm e problemas de compatibilidade de versões.
  - **Granularidade Reativa via Signals:** O Dioxus 0.6 utiliza reatividade fina (`use_signal`) que recompila em código nativo sem passar por uma *Virtual DOM* pesada em JavaScript.
  - **Consistência de Tipagem:** Os parâmetros da simulação (`Pharmaco`, `HudMetrics`, `HeartSystem`) são compartilhados de forma transparente entre o motor e a interface como structs nativas de Rust, sem duplicação de interfaces TypeScript ou risco de incompatibilidade de campos.

---

## 3. Condução Elétrica: Injeção Transitória de Corrente vs. Grampeamento Abrupto de Voltagem

* **Decisão Adotada:** Propagação de estímulo por **injeção calibrada de corrente despolarizante transitória** ($I_{stim} = -40\text{ a }-52\text{ pA/pF}$ por 1.5 a 2.0 ms).
* **Alternativa Recusada:** Grampeamento forçado de voltagem de membrana ($V = -15\text{ mV}$ ou $V = -30\text{ mV}$) a cada ciclo.
* **Justificativa Técnica:**
  - O grampeamento abrupto viola a conservação de carga e "reinicia" artificialmente os portões de inativação das correntes iônicas (especialmente os portões $h$ e $j$ dos canais rápidos de sódio e $f$ de cálcio). Isso gerava assincronia e dissociação ventricular artificial.
  - A injeção transitória de corrente reproduz exatamente o estímulo eletrotônico local que uma célula transmite à outra pelas junções comunicantes (*gap junctions*). Se a célula receptora estiver em seu período refratário absoluto, o pulso não causará disparo, simulando fisiologicamente fenômenos reais como extrassístoles bloqueadas e condução oculta.

---

## 4. Dromotropismo Dinâmico do Nó AV vs. Latências Estáticas

* **Decisão Adotada:** Atraso de condução atrioventricular dinâmico sensível à frequência cardíaca (condução decremental), tônus autonômico simpático/vagal, bloqueadores de cálcio e isquemia.
* **Alternativa Recusada:** Cronômetros estáticos fixos (`timer = time + 40.0 ms`).
* **Justificativa Técnica:**
  - O Nó AV é a estrutura mais plástica do sistema de condução cardíaco. Canais de cálcio $I_{Ca,L}$ requerem tempo para se recuperar da inativação. Em taquicardias, a condução decremental alarga o intervalo PR naturalmente, prevenindo arritmias ventriculares descontroladas.
  - A inclusão da modulação por Verapamil e tônus autonômico permite demonstrar em aula e bancada virtual a emergência de Bloqueios AV de 1º, 2º e 3º grau com precisão farmacológica.

---

## 5. Eletrocardiograma Sintético: Gradiente Dipolar Transmural vs. Combinações Lineares Arbitrárias

* **Decisão Adotada:** Gênese do traçado de ECG a partir da diferença de potencial transmural instantânea entre as camadas ventriculares:
  $$\text{ECG}(t) \propto V_{endo}(t) - V_{epi}(t)$$
* **Alternativa Recusada:** Combinação linear arbitrária de uma única célula ventricular com o átrio ($0.1 \cdot V_{atrio} + 0.9 \cdot V_{vent}$).
* **Justificativa Técnica:**
  - Uma única célula ventricular repolariza de cima para baixo ($+20 \to -86\text{ mV}$), o que inevitavelmente produz uma deflexão negativa (Onda T invertida no traçado clássico).
  - No miocárdio humano real, o epicárdio repolariza antes do endocárdio devido à maior densidade de canais $I_{to}$. Portanto, o vetor elétrico de repolarização aponta do endocárdio para o epicárdio, na mesma direção do vetor de despolarização (QRS).
  - A heterogeneidade transmural das 3 camadas (Endocárdio, Célula M, Epicárdio) permite que o complexo **P - Q - R - S - T** com **Onda T positiva e concordante** emerja de forma 100% orgânica a partir das leis de Maxwell e da biofísica celular.

---

## 6. Acoplamento Eletrotônico Miócito-Fibroblasto: Dinâmica Biofísica vs. Fronteiras Rígidas Isoladas

* **Decisão Adotada:** Implementação do modelo biofísico de **MacCannell et al. (2007)** com acoplamento eletrotônico bidirecional contínuo via condutância de junção comunicante ($G_{gap} = \text{fibrose} \times 4.0\text{ nS}$):
  $$I_{gap,myo} = \frac{G_{gap} \cdot (V_{myo} - V_{fib})}{C_{m,myo}}$$
  $$I_{gap,fib} = \frac{G_{gap} \cdot (V_{fib} - V_{myo})}{C_{m,fib}}$$
* **Alternativa Recusada:** Modelação de fibrose puramente como barreira não-condutora estática ou redução paramétrica arbitrária de $G_{Na}$ no miócito.
* **Justificativa Técnica:**
  - Fibroblastos e miofibroblastos cardíacos in vivo não são meros isolantes dielétricos; eles expressam conexinas (Cx43 e Cx45) e formam acoplamento elétrico direto com cardiomiócitos vizinhos.
  - Como os fibroblastos possuem capacitância menor ($C_{m,fib} = 6.3\text{ pF}$) e potencial de repouso elevado ($-35\text{ a }-45\text{ mV}$), o acoplamento eletrotônico atua como um **dreno capacitivo** durante a fase rápida de despolarização (diminuindo $dV/dt_{\max}$) e como uma **fonte de corrente despolarizante diastólica** em repouso (elevando $V_{rest}$ do miócito para $-78\text{ mV}$).
  - Essa despolarização parcial diastólica induz a inativação dependente de voltagem dos canais de sódio ($h_{\infty}$ e $j_{\infty}$ de ten Tusscher), explicando mecanisticamente a lentificação da condução intramiocárdica e o bloqueio unidirecional observados em miocárdios infartados e senescentes.
  - O buffer linear de amostragem no WASM foi expandido de 9 para 10 canais contíguos (`[sa, av, atrium, purkinje, vent_endo, vent_epi, fibroblast, cai, force, ecg]`), preservando custo de memória zero-copy e permitindo inspecionar o potencial de ação do fibroblasto e a corrente de acoplamento em tempo real.
