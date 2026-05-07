mod apply;
mod bench;
mod profiles;
mod snapshot;
mod status;
mod style;
mod sysfs;

use clap::{Parser, Subcommand};
use profiles::ProfileName;

#[derive(Parser)]
#[command(
    name = "cachyos-tune",
    about = "System tuning CLI for CachyOS on AMD APU",
    version,
    after_help = "Examples:\n  cachyos-tune status\n  cachyos-tune diff ml-inference\n  sudo cachyos-tune apply gaming\n  cachyos-tune save\n  sudo cachyos-tune restore\n  cachyos-tune bench"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply a tuning profile (requires sudo)
    Apply {
        /// Profile to apply
        #[arg(value_enum)]
        profile: ProfileName,
    },
    /// Show current settings vs all profiles
    Status,
    /// Show what would change for a profile
    Diff {
        /// Profile to compare against
        #[arg(value_enum)]
        profile: ProfileName,
    },
    /// Snapshot current settings for later restore
    Save,
    /// Restore settings from a saved snapshot (requires sudo)
    Restore,
    /// Run quick benchmarks (CPU, memory, sequential read)
    Bench,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Apply { profile } => {
            style::banner();
            let p = profiles::Profile::from_name(profile);
            style::header(&format!("Applying profile: {}", p.name));
            match apply::apply_profile(&p) {
                Ok(()) => {
                    println!(
                        "\n  {} Profile {} applied successfully.\n",
                        style::green(">>"),
                        style::bold_green(&p.name.to_string())
                    );
                }
                Err(e) => {
                    style::print_error(&format!("{}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Status => {
            style::banner();
            status::print_status();
        }

        Commands::Diff { profile } => {
            style::banner();
            let p = profiles::Profile::from_name(profile);
            status::print_diff(&p);
        }

        Commands::Save => {
            style::banner();
            style::header("Saving snapshot");
            match snapshot::save_snapshot() {
                Ok(path) => {
                    println!(
                        "  {} Snapshot saved to {}\n",
                        style::green(">>"),
                        style::bright_green(&path.display().to_string())
                    );
                }
                Err(e) => {
                    style::print_error(&format!("Failed to save snapshot: {}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Restore => {
            style::banner();
            style::header("Restoring from snapshot");
            match snapshot::load_snapshot() {
                Ok(snap) => {
                    println!(
                        "  {} Restoring snapshot from {}\n",
                        style::green(">>"),
                        style::dim_green(&snap.timestamp)
                    );
                    match apply::apply_snapshot(&snap) {
                        Ok(()) => {
                            println!(
                                "\n  {} Settings restored successfully.\n",
                                style::green(">>")
                            );
                        }
                        Err(e) => {
                            style::print_error(&format!("{}", e));
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    style::print_error(&format!("{}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Bench => {
            style::banner();
            bench::run_bench();
        }
    }
}
