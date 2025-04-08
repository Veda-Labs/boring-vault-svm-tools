import boring_vault_svm
import asyncio
import json
from pathlib import Path


# Default funded keypair
def get_default_keypair_secret():
    home = str(Path.home())
    with open(f"{home}/.config/solana/id.json", "r") as f:
        secret_key_data = json.load(f)
        return bytes(secret_key_data)


async def main():
    signer_bytes = get_default_keypair_secret()

    try:
        print("Here")
        authority_pubkey_str = "DuheUFDBEGh1xKKvCvcTPQwA8eR3oo58kzVpB54TW5TP"

        # Initialize using our Rust binding
        boring_vault_svm.initialize(authority_pubkey_str, signer_bytes)

        # boring_vault_svm.deploy(authority_pubkey_str, signer_bytes)

        print(f"Successfully initialized with:")
        print(f"Authority: {authority_pubkey_str}")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    # Make sure you have a local validator running (solana-test-validator)
    asyncio.run(main())
