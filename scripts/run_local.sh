#!/bin/bash
set -e

# Garante que o script está rodando a partir da raiz do projeto
cd "$(dirname "$0")/.."

echo "🔨 Compilando o projeto principal Wasm..."
cd ui
wasm-pack build --target web

echo "🚀 Iniciando servidor local na porta 8081..."
python3 server.py
