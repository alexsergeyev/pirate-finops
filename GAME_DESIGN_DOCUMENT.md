# 🏴‍☠️ Pirate FinOps: Game Design Document

## Game Overview

**Title:** Pirate FinOps Treasure Hunt
**Genre:** Strategic Resource Management Puzzle
**Platform:** Web Browser (WebAssembly)
**Engine:** Bevy (Rust)
**Target Audience:** Cloud engineers, FinOps practitioners, IT professionals
**Playtime:** 5-10 minutes per session
**Win Rate Target:** 20-30%

## Core Concept

Players navigate a pirate ship through foggy waters to discover and fix Azure cloud resource waste. The game combines exploration, resource management, and strategic decision-making where players must choose which resources to fix and which to abandon as they race against a dwindling budget.

---

## Game Mechanics

### 1. Movement & Exploration

#### Controls
- **Arrow Keys/WASD:** Move ship one tile in cardinal directions
- **Space:** Fix resource at current position
- **Movement Cost:** $100 per move

#### Scanning System
- **Automatic Scan:** Moving reveals 3 tiles in an arc ahead
- **Scan Pattern:** Left-ahead, center-ahead, right-ahead tiles
- **Distance:** Adjacent tiles only (distance = 1)
- **Ship Rotation:** Ship visually rotates to face movement direction

### 2. Grid System

- **Size:** 8x8 tiles
- **Starting Position:** Center of grid (4,4)
- **Tile Types:**
  - Empty (Clear waters) - No resources
  - Resource tiles - Contains Azure waste to fix
- **Resource Density:** ~40% of tiles contain resources
- **Fog of War:** Tiles start hidden, revealed by scanning

### 3. Resource System

#### Resource Distribution
Resources are randomly distributed with weighted frequency:
- **Critical (Rare):** 3 types, low spawn rate
- **High Value (Common):** 4 types, medium spawn rate
- **Medium Value (Common):** 3 types, medium spawn rate
- **Low Value (Very Common):** 6 types, high spawn rate

#### Complete Resource List

| Resource | Category | Waste/mo | Fix Time | Savings/mo | ROI |
|----------|----------|----------|----------|------------|-----|
| **GPU Cluster (Idle)** | Critical | $450 | 3 sprints | $600-750 | 1.5x |
| **OpenShift (Overprovisioned)** | Critical | $320 | 3 sprints | $420-520 | 1.4x |
| **Reservations (Missed)** | Critical | $280 | 2 sprints | $380-480 | 1.5x |
| **SQL Database (Unused)** | High | $180 | 2 sprints | $240-300 | 1.5x |
| **Cosmos DB (Overprovisioned)** | High | $160 | 3 sprints | $210-280 | 1.5x |
| **Virtual Machines (Idle)** | High | $150 | 2 sprints | $200-250 | 1.5x |
| **App Service (Oversized)** | High | $120 | 2 sprints | $160-200 | 1.5x |
| **Load Balancer (Redundant)** | Medium | $90 | 2 sprints | $110-140 | 1.3x |
| **Log Ingestion (Bloat)** | Medium | $80 | 2 sprints | $100-130 | 1.4x |
| **Backups (Expired)** | Medium | $60 | 1 sprint | $75-95 | 1.4x |
| **CDN Endpoint (Idle)** | Low | $35 | 1 sprint | $40-50 | 1.3x |
| **Disk (Orphaned)** | Low | $30 | 1 sprint | $35-45 | 1.3x |
| **Snapshot (Stale)** | Low | $25 | 1 sprint | $28-35 | 1.2x |
| **Public IP (Unused)** | Low | $20 | 1 sprint | $22-28 | 1.2x |
| **Storage Account (Empty)** | Low | $15 | 1 sprint | $16-20 | 1.2x |
| **Resource (Untagged)** | Low | $10 | 1 sprint | $10-12 | 1.1x |

### 4. Strategic Mechanics

#### Resource Decay System
- **Trigger:** Every 5 sprints
- **Effect:** All unfixed resources increase by 20%
- **Compound:** Decay stacks (1.2x, 1.44x, 1.73x, etc.)
- **Strategy:** Forces prioritization - can't fix everything

#### Waste Neighborhoods
- **Adjacent Effect:** Only cardinal directions (not diagonal)
- **Unfixed Neighbor:** +10% waste per unfixed adjacent resource
- **Fixed Neighbor:** -10% waste per fixed adjacent resource
- **Stacking:** Effects multiply (e.g., 2 unfixed = 1.1 × 1.1 = 1.21x)
- **Strategy:** Fixing clusters provides compound benefits

#### Combo System
- **Same Type:** Fixing same resource type consecutively
- **Related Types:** Fixing related resources (e.g., VM + GPU)
- **Multiplier:** +0.2x per combo, max 2.0x
- **Reset:** Breaks when fixing unrelated resource

#### Related Resource Groups
- **Compute:** VMs, GPU Clusters, Reservations
- **Storage:** Disks, Snapshots, Backups, Storage Accounts
- **Database:** SQL Database, Cosmos DB
- **Container:** OpenShift, App Services, Load Balancers
- **Network:** Public IPs, CDN Endpoints, Load Balancers
- **Monitoring:** Log Ingestion, Backups

### 5. Economy System

#### Starting Resources
- **Budget:** $23,000
- **Sprint:** 0
- **Grid:** 8×8 (64 tiles)

#### Burn Rate Calculation
```
Base Burn = $30/sprint
Waste Burn = Total Unfixed Waste × 0.5
Total Burn = Base Burn + Waste Burn
```

#### Sprint Triggers
- **Fix Action:** Costs 1-3 sprints (varies by resource)
- **Movement:** No sprint cost (only $ cost)
- **Each Sprint:** Apply burn rate to budget

### 6. Win/Loss Conditions

#### Win Conditions (Multiple Paths)
1. **Elimination Victory:** Total waste ≤ $100
2. **Savings Victory:** Monthly savings ≥ $1,500 AND waste ≤ 40%
3. **Combo Master:** Combo multiplier ≥ 1.8x AND waste ≤ $300
4. **Balanced Victory:** Budget > $1,000 AND savings > $800 AND waste ≤ 50%

#### Loss Condition
- **Budget Depletion:** Budget ≤ $0

#### Victory Check
- Checked after sprint 5 (gives time to explore)
- Multiple paths encourage different strategies

---

## User Interface

### HUD Elements

#### Top Bar
- **Budget:** Current funds (color-coded by amount)
  - Green: > $5,000
  - Yellow: $2,000 - $5,000
  - Red: < $2,000
- **Sprint:** Current sprint number
- **Waste %:** Percentage of burn from waste
- **Savings:** Monthly savings from fixed resources
- **Burn Rate:** Current $ per sprint
- **Combo:** Current combo multiplier

#### Current Tile Panel
- Shows resource at player's position
- Displays waste cost, fix time, potential savings
- Status indicators: ✓ Fixed, ⚙ Fixing, ⚠ Available

#### Controls Panel
- "Move: Arrow/WASD ($100 + auto-scan)"
- "Fix: Space (time varies by resource)"

### Visual Design

#### Tile States
- **Unrevealed:** Dark fog
- **Empty (Water):** Blue (0.3, 0.5, 0.7)
- **Resource (Land):** Sandy (0.8, 0.7, 0.5)
- **Fixed:** Green (0.3, 0.7, 0.4)
- **Fixing:** Orange animation

#### Resource Icons
- Pattern textures from Kenney Pattern Pack 2
- Unique pattern for each resource type
- 30×30 pixel sprites on 64×64 tiles

#### Ship
- Pirate ship sprite from Kenney Pirate Pack
- Rotates to face movement direction
- Size: 80% of tile (51×51 pixels)

---

## Game Balance

### Difficulty Tuning
- **Starting Budget:** $23,000
- **Resource Distribution:** 40% of tiles
- **Movement Cost:** $100
- **Base Burn:** $30/sprint
- **Waste Contribution:** 50% of unfixed waste
- **Decay Rate:** 20% every 5 sprints
- **Neighborhood Effect:** ±10% per adjacent

### Strategic Decisions

#### Key Choices
1. **Exploration vs Exploitation:** Scan more or fix known resources?
2. **High Value vs Clusters:** Chase criticals or fix neighborhoods?
3. **Decay Management:** Fix before decay or abandon low-value?
4. **Route Optimization:** Minimize movement cost
5. **Combo Planning:** Chain related resources or take best available?

#### Optimal Strategies
- **Rush Strategy:** Find and fix critical resources quickly
- **Cluster Strategy:** Focus on dense resource neighborhoods
- **Combo Strategy:** Chain related resources for multipliers
- **Balanced Strategy:** Mix of high-value and strategic fixes

### Win Rate Analysis
- **Target:** 20-30% win rate
- **Current:** ~25% (based on simulations)
- **Average Game Length:** 20-30 sprints
- **Decision Points:** ~15-20 meaningful choices per game

---

## Technical Implementation

### Architecture
- **Language:** Rust
- **Engine:** Bevy 0.14
- **Rendering:** WebGL2
- **Deployment:** WebAssembly
- **Build Size:** ~5MB compressed

### Core Systems
- **ECS Components:**
  - Tile, TileState, ResourceType
  - Player, PlayerMarker
  - TileIcon, GameUI elements

- **Resources:**
  - GameData (global state)
  - GameState enum (Menu, Playing, GameOver)

- **Systems:**
  - Movement & scanning
  - Resource fixing
  - Decay & neighborhoods
  - UI updates
  - Win/loss checking

### Performance
- **Frame Rate:** 60 FPS target
- **Load Time:** < 3 seconds
- **Memory:** < 100MB

---

## Future Enhancements

### Potential Features
1. **Difficulty Modes:** Easy/Normal/Hard with different budgets
2. **Daily Challenges:** Preset maps with leaderboards
3. **Achievement System:** Unlock bonuses for specific accomplishments
4. **Tutorial Mode:** Guided first game
5. **Statistics Tracking:** Win rate, best scores, strategies used

### Content Expansion
- Additional Azure resource types
- Special event tiles (budget boosts, instant fixes)
- Weather effects affecting movement/scanning
- Multi-stage campaigns

---

## Design Principles

1. **Simple Controls, Complex Decisions**
   - Two buttons (move + fix)
   - Every decision has multiple factors

2. **Meaningful Trade-offs**
   - No perfect solution
   - Multiple valid strategies

3. **Emergent Complexity**
   - Simple rules create deep gameplay
   - Different optimal plays for different maps

4. **Educational Value**
   - Learn real Azure cost optimization
   - Understand resource relationships

5. **Respect Player Time**
   - 5-10 minute sessions
   - No grinding or progression gates
   - Immediate gameplay

---

## Metrics & KPIs

- **Engagement:** Average session length (target: 7 minutes)
- **Retention:** Return player rate (target: 40%)
- **Completion:** Games finished vs abandoned (target: 80%)
- **Strategy Diversity:** Distribution of win strategies
- **Learning Curve:** Win rate improvement over first 10 games

---

## Version History

### v1.0 - Current Build
- Core gameplay loop
- 17 Azure resource types
- Decay & neighborhood mechanics
- Combo system
- Multiple win conditions

### v0.5 - MVP
- Basic movement and fixing
- 9 resource types
- Simple burn rate
- Single win condition

---

*Last Updated: November 2024*
*Game Designer: Claude & Human Collaboration*
*Target Platform: Web Browser (WASM)*