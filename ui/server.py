import http.server
import socketserver
import os
import sys

PORT = int(os.environ.get("PORT", 8081))
PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIST_DIR = os.path.join(PROJECT_ROOT, "dist")

if len(sys.argv) > 1 and os.path.isdir(sys.argv[1]):
    SERVE_DIR = os.path.abspath(sys.argv[1])
elif os.path.isdir(DIST_DIR):
    SERVE_DIR = DIST_DIR
else:
    SERVE_DIR = os.path.dirname(os.path.abspath(__file__))

BUILD_ID_FILE = os.path.join(SERVE_DIR, ".build_id")

LIVERELOAD_SNIPPET = b"""
    <!-- Injetado dinamicamente pelo KokoroSim Dev Server (apenas em dev local) -->
    <script>
        (function() {
            let lastBuildId = null;
            async function pollLiveReload() {
                try {
                    const res = await fetch('/__livereload__?t=' + Date.now());
                    if (res.ok) {
                        const id = (await res.text()).trim();
                        if (lastBuildId !== null && id && id !== '0' && id !== lastBuildId) {
                            console.log('[KokoroSim Dev] Nova build detectada (' + id + '). Recarregando...');
                            window.location.reload();
                            return;
                        }
                        if (id && id !== '0') {
                            lastBuildId = id;
                        }
                    }
                } catch (_) {}
                setTimeout(pollLiveReload, 1500);
            }
            pollLiveReload();
        })();
    </script>
</body>
"""

class DevServerHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=SERVE_DIR, **kwargs)

    def do_GET(self):
        if self.path.startswith("/ui/"):
            self.path = self.path[3:]

        if self.path.startswith("/__livereload__"):
            token = "0"
            if os.path.exists(BUILD_ID_FILE):
                try:
                    with open(BUILD_ID_FILE, "r") as f:
                        token = f.read().strip()
                except Exception:
                    token = "0"
            self.send_response(200)
            self.send_header("Content-Type", "text/plain; charset=utf-8")
            self.send_header("Cache-Control", "no-store, no-cache, must-revalidate, max-age=0")
            self.end_headers()
            self.wfile.write(token.encode("utf-8"))
            return

        # Para arquivos HTML, injeta dinamicamente o script de LiveReload em tempo de execucao
        clean_path = self.path.split("?")[0].split("#")[0]
        local_file = self.translate_path(clean_path)
        if os.path.isdir(local_file):
            local_file = os.path.join(local_file, "index.html")

        if local_file.endswith(".html") and os.path.isfile(local_file):
            try:
                with open(local_file, "rb") as f:
                    content = f.read()
                if b"</body>" in content:
                    content = content.replace(b"</body>", LIVERELOAD_SNIPPET)
                elif b"</html>" in content:
                    content = content.replace(b"</html>", LIVERELOAD_SNIPPET + b"\n</html>")

                self.send_response(200)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(content)))
                self.send_header("Cache-Control", "no-store, no-cache, must-revalidate, max-age=0")
                self.end_headers()
                self.wfile.write(content)
                return
            except Exception:
                pass

        super().do_GET()

    def end_headers(self):
        self.send_header("Cache-Control", "no-store, no-cache, must-revalidate, max-age=0")
        super().end_headers()

DevServerHandler.extensions_map.update({
    ".wasm": "application/wasm",
})

socketserver.TCPServer.allow_reuse_address = True

if __name__ == "__main__":
    try:
        with socketserver.TCPServer(("", PORT), DevServerHandler) as httpd:
            print(f"🚀 KokoroSim Dev Server rodando em http://localhost:{PORT} (servindo {SERVE_DIR})", flush=True)
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Servidor finalizado com sucesso.", flush=True)
        sys.exit(0)
