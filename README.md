# CLR3 – Solana Native SOL Transfers

**Assignment:** CLR3 Software Engineer Hiring Assignment  
**Developer:** Prashant Kumar  
**GitHub:** https://github.com/prashantcoder123/clr3-sol-transfers  
**Network:** Solana Mainnet Beta  
**Block Range:** 385870151–385870156

## 1. Project Overview

This project is a Rust-based Substreams application that processes Solana blockchain data and extracts native SOL transfer instructions.

The application identifies successful transactions invoking the Solana System Program, decodes native transfer instructions, and produces structured transfer records.

Each extracted transfer contains:

- `from`: Sender wallet address.
- `to`: Receiver wallet address.
- `amount`: Transfer amount in lamports.
- `signature`: Transaction signature.

## 2. Technology Stack

The following technologies and tools were installed and used during development:

| Technology | Purpose |
|---|---|
| Rust | Core application development |
| Cargo | Dependency management and compilation |
| Substreams CLI | Building and running the Substreams package |
| Buf | Protocol Buffer code generation |
| Protocol Buffers | Defining structured output messages |
| WebAssembly (`wasm32-unknown-unknown`) | Compiling Rust into WASM |
| Solana Substreams package | Accessing Solana transaction data |
| Base58 (`bs58`) | Encoding Solana addresses and signatures |
| Pinax Substreams endpoint | Streaming Solana blockchain data |
| Git and GitHub | Version control and source-code hosting |
| Ubuntu on WSL | Development environment |

## 3. Development Environment Setup

### Prerequisites

- Ubuntu / WSL
- Rust and Cargo
- Substreams CLI
- Buf CLI
- WebAssembly Rust target
- Git

### Installation and verification

Rust and Cargo:

```bash
rustc --version
cargo --version
```

WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
rustup target list --installed
```

Substreams CLI:

```bash
substreams version
```

Buf:

```bash
buf --version
```

Base58 dependency:

```bash
cargo add bs58@0.4
```

The Solana Substreams package and its protobuf definitions are configured through `substreams.yaml` and `buf.gen.yaml`.

**Note:** A Pinax API key is required to access the configured streaming endpoint. Keep API keys in environment variables and never commit them to GitHub.

## 4. Project Structure

```text
clr3-sol-transfers/
│
├── proto/
│   └── mydata.proto
│
├── src/
│   ├── lib.rs
│   └── pb/
│       └── Generated Protocol Buffer files
│
├── Cargo.toml
├── Cargo.lock
├── buf.gen.yaml
├── substreams.yaml
├── .gitignore
├── README.md
│
└── clr3-sol-transfers-v0.1.0.spkg
```

## 5. Implementation Approach

### Step 1: Understand the assignment

I first reviewed the assignment requirements and identified the main tasks:

1. Understand how Solana transaction data is represented.
2. Configure a Substreams package.
3. Filter transactions involving the Solana System Program.
4. Decode native SOL transfer instructions.
5. Generate structured output.
6. Test the solution against the requested block range.

### Step 2: Configure the Substreams package

The project uses the Solana Substreams package and configures a transaction-filtering module.

The System Program address is:

```text
11111111111111111111111111111111
```

The configured module pipeline is:

```text
Solana Blockchain
       |
       v
Filtered Transactions
       |
       v
map_my_data
       |
       v
Native SOL Transfer Records
```

### Step 3: Define the Protocol Buffer schema

The output schema is defined in `proto/mydata.proto`.

```protobuf
syntax = "proto3";

package mydata.v1;

message Transfer {
    string from = 1;
    string to = 2;
    uint64 amount = 3;
    string signature = 4;
}

message MyData {
    repeated Transfer transfers = 1;
}
```

### Step 4: Implement transfer extraction

The Rust implementation in `src/lib.rs`:

- Iterates through filtered transactions.
- Skips transactions that failed or have missing metadata.
- Reads transaction account keys and signatures.
- Identifies System Program instructions.
- Checks the native transfer instruction format.
- Decodes the instruction discriminator and amount.
- Resolves sender and receiver account indices.
- Encodes wallet addresses and signatures using Base58.
- Adds extracted records to the output message.

For the native System Program transfer instruction, the data contains:

| Field | Description |
|---|---|
| First 4 bytes | Instruction discriminator |
| Next 8 bytes | Amount in lamports |
| Discriminator value | `2` for Transfer |

The amount is retained as an unsigned 64-bit integer.

## 6. Build and Run

### Generate protobuf code and build the package

```bash
substreams build
```

### Inspect the generated package

```bash
substreams info clr3-sol-transfers-v0.1.0.spkg
```

### Run against the requested six blocks

```bash
substreams run \
  -e solana.substreams.pinax.network:443 \
  clr3-sol-transfers-v0.1.0.spkg \
  map_my_data \
  -s 385870151 \
  -t +6
```

The endpoint requires valid Pinax authentication.

## 7. Testing Results

The Substreams package was successfully built and executed against the requested block range.

| Test | Result |
|---|---|
| Rust/WASM compilation | Successful |
| Protocol Buffer generation | Successful |
| SPKG generation | Successful |
| Solana endpoint connection | Successful |
| Blocks processed | 6 |
| Blocks received | 6 |
| Execution | Completed successfully |

The output contains structured transfer records with sender, receiver, amount, and transaction signature.

**Example output structure:**

```json
{
  "transfers": [
    {
      "from": "SENDER_WALLET_ADDRESS",
      "to": "RECEIVER_WALLET_ADDRESS",
      "amount": "325253",
      "signature": "TRANSACTION_SIGNATURE"
    }
  ]
}
```

Note: Protobuf JSON represents `uint64` values as strings. The underlying field remains an unsigned 64-bit integer.

## 8. Challenges and Problem-Solving

During development, I encountered several unfamiliar tools and configuration issues.

Some of the challenges included:

- Setting up the Rust development environment in Ubuntu WSL.
- Installing and configuring the Substreams CLI.
- Understanding the Substreams module pipeline.
- Generating Rust code from Protocol Buffer definitions.
- Resolving duplicate declarations in the protobuf schema.
- Understanding Rust compiler warnings and build output.
- Configuring the Base58 dependency.
- Connecting to the Pinax Solana streaming endpoint.
- Troubleshooting GitHub authentication and repository permissions.

Instead of stopping when an error appeared, I approached each issue step by step, inspected terminal output, checked configurations, and tested changes independently.

## 9. My Approach to Learning New Technologies

One important thing I learned while completing this assignment is that working with an unfamiliar technology should not be intimidating.

My approach was:

1. **Don't panic:** An unfamiliar tool or error is a normal part of software development.
2. **Explore first:** Understand what the technology does before trying to implement everything.
3. **Break down the problem:** Divide a large assignment into smaller, manageable tasks.
4. **Read errors carefully:** Compiler errors and terminal messages often provide useful clues.
5. **Experiment:** Try small changes, verify their impact, and learn from the results.
6. **Use available resources:** Read documentation, inspect examples, and explore new tools when necessary.
7. **Keep improving:** Focus on understanding the solution rather than blindly copying commands.

I had not previously worked extensively with Solana Substreams, but I was willing to explore Rust, WebAssembly, Protocol Buffers, and blockchain streaming to complete the task.

I believe adaptability, curiosity, and the ability to learn unfamiliar technologies are important qualities for a software engineer.

## 10. Future Improvements

Potential improvements include:

- Supporting inner System Program transfer instructions.
- Resolving account addresses loaded through address lookup tables.
- Adding more comprehensive transaction validation.
- Expanding automated tests for transfer decoding.
- Improving output metadata and observability.

## 11. Author

**Prashant Kumar**  
B.Tech – Computer Science and Engineering

GitHub: https://github.com/prashantcoder123

---

*Built with curiosity, persistence, and a willingness to explore new technologies.*
