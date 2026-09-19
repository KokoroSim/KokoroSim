#!/bin/bash
# =============================================================================
# KokoroSim Local Development Server with Auto-Watch, Concurrency Queue & LiveReload
# =============================================================================

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
UI_DIR="$PROJECT_ROOT/ui"
DIST_DIR="$PROJECT_ROOT/dist"
BUILD_ID_FILE="$DIST_DIR/.build_id"
PORT="${PORT:-8081}"

# Verify required tools
if ! command -v wasm-pack >/dev/null 2>&1; then
    echo "❌ Erro: wasm-pack não foi encontrado no PATH." >&2
    echo "   Instale com: cargo install wasm-pack" >&2
    exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
    echo "❌ Erro: python3 não foi encontrado no PATH." >&2
    exit 1
fi

if ! command -v inotifywait >/dev/null 2>&1; then
    echo "❌ Erro: inotifywait não foi encontrado no PATH." >&2
    echo "   Instale no Debian/Ubuntu com: sudo apt-get install inotify-tools" >&2
    exit 1
fi

FIFO=$(mktemp -u /tmp/kokorosim_watch.XXXXXX)
mkfifo "$FIFO"

cleanup() {
    trap - INT TERM EXIT
    echo ""
    echo "🛑 Encerrando ambiente local do KokoroSim..."
    if [ -n "${SERVER_PID:-}" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
        kill -TERM "$SERVER_PID" 2>/dev/null || true
        wait "$SERVER_PID" 2>/dev/null || true
    fi
    if [ -n "${WATCHER_PID:-}" ] && kill -0 "$WATCHER_PID" 2>/dev/null; then
        kill -TERM "$WATCHER_PID" 2>/dev/null || true
        wait "$WATCHER_PID" 2>/dev/null || true
    fi
    rm -f "$FIFO" 2>/dev/null || true
    exit 0
}
trap cleanup INT TERM EXIT

build_wasm() {
    local timestamp
    timestamp="$(date '+%Y-%m-%d %H:%M:%S')"
    echo "----------------------------------------------------------------------"
    echo "🔨 [$(date '+%H:%M:%S')] Compilando projeto Wasm e montando dist/ (Build Time: $timestamp)..."
    
    mkdir -p "$DIST_DIR"
    export KOKOROSIM_BUILD_TIME="$timestamp"
    if (cd "$UI_DIR" && wasm-pack build --target web --out-dir "$DIST_DIR/pkg"); then
        python3 "$PROJECT_ROOT/scripts/build_docs.py" "$DIST_DIR" >/dev/null 2>&1 || true
        # Gera novo token de build para acionar o LiveReload no navegador
        date +%s%N > "$BUILD_ID_FILE"
        echo "✅ [$(date '+%H:%M:%S')] Build concluída com sucesso em dist/! Recarregando navegador..."
        return 0
    else
        echo "❌ [$(date '+%H:%M:%S')] Falha na compilação do Rust/Wasm!"
        echo "   O servidor permanece ativo. Corrija o código para disparar nova build."
        return 1
    fi
}

echo "======================================================================"
echo " ★ KokoroSim — Ambiente de Desenvolvimento Integrado com LiveReload ★"
echo "======================================================================"

# 1. Executa build inicial
build_wasm || true

# 2. Inicia o servidor Python com suporte a LiveReload em background servindo dist/
echo "🚀 Iniciando servidor local na porta $PORT..."
PORT="$PORT" python3 "$UI_DIR/server.py" "$DIST_DIR" &
SERVER_PID=$!

# Aguarda inicialização do servidor
sleep 0.5
echo "🌐 Acesso local disponível em: http://localhost:$PORT"

# 3. Define diretórios e arquivos monitorados (estritamente fontes, sem dist)
WATCH_PATHS=(
    "$PROJECT_ROOT/engine/src"
    "$PROJECT_ROOT/engine/Cargo.toml"
    "$PROJECT_ROOT/ui/src"
    "$PROJECT_ROOT/ui/assets"
    "$PROJECT_ROOT/ui/app.html"
    "$PROJECT_ROOT/ui/simulador.html"
    "$PROJECT_ROOT/ui/Cargo.toml"
    "$PROJECT_ROOT/ui/build.rs"
    "$PROJECT_ROOT/README.md"
    "$PROJECT_ROOT/ROADMAP.md"
    "$PROJECT_ROOT/ARCHITECTURE.md"
    "$PROJECT_ROOT/docs"
    "$PROJECT_ROOT/VERSION"
)

EXISTING_WATCH_PATHS=()
for p in "${WATCH_PATHS[@]}"; do
    if [ -e "$p" ]; then
        EXISTING_WATCH_PATHS+=("$p")
    fi
done

echo "👀 Monitorando alterações em tempo real..."
echo "   Pressione Ctrl+C para encerrar o ambiente."
echo "======================================================================"

# 4. Inicia watcher em background enviando eventos para a FIFO
inotifywait -m -r -q -e modify,create,delete,move \
    --exclude '(\.git|target|pkg|dist|\.build_id|\.swp|~)' \
    "${EXISTING_WATCH_PATHS[@]}" > "$FIFO" 2>/dev/null &
WATCHER_PID=$!

# 5. Loop de consumo de eventos com debouncing e fila de coalescência
while read -r event; do
    # Drena eventos adicionais gerados em cascata pelo editor nos próximos 300ms
    while read -t 0.3 -r drained; do
        :
    done
    build_wasm || true
    # Drena qualquer eco gerado durante o processo de compilação
    while read -t 0.1 -r _; do
        :
    done
done < "$FIFO"
