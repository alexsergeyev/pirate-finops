# 🏴‍☠️ Pirate FinOps - Rebalanced Game Guide

## 🎮 New Balance with True Burn Mechanic

### Core Mechanic: ALL Waste Burns From Start
- **Hidden waste costs money** - just like real cloud resources
- **Revealing is discovery** - doesn't increase burn
- **Fixing reduces burn** - the only way to save money

## 📊 Balanced Economics

### Starting Conditions (Normal Mode)
- **Budget**: $20,000
- **Base Burn**: $100/turn
- **Resource Density**: 35% of tiles (22 resources on 8x8)
- **Movement Cost**: $20 per move
- **Expected Initial Waste**: $500-800

### Typical Burn Scenarios
| Situation | Calculation | Total Burn |
|-----------|-------------|------------|
| Game Start | $100 + ~$650 | ~$750/turn |
| After fixing 2 big items | $100 + ~$350 | ~$450/turn |
| Near victory | $100 + ~$150 | ~$250/turn |

### Resource Costs & Savings
| Resource | Waste/Turn | Fix Turns | Savings Range |
|----------|------------|-----------|---------------|
| **Idle VM** | $120 | 2 | $140-180 |
| **Orphaned Disk** | $30 | 1 | $40-50 |
| **Overprovisioned AKS** | $250 | 3 | $300-350 |
| **Unused Public IP** | $5 | 1 | $8-10 |
| **Stale Snapshot** | $20 | 1 | $25-35 |
| **Missed Reservation** | $200 | 3 | $250-300 |
| **Log Ingestion** | $60 | 2 | $80-100 |
| **Oversized App Service** | $100 | 2 | $130-170 |

## 🎯 Win Conditions (More Achievable)

**Win if EITHER:**
- Waste ≤ 25% of total burn
- Monthly Savings ≥ $2,500

This means you can win by:
1. **Efficiency Route**: Fix enough to get waste down to 25%
2. **Savings Route**: Fix high-value items for $2,500+ savings

## 💰 Budget Analysis

### Normal Mode Timeline
```
Turn 0:  $20,000 (Start)
Turn 5:  ~$16,000 (Exploration phase, ~$750/turn)
Turn 10: ~$12,500 (Started fixing, ~$600/turn)
Turn 15: ~$9,500 (Major fixes done, ~$400/turn)
Turn 20: ~$7,000 (Optimization, ~$300/turn)
Turn 25: ~$5,000 (Victory likely)
```

### Movement Budget
- $20 per move = 1,000 total moves possible
- Typical game uses 100-150 moves
- Movement cost = $2,000-3,000 total

## 🎮 Strategic Play Guide

### Phase 1: Discovery (Turns 1-7)
**Goal**: Find the big burners
- Use burn rate to estimate total waste
- Scout efficiently with Space reveal
- Identify AKS and Reservations
- Budget: $20,000 → ~$15,000

### Phase 2: Triage (Turns 8-15)
**Goal**: Fix high-impact items
- Priority: AKS ($250) > Reservations ($200) > VMs ($120)
- Ignore small items initially
- Watch burn rate drop
- Budget: ~$15,000 → ~$10,000

### Phase 3: Victory Push (Turns 16-25)
**Goal**: Achieve win condition
- Either push waste% down OR savings up
- Fix medium items for final push
- Clean up for victory
- Budget: ~$10,000 → Victory!

## 📈 Difficulty Comparison

| Difficulty | Budget | Grid | Base Burn | Resource % | Challenge |
|------------|--------|------|-----------|------------|-----------|
| **Easy** | $25,000 | 6x6 | $50 | 35% | Forgiving |
| **Normal** | $20,000 | 8x8 | $100 | 35% | Balanced |
| **Hard** | $15,000 | 10x10 | $150 | 35% | Challenging |

## 🏆 Success Metrics

### Skill Levels
- **Beginner**: Win with <$5,000 remaining
- **Intermediate**: Win with $5,000-10,000 remaining
- **Expert**: Win with >$10,000 remaining
- **Master**: Win in <20 turns

### Estimated Win Rates
- Easy: 70-80%
- Normal: 50-60%
- Hard: 30-40%

## 💡 Tips for Success

### Do's ✅
1. **Watch the burn rate** - It's your key metric
2. **Calculate waste** - Burn minus base = total waste
3. **Fix big first** - AKS and Reservations priority
4. **Plan routes** - Movement costs add up
5. **Use both win conditions** - Pick easier path

### Don'ts ❌
1. **Don't explore aimlessly** - $20 per move
2. **Don't fix tiny items early** - Not worth the turns
3. **Don't reveal everything** - You know the total from burn
4. **Don't panic** - You have more budget now
5. **Don't ignore savings** - Easier win condition

## 🔥 Understanding Burn Rate

### Reading the Burn Display
- **Orange (<$500)**: Good progress
- **Orange ($500-800)**: Need action
- **Red (>$800)**: Critical - fix immediately

### Quick Math
```
Current Burn - Base Burn = Active Waste
$750 - $100 = $650 of waste somewhere

After fixing AKS:
$500 - $100 = $400 of waste remaining
```

## 🎲 Example Winning Game

```
Turn 1-5: Explore, find AKS and 2 VMs
  Burn: $750 → Budget: $20,000 → $16,250

Turn 6-8: Fix first VM (2 turns)
  Burn: $750 → $630 → Budget: $16,250 → $14,140

Turn 9-11: Start fixing AKS (3 turns)
  Burn: $630 → Budget: $14,140 → $12,250

Turn 12: AKS fixed!
  Burn: $380 → Savings: $325/mo

Turn 13-18: Fix more resources
  Burn: $380 → $250
  Savings: $2,600/mo

Turn 19: VICTORY!
  Win condition: Savings > $2,500 ✓
  Budget remaining: $8,000+
```

## 🚀 Why This Balance Works

1. **Realistic pressure** without being overwhelming
2. **Multiple victory paths** for different strategies
3. **Meaningful choices** between exploration and action
4. **Clear feedback** through burn rate display
5. **Fair challenge** that rewards smart play

The game is now properly balanced to be challenging but fair, with the burn mechanic creating authentic cloud cost management pressure!