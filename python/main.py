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
    # program_signer_bytes = get_program_keypair_secret()

    try:
        # Create builder
        print("Creating builder...")
        # builder = boring_vault_svm.TransactionBuilder("http://127.0.0.1:8899")
        builder = boring_vault_svm.Builder(
            rpc_url="https://api.mainnet-beta.solana.com",
            data_path = None,
            kamino_file = None,
            vault_file = None,
            lend_mint = None,
            borrow_mint = None
        )
        authority_pubkey_str = "CSsqdfpwwBK8iueo9CuTLHc1M2uubj88UwXKCgZap7H2"

        # Add instructions
        # print("Adding initialize instruction...")
        # builder.initialize(authority_pubkey_str, signer_bytes, program_signer_bytes)
        # print("Adding deploy instruction...")
        # builder.deploy(
        #     authority="DuheUFDBEGh1xKKvCvcTPQwA8eR3oo58kzVpB54TW5TP",
        #     signer_bytes=signer_bytes,
        #     base_asset="So11111111111111111111111111111111111111112",  # wSOL
        #     name="Test Vault",
        #     symbol="TV",
        #     exchange_rate_provider=None,  # Optional
        #     exchange_rate=1_000_000_000,
        #     payout_address=None,  # Optional
        #     allowed_exchange_rate_change_upper_bound=10_100,
        #     allowed_exchange_rate_change_lower_bound=9_900,
        #     minimum_update_delay_in_seconds=3_600,
        #     platform_fee_bps=None,  # Optional
        #     performance_fee_bps=None,  # Optional
        #     withdraw_authority=None,  # Optional
        #     strategist=None  # Optional
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

        # print("Setting up SOL as a deposit asset...")
        # builder.update_asset_data(
        #         signer_bytes=signer_bytes,
        #         vault_id=3,
        #         mint="J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn",
        #         allow_deposits=True,  # allow_deposits
        #         allow_withdrawals=True,  # allow_withdrawals
        #         share_premium_bps=0,     # share_premium_bps
        #         is_pegged_to_base_asset=False, # is_pegged_to_base_asset
        #         price_feed="4Z1SLH9g4ikNBV8uP2ZctEouqjYmVqB2Tz5SZxKYBN7z",
        #         inverse_price_feed=False, # inverse_price_feed
        #         max_staleness=150,    # max_staleness
        #         min_samples=3,     # min_samples
        # )

        # print("Pausing vault...")
        # builder.pause(
        #     signer_bytes=signer_bytes,
        #     vault_id=3
        # )
        
        # print("Unpausing vault...")
        # builder.unpause(
        #     signer_bytes=signer_bytes,
        #     vault_id=3
        # )

        # print("Transferring authority...")
        # new_pending_authority = "CSsqdfpwwBK8iueo9CuTLHc1M2uubj88UwXKCgZap7H2"
        # builder.transfer_authority(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     pending_authority=new_pending_authority
        # )

        # print("Accepting authority...")
        # builder.accept_authority(
        #     signer_bytes=signer_bytes,
        #     vault_id=3
        # )

        # print("Closing CPI digest...")
        # digest = [0] * 32  # Replace with actual digest bytes
        # builder.close_cpi_digest(
        #     signer_bytes=signer_bytes,
        #     vault_id=vault_id,
        #     digest=digest
        # )

        # print("Updating exchange rate provider...")
        # new_provider = "31Uys8oYqNAiRUKR9i24qLaG5ninMFuXckpkfV3FaPDp"
        # builder.update_exchange_rate_provider(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     new_provider=new_provider
        # )

        # print("Configuring exchange rate update bounds...")
        # builder.configure_exchange_rate_update_bounds(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     upper_bound=10500,  # 105% (in basis points)
        #     lower_bound=9500,   # 95% (in basis points)
        #     minimum_update_delay=4200  # 1 hour in seconds
        # )

        # print("Updating exchange rate...")
        # builder.update_exchange_rate(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     new_exchange_rate=1_050_000_000  # 1.05 with 9 decimals
        # )

        # print("Setting fees...")
        # builder.set_fees(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     platform_fee_bps=50,    # 0.5% platform fee
        #     performance_fee_bps=100 # 1% performance fee
        # )

        # print("Setting payout address...")
        # new_payout = "CSsqdfpwwBK8iueo9CuTLHc1M2uubj88UwXKCgZap7H2"
        # builder.set_payout(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     new_payout=new_payout
        # )

        # print("Claiming fees in base asset...")
        # builder.claim_fees_in_base(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     sub_account=0
        # )

        # print("Setting withdraw authority...")
        # new_withdraw_authority = "CSsqdfpwwBK8iueo9CuTLHc1M2uubj88UwXKCgZap7H2"
        # builder.set_withdraw_authority(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     new_authority=new_withdraw_authority
        # )

        # print("Setting strategist...")
        # new_strategist = "CSsqdfpwwBK8iueo9CuTLHc1M2uubj88UwXKCgZap7H2"
        # builder.set_strategist(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     new_strategist=new_strategist
        # )

        # print("Depositing SOL...")
        # builder.deposit_sol(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     deposit_amount=10000000, # deposit_amount in lamports
        #     min_mint_amount=0,          # min_mint_amount
        # )

        # print("Depositing tokens...")
        # token_mint = "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"
        # builder.deposit(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     deposit_mint=token_mint,
        #     deposit_amount=1_000_000,
        #     min_mint_amount=0
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")


        # print("Withdrawing tokens...")
        # withdraw_mint = "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"
        # builder.withdraw(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     withdraw_mint=withdraw_mint,
        #     share_amount=1_000,
        #     min_asset_amount=0
        # )

        # print("Transferring SOL from sub account 0 to sub account 1")
        # builder.manage_transfer_sol_between_sub_accounts(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=None,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=0,  # source sub account
        #     to_sub_account=2,  # destination sub account
        #     amount=10000  # amount in lamports
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

        # print("Setting deposit sub-account...")
        # builder.set_deposit_sub_account(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     new_sub_account=2
        # )

        # print("Setting withdraw sub-account...")
        # builder.set_withdraw_sub_account(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     new_sub_account=2
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

        # print("Calling init_user_metadata...")
        # builder.manage_kamino_init_user_metadata(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=1,  # your vault ID
        #     sub_account=0,  # source sub account
        # )

        # print("Calling init_user_obligation...")
        # builder.manage_kamino_init_obligation(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=2,  # source sub account
        #     tag=0,
        #     id=0,
        # )
 
        # print("Calling lend jitoSOL...")
        # builder.lend(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=2,  # source sub account
        #     amount=25650000,
        #     tag=0,
        #     id=0,
        # )

        # print("Borrowing jitoSOL...")
        # builder.borrow(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=2,  # source sub account
        #     amount=10,
        #     tag=0,
        #     id=0,
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

        # print("Calling init obligation farms for reserve")
        # builder.manage_kamino_init_obligation_farms_for_reserve(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=2,  # source sub account
        #     tag=0,
        #     id=0,
        #     mode=0,
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

        # print("Refreshing price list...")
        # builder.manage_kamino_refresh_price_list(
        #     signer_bytes=signer_bytes,
        #     vault_id=3,
        #     sub_account=2,
        # )

        # print("Refreshing Kamino Reserve")
        # builder.manage_kamino_refresh_reserve(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,
        #     vault_id=3,
        #     sub_account=2,
        # )

        # print("Refreshing obligation")
        # builder.manage_kamino_refresh_obligation(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,
        #     vault_id=3,
        #     sub_account=2,
        #     tag=0,
        #     id=0,
        # )

        # print("Calling refresh obligation farms for reserve")
        # builder.manage_kamino_refresh_obligation_farms_for_reserve(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=2,  # source sub account
        #     tag=0,
        #     id=0,
        #     mode=0,
        # )

        # print("Depositing into Kamino lending...")
        # builder.manage_kamino_deposit(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=2,  # source sub account
        #     tag=0,
        #     id=0,
        #     amount=1000000,  # amount in lamports (0.0001 JitoSOL)
        # )

        # builder.manage_kamino_borrow(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,
        #     vault_id=3,
        #     sub_account=2,
        #     tag=0,
        #     id=0,
        #     amount=1,
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

        # Example usage
        # print("Depositing solend")
        # builder.manage_deposit_solend(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or authority.keypair().to_bytes() if needed
        #     vault_id=1,
        #     sub_account=0,
        #     deposit_mint="J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn",  # SOL mint
        #     reserve_collateral_mint="6mFgUsvXQTEYrYgowc9pVzYi49XEJA5uHA9gVDURc2pM",
        #     lending_market="4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY",
        #     reserve="BRsz1xVQMuVLbc4YjLP1FXhEx1LxSYig2nLqRgJEzR9r",
        #     reserve_liquidity_supply_spl_token_account="2Khz77qDAL4yY1wG6mTLhLnKiN7sDjQCtrFDEEUFPpiB",
        #     lending_market_authority="DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby",
        #     destination_deposit_reserve_collateral_supply_spl_token_account="3GynM9cRtZsZ2s1SyoAuSgTDjx8ANcVZJXZayuWZbMpd",
        #     pyth_price_oracle="7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE",
        #     switchboard_price_oracle="nu11111111111111111111111111111111111111111",
        #     amount=100000  # Amount in lamports
        # )

        # print("Wrapping SOL (first time)...")
        # builder.manage_wrap_sol(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=1,  # your vault ID
        #     sub_account=0,  # source sub account
        #     amount=100000  # amount in lamports (0.0001 SOL)
        # )

        # print("Unwrapping SOL (first time)...")
        # builder.manage_unwrap_sol(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=1,  # your vault ID
        #     sub_account=0,  # source sub account
        # )


        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

        # print("Wrapping SOL (second time)...")
        # builder.manage_wrap_sol(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=1,  # your vault ID
        #     sub_account=0,  # source sub account
        #     amount=100000  # amount in lamports (0.0001 SOL)
        # )

        # print("Unwrapping SOL (second time)...")
        # builder.manage_unwrap_sol(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=1,  # your vault ID
        #     sub_account=0,  # source sub account
        # )

        # print("Minting JitoSOL...")
        # builder.manage_mint_jito_sol(
        #     signer_bytes=signer_bytes,
        #     authority_bytes=signer_bytes,  # or None if no authority needed
        #     vault_id=3,  # your vault ID
        #     sub_account=2,  # source sub account
        #     amount=30000000  # amount in lamports (0.0001 SOL)
        # )

        # print("Getting lend digest...")
        # data = builder.get_lend_digest(
        #     vault_id=3,
        #     sub_account=2,
        #     tag=0,
        #     id=0
        # )

        print("Getting borrow digest...")
        data = builder.get_borrow_digest(
            vault_id=3,
            sub_account=2,
            tag=0,
            id=0
        )

        print(data)

        # print("Getting jito digest...")
        # builder.get_jito_digest(
        #     vault_id=3,
        #     sub_account=2
        # )

        # print("Sending instructions as one bundle...")
        # tx_hash = builder.try_bundle_all(signer_bytes)
        # print(f"Success! Transaction hash: {tx_hash}")

    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    # Make sure you have a local validator running (solana-test-validator)
    asyncio.run(main())
