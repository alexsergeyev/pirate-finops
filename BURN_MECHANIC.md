# 🔥 Burn Mechanic - How It Really Works

## Core Formula
**Burn Rate = Base Burn + ALL Waste on Map (fixed or not)**

The burn represents your **entire cloud infrastructure** burning money, whether you know about it or not!

## Key Concept: Hidden Waste Still Burns

### ALL waste items on the map burn budget from turn 1
- Hidden tiles with waste: **BURNING** (you just don't know it)
- Revealed waste tiles: **BURNING** (now you can see them)
- Fixed tiles: **NOT BURNING** (problem solved!)
- Empty tiles: Never burn

### This creates strategic priority:
1. **Find the biggest burners first** (exploration phase)
2. **Fix high-cost items immediately** (triage phase)
3. **Clean up remaining waste** (optimization phase)

## Example Game Flow

### Initial State (Turn 1)
```
Map has (hidden):
- 2x Idle VMs ($150 each) = $300
- 1x AKS ($300) = $300
- 3x Orphaned Disks ($40 each) = $120
- 5x Small items = $180
Total Waste: $900

Burn Rate = $300 (base) + $900 (waste) = $1,200/turn
Budget: $15,000 → $13,800
```

### After Exploration (Turn 5)
```
Revealed: 1 VM, 1 AKS, 2 Disks
Hidden: 1 VM, 1 Disk, 5 small items

Burn Rate = $300 + $900 = $1,200/turn (SAME!)
Budget: $13,800 - (4 × $1,200) = $9,000
```
**Revealing doesn't increase burn - it just shows you what's already burning!**

### After Fixing AKS (Turn 8)
```
Fixed: 1 AKS (-$300 waste)
Active Waste: $600

Burn Rate = $300 + $600 = $900/turn (REDUCED!)
Budget: $9,000 - (3 × $1,200) = $5,400
Savings: +$400/month
```

### Strategic Victory (Turn 20)
```
Fixed: All major items
Remaining Waste: $120 (small items)

Burn Rate = $300 + $120 = $420/turn
Monthly Savings: $3,200
Win Condition Met!
```

## Strategic Implications

### 1. **Time Pressure is Real**
- High initial burn means you're racing against time
- Can't leisurely explore - budget drains fast
- Must act decisively

### 2. **Prioritization is Critical**
```
Priority Order:
1. AKS ($300) - Fix immediately when found
2. Reservations ($250) - High impact
3. Idle VMs ($150) - Medium priority
4. App Services ($120) - Worth fixing
5. Small items (<$50) - Only if time permits
```

### 3. **Exploration Strategy**
- **Scout for big items** - Use Space reveal to find clusters
- **Don't reveal everything** - You already know total burn from UI
- **Focus on fixing** - Finding is useless without fixing

### 4. **Risk Management**
- Starting burn might be $800-1,500 depending on RNG
- Budget gives you 10-15 turns at high burn
- Each fix extends your runway
- Can win even without finding everything

## UI Indicators

### Burn Rate Display
Shows your current total burn rate:
- 🟢 Green: <$350/turn (manageable)
- 🟠 Orange: $350-500/turn (concerning)
- 🔴 Red: >$500/turn (critical)

### Budget Colors
- 🟢 Green: >$5,000 (healthy)
- 🟡 Yellow: $2,000-5,000 (caution)
- 🔴 Red: <$2,000 (danger)

## Winning Strategy

### Phase 1: Rapid Discovery (Turns 1-5)
- Move efficiently to reveal large areas
- Use Space to reveal 3x3 sections
- Identify the biggest burners
- Budget: $15,000 → ~$9,000

### Phase 2: Triage (Turns 6-15)
- Fix items with best waste/turn reduction
- Ignore small items initially
- Focus on items that take 1-2 turns
- Budget: ~$9,000 → ~$4,000

### Phase 3: Optimization (Turns 16-25)
- Clean up remaining waste for win condition
- Fix smaller items if needed
- Achieve both win conditions
- Budget: ~$4,000 → Victory!

## Key Differences from Before

| Aspect | Old System | New System |
|--------|------------|------------|
| Hidden tiles | No cost | Full waste cost |
| Revealing | Increases burn | No change to burn |
| Strategy | Avoid revealing | Find and fix ASAP |
| Pressure | Low initially | High from start |
| Priority | Explore carefully | Fix aggressively |

## Tips

1. **Watch the burn rate** - It tells you total waste even before revealing
2. **Do the math** - If burn is $1,200 and base is $300, there's $900 of waste somewhere
3. **Fix big first** - A single AKS fix can reduce burn by 25%
4. **Movement matters** - $50 per move adds up when exploring
5. **Time is money** - Literally! Each turn costs your burn rate

The burn mechanic now creates authentic cloud cost pressure - waste is burning your budget whether you see it or not, just like real cloud infrastructure!