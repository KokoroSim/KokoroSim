# Changelog

## [Unreleased]

### Features
  - (ui) implementar tipografia kokor心sim, telemetria bilingue e estetica retro-futurista no simulador
  - (assets,style) adicionar logo/icone oficiais e implementar paleta neon carmesim com estetica otomo
  - (ui,docs) reestruturar navegacao com home em sobre, mover simulador para app.html e consolidar marca KokoroSim
  - (ui,ci) publicar documentacao em HTML no GitHub Pages e abrir manual em nova aba
  - (ui) implementar sincronização bio-disparada RR da onda fantasma e linha sólida esmaecida anti-fadiga
  - (ui) implementar congelamento da simulação, onda fantasma sincronizada e reset limpo no gatilho automático
  - (ui) implementar 5 modos de osciloscópio, captura de onda fantasma e marcadores acústicos multicanal
  - (engine) adicionar evento de disparo da Fase 0 do Nó SA ao fluxo de dados em lote
  - (ui) calibrar sons cardíacos para alto-falantes de celular e fixar orçamento de cálculo em 1x tempo real
  - (engine) otimizar integração numérica com método híbrido de Rush-Larsen para dispositivos móveis e hardware modesto
  - (ui) adicionar sintetizador Web Audio para bip de UTI e bulhas B1/B2, gráfico hemodinâmico de Wiggers e compressão temporal
  - (engine) implementar acoplamento eletromecânico, elastância variável no tempo e hemodinâmica Windkessel de 3 elementos
  - (ui) adicionar controle deslizante de fibrose miocárdica e visualização de fibroblastos
  - (engine) implementar modelo de fibroblasto cardíaco de MacCannell et al. (2007) e acoplamento eletrotônico miocitário
  - (ui) exibir potenciais de ação ventriculares transmurais e traçado de ECG dipolar
  - (engine) implementar dromotropismo dinâmico na condução AV e heterogeneidade ventricular transmural (Epi, M-cell, Endo)
  - (engine,ui) integrar modelo de células de Purkinje humanas de Stewart et al. (2009) à cadeia de condução
  - (dev) adicionar monitoramento de arquivos, livereload e rodapé de versão no ambiente local
  - (ui/engine) adiciona sistema de plotagem em tempo real e modelo de força mecânica

### Bug Fixes
  - (dev) isolar injecao do livereload no server.py e limpar htmls estaticos de producao
  - (engine) corrigir dissincronia ventricular e implementar condução fisiológica por pulsos

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

