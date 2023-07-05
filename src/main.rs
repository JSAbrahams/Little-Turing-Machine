use clap::{command, Parser, Subcommand};

use little_turing_machine::display::animation::{animate, AnimateMoving};
use little_turing_machine::display::cli::print_machine;
use little_turing_machine::presets::UniverseMetadata;

#[derive(Debug, Parser)]
#[command(about = "Little Turing Machine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Cli {
        #[arg(short, long)]
        preset: Option<String>,
    },
    Animate {
        #[arg(short, long)]
        preset: Option<String>,
        #[arg(short, long, default_value_t = false)]
        full_screen: bool,
        #[arg(long, default_value_t=AnimateMoving::default(), value_enum)]
        animate_moving: AnimateMoving,
        #[arg(long, default_value_t = false)]
        show_tick_count: bool,
    },
}

pub fn main() -> Result<(), String> {
    let args = Cli::parse();

    match args.command {
        Commands::Cli { preset } => {
            let universe_meta = if let Some(preset) = preset {
                UniverseMetadata::try_from(preset)?
            } else {
                UniverseMetadata::default()
            };

            print_machine(universe_meta)
        }
        Commands::Animate {
            preset,
            full_screen,
            animate_moving: move_item,
            show_tick_count,
        } => {
            let universe_meta = if let Some(preset) = preset {
                UniverseMetadata::try_from(preset)?
            } else {
                UniverseMetadata::default()
            };

            animate(universe_meta, None, full_screen, move_item, show_tick_count)
        }
    }

    Ok(())
}
