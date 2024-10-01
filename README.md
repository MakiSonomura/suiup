# Suiup

Suiup is a version manager for Sui, enabling you to install Sui toolkits directly from GitHub releases. It allows for easy management of multiple Sui versions and will soon support downloading binaries from the blockchain. You can run it on Linux/Windows/MacOS
## Usage

### Installation

#### Build from Source
1. **Clone the repository**
   ```bash
   git clone https://github.com/MakiSonomura/suiup --depth 1
   ```
2. **Build the project**
   ```bash
   cargo build --release
   ```
   You can run the executable directly from `target/release/suiup` without adding it to your `$PATH`.

3. **Install it to your binary crates directory**
   ```bash
   cd suiup
   cargo install --path .
   ```

### CLI
```text
Usage: suiup <COMMAND>

Commands:
  list     List all installed Sui toolkits
  config   Print the default configuration of Suiup
  default  Set the default toolkit
  install  Install a Sui toolkit
  latest   Install the latest toolkit
  help     Print this message or help for the given subcommand(s)

Options:
  -h, --help  Print help

```