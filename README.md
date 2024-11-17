# Lumix CLI Tool

**Lumix** is a simple command-line tool for compiling Rust source files using the `rustc` compiler. It allows you to compile `.rs` files from the command line with options for specifying the output directory for compiled binaries.

## Features

- **Custom Output Directory**: Compile your Rust source code and specify a custom directory for the output binary.
- **Help Menu**: Get a comprehensive help menu using the `--help` flag.
- **Self-compilation**: The tool requires users to compile it from the source using `rustc`.

## Prerequisites

Before compiling and using Lumix, make sure you have the following installed:

1. **Rust**: You must have Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/tools/install).
2. **Git**: To clone the repository, you will need Git installed. You can get it from [git-scm.com](https://git-scm.com/).

To check if Rust and Git are installed, run the following commands in your terminal:

```bash
rustc --version
git --version
```

## How to Compile the Tool

1. **Clone the Repository**:
   First, clone the repository to your local machine:

   ```bash
   git clone https://github.com/SamTheDevDE/Lumix
   ```

2. **Navigate to the Project Directory**:
   Go to the directory where you cloned the Lumix repository:

   ```bash
   cd lumix
   ```

3. **Compile the Project with `rustc`**:
   Compile the project using the `rustc` command:

   ```bash
   rustc src/main.rs -o lumix
   ```

   This will compile the `main.rs` file and output the executable as `lumix`.

4. **Run the Tool**:
   After compilation, you can run the tool like this:

   ```bash
   ./lumix -c src/main.rs lumix.exe --bin-dir=./bin
   ```

   This will compile `src/main.rs` and output the binary as `lumix.exe` in the `./bin` directory.

## Usage

### Basic Command
```bash
./lumix -c <source_file> <output_file> --bin-dir=<output_directory>
```

### Options:
- `-c, --compile` - Compiles the specified Rust source file (`<source_file>`) and outputs the specified file (`<output_file>`).
- `--bin-dir=<directory>` - Specifies the directory where the compiled binary will be placed. If the directory doesn't exist, it will be created.
- `--help, -h` - Displays the help menu with available options.

### Example Usage:
```bash
./lumix -c src/main.rs lumix.exe --bin-dir=./bin
```

This will compile the `src/main.rs` file and place the resulting `lumix.exe` binary into the `./bin` directory.

## Contributing

We welcome contributions to Lumix! To contribute:

1. Fork the repository.
2. Create a new branch for your changes.
3. Open a pull request with your changes.

Before submitting a pull request, please open an issue to discuss large changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Thank you to everyone who has contributed to Lumix. Special thanks to the open-source community for their continuous support!