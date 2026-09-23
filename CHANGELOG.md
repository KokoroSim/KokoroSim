# Changelog

## [Unreleased]

### Features
  - (microvascular) implementa Bloco C Starling, dinâmica capilar e edema (#2)
  - (ui,hemodynamics) adicionar diagrama 2D de Guyton no canal CH-2D (refs #2)
  - (hemodynamics) implementar retorno venoso de Guyton e resposta a ortostase (refs #2)
  - (baroreflex) implementar barorreflexo em malha fechada e estabilizar cinetica sinusal
  - (ui) adicionar controles de valvopatias e telemetria sistolica no hud (closes #10)
  - (ui) implementar plotter 2d em plano de fase para alca p x v (closes #9)
  - (hemo) modelar valvopatias aorticas e mitrais (closes #8)
  - (hemo) rastrear volumes sistolicos e calibrar pressao atrial de wiggers (closes #7)
  - (ui) implementar espacos de trabalho cardio e pulmo com osciloscopios dedicados (closes #4)
  - (ui) adicionar estilos das abas de laboratorio e painel de espirometria (closes #3)
  - (engine) integrar acoplamento RSA e expandir telemetria para 15 canais (closes #2)
  - (resp) implementar modelo biofisico de mecanica respiratoria e espirometria (closes #1)
  - (ui) modulariza sanfonas e adiciona seletor de ion e celula no ch-03
  - (ui) adicionar suporte a multiplas derivacoes de ECG e grade milimetrada

### Bug Fixes
  - (engine,ui) recalibrate RSA homeostasis, add UI feedback and 300s baseline test suite
  - (ecg) ajusta margem de picos, rolagem de grade e novo icone com zona segura
  - (ecg) corrige amplitude e grade dinamica do ecg

## Previous Releases

### [v2.1.0] - Otimização de Interface e Controle Eletrofisiológico

### Features
  - (ui) iniciar sanfonas fechadas com abertura exclusiva e compactar visualizacao
  - (ui,docs) padronizar subtitulo, metatags e banner com safe zone 4:3
  - (ui,docs) otimizar banner de compartilhamento social com safe zone e metatags
  - (ui) simplificar camadas ativas por padrao e humanizar marcadores de hemodinamica
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
  - (ci) corrigir resolucao do caminho dist e adicionar validacao de index.html no deploy
  - (ui,docs) simplificar menu superior para quatro itens, blindar logotipo e corrigir citacao
  - (engine) calibrar potencial de repouso despolarizado do fibroblasto maccannell
  - (engine) adaptar conducao e no sa para evitar travamento sob variacao de parametros
  - (engine) equilibrar potencial de repouso de purkinje e adicionar testes biofisicos
  - (dev) isolar injecao do livereload no server.py e limpar htmls estaticos de producao
  - (engine) corrigir dissincronia ventricular e implementar condução fisiológica por pulsos

### [v2.0.0-dev]

- Routine maintenance, documentation updates, and operational improvements.

### [v2.0.0] - Marco Eletrofisiológico e Hemodinâmico

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

