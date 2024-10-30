# RustHW

Welcome to **RustHW**! This is a blazing fast, extensible, scalable, customizable, AI-powered, cross platform \"Hello, World!\" application written in Rust.

## Features

- **Blazing Fast**: Built with Rust's performance in mind, this application executes quickly and efficiently.
- **Extensible**: Designed with extensibility in mind, you can easily add new features and functionalities.
- **Scalable**: Suitable for various use cases, from small scripts to larger applications.
- **Customizable**: Customize your greetings with different colors, casing styles, and more.
- **AI-Powered**: All the code is written by ChatGPT.
- **Cross Platform**: Works on Windows, macOS, Linux, over SSH, and more.


## Download

To get started, simply download the project from [releases](https://github.com/szrabinowitz/hwrust/releases).

## Compiling

### Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine. You can install Rust using `rustup`, the Rust toolchain installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/szrabinowitz/hwrust.git
   cd hello_custom
   ```

2. Build the project:

   ```bash
   cargo build
   ```

### Usage

Run the application with optional command-line arguments to customize the output:

```bash
rusthw # run with default options
rusthw --hello-color [color] --world-color [color] --exclamation --casing uppercase # run with custom options
```

### Command-Line Arguments

- `--hello-color`: Set the color for \"Hello\". Options: red, green, blue, yellow, magenta, cyan, white, black (default: green).
- `--world-color`: Set the color for \"World\". Options: red, green, blue, yellow, magenta, cyan, white, black (default: blue).
- `--exclamation`: Add an exclamation mark at the end of the output (default: false).
- `--casing`: Specify the casing style. Options: uppercase, capitalize, lowercase (default: Capitalize).

### Build for Release

To create an optimized release version, run:

```bash
cargo build --release
```

The executable will be located in the `target/release` directory.


## Contributing

Contributions are welcome! If you have suggestions or improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


Enjoy customizing your greetings with **rusthw**! 🌟

## Roadmap

- Add the ability to Capitalize "Hello" without capitalizing "World".
- Add the ability to color each character individually.
- Make WASM version