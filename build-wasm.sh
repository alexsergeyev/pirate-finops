#!/bin/bash

set -e

echo "🏴‍☠️ Building Pirate FinOps for WebAssembly..."

echo "Installing wasm-bindgen-cli if not present..."
cargo install wasm-bindgen-cli --version 0.2.92

echo "Adding WASM target if not present..."
rustup target add wasm32-unknown-unknown

echo "Building WASM binary..."
cargo build --target wasm32-unknown-unknown --profile wasm-release

echo "Generating JS bindings..."
wasm-bindgen --out-dir pkg --target web \
    target/wasm32-unknown-unknown/wasm-release/pirate-finops.wasm

echo "Optimizing WASM size (optional - requires wasm-opt)..."
if command -v wasm-opt &> /dev/null; then
    wasm-opt -Oz pkg/pirate_finops_bg.wasm -o pkg/pirate_finops_bg_opt.wasm
    mv pkg/pirate_finops_bg_opt.wasm pkg/pirate_finops_bg.wasm
    echo "✅ WASM optimized!"
else
    echo "⚠️  wasm-opt not found. Install with: npm install -g wasm-opt"
fi

echo "Creating simple HTTP server script..."
cat > serve.py << 'EOF'
#!/usr/bin/env python3
import http.server
import socketserver
import os

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()

    def guess_type(self, path):
        mimetype = super().guess_type(path)
        if path.endswith('.wasm'):
            return 'application/wasm'
        return mimetype

PORT = 8080
os.chdir(os.path.dirname(os.path.abspath(__file__)))

with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
    print(f"🏴‍☠️ Pirate FinOps running at http://localhost:{PORT}")
    print("Press Ctrl+C to stop")
    httpd.serve_forever()
EOF

chmod +x serve.py

echo "✅ Build complete!"
echo ""
echo "To run the game:"
echo "  python3 serve.py"
echo "Then open http://localhost:8080 in your browser"