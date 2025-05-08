#!/bin/bash
cd python && 
source venv/bin/activate && 
maturin develop -m ../boring-vault-svm-py/Cargo.toml