use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "lnx",
    about = "Creates symbolic links with resolved absolute paths."
)]
struct Cli {
    #[arg(long)]
    real_path: String,

    #[arg(long)]
    fake_path: String,
}

fn get_absolute_path(path: &str, is_real: bool) -> PathBuf {
    let p = PathBuf::from(path);
    if is_real {
        // For real_path, we need to ensure it exists
        fs::canonicalize(&p).unwrap_or_else(|_| {
            eprintln!("Error: real_path '{}' does not exist.", path);
            std::process::exit(1);
        })
    } else {
        // For fake_path, it might not exist yet
        if p.is_absolute() {
            p
        } else {
            env::current_dir().unwrap().join(p)
        }
    }
}

fn main() {
    let args = Cli::parse();

    // Resolve paths to absolute paths
    let real_path = get_absolute_path(&args.real_path, true);
    let fake_path = get_absolute_path(&args.fake_path, false);

    // Create parent directories for fake_path if they don't exist
    if let Some(parent_dir) = fake_path.parent() {
        if !parent_dir.exists() {
            if let Err(e) = fs::create_dir_all(parent_dir) {
                eprintln!(
                    "Error creating directories '{}': {}",
                    parent_dir.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    }

    // Build and execute ln -s real_path fake_path
    let status = Command::new("ln")
        .arg("-s")
        .arg(real_path.as_os_str())
        .arg(fake_path.as_os_str())
        .status()
        .expect("Failed to execute ln");

    if !status.success() {
        eprintln!("ln command failed");
        std::process::exit(status.code().unwrap_or(1));
    }
}
