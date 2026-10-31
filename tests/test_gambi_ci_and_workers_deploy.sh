#!/usr/bin/env bash
set -euo pipefail

# Test Suite: Centralização da Automação na Gambi e Publicação em kokorosim.2lp.in (#3)
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SPEC_FILE="$REPO_ROOT/docs/specs/3-centralizar-automacao-na-gambi-e-publicar-em-kokorosim-2lp-in.md"
FORGEJO_CI="$REPO_ROOT/.forgejo/workflows/ci.yml"
WRANGLER_JSONC="$REPO_ROOT/wrangler.jsonc"
BUILD_DOCS="$REPO_ROOT/scripts/build_docs.py"
GITHUB_WORKFLOWS_DIR="$REPO_ROOT/.github/workflows"

echo "==> [TEST] Validando conformidade de CI na Gambi e deploy Workers (#3)..."

# 1. Validação da Spec
echo -n "-> Verificando especificação formal... "
if [ ! -f "$SPEC_FILE" ]; then
    echo "FALHA: Arquivo de especificação $SPEC_FILE não encontrado."
    exit 1
fi
echo "OK: $SPEC_FILE presente."

# 2. Ausência de workflows no GitHub
echo -n "-> Verificando ausência de workflows legados em .github/... "
if [ -d "$GITHUB_WORKFLOWS_DIR" ] && [ "$(ls -A "$GITHUB_WORKFLOWS_DIR" 2>/dev/null)" ]; then
    echo "FALHA: Workflows legados encontrados em $GITHUB_WORKFLOWS_DIR."
    exit 1
fi
echo "OK: Nenhum workflow legado em .github/."

# 3. Presença e configuração de .forgejo/workflows/ci.yml
echo -n "-> Verificando .forgejo/workflows/ci.yml... "
if [ ! -f "$FORGEJO_CI" ]; then
    echo "FALHA: $FORGEJO_CI não encontrado."
    exit 1
fi
echo "OK: $FORGEJO_CI existe."

echo -n "-> Verificando imagem centralizada gambi.2lp.in em ci.yml... "
if ! grep -q 'gambi\.2lp\.in/lumen/gost-ci-cross:latest' "$FORGEJO_CI"; then
    echo "FALHA: Imagem da gambi não declarada no job de testes."
    exit 1
fi
echo "OK: Imagem interna da Gambi configurada."

echo -n "-> Verificando deploy via Cloudflare Workers em ci.yml... "
if ! grep -q 'wrangler deploy' "$FORGEJO_CI" || ! grep -q 'CLOUDFLARE_API_TOKEN' "$FORGEJO_CI"; then
    echo "FALHA: Etapa de deploy via wrangler ausente em $FORGEJO_CI."
    exit 1
fi
echo "OK: Job de deploy Cloudflare Workers presente."

# 4. Presença e configuração do wrangler.jsonc
echo -n "-> Verificando wrangler.jsonc... "
if [ ! -f "$WRANGLER_JSONC" ]; then
    echo "FALHA: $WRANGLER_JSONC não encontrado."
    exit 1
fi
if ! grep -q '"kokorosim\.2lp\.in"' "$WRANGLER_JSONC" || ! grep -q '"./dist"' "$WRANGLER_JSONC"; then
    echo "FALHA: wrangler.jsonc não configurado para kokorosim.2lp.in e ./dist."
    exit 1
fi
echo "OK: wrangler.jsonc configurado corretamente."

# 5. Atualização de URLs canônicas em scripts/build_docs.py
echo -n "-> Verificando URLs canônicas em scripts/build_docs.py... "
if grep -q 'kokorosim\.github\.io' "$BUILD_DOCS"; then
    echo "FALHA: Referências legadas a kokorosim.github.io ainda presentes em $BUILD_DOCS."
    exit 1
fi
if ! grep -q 'kokorosim\.2lp\.in' "$BUILD_DOCS"; then
    echo "FALHA: URL canônica kokorosim.2lp.in ausente em $BUILD_DOCS."
    exit 1
fi
echo "OK: build_docs.py configurado com o domínio canônico kokorosim.2lp.in."

echo "================================================================="
echo "SUCESSO: Todas as asserções passaram (GREEN)!"
