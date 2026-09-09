import http.server
import socketserver

PORT = 8081

class NoCacheHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate, max-age=0')
        super().end_headers()

NoCacheHandler.extensions_map.update({
    ".wasm": "application/wasm",
})

socketserver.TCPServer.allow_reuse_address = True
with socketserver.TCPServer(("", PORT), NoCacheHandler) as httpd:
    print("serving at port", PORT, "without cache")
    httpd.serve_forever()
