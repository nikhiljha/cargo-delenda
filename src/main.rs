use std::{env, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // cargo passes "delenda" as first arg when invoked as `cargo delenda`
    let rest: &[String] = if args.get(1).map(|s| s.as_str()) == Some("delenda") {
        &args[2..]
    } else {
        &args[1..]
    };

    if rest.first().map(|s| s.as_str()) != Some("est") {
        eprintln!("Usage: cargo delenda est");
        eprintln!();
        eprintln!("Carthago delenda est. So is your target folder.");
        process::exit(1);
    }

    let target = Path::new("target");
    if !target.exists() {
        println!("No target/ to destroy. Carthago iam deleta est.");
        return;
    }

    match fs::remove_dir_all(target) {
        Ok(()) => println!("Carthago deleta est."),
        Err(e) => {
            eprintln!("Failed to destroy target/: {e}");
            process::exit(1);
        }
    }
}
