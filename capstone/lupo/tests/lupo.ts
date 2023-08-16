import * as anchor from "@coral-xyz/anchor"
import { BN } from "@coral-xyz/anchor"
import { Lupo, IDL } from "../target/types/lupo"
import { PublicKey, Commitment, Keypair, SystemProgram } from "@solana/web3.js"
import { ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram, TOKEN_PROGRAM_ID as tokenProgram, createMint, createAccount, mintTo, getAssociatedTokenAddress } from "@solana/spl-token"
import { assert } from "chai"
import wallet from "../wallet.json"

const commitment: Commitment = "confirmed"

describe("lupo", () => {

  const admin = Keypair.fromSecretKey(new Uint8Array(wallet));
  const connection = new anchor.web3.Connection("http://localhost:8899");
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(admin), { commitment: "finalized" } );

 

  // Program address
  const programId = new anchor.web3.PublicKey("CwWDEWrgBhJMt6Z23msSFQUZ2B2axptwFsmEFa5dW9dT");

  // Create program
  const program = new anchor.Program<Lupo>(IDL, programId, anchor.getProvider());

  // Create PDA VAULT STATE
  const vaultState =anchor.web3.Keypair.generate();

  // Create PDA VAULT AUTH
  const vault_auth_seeds = [Buffer.from("auth"), vaultState.publicKey.toBuffer()];
  const vault_auth = anchor.web3.PublicKey.findProgramAddressSync(vault_auth_seeds, program.programId)[0];

  // Create Vault system Program
  const vault_seeds = [Buffer.from("vault"), vault_auth.toBuffer()];
  const vaultDao = anchor.web3.PublicKey.findProgramAddressSync(vault_seeds, program.programId)[0];

  const global = PublicKey.findProgramAddressSync([Buffer.from("global"), admin.publicKey.toBytes()], program.programId)[0];

  const game = PublicKey.findProgramAddressSync([Buffer.from("game"), admin.publicKey.toBytes()], program.programId)[0];

  let title ="Game1";

  it("Is initialized!", async () => {
    // Add your test here.
        const txhash = await program.methods
        .initialize()
        .accounts({
            global,
            auth: vault_auth,
            vaultDao,
            usdcMint: vaultState.publicKey,
            admin: admin.publicKey,
            tokenProgram,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([
            admin,
            vaultState,
          ]).rpc();
          console.log(`Success! ${txhash}`);
  })

  it("Create Game!", async () => {
    // Add your test here.
        const txhash = await program.methods
        .createGame(title, new anchor.BN(10 * 1e6))
        .accounts({
            game,
            auth: vault_auth,
            vault: vaultDao,
            usdcMint: vaultState.publicKey,
            creator: admin.publicKey,
            tokenProgram,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([
            admin,
            vaultState,
          ]).rpc();
          console.log(`Success! ${txhash}`);
  })
  
  //player
  //creator == admin
  //create_game
  //make_prediction
  //finalize_game
  //claim
  
});

