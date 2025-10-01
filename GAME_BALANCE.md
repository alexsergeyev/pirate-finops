# 🏴‍☠️ Pirate FinOps - Game Balance Guide (CHALLENGING MODE)

## ⚔️ Difficulty Rebalanced for Challenge

### Key Changes for Increased Difficulty

#### 1. **Movement Costs Money**
- Each move costs $50
- Strategic routing is crucial
- Can't freely explore the entire map

#### 2. **Reduced Starting Budget**
- Normal: $8,000 (was $15,000)
- Easy: $12,000
- Hard: $5,000

#### 3. **Higher Base Burn Rate**
- Normal: $300/turn
- Easy: $200/turn
- Hard: $400/turn

#### 4. **Higher Waste Costs**
| Resource | Waste Cost | Fix Turns | Savings Range |
|----------|------------|-----------|---------------|
| Idle VM | $150 | 2 | $180-220 |
| Orphaned Disk | $40 | 1 | $50-70 |
| Overprovisioned AKS | $300 | 3 | $350-450 |
| Unused Public IP | $10 | 1 | $12-18 |
| Stale Snapshot | $25 | 1 | $30-40 |
| Missed Reservation | $250 | 3 | $280-350 |
| Log Ingestion Bloat | $80 | 2 | $100-140 |
| Oversized App Service | $120 | 2 | $150-200 |

#### 5. **Harder Win Conditions**
- Must achieve BOTH:
  - Waste ≤ 15% (was 20%)
  - Savings ≥ $3,000/mo (was $2,000)
- Both conditions required (not OR)

#### 6. **Denser Resource Grid**
- 50% of tiles have resources (was 40%)
- More waste to navigate around
- Harder to find safe paths

## 📊 Strategic Considerations

### Budget Management (Normal Mode)
- **Starting**: $8,000
- **Base burn**: $300/turn = ~26 turns max
- **Movement**: $50 each = 160 moves max
- **Typical game**: 20-30 turns + 40-60 moves

### Optimal Strategy
1. **Scout Efficiently**: Plan routes, don't wander
2. **Prioritize High Value**: AKS ($300 waste → $400 savings)
3. **Batch Operations**: Reveal multiple tiles per turn
4. **Fix Critical Items**: Focus on high waste first
5. **Path Planning**: Minimize movement between targets

### Turn Economy
| Action | Cost | Notes |
|--------|------|-------|
| Move | $50 | Plan efficient routes |
| Wait | $300+ | Base burn + revealed waste |
| Reveal (Space) | 1 turn | Reveals 3x3 area |
| Fix | 1-3 turns | Worth it for high-value items |

## 🎯 Challenge Rating

### Difficulty Assessment
- **Win Rate**: ~40-60% with good strategy
- **Skill Required**: Medium-High
- **Planning**: Essential
- **Luck Factor**: Moderate (resource placement)

### Common Failure Points
1. **Over-exploration**: Spending too much on movement
2. **Poor prioritization**: Fixing low-value items first
3. **Revealing too much**: Each revealed waste increases burn
4. **Slow fixing**: High-turn fixes drain budget

## 🏆 Victory Path

### Efficient Win Strategy (~25-30 turns)
1. **Turns 1-5**: Strategic scouting ($250 movement)
2. **Turns 6-10**: Reveal high-value clusters
3. **Turns 11-20**: Fix major waste (AKS, VMs, Reservations)
4. **Turns 21-25**: Clean up remaining for 15% threshold
5. **Victory**: $3,000+ savings with <15% waste

### Budget Breakdown (Typical Win)
- Movement: $1,500 (30 moves)
- Turn costs: $6,000 (20 turns @ $300 base)
- Waste burden: $2,000 (accumulated)
- **Total spent**: ~$7,500
- **Remaining**: $500 buffer

## 🎮 Tips for Success

### Do's
- Plan your route before moving
- Fix high-waste items first (AKS, Reservations)
- Use Space reveal strategically
- Track your burn rate constantly
- Aim for both win conditions early

### Don'ts
- Don't explore randomly
- Don't reveal everything
- Don't fix small items early
- Don't ignore the dual win requirement
- Don't waste moves backtracking

## 📈 Difficulty Progression

### Easy Mode
- Budget: $12,000
- Grid: 6x6
- More forgiving for learning

### Normal Mode
- Budget: $8,000
- Grid: 8x8
- Balanced challenge

### Hard Mode
- Budget: $5,000
- Grid: 10x10
- For experts only

## 🎲 Success Metrics

- **Perfect Run**: Win in <20 turns with $2,000+ remaining
- **Good Run**: Win in 25-30 turns
- **Close Call**: Win with <$500 remaining
- **Learning Run**: Lose but understand why

The game now requires careful planning, efficient movement, and strategic decision-making. Every move matters!