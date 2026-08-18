# SimCardio - Simulador de Eletrofisiologia Cardíaca

Um simulador interativo, rodando inteiramente no navegador, desenhado para auxiliar estudantes de medicina e biociências a compreenderem a relação direta entre correntes iônicas celulares, potenciais de ação e o traçado eletrocardiográfico (ECG) resultante.

## ⚠️ Aviso Crítico de Modelagem (Escopo Didático)

Este software emprega **modelagem fenomenológica** (baseada em equações tipo FitzHugh-Nagumo / Aliev-Panfilov modificadas) e não uma modelagem biofísica estrita ponta a ponta (como o modelo de Hodgkin-Huxley ou Luo-Rudy).

**O que isso significa na prática?**
As equações aqui reproduzem com fidelidade visual e temporal as dinâmicas de macrocomportamento do tecido cardíaco — como duração do potencial, períodos refratários e inversão vetorial em diferentes derivações —, mas não calculam a cinética molecular exata de cada subunidade proteica dos canais iônicos. O objetivo primário é pedagógico: ilustrar *causa e efeito* de alterações sistêmicas (como hipercalemia ou bloqueio de cálcio) em tempo real, sem a complexidade matemática que inviabilizaria a execução fluida em navegadores comuns. Não utilize os dados brutos gerados aqui para pesquisas de bioengenharia ou tomada de decisão clínica.

## 🧠 Decisões de Engenharia e Arquitetura

Para manter o simulador leve, rodando a 60 quadros por segundo (FPS) e responsivo a alterações em tempo real, adotamos as seguintes premissas técnicas:

### 1. Motor Matemático: Por que não usar Hodgkin-Huxley?

Modelos completos de miócitos (como *Luo-Rudy*) exigem a resolução simultânea de mais de 20 equações diferenciais ordinárias (ODEs) por célula e lidam com extrema rigidez numérica (*stiffness*). Durante a Fase 0 (ativação do sódio), a escala de tempo é de frações de milissegundo, enquanto a repolarização leva centenas de milissegundos. Rodar isso em JavaScript causaria travamentos na *thread* principal. O modelo fenomenológico implementado resolve a dinâmica macroscópica da voltagem ($V$) e do restabelecimento celular ($w$) em passos fixos simples, consumindo uma fração ínfima de CPU e garantindo estabilidade irrestrita.

### 2. Renderização: Por que Canvas 2D puro?

O navegador sofre severamente para renderizar gráficos densos atualizados a 60 Hz caso se utilize DOM/SVG (como as bibliotecas D3.js ou Recharts). Cada nó do SVG força um recálculo de layout. Optamos pela API nativa de **Canvas 2D** aplicando um padrão de osciloscópio com *Ring Buffer* (Buffer Circular de arrays tipados `Float32Array`). Ao invés de reprocessar todo o sinal, apenas reescrevemos o "ponteiro de leitura", permitindo complexidade $\mathcal{O}(1)$ na atualização da memória.

### 3. Foco Eletrocardiográfico: Por que apenas DII, V1 e aVR?

Um ECG completo possui 12 derivações, mas didaticamente, 3 são suficientes para treinar o raciocínio vetorial sobre o dipolo cardíaco:

* **DII:** É a derivação "padrão ouro". Seu eixo (+60°) enxerga a despolarização normal de frente, gerando P, QRS e T integralmente positivos.
* **V1:** Derivação precordial essencial. Permite treinar a inversão fisiológica da morfologia ventricular (padrão rS), ilustrando que a mesma frente de onda é registrada de forma negativa dependendo do ângulo de observação da parede anterior e do septo.
* **aVR:** Derivação de controle negativo extremo (-150°). Útil para demonstrar a total inversão de todos os vetores elétricos, ajudando o estudante a desapegar da ideia de que "onda boa é onda para cima".

## 🚀 Como Executar o Ambiente de Desenvolvimento (Dev Server)

Este projeto foi construído sobre o [Bun](https://bun.sh/) e utiliza o [Vite](https://vitejs.dev/) para empacotamento ultrarrápido do TypeScript.

### Pré-requisitos

Ter o Bun instalado no seu sistema. Em sistemas Linux/macOS ou via WSL:

```bash
curl -fsSL [https://bun.sh/install](https://bun.sh/install) | bash
```

### Levantando o servidor

Clone o repositório.

Na raiz da pasta do projeto, instale as dependências:

```bash
bun install
```

Inicie o servidor de desenvolvimento:

```bash
bun run dev
```

O terminal informará o endereço local (geralmente <http://localhost:5173>). Abra em seu navegador. O Vite garantirá Hot Module Replacement (HMR), atualizando a interface instantaneamente a cada salvamento no código.

## 📜 Licença

Este projeto é licenciado sob a [Licença Pública Geral GNU v3.0 (GPLv3)](https://www.gnu.org/licenses/gpl-3.0.html).
A GPLv3 garante que qualquer pessoa possa usar, modificar e distribuir este software, desde que as modificações e trabalhos derivados também sejam distribuídos sob a mesma licença de código aberto.
