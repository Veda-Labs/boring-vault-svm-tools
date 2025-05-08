# Python Example

## Summary

This project provides Python bindings for interacting with Solana's Boring Vault programs, abstracting away the complexity of direct Solana interactions. The goal is to enable Python developers to work with Boring Vaults without needing deep knowledge of Solana's architecture or having to manage Solana-specific dependencies in their projects.

Key features:

- Simple Python interface for Boring Vault operations
- Handles Solana-specific details internally
- Minimal Solana dependencies required in user projects
- Supports both local development and mainnet deployment
- Provides transaction building and management capabilities

## Local Setup and Running

1. Make edits to the boring-vault-svm-py crate

   ```bash
   # Edit your Rust code in boring-vault-svm-py/src/
   ```

2. Set up Python environment and install the package

   ```bash
   # Navigate to the python example directory
   cd python

   # Create and activate virtual environment (if not already done)
   python -m venv venv
   # Install the package using maturin
   . ./scripts/python_ini.sh
   ```

3. Start the Solana test validator

   ```bash
   # In a new terminal window:

   # For a fresh state:
   solana-test-validator --reset
   # Then run anchor deploy in the boring-vault-svm repo

   # OR, to preserve existing state:
   solana-test-validator
   ```

4. Run the example

   ```bash
   # Make sure you're in the python_example directory
   python main.py
   ```

## Mainnet Setup and Running

1. Make edits to the boring-vault-svm-py crate

   ```bash
   # Edit your Rust code in boring-vault-svm-py/src/
   ```

2. Start venv and install package

   ```bash
   # Navigate to the python example directory
   cd python_example

   # Start virtual environment
   source venv/bin/activate

   # Install the package using maturin
   maturin develop -m ../../boring-vault-svm-py/Cargo.toml
   ```

3. Run the example
   ```bash
   # Make sure you're in the python_example directory
   python main.py
   ```

## Development Workflow

After making changes to the Rust code:

1. Rebuild the package using maturin
   ```bash
   maturin develop -m ../../boring-vault-svm-py/Cargo.toml
   ```
2. Run your Python script to test the changes
   ```bash
   python main.py
   ```

## Requirements

- Python 3.x
- Rust/Cargo
- maturin (`pip install maturin`)
- Solana test validator
- Anchor

## Notes

- Make sure the Solana test validator is running before executing the example
- For a complete reset: `solana-test-validator --reset` followed by `anchor deploy`
- For release builds, add `--release` to the maturin command
