// ============================================================================
//  PerfReport (Rust)  |  IS2083 Lab 3
// ============================================================================
//  Rust migration of the Windows-only C++ PerfReport. It reports processors,
//  memory, uptime, and disk usage, then an overall efficiency score. The
//  sysinfo crate hides OS differences, so this one code base runs on Windows,
//  Linux, macOS, and inside a container.
// ============================================================================

use sysinfo::{Disks, System};

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// --- Given: findings + scoring (do not change) ------------------------------
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");

    // This line makes the sysinfo tools available. You will read from `sys`.
    let mut sys = System::new_all();
    sys.refresh_all();

    // Section 1: processors.
    let cpus = sys.cpus().len();
    println!("Logical processors : {}", cpus);

    // Section 2: memory (sysinfo reports bytes).
    let total = sys.total_memory();
    let used = sys.used_memory();
    let mem_load = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    println!("Memory total (GB)  : {:.2}", total as f64 / GB);
    println!("Memory free  (GB)  : {:.2}", (total - used) as f64 / GB);
    println!("Memory in use (%)  : {}", mem_load);

    // Section 3: uptime (seconds -> minutes).
    println!("Uptime (minutes)   : {}", System::uptime() / 60);

    // Section 4: disk (first disk in the list).
    let disks = Disks::new_with_refreshed_list();
    let mut disk_used_pct: f64 = 0.0;
    if let Some(disk) = disks.list().first() {
        let dtotal = disk.total_space();
        let dfree = disk.available_space();
        let mut used_pct = 0.0;
        if dtotal > 0 {
            used_pct = 100.0 * (dtotal - dfree) as f64 / dtotal as f64;
        }
        println!("Disk total (GB)    : {:.2}", dtotal as f64 / GB);
        println!("Disk free  (GB)    : {:.2}", dfree as f64 / GB);
        println!("Disk used  (%)     : {:.2}", used_pct);
        disk_used_pct = used_pct;
    }

    // Section 5: score.
    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!("Efficiency score   : {} / 100 ({})", score, rating(score));
    println!("Done.");
}

// --- Tests: the score rule must match the C++ version exactly ---------------
#[cfg(test)]
mod tests {
    use super::*;

    fn score(mem_load: u64, disk_used_pct: f64) -> i32 {
        compute_score(&Findings {
            mem_load,
            disk_used_pct,
        })
    }

    #[test]
    fn healthy_machine_scores_100_good() {
        assert_eq!(score(40, 50.0), 100);
        assert_eq!(rating(100), "GOOD");
    }

    #[test]
    fn moderate_memory_costs_10() {
        assert_eq!(score(71, 10.0), 90);
        assert_eq!(score(70, 10.0), 100); // boundary: must be strictly above 70
    }

    #[test]
    fn high_memory_costs_25() {
        assert_eq!(score(86, 10.0), 75);
        assert_eq!(score(85, 10.0), 90); // boundary: 85 falls in the 10-point band
    }

    #[test]
    fn disk_penalties() {
        assert_eq!(score(10, 75.5), 90);
        assert_eq!(score(10, 90.5), 75);
        assert_eq!(score(10, 75.0), 100);
    }

    #[test]
    fn worst_case_is_50_poor() {
        assert_eq!(score(99, 99.0), 50);
        assert_eq!(rating(50), "POOR");
    }

    #[test]
    fn rating_thresholds() {
        assert_eq!(rating(85), "GOOD");
        assert_eq!(rating(84), "FAIR");
        assert_eq!(rating(60), "FAIR");
        assert_eq!(rating(59), "POOR");
    }
}
