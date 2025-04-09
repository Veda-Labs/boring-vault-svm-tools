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
        w_sol_pubkey_str = "So11111111111111111111111111111111111111112"

        # Initialize using our Rust binding
        # boring_vault_svm.initialize(authority_pubkey_str, signer_bytes)

        boring_vault_svm.deploy(    
            authority="DuheUFDBEGh1xKKvCvcTPQwA8eR3oo58kzVpB54TW5TP",
            signer_bytes=signer_bytes,
            base_asset="So11111111111111111111111111111111111111112",  # wSOL
            name="Test Vault",
            symbol="TV",
            exchange_rate_provider=None,  # Optional
            exchange_rate=1_000_000_000,
            payout_address=None,  # Optional
            allowed_exchange_rate_change_upper_bound=10_100,
            allowed_exchange_rate_change_lower_bound=9_900,
            minimum_update_delay_in_seconds=3_600,
            platform_fee_bps=None,  # Optional
            performance_fee_bps=None,  # Optional
            withdraw_authority=None,  # Optional
            strategist=None  # Optional
        )
        

        print(f"Successfully initialized with:")
        print(f"Authority: {authority_pubkey_str}")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    # Make sure you have a local validator running (solana-test-validator)
    asyncio.run(main())
