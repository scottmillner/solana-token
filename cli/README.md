# Solana Token CLI

A command-line interface for interacting with the Solana Token program.

## Architecture

This CLI uses a **code generation approach** where Rust types are automatically generated from the Anchor IDL (Interface Description Language). This ensures type safety and keeps the CLI in sync with the on-chain program.

### Build Script (`build.rs`)

The build script runs during compilation and:

1. **Reads the IDL**: Loads `../target/idl/solana_token.json`
2. **Parses Instructions**: Extracts instruction names, discriminators, accounts, and arguments
3. **Generates Rust Code**: Creates type-safe structs in `src/generated.rs` including:
   - Instruction argument structs with `AnchorSerialize`
   - Discriminator implementations (8-byte instruction identifiers)
   - Account context structs
   - `ToAccountMetas` implementations for account ordering
4. **Auto-regenerates**: Runs automatically on `cargo build` if the IDL changes

### Why This Approach?

**Pros:**
- ✅ IDL is the single source of truth
- ✅ Type-safe: Wrong types = compile error
- ✅ Auto-syncs: Refactor program → rebuild CLI → types updated
- ✅ No manual discriminators (they're read from IDL)
- ✅ Similar to TypeScript Anchor client behavior

**Cons:**
- ⚠️ Requires `anchor build` before building CLI
- ⚠️ Generated code is not human-editable

## Prerequisites

Before building the CLI, you must generate the IDL:

```bash
# From repository root
anchor build
```

This creates `target/idl/solana_token.json` which the build script requires.

## Building

```bash
cd cli
cargo build --release
```

The build process will:
1. Run `build.rs` to generate `src/generated.rs` from the IDL
2. Compile the CLI with the generated types
3. Output binary to `../target/release/solana-token-cli`

## Usage

### Global Options

```bash
solana-token-cli [OPTIONS] <COMMAND>

Options:
  -r, --rpc-url <RPC_URL>    RPC URL for Solana cluster [default: http://localhost:8899]
  -k, --keypair <KEYPAIR>    Path to payer keypair [default: ~/.config/solana/id.json]
  -h, --help                 Print help
```

### Commands

#### Initialize a Token Mint

```bash
solana-token-cli init --decimals <DECIMALS> [--mint-keypair <PATH>]
```

**Options:**
- `-d, --decimals <DECIMALS>` - Number of decimal places (required)
- `-m, --mint-keypair <PATH>` - Path to mint keypair file (optional, generates new if not provided)

**Example:**
```bash
# Create a new token with 9 decimals
solana-token-cli init --decimals 9

# Use existing mint keypair
solana-token-cli init --decimals 6 --mint-keypair ./my-mint.json
```

#### Create Token Account

```bash
solana-token-cli create-account --mint <MINT> [--owner <OWNER>]
```

**Status:** 🚧 Not yet implemented

#### Mint Tokens

```bash
solana-token-cli mint --mint <MINT> --to <OWNER> --amount <AMOUNT>
```

**Status:** 🚧 Not yet implemented

#### Transfer Tokens

```bash
solana-token-cli transfer --mint <MINT> --to <RECIPIENT> --amount <AMOUNT>
```

**Status:** 🚧 Not yet implemented

#### Burn Tokens

```bash
solana-token-cli burn --mint <MINT> --amount <AMOUNT>
```

**Status:** 🚧 Not yet implemented

#### Query Balance

```bash
solana-token-cli balance --mint <MINT> [--owner <OWNER>]
```

**Status:** 🚧 Not yet implemented

#### Query Mint Info

```bash
solana-token-cli mint-info --mint <MINT>
```

**Status:** 🚧 Not yet implemented

## Development

### Rebuilding After Program Changes

If you modify the Solana program:

```bash
# 1. Rebuild the program and regenerate IDL
anchor build

# 2. Rebuild the CLI (build.rs will regenerate types)
cd cli
cargo build --release
```

The build script automatically detects IDL changes via:
```rust
println!("cargo:rerun-if-changed=../target/idl/solana_token.json");
```

### Viewing Generated Code

Generated types are in `cli/src/generated.rs`. This file is auto-generated and should not be manually edited.

**Example generated code:**
```rust
pub mod initialize {
    #[derive(AnchorSerialize)]
    pub struct Initialize {
        pub decimals: u8,
    }

    impl Discriminator for Initialize {
        const DISCRIMINATOR: &'static [u8] = &[175, 175, 109, 31, 13, 152, 155, 237];
    }

    pub struct Accounts {
        pub mint: Pubkey,
        pub authority: Pubkey,
        pub system_program: Pubkey,
    }

    impl ToAccountMetas for Accounts { /* ... */ }
}
```

### Project Structure

```
cli/
├── build.rs              # Build script (generates code from IDL)
├── Cargo.toml            # Dependencies + build dependencies
├── src/
│   ├── main.rs          # CLI implementation
│   └── generated.rs     # Auto-generated from IDL (git-ignored)
└── README.md            # This file
```

## Troubleshooting

### Error: "Failed to read IDL file"

**Cause:** IDL hasn't been generated yet.

**Solution:**
```bash
anchor build
```

### Error: Types don't match program

**Cause:** IDL is stale (program was modified but IDL not regenerated).

**Solution:**
```bash
anchor build
cd cli
cargo clean
cargo build --release
```

### Build script warnings

The build script outputs warnings during compilation:
```
warning: Generated 5 instructions from IDL
```

This is informational and confirms code generation succeeded.

## License

[Your license here]
