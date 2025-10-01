use crate::components::ResourceType;
use crate::resources::{Difficulty, GameData, generate_random_grid};
use rand::prelude::*;

#[derive(Debug)]
struct SimulationResult {
    won: bool,
    sprints: u32,
    final_budget: i32,
    final_waste: i32,
    final_savings: i32,
    waste_percentage: f32,
}

pub fn run_simulation_suite(num_simulations: u32, difficulty: Difficulty) -> SimulationReport {
    let mut wins = 0;
    let mut total_sprints_to_win = 0;
    let mut total_sprints_to_lose = 0;
    let mut min_sprints_to_win = u32::MAX;
    let mut max_sprints_to_win = 0;
    let mut avg_final_budget_win = 0;
    let mut avg_final_budget_lose = 0;
    let mut waste_percentage_wins = Vec::new();
    let mut savings_wins = Vec::new();

    for i in 0..num_simulations {
        let result = simulate_single_game(difficulty);

        if result.won {
            wins += 1;
            total_sprints_to_win += result.sprints;
            min_sprints_to_win = min_sprints_to_win.min(result.sprints);
            max_sprints_to_win = max_sprints_to_win.max(result.sprints);
            avg_final_budget_win += result.final_budget;
            waste_percentage_wins.push(result.waste_percentage);
            savings_wins.push(result.final_savings);
        } else {
            total_sprints_to_lose += result.sprints;
            avg_final_budget_lose += result.final_budget;
        }

        if i % 20 == 0 {
            println!("Simulated {} games...", i);
        }
    }

    let losses = num_simulations - wins;
    let win_rate = (wins as f32 / num_simulations as f32) * 100.0;

    let avg_sprints_win = if wins > 0 {
        total_sprints_to_win as f32 / wins as f32
    } else {
        0.0
    };

    let avg_sprints_lose = if losses > 0 {
        total_sprints_to_lose as f32 / losses as f32
    } else {
        0.0
    };

    let avg_budget_win = if wins > 0 {
        avg_final_budget_win / wins as i32
    } else {
        0
    };

    let avg_budget_lose = if losses > 0 {
        avg_final_budget_lose / losses as i32
    } else {
        0
    };

    let avg_waste_percentage = if !waste_percentage_wins.is_empty() {
        waste_percentage_wins.iter().sum::<f32>() / waste_percentage_wins.len() as f32
    } else {
        0.0
    };

    let avg_savings = if !savings_wins.is_empty() {
        savings_wins.iter().sum::<i32>() / savings_wins.len() as i32
    } else {
        0
    };

    SimulationReport {
        total_games: num_simulations,
        wins,
        losses,
        win_rate,
        avg_sprints_to_win: avg_sprints_win,
        avg_sprints_to_lose: avg_sprints_lose,
        min_sprints_to_win: if wins > 0 { min_sprints_to_win } else { 0 },
        max_sprints_to_win,
        avg_final_budget_win: avg_budget_win,
        avg_final_budget_lose: avg_budget_lose,
        avg_waste_percentage_win: avg_waste_percentage,
        avg_savings_win: avg_savings,
    }
}

fn simulate_single_game(difficulty: Difficulty) -> SimulationResult {
    let mut game_data = GameData::new(difficulty);
    let grid = generate_random_grid(game_data.grid_size);

    // Calculate initial total waste from all resources on the map
    let mut initial_waste = 0;
    for x in 0..game_data.grid_size {
        for y in 0..game_data.grid_size {
            let resource = &grid[x as usize][y as usize];
            if !matches!(resource, ResourceType::Empty) {
                initial_waste += resource.waste_cost();
            }
        }
    }
    game_data.total_waste = initial_waste;

    // Simulate optimal play strategy
    let mut resources_to_fix = Vec::new();
    for x in 0..game_data.grid_size {
        for y in 0..game_data.grid_size {
            let resource = &grid[x as usize][y as usize];
            if !matches!(resource, ResourceType::Empty) {
                let waste = resource.waste_cost();
                let (min_savings, max_savings) = resource.savings_range();
                let avg_savings = (min_savings + max_savings) / 2;
                let fix_sprints = resource.fix_sprints();
                let efficiency = (waste + avg_savings) as f32 / fix_sprints as f32;
                resources_to_fix.push((waste, avg_savings, fix_sprints, efficiency, x, y));
            }
        }
    }

    // Sort by efficiency (higher is better)
    resources_to_fix.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());

    // Simulate fixing resources in order of efficiency
    let mut sprints = 0;
    let mut resources_fixed = 0;

    for (waste, savings, fix_sprints, _efficiency, _x, _y) in resources_to_fix.iter() {
        // Simulate movement cost (average 3 moves per resource)
        let movement_cost = 20 * 3;
        game_data.budget -= movement_cost;

        // Simulate reveal cost (1 sprint of burn)
        sprints += 1;
        game_data.budget -= game_data.calculate_burn_rate();

        if game_data.budget <= 0 {
            break;
        }

        // Simulate fixing (always 1 sprint now)
        sprints += 1;
        game_data.budget -= game_data.calculate_burn_rate();

        if game_data.budget <= 0 {
            break;
        }

        // Resource is fixed
        game_data.total_waste -= waste;

        // Apply combo multiplier (simplified for simulator)
        let combo_bonus = if resources_fixed > 0 { 1.2 } else { 1.0 };
        let effective_savings = (*savings as f32 * combo_bonus) as i32;
        game_data.monthly_savings += effective_savings;

        // Add adjacency bonus (simplified)
        if resources_fixed > 0 {
            game_data.adjacency_bonus += 50;
        }

        resources_fixed += 1;

        // Check win condition
        game_data.sprint = sprints;
        if sprints >= 5 {
            game_data.check_win_condition();
            if game_data.game_won {
                break;
            }
        }
    }

    SimulationResult {
        won: game_data.game_won,
        sprints,
        final_budget: game_data.budget.max(0),
        final_waste: game_data.total_waste,
        final_savings: game_data.monthly_savings,
        waste_percentage: game_data.waste_percentage(),
    }
}

#[derive(Debug)]
pub struct SimulationReport {
    pub total_games: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: f32,
    pub avg_sprints_to_win: f32,
    pub avg_sprints_to_lose: f32,
    pub min_sprints_to_win: u32,
    pub max_sprints_to_win: u32,
    pub avg_final_budget_win: i32,
    pub avg_final_budget_lose: i32,
    pub avg_waste_percentage_win: f32,
    pub avg_savings_win: i32,
}

impl SimulationReport {
    pub fn print(&self) {
        println!("\n🏴‍☠️ === Pirate FinOps Simulation Report === 🏴‍☠️");
        println!("\n📊 Overall Statistics:");
        println!("  Total Games: {}", self.total_games);
        println!("  Wins: {} | Losses: {}", self.wins, self.losses);
        println!("  🎯 Win Rate: {:.1}%", self.win_rate);

        println!("\n⏱️ Sprint Statistics:");
        if self.wins > 0 {
            println!("  Avg Sprints to Win: {:.1}", self.avg_sprints_to_win);
            println!("  Fastest Win: {} sprints", self.min_sprints_to_win);
            println!("  Slowest Win: {} sprints", self.max_sprints_to_win);
        }
        if self.losses > 0 {
            println!("  Avg Sprints to Lose: {:.1}", self.avg_sprints_to_lose);
        }

        println!("\n💰 Budget Statistics:");
        if self.wins > 0 {
            println!("  Avg Final Budget (Wins): ${}", self.avg_final_budget_win);
            println!("  Avg Waste % (Wins): {:.1}%", self.avg_waste_percentage_win);
            println!("  Avg Savings (Wins): ${}", self.avg_savings_win);
        }
        if self.losses > 0 {
            println!("  Avg Final Budget (Losses): ${}", self.avg_final_budget_lose);
        }

        println!("\n📈 Balance Assessment:");
        if self.win_rate < 10.0 {
            println!("  ⚠️ TOO HARD: Win rate is very low!");
        } else if self.win_rate < 20.0 {
            println!("  🔴 HARD: Win rate is below target (20-30%)");
        } else if self.win_rate <= 30.0 {
            println!("  ✅ PERFECT: Win rate is in target range (20-30%)");
        } else if self.win_rate <= 40.0 {
            println!("  🟡 SLIGHTLY EASY: Win rate is above target");
        } else {
            println!("  ⚠️ TOO EASY: Win rate is too high!");
        }
    }
}