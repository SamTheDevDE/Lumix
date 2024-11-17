use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

fn print_help() {
    println!("Usage: ./lumix.exe -c <source_file.rs> <output_file_name> [--bin-dir=<custom_directory>]");
    println!();
    println!("Options:");
    println!("  -c, --compile          Compile the Rust source file");
    println!("  --bin-dir=<dir>        Specify a custom directory for the compiled binary (default is './bin')");
    println!("  -h, --help             Display this help menu");
}

fn main() {
    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();

    // Check if help flag is passed
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print_help();
        return;
    }

    // Check if there are enough arguments
    if args.len() < 4 {
        println!("Usage: ./lumix.exe -c <source_file.rs> <output_file_name> [--bin-dir=<custom_directory>]");
        return;
    }

    let mut bin_dir = Path::new("./bin"); // Default output directory

    // Look for --bin-dir flag
    if let Some(bin_dir_arg) = args.iter().find(|arg| arg.starts_with("--bin-dir=")) {
        if let Some(custom_dir) = bin_dir_arg.split_once('=') {
            bin_dir = Path::new(custom_dir.1); // Set to custom directory from the flag
        }
    }

    // Check if -c (compile) flag is present
    if args.contains(&String::from("-c")) || args.contains(&String::from("--compile")) {
        // Get the source file and output file name from the arguments
        let source_file = args.get(2).unwrap();  // The Rust source file
        let output_file = args.get(3).unwrap();  // The output file (compiled binary)

        // Make sure the bin directory exists (or create it if it doesn't)
        if !bin_dir.exists() {
            match fs::create_dir_all(bin_dir) {
                Ok(_) => println!("Created directory: {:?}", bin_dir),
                Err(e) => {
                    eprintln!("Failed to create directory: {}", e);
                    return;
                }
            }
        }

        // Construct the full path for the output file
        let output_path: PathBuf = bin_dir.join(output_file);

        println!(
            "Compiling Rust source: {} to output file: {:?}",
            source_file,
            output_path
        );

        // Start timer
        let start_time = Instant::now();

        // Execute the rustc command to compile the Rust source file
        let output = Command::new("rustc")
            .arg(source_file) // The Rust source file to compile
            .arg("-o") // The output flag
            .arg(output_path.to_str().unwrap()) // The full output file path
            .output();

        // Stop timer
        let duration = start_time.elapsed();

        // Handle output
        match output {
            Ok(output) => {
                if !output.stderr.is_empty() {
                    eprintln!(
                        "Error during compilation: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                } else {
                    println!("Done.");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    println!("Compilation completed in {:.2} seconds.", duration.as_secs_f64());
                }
            }
            Err(e) => {
                eprintln!("Failed to execute rustc: {}", e);
            }
        }
    } else {
        println!("Run './lumix.exe --help' to see available options.");
    }
}
