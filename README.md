# 🏴‍☠️ Pirate FinOps Treasure Hunt

A browser-based puzzle game where you hunt for wasted Azure cloud resources and optimize FinOps before your budget runs out!

## 🎮 Game Overview

You are a FinOps Captain hunting for hidden savings in your Azure cloud estate. The estate is a grid map of resources - some wasteful, some fine. Fix the wasteful resources before your budget hits $0 or you're FIRED by the CFO!

### How to Play
- **Click tiles** to reveal what's underneath (costs 1 turn)
- **Press F** on revealed wasteful resources to fix them (costs 2-3 turns)
- Each turn burns budget based on your current waste
- **Win**: Reduce waste to ≤10% or achieve $5,000/mo savings
- **Lose**: Budget reaches $0

### Resource Types
- 💻 **Idle VM** - $200/turn waste
- 💾 **Orphaned Disk** - $50/turn waste
- 🐙 **Overprovisioned AKS** - $400/turn waste
- 🗂 **Untagged Resource** - Affects allocation
- 🌐 **Unused Public IP** - $10/turn waste
- Plus more to discover!

## 🚀 Quick Start

### Play Native (Development)
```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and run
cd pirate-finops
cargo run
```

### Play in Browser (WebAssembly)
```bash
# Build for web
./build-wasm.sh

# Start local server
python3 serve.py

# Open in browser
open http://localhost:8080
```

## 🛠️ Development

### Prerequisites
- Rust 1.75+
- Python 3 (for local web server)
- wasm-bindgen-cli (auto-installed by build script)

### Project Structure
```
pirate-finops/
├── src/
│   ├── main.rs         # Game entry point & state management
│   ├── components.rs   # ECS components (tiles, resources)
│   ├── resources.rs    # Game data & grid generation
│   ├── systems.rs      # Game logic & mechanics
│   └── ui.rs          # Menus & HUD
├── index.html         # Web page wrapper
├── build-wasm.sh      # WASM build script
└── Cargo.toml         # Dependencies
```

### Building from Source

**Native Build:**
```bash
cargo build --release
cargo run --release
```

**Web Build:**
```bash
# Full build with optimization
./build-wasm.sh

# Quick rebuild during development
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/debug/pirate-finops.wasm
```

### Testing Changes
1. Make code changes
2. Run `cargo check` for quick validation
3. Run `cargo build` for native testing
4. Use `./build-wasm.sh` for web deployment

## 🎯 Game Design

### Difficulty Levels (Coming Soon)
- **Easy**: $15,000 budget, 6x6 grid
- **Normal**: $10,000 budget, 8x8 grid
- **Hard**: $7,500 budget, 10x10 grid

### Scoring System
- Efficiency: Waste reduced / turns taken
- Speed: Turns to victory
- Savings: Total monthly savings achieved

## 📦 Dependencies

- **Bevy 0.14** - Game engine (ECS, rendering, input)
- **rand** - Procedural grid generation
- **serde** - Save/load game state
- **wasm-bindgen** - WebAssembly bindings

## 🚢 Deployment

### GitHub Pages
```bash
# Build WASM
./build-wasm.sh

# Create gh-pages branch
git checkout -b gh-pages
git add index.html pkg/ serve.py
git commit -m "Deploy game"
git push origin gh-pages

# Access at: https://[username].github.io/pirate-finops
```

### Vercel/Netlify
1. Build with `./build-wasm.sh`
2. Deploy the `index.html` and `pkg/` directory
3. Configure headers for SharedArrayBuffer support

## 🐛 Troubleshooting

### Game won't load in browser
- Check browser console for errors
- Ensure you're using a modern browser (Chrome 90+, Firefox 89+, Safari 15+)
- Try clearing cache and refreshing

### Build fails
- Update Rust: `rustup update`
- Clear build cache: `cargo clean`
- Reinstall wasm-bindgen: `cargo install wasm-bindgen-cli --force`

### Performance issues
- Reduce grid size in GameData::new()
- Disable animations in RevealAnimation
- Use release build: `cargo build --release`

## 📄 License

MIT License - Feel free to modify and share!

## 🤝 Contributing

Contributions welcome! Ideas for improvements:
- More Azure resource types
- Multiplayer leaderboard
- Daily challenges
- Power-ups and special events
- Mobile touch controls
- Sound effects and music

---

Built with 🦀 Rust + Bevy | Hunt wisely, optimize ruthlessly! 🏴‍☠️