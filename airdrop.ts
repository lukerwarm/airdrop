import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js"
// We're also going to import our wallet and recreate the Keypair object
// using its private key:
import wallet from "./dev-wallet.json"
// We're going to import our keypair from the wallet file 
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet)); 
// Now we're going to establish a connection to the Solana devnet: 
// //Create aSolana devnet connection to devnet SOL tokens 
const connection = new Connection("https://api.devnet.solana.com"); 
//Finally, we're going to
// claim 2 devnet SOL tokens:
(async () => {
try {
// We're going to claim 2 devnet SOL tokens
    const txhash = await
    connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
} catch(e) {
console.error(`Oops, something went wrong: ${e}`)
}
})();