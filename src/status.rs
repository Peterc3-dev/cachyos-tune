use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};

use crate::profiles::{Profile, ProfileName};
use crate::snapshot::Snapshot;
use crate::style;

/// Current system state as a flat vec of (knob, value) pairs.
fn current_settings() -> Vec<(&'static str, String)> {
    let snap = Snapshot::capture();
    vec![
        ("cpu_governor", snap.cpu_governor),
        ("vm.swappiness", snap.swappiness),
        ("vm.vfs_cache_pressure", snap.vfs_cache_pressure),
        ("vm.dirty_ratio", snap.dirty_ratio),
        ("vm.dirty_background_ratio", snap.dirty_background_ratio),
        ("vm.dirty_expire_centisecs", snap.dirty_expire_centisecs),
        ("vm.dirty_writeback_centisecs", snap.dirty_writeback_centisecs),
        ("transparent_hugepages", snap.transparent_hugepages),
        ("zram_comp_algorithm", snap.zram_comp_algorithm),
        ("gpu_power_profile", snap.gpu_power_profile),
        ("io_scheduler", snap.io_scheduler),
        ("tcp_congestion", snap.tcp_congestion),
    ]
}

fn profile_values(p: &Profile) -> Vec<String> {
    vec![
        p.cpu_governor.clone(),
        p.swappiness.to_string(),
        p.vfs_cache_pressure.to_string(),
        p.dirty_ratio.to_string(),
        p.dirty_background_ratio.to_string(),
        p.dirty_expire_centisecs.map_or("-".into(), |v| v.to_string()),
        p.dirty_writeback_centisecs.map_or("-".into(), |v| v.to_string()),
        p.transparent_hugepages.clone(),
        p.zram_comp_algorithm.clone(),
        p.gpu_power_profile.clone(),
        p.io_scheduler.clone(),
        p.tcp_congestion.clone(),
    ]
}

/// Print full status table comparing current settings against all profiles.
pub fn print_status() {
    style::header("System Status");

    let current = current_settings();
    let profiles = Profile::all_profiles();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Header row
    let mut header = vec![
        Cell::new("Knob")
            .fg(Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new("Current")
            .fg(Color::Green)
            .add_attribute(Attribute::Bold),
    ];
    for p in &profiles {
        header.push(
            Cell::new(p.name.to_string())
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
        );
    }
    table.set_header(header);

    // Data rows
    for (i, (knob, current_val)) in current.iter().enumerate() {
        let mut row = vec![
            Cell::new(*knob).fg(Color::DarkGreen),
            Cell::new(current_val).fg(Color::White),
        ];
        for p in &profiles {
            let pvals = profile_values(p);
            let pval = &pvals[i];
            if pval == current_val {
                row.push(Cell::new(format!("{} =", pval)).fg(Color::DarkGreen));
            } else {
                row.push(Cell::new(pval).fg(Color::Yellow));
            }
        }
        table.add_row(row);
    }

    println!("{}", table);

    // Show which profile matches best
    let mut best_match = ProfileName::Default;
    let mut best_count = 0;
    for p in &profiles {
        let pvals = profile_values(p);
        let count = current
            .iter()
            .enumerate()
            .filter(|(i, (_, cv))| &pvals[*i] == cv)
            .count();
        if count > best_count {
            best_count = count;
            best_match = p.name;
        }
    }

    println!(
        "\n  Closest profile: {} ({}/{} knobs match)\n",
        style::bold_green(&best_match.to_string()),
        best_count,
        current.len()
    );
}

/// Print diff: what would change if we applied the given profile.
pub fn print_diff(profile: &Profile) {
    style::header(&format!("Diff vs profile: {}", profile.name));

    let current = current_settings();
    let pvals = profile_values(profile);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Knob")
            .fg(Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new("Current")
            .fg(Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new("Target")
            .fg(Color::Green)
            .add_attribute(Attribute::Bold),
        Cell::new("Change?")
            .fg(Color::Green)
            .add_attribute(Attribute::Bold),
    ]);

    let mut change_count = 0;
    for (i, (knob, current_val)) in current.iter().enumerate() {
        let target = &pvals[i];
        let changed = target != current_val;
        if changed {
            change_count += 1;
        }
        let marker = if changed { "<<" } else { "==" };
        let marker_color = if changed {
            Color::Yellow
        } else {
            Color::DarkGreen
        };

        table.add_row(vec![
            Cell::new(*knob).fg(Color::DarkGreen),
            Cell::new(current_val).fg(Color::White),
            Cell::new(target).fg(if changed {
                Color::Yellow
            } else {
                Color::DarkGreen
            }),
            Cell::new(marker).fg(marker_color),
        ]);
    }

    println!("{}", table);

    if change_count == 0 {
        println!(
            "\n  {} No changes needed — system already matches this profile.\n",
            style::green(">>")
        );
    } else {
        println!(
            "\n  {} {} knob(s) would change.\n  Run: {}sudo cachyos-tune apply {}{}\n",
            style::yellow(">>"),
            change_count,
            "\x1b[1m",
            profile.name,
            "\x1b[0m",
        );
    }
}
