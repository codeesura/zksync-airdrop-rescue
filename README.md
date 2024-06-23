# ZkSync Airdrop Rescue Tool

## Overview

This tool automates the process of claiming airdrops on the ZkSync Era network, specifically designed for compromised wallets with no ETH. It uses a paymaster for gasless transactions, allowing claims even from wallets without funds.

## ⚠️ Disclaimer

This tool is for educational and recovery purposes only. Use at your own risk and ensure you have the right to claim the targeted airdrops.

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/codeesura/zksync-airdrop-rescue.git
   cd zksync-airdrop-rescue
   ```

2. Install Rust:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Create required JSON files:
   - `airdrop_data.json`: Contains merkle proofs and airdrop data.
   - `wallets.json`: Contains private keys of wallets to claim for.

4. Format of `airdrop_data.json`:
   ```json
   {
     "0x1234...": [{
       "userId": "0x1234...",
       "tokenAmount": "1000000000000000000",
       "merkleIndex": "123456",
       "merkleProof": [
         "0xabcd...",
         "0xdef0...",
         ...
       ]
     }],
     ...
   }
   ```

5. Format of `wallets.json`:
   ```json
   {
     "private_keys": [
       "0xabcd1234...",
       "0xefgh5678...",
       ...
     ]
   }
   ```

6. Adjust constants in `src/utils/constants.rs` if needed.

7. Deploy a paymaster contract and fund it with ETH. Update the paymaster address in your configuration.

## Usage

1. Ensure `airdrop_data.json` and `wallets.json` are properly set up.

2. Deploy and fund the paymaster contract:
   - Deploy the paymaster contract to ZkSync Era.
   - Send sufficient ETH to the paymaster contract to cover gas fees.

3. Update the paymaster contract address in `src/utils/constants.rs`.

4. Run the tool:
   ```
   cargo run --release
   ```

5. The tool will automatically:
   - Load wallet configurations and airdrop data.
   - Attempt to claim airdrops for each wallet using gasless transactions.
   - Transfer claimed tokens to a secure address (if configured).

6. Monitor the console output for progress and any error messages.

## Security Considerations

- Never share or commit your `wallets.json` file.
- Run this tool in a secure environment.
- After successful claims, move funds to a new, secure wallet.
- Ensure the paymaster contract is properly audited and secure.

## Technical Details

- Built with Rust for high performance and memory safety.
- Uses `alloy_signer_wallet::LocalWallet` for wallet management.
- Implements concurrent processing for faster execution.
- Utilizes a paymaster contract for gasless transactions.
- Employs Merkle proofs for secure airdrop claiming.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions, issues, and feature requests are welcome! Check the [issues page](https://github.com/codeesura/zksync-airdrop-rescue/issues).

## Contact

ArmutBey - [@codeesura](https://twitter.com/codeesura) - codeesura@gmail.com

Project Link: [https://github.com/codeesura/zksync-airdrop-rescue](https://github.com/yourusername/zksync-airdrop-rescue)
