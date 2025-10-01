use pirate_finops::resources::Difficulty;
use pirate_finops::simulator::run_simulation_suite;

fn main() {
    println!("\n🏴‍☠️ Running Pirate FinOps Balance Simulation...\n");

    println!("Testing Strategic Gameplay (Normal Difficulty Only):");
    println!("====================================================");
    let report = run_simulation_suite(100, Difficulty::Normal);
    report.print();

    println!("\n🎯 Strategic Analysis:");
    println!("  - Critical resources (AKS, Reservations): $300-500/mo savings");
    println!("  - High-value resources (VMs, App Services): $150-250/mo savings");
    println!("  - Medium resources (Logs): $120-150/mo savings");
    println!("  - Low-priority resources (Disks, IPs): $15-40/mo savings");
    println!("  - Players must prioritize high-value fixes and skip low-value ones");
}