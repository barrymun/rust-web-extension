# Rust WebAssembly Extension

This is a web extension which is written in Rust and then compiled to WebAssembly.

## Prerequisites

Before using this extension, ensure that you have the following software installed:

- `Rust`
- `Cargo`
- `wasm-pack`

## Installation

1. Clone this repository to your local machine
2. Change into the project directory
3. Run the following command to set up the pre-commit hook:
   - `make setup`
   This will add the pre-commit file to the ".git/hooks" directory, ensuring code quality and formatting checks before committing.

## Usage

You can test this WebAssembly extension on example.com. Follow these steps:

1. Build the WebAssembly module:
   - `make build-chromium`
2. A `dist` directory will appear at the root level.
3. Navigate to chrome://extensions and ensure that "Developer mode" is enabled.
4. Click the "Load unpacked" button and select the contents of the `dist` directory.
5. Navigate to example.com to test.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## TODO

- [ ] support Firefox addons
