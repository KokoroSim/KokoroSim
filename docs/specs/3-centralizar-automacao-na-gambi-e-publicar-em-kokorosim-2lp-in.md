# Especificação Técnica: Centralização da Automação na Gambi e Publicação em kokorosim.2lp.in

- **Ticket**: #3
- **Fase**: GOST DD-TDD Fase 3 (Spec)
- **Escopo**: Eliminar fluxos legados e deploys externos; unificar automação de CI no Forgejo (.forgejo/workflows/ci.yml) utilizando a infraestrutura Gambi e configurar publicação do simulador em kokorosim.2lp.in via Cloudflare Workers.

## 1. Contexto e Motivação
Todas as dependências externas de CI/CD (imagens em registros de terceiros como Codeberg e deploys remotos para GitHub Pages) foram descontinuadas. Toda a infraestrutura está centralizada na Gambi (`gambi.2lp.in`).
O KokoroSim continha um fluxo obsoleto (`.github/workflows/deploy.yml`) que tentava executar deploy no GitHub Pages a partir de imagem inexistente do Codeberg, resultando em falha contínua.
Além disso, KokoroSim deve ser entregue publicamente através da nossa rede centralizada no endereço `https://kokorosim.2lp.in/` utilizando Cloudflare Workers com Assets Estáticos.

## 2. Requisitos Técnicos
1. **Eliminação de Legados Externos:**
   - Remover `.github/workflows/deploy.yml` e desativar qualquer deploy externo.
   - Remover `.github/workflows/test.yml` e a pasta `.github/`.
2. **Centralização no Forgejo (.forgejo/workflows/ci.yml):**
   - Job `test`: Execução da suíte de testes biofísicos com `cargo test --workspace` utilizando a imagem `gambi.2lp.in/lumen/gost-ci-cross:latest`.
   - Job `deploy-site`: Disparado após sucesso dos testes na branch `main` (ou tags de release), executando `npx wrangler deploy` com os segredos centralizados `CLOUDFLARE_API_TOKEN` e `CLOUDFLARE_ACCOUNT_ID`.
3. **Configuração de Publicação do Wrangler (wrangler.jsonc):**
   - Nome do worker: `kokorosim`.
   - Rota de domínio customizado: `kokorosim.2lp.in`.
   - Diretório de ativos estáticos: `./dist`.
4. **Metadados, Cabeçalhos e URLs Canônicas:**
   - Atualizar `scripts/build_docs.py` substituindo as referências legadas `https://kokorosim.github.io/` pela URL canônica `https://kokorosim.2lp.in/`.
   - Assegurar a presença do arquivo `dist/_headers` com políticas estritas de segurança (`nosniff`, `DENY`, `strict-origin-when-cross-origin`).
5. **Critérios de Aceite:**
   - Zero workflows sob `.github/`.
   - `.forgejo/workflows/ci.yml` válido e referenciando apenas `gambi.2lp.in`.
   - `wrangler.jsonc` sintaticamente válido e apontando para `kokorosim.2lp.in`.
   - Teste automatizado validando a conformidade dos workflows e ausência de imagens/deploys externos.

## 3. Análise de Mutação (Fase 7)
- **Mutante 1 (Rota do Wrangler):** Testou-se a alteração forçada da rota no `wrangler.jsonc`. O teste validou que a rota deve ser estritamente `kokorosim.2lp.in`.
- **Mutante 2 (Retorno de Imagem Externa no CI):** Testou-se a injeção da imagem legada `codeberg.org/lumenpink/gost-ci-wasm:latest`. O teste rejeitou o mutante, assegurando o uso exclusivo de `gambi.2lp.in/lumen/gost-ci-cross:latest`.

