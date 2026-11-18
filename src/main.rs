mod cli;
mod collector;
mod error;
mod model;
mod util;

use std::cmp::Ordering;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use collector::{Collector, SystemCollector};
use model::Snapshot;
use util::{calc_percent, format_bytes, format_duration, format_percent};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Snapshot { json }) => run_snapshot(json)?,
        None => {
            println!("Anteiku v{}", env!("CARGO_PKG_VERSION"));
            println!("La TUI en vivo llegará en un hito posterior (M3).");
            println!("Por ahora prueba: anteiku snapshot [--json] o anteiku --help");
        }
    }

    Ok(())
}

fn run_snapshot(json: bool) -> Result<()> {
    let mut collector = SystemCollector::new()?;
    let snapshot = collector.collect()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    } else {
        print_snapshot(&snapshot);
    }

    Ok(())
}

fn print_snapshot(snap: &Snapshot) {
    println!("=== Anteiku Snapshot ({}) ===", snap.timestamp.to_rfc3339());
    println!(
        "Sistema: {} | Kernel: {} | Host: {} | Usuario: {} | Uptime: {}",
        snap.system.os_name,
        snap.system.kernel,
        snap.system.hostname,
        snap.system.username,
        format_duration(snap.system.uptime_secs)
    );

    let freq_text = snap
        .cpu
        .frequency_mhz
        .map(|mhz| format!(" ({mhz} MHz)"))
        .unwrap_or_default();
    println!(
        "\nCPU: {} ({} núcleos){freq_text}",
        format_percent(snap.cpu.total_percent),
        snap.cpu.per_core.len()
    );

    let ram_pct = calc_percent(snap.memory.used_bytes, snap.memory.total_bytes);
    let swap_pct = calc_percent(snap.memory.swap_used_bytes, snap.memory.swap_total_bytes);
    println!(
        "RAM: {} / {} ({}) | Swap: {} / {} ({})",
        format_bytes(snap.memory.used_bytes),
        format_bytes(snap.memory.total_bytes),
        format_percent(ram_pct),
        format_bytes(snap.memory.swap_used_bytes),
        format_bytes(snap.memory.swap_total_bytes),
        format_percent(swap_pct),
    );

    if !snap.disks.is_empty() {
        println!("\nDiscos:");
        for disk in &snap.disks {
            let pct = calc_percent(disk.used_bytes, disk.total_bytes);
            println!(
                "  {:<20} {} / {} ({})",
                disk.mount_point,
                format_bytes(disk.used_bytes),
                format_bytes(disk.total_bytes),
                format_percent(pct),
            );
        }
    }

    if !snap.networks.is_empty() {
        println!("\nRed:");
        for net in &snap.networks {
            println!(
                "  {:<15} ↓ {}/s  ↑ {}/s",
                net.interface,
                format_bytes(net.rx_bytes_per_sec),
                format_bytes(net.tx_bytes_per_sec),
            );
        }
    }

    if !snap.temperatures.is_empty() {
        println!("\nTemperaturas:");
        for temp in &snap.temperatures {
            println!("  {:<25} {:.1} °C", temp.label, temp.celsius);
        }
    }

    let mut top_procs = snap.processes.clone();
    top_procs.sort_by(|a, b| {
        b.cpu_percent
            .partial_cmp(&a.cpu_percent)
            .unwrap_or(Ordering::Equal)
    });

    println!("\nTop 5 procesos (por CPU):");
    println!(
        "  {:<8} {:<24} {:>7} {:>10}",
        "PID", "NOMBRE", "CPU%", "MEM"
    );
    for proc_ in top_procs.iter().take(5) {
        println!(
            "  {:<8} {:<24} {:>6}% {:>10}",
            proc_.pid,
            proc_.name,
            format!("{:.1}", proc_.cpu_percent),
            format_bytes(proc_.memory_bytes),
        );
    }
}
