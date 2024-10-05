use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "lnx",
    about = "Creates symbolic links with resolved absolute paths."
)]
struct Cli {
    /// The target path the symlink will point to (real path)
    #[arg(long, short)]
    real_path: String,

    /// The path where the symlink will be created (fake path)
    #[arg(long, short)]
    fake_path: String,
}

fn resolve_real_path(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    fs::canonicalize(&p).unwrap_or_else(|_| {
        eprintln!("Error: real_path '{}' does not exist.", path);
        std::process::exit(1);
    })
}

fn resolve_fake_path(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    if p.is_absolute() {
        p
    } else {
        env::current_dir().unwrap().join(p)
    }
}

fn make_parent_dirs(path: &Path) {
    if let Some(parent_dir) = path.parent() {
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
}

fn make_link(real_path: &Path, fake_path: &Path) {
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

fn main() {
    let args = Cli::parse();

    let real_path = resolve_real_path(&args.real_path);
    let fake_path = resolve_fake_path(&args.fake_path);

    make_parent_dirs(&fake_path);
    make_link(&real_path, &fake_path);
}
