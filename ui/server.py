import http.server
import socketserver
import os
import sys

PORT = int(os.environ.get("PORT", 8081))
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
BUILD_ID_FILE = os.path.join(BASE_DIR, ".build_id")

class DevServerHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=BASE_DIR, **kwargs)

    def do_GET(self):
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
            print(f"🚀 KokoroSim Dev Server rodando em http://localhost:{PORT} (LiveReload ativo)", flush=True)
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Servidor finalizado com sucesso.", flush=True)
        sys.exit(0)
