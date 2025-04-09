import boring_vault_svm
import asyncio
import json
from pathlib import Path
import os
import json

# Default funded keypair
def get_default_keypair_secret():
    home = str(Path.home())
    with open(f"{home}/.config/solana/id.json", "r") as f:
        secret_key_data = json.load(f)
        return bytes(secret_key_data)

def get_program_keypair_secret():
    with open("../../program_keypairs/boring_vault_svm-keypair.json", "r") as f:
        secret_key_data = json.load(f)
        return bytes(secret_key_data)

async def main():
    signer_bytes = get_default_keypair_secret()
    program_signer_bytes = get_program_keypair_secret()

    try:
        # Create builder
        print("Creating builder...")
        builder = boring_vault_svm.TransactionBuilder("http://127.0.0.1:8899")

        authority_pubkey_str = "DuheUFDBEGh1xKKvCvcTPQwA8eR3oo58kzVpB54TW5TP"

        # Add instructions
        print("Adding initialize instruction...")
        builder.initialize(authority_pubkey_str, signer_bytes, program_signer_bytes)
        print("Adding deploy instruction...")
        builder.deploy(
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

        print("Sending instructions as one bundle...")
        tx_hash = builder.try_bundle_all(signer_bytes)
        print(f"Success! Transaction hash: {tx_hash}")

        print("Setting up SOL as a deposit asset...")
        builder.update_asset_data(
                signer_bytes=signer_bytes,
                vault_id=0,
                mint="11111111111111111111111111111111",
                allow_deposits=True,  # allow_deposits
                allow_withdrawals=True,  # allow_withdrawals
                share_premium_bps=0,     # share_premium_bps
                is_pegged_to_base_asset=True, # is_pegged_to_base_asset
                price_feed="11111111111111111111111111111111",
                inverse_price_feed=False, # inverse_price_feed
                max_staleness=0,    # max_staleness
                min_samples=0,     # min_samples
        )

        print("Depositing SOL...")
        builder.deposit_sol(
            signer_bytes=signer_bytes,
            vault_id=0,
            user_pubkey="DuheUFDBEGh1xKKvCvcTPQwA8eR3oo58kzVpB54TW5TP",
            deposit_amount=1000000000, # deposit_amount in lamports
            min_mint_amount=0,          # min_mint_amount
        )

        print("Sending instructions as one bundle...")
        tx_hash = builder.try_bundle_all(signer_bytes)

        print(f"Success! Transaction hash: {tx_hash}")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    # Make sure you have a local validator running (solana-test-validator)
    asyncio.run(main())
