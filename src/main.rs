use clap::{Arg, Command};
use std::{env, fs};
use std::process::{Command as ProcessCommand};

fn main() {
    let matches = Command::new("Lumix")
        .version("1.0")
        .author("Your Name")
        .about("A CLI tool for compiling Rust projects")
        .arg(Arg::new("compile")
            .short('c')
            .long("compile")
            .value_parser(clap::value_parser!(String))
            .help("Path to the source file or directory to compile"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_parser(clap::value_parser!(String))
            .default_value("lumix") // This now uses a string literal
            .help("Name of the output file"))
        .arg(Arg::new("bin-dir")
            .long("bin-dir")
            .value_parser(clap::value_parser!(String))
            .default_value("./bin") // This also uses a string literal
            .help("Directory to output the compiled binary"))
        .arg(Arg::new("release")
            .short('r')
            .long("release")
            .help("Compile in release mode"))
        .arg(Arg::new("target")
            .long("target")
            .value_parser(clap::value_parser!(String))
            .help("Specify the target architecture"))
        .arg(Arg::new("features")
            .long("features")
            .value_parser(clap::value_parser!(String))
            .help("Specify features to enable during the build"))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(clap::ArgAction::Count)
            .help("Increase verbosity of output (use multiple times for more verbosity)"))
        .arg(Arg::new("command")
            .long("command")
            .value_parser(clap::value_parser!(String))
            .default_value("build")
            .help("Specify the cargo command to use (e.g., build, clean, test)"))
        .get_matches();

    // Retrieve arguments safely
    let file = matches.get_one::<String>("compile").unwrap_or_else(|| {
        eprintln!("Error: 'compile' argument is required.");
        std::process::exit(1);
    });

    // Use unwrap_or_else to provide default values for "output" and "bin-dir"
    let output = matches.get_one::<String>("output").map(|s| s.clone()).unwrap_or_else(|| "lumix".to_string());
    let bin_dir = matches.get_one::<String>("bin-dir").map(|s| s.clone()).unwrap_or_else(|| "./bin".to_string());

    let release = matches.get_flag("release");
    let target = matches.get_one::<String>("target");
    let features = matches.get_one::<String>("features");
    let verbose = matches.get_count("verbose");
    let command = matches.get_one::<String>("command").unwrap();

    // Ensure the output directory exists
    fs::create_dir_all(bin_dir.clone()).unwrap(); // Clone bin_dir here to avoid moving it

    // Determine the build mode
    let mode = if release { "--release" } else { "" };

    // Verbosity adjustment
    let verbosity = match verbose {
        0 => "--quiet",  // No verbosity
        1 => "--verbose",  // Standard verbosity
        _ => "--verbose --verbose",  // Maximum verbosity
    };

    // Build the cargo command
    let mut cargo_command = ProcessCommand::new("cargo");

    // Add the custom command (build, clean, test)
    cargo_command.arg(command);

    // Add the optional target architecture
    if let Some(target_arch) = target {
        cargo_command.arg("--target").arg(target_arch);
    }

    // Add features if specified
    if let Some(feature_list) = features {
        cargo_command.arg("--features").arg(feature_list);
    }

    // Add verbosity level
    cargo_command.arg(verbosity);

    // Add other necessary arguments
    cargo_command.arg(mode).arg("--bin").arg(output.clone()).current_dir(file); // Clone output here as well

    // Execute the command and handle the result
    let status = cargo_command.status().expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!("Error during compilation.");
    } else {
        println!("Compilation successful!");
        println!("Binary saved to {}/{}", bin_dir, output); // This works now because we cloned bin_dir and output
    }
}
