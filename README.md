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

### Agreements
- On Linux, the default directory for `suiup` is `$HOME/.suiup`, and on Windows, it's `%USERPROFILE%\.suiup`.
- The `.tgz` file downloaded from the backend (currently GitHub) will be found in `.suiup/download`.
- The archive will be extracted to `.suiup/toolkites/${desc}`.
- To use the `sui` command directly, make sure to add `.suiup/bin` to your environment path.


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




### Examples

1. **Install the latest toolkit of mainnet and set it as the default one**
   ```bash
   suiup latest --network mainnet 
   ```
2. **Install a specific version of sui toolkit and set it as the default one**
   ```bash
   suiup install mainnet-v1.25.3
   ```
3. **List all toolkits in your system**
   ```bash
   suiup list
   ```
4. **Change the default toolkit**

   ```bash
   suiup default testnet-v1.34.1
   ```
5. **Remove which you donot need**
    ```bash
    suiup remove mainnet-v1.25.3
    ```
6. **Show suiup's config**
    ```bash
    suiup config
    ```