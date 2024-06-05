# json2jq

`json2jq` is a simple Rust program that reads a JSON string from stdin and outputs a set of unique jq paths. This can be useful for exploring the structure of JSON data and integrating with other command-line tools.

## Features

- Reads JSON input from stdin
- Outputs unique jq paths to stdout
- Supports nested objects and arrays

## Installation

### With Homebrew
```sh
brew install json2jq
```

### Precompiled Binaries
Precompiled binaries are available for Windows, Linux and macOS in the [releases](https://github.com/Zeyshi/json2jq/releases).

### From source
You'll need to have Rust installed. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/yourusername/json2jq.git
cd json2jq
cargo install --path .
```
Make sure `~/.cargo/bin` is in your PATH.


## Usage
You can use json2jq by piping JSON data into it. For example:

```sh
echo '{"name": "Alice", "type": "paying", "features": [{"premium": true}]}' | ./target/release/json2jq
```
This will output:
```sh
.name
.type
.features
.features[].premium
```

### Piping Output to Other Programs
You can pipe the output of json2jq into other command-line tools, such as grep:

```sh
echo '{"name": "Alice", "type": "paying", "features": [{"premium": true}]}' | ./target/release/json2jq | grep features
```
This will output:
```sh
.features
.features[].premium
```

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or find any bugs.
