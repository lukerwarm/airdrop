mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::Turbin3_prereq::{self, *};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        bs58, hash,
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        system_program,
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;
    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    // Success! Check out your TX here:
    // https://explorer.solana.com/tx/33uLxrgWRfJFSxUwUHsCj9u3eUVZ13VrbuF54ss67pCaQoEqgsC82XfbJU4QTZ6NdaZ12FhkHmrRcrZpQTi8Pdiu?cluster=devnet
    #[test]
    fn airdop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");

                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    //Verification failed
    //Success! Check out your TX here: https://explorer.solana.com/tx/5e1DvUWszrSukabhsRX7Um6onA8pLvfY23pD7UpUQP8i6B9Nv2NJnb71ExtTRLCpvMRgqcXWw9EQVzetW3X4kfza/?cluster=devnet

    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // With the imported Keypair, we can sign a new message.
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my solana Keypair!";
        let msg_hashed = hash::hash(message_bytes);
        let sig = keypair.sign_message(msg_hashed.as_ref());

        // I'm not sure why we were hashing the signature.
        let sig_hashed = hash::hash(sig.as_ref());
        // After that we can verify the singature, using the default implementation
        match sig.verify(&pubkey.to_bytes(), msg_hashed.as_ref()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("3LkYeiDfmkDfvbtaedgWUA3T4jU6f8GuY5st77FcVJMG").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recentblockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    // Success! Check out your TX here: https://explorer.solana.com/tx/4JGyCZUMCcrNJXxxxsvxSzXyfAKbBHwLXF64ShdLgADFhZWQys76w4t8d8F6Z7mmfJwdBSGbYMDNZR47YWQ2YYtH/?cluster=devnet
    #[test]
    fn empty_wallet() {
        // Get balance of dev wallet
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recentblockhash");

        let to_pubkey = Pubkey::from_str("3LkYeiDfmkDfvbtaedgWUA3T4jU6f8GuY5st77FcVJMG").unwrap();

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let t = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&t)
            .expect("Failed to send transaction");
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("../../turbin3_devnet_wallet.json")
            .expect("Couldn't find wallet file");
        let prereq = Turbin3_prereq::TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        println!("Prereq:{:?}", prereq.to_string());
        let args = Turbin3_prereq::CompleteArgs {
            github: b"lukerwarm".to_vec(),
        };
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recentblockhash");

        let transaction2 = Turbin3_prereq::TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction2)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
