# Krak

This is a simple Solana contract where users can place bounties on Sha256 / Sha3 and Blake3 hashes.

## Prerequisites

Ensure you have the following installed:

- Rust
- Solana CLI
- Node.js and Yarn

## Setup

1. **Install Anchor-lang and its dependencies:**

   https://www.anchor-lang.com/docs/installation

2. **Upgrade solana**

   solana-install init 1.18.17

3. **Use anchor version 0.30.1 via avm (See above link)**
4. **Build the project**

   anchor build

5. **Install js dependencies using Yarn**

   yarn install

6. **Run tests**
   anchor test

## Known Issues during Installation

1. If you get any of the errors while running `anchor build`,

```zsh
error: package `toml_edit v0.21.1` cannot be built because it requires rustc 1.69 or newer, while the currently active rustc version is 1.68.0-dev
Either upgrade to rustc 1.69 or newer, or use
cargo update -p toml_edit@0.21.1 --precise ver
where `ver` is the latest version of `toml_edit` supporting rustc 1.68.0-dev
```

or

```zsh
error: package `solana-program v1.18.17` cannot be built because it requires rustc 1.75.0 or newer, while the currently active rustc version is 1.68.0-dev
Either upgrade to rustc 1.75.0 or newer, or use
cargo update -p solana-program@1.18.17 --precise ver
where `ver` is the latest version of `solana-program` supporting rustc 1.68.0-dev
```

What you need to do is to run `solana-install init 1.18.17` or the [latest available version](https://github.com/solana-labs/solana/releases) of Solana. This error occurs because `anchor build` it uses `rustc` installed with Solana tools.

```sh

```

2. If you run into the error below, while running `solana-test-validator`

```sh
Ledger location: test-ledger
Log: test-ledger/validator.log
Error: failed to start validator: Failed to create ledger at test-ledger: io error: Error checking to unpack genesis archive: Archive error: extra entry found: "._genesis.bin" Regular
```

Add this to your `.zshrc` or `.bashrc` file

```
export PATH="/opt/homebrew/opt/gnu-tar/libexec/gnubin:$PATH"
export PATH="/Users/favour/.local/share/solana/install/active_release/bin:$PATH"
```

when you are done, remember to run before you run any other command

```sh
source ~/.zshrc
```

then re-run

```sh
rm -rf test-ledger
solana-test-validator
```

3. If you run into the error below while running `anchor test`

```sh
[
  "Program BsZZZGaXNmXyDPfbHqdVEi2dDoaBYYMaKvXmjfmeSuzL invoke [1]",
  "Program log: AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id.",
  "Program BsZZZGaXNmXyDPfbHqdVEi2dDoaBYYMaKvXmjfmeSuzL consumed 3287 of 200000 compute units",
  "Program BsZZZGaXNmXyDPfbHqdVEi2dDoaBYYMaKvXmjfmeSuzL failed: custom program error: 0x1004"
].
```

It means that you need run `anchor keys sync` to update the actual program id with the defined program id in `Anchor.toml` and `lib.rs`.
