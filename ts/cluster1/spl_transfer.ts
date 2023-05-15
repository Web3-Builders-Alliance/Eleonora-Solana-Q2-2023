import { Keypair, PublicKey, Connection, Commitment, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, transfer } from '@solana/spl-token';
import wallet from "../wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "max";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const mint = new PublicKey("45UVhvPtgZ4hWvumK4Yf5vnuVN3eCpg6ZsLJZjYNgwvW");

const to = new PublicKey("3B1LZsqUuayYhNnMrr7pLdZQpnU5yFkBpKdGs3sC4HFn");

(async () => {
    try{
        const from_ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );
        const to_ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );
        const txhash = transfer(
            connection,
            keypair,
            from_ata.address,
            to_ata.address,
            keypair.publicKey,
            1000
        );
        console.log(`Success! Check ouy your TX here:\nhttps://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }catch(e) {
            console.error(`Oops, something went wrong: ${e}`)
        }
})();