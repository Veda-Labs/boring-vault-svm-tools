# Python Example

This example demonstrates how to use the boring-vault-svm-py Rust bindings in Python.

## Setup and Running

1. Make edits to the boring-vault-svm-py crate

   ```bash
   # Edit your Rust code in boring-vault-svm-py/src/
   ```

2. Build the Rust crate

   ```bash
   cargo build
   ```

3. Rename the library file

   ```bash
   # For macOS
   mv target/debug/libboring_vault_svm.dylib target/debug/boring_vault_svm.so
   ```

4. Set up Python environment and install the package

   ```bash
   # Navigate to the python example directory
   cd python_example

   # Create and activate virtual environment (if not already done)
   python -m venv venv
   source venv/bin/activate

   # Install the package using maturin
   maturin develop -m ../../boring-vault-svm-py/Cargo.toml
   ```

5. Start the Solana test validator

   ```bash
   # In a new terminal window:

   # For a fresh state:
   solana-test-validator --reset
   # Then run anchor deploy in the boring-vault-svm repo

   # OR, to preserve existing state:
   solana-test-validator
   ```

6. Run the example
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
