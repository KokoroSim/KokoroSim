# Changelog

## [Unreleased]

### Features
  - (engine) implement MacCannell et al. (2007) cardiac fibroblast model and electrotonic myocyte coupling
  - (ui) display transmural ventricular action potentials and dipolar ECG waveform
  - (engine) implement dynamic AV conduction dromotropism and transmural ventricular heterogeneity (Epi, M-cell, Endo)
  - (engine,ui) integrate Stewart et al. (2009) human Purkinje cell model into conduction chain
  - (dev) add file watching, livereload, and version footer to local dev environment
  - add initial cardiac electrophysiology UI dashboard and simulation engine
  - (ui/engine) adiciona sistema de plotagem em tempo real e modelo de força mecânica

### Bug Fixes
  - (engine) resolve ventricular dyssynchrony and implement pulse-based physiological conduction

## Previous Releases

### [v2.0.0-dev]

- Routine maintenance, documentation updates, and operational improvements.

### [v1.0.0]

### Features
  - (audio) adicionar sistema de som com suporte a bips e bulhas cardíacas (B1/B2)
  - (ui) importar README estaticamente para o modal sobre
  - (ui) adicionar modal de informacoes "Sobre" com carregamento do README
  - (controls) atualiza limites e descricoes dos ions e adiciona PR/RR no HUD
  - (engine) adicionar botao global de reset e limpeza de dados
  - (engine) melhora precisao da corrente de sodio e adiciona filtro de linha de base
  - (engine) implementar gerador de pseudo-ECG híbrido com ondas P, QRS e T
  - (hud) adiciona calculo de metricas cardiacas e atualizacao do HUD
  - (worker) otimizar calculo de parametros e adicionar suporte a SNA, farmacos e isquemia
  - (ui) adicionar suporte para exibição do átrio no canvas e configurações
  - (scripts) adiciona scripts de conversao, parsing e transpilar modelos CellML
  - (engine) aplicar tuning fisiologico e acoplamento eletrotonico nos nos cardiacos
  - (worker) integrar modelos cardíacos Inada e Ten Tusscher no engine
  - (engine) integrar modelo eletrofisiologico de Severi (2012) e atualizar renderizacao do canvas
  - (engine) integrar worker de simulacao com canvas e controles da sidebar
  - (worker) adicionar Web Worker para o motor de simulação e integração com Euler
  - (ui) refatorar interface e adicionar suporte a novos gráficos no cockpit
  - (ui) atualizar interface e documentacao do simulador de eletrofisiologia
  - (core) inicializar projeto com simulador cardíaco e fluxo de CI/CD

### Bug Fixes
  - (worker) ajusta fator multiplicador da severidade C_SEV[3]
  - (ui) ajusta o cálculo de escala e range biológico no canvas

