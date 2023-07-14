import * as anchor from "@coral-xyz/anchor"
import { BN } from "@coral-xyz/anchor"
import { AnchorEscrow2023Timed, IDL } from "../target/types/Lupo"
import { PublicKey, Commitment, Keypair, SystemProgram } from "@solana/web3.js"
import { ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram, TOKEN_PROGRAM_ID as tokenProgram, createMint, createAccount, mintTo, getAssociatedTokenAddress } from "@solana/spl-token"
import { randomBytes } from "crypto"
import { assert } from "chai"

const commitment: Commitment = "confirmed"

describe("lupo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const commitment: Commitment = "confirmed"; // processed, confirmed, finalized

  const programId = new PublicKey("JB8tSGwspD1rjBRiE9G6yuoSgwmSnpPSL2XcyLoeegn6");
  const program = new anchor.Program<Lupo>(IDL, programId, anchor.getProvider());

  // Set up our keys
  const creator = new Keypair();
  const player = new Keypair();

  // Random seed
  const seed = new BN(randomBytes(8));
  
  // PDAs
  const auth = PublicKey.findProgramAddressSync([Buffer.from("auth")], program.programId)[0];


  // Mints
  let creator_token: PublicKey;
  let player_token: PublicKey;

  // ATAs
  let creator_ata: PublicKey;
  let player_ata: PublicKey;
  
  it("Airdrop", async () => {
    await Promise.all([creator].map(async (c) => {
      return await anchor.getProvider().connection.requestAirdrop(c.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL)
    })).then(confirmTxs);
  });

  it("Mint creator tokens", async () => {
    // Create mints and ATAs
    let [c , p] = await Promise.all([creator, player].map(async(a) => { return await newMintToAta(anchor.getProvider().connection, a) }))
    creator_token = c.mint;
    creator_ata = c.ata;
    player_token = p.mint;
    player_ata = p.ata;
  })

  it("Make a prediction", async () => {
    const signature = await program.methods
    .make_prediction(
      seed,
      new anchor.BN(10 * 1e6),
      new anchor.BN(20 * 1e6),
      new anchor.BN(10_000),
    )
    .accounts({
      creator: creator.publicKey,
      creatorAta: creator_ata,
      player: player.publicKey,
      playerAta: player_ata,
      auth,
      tokenProgram,
      associatedTokenProgram,
      systemProgram: SystemProgram.programId
    })
    .signers(
      [
        player
      ]
    )
    .rpc()
    await(confirmTx);
  });

  it("Claim", async () => {
    const signature = await program.methods
    .claim(
      seed,
      new anchor.BN(10 * 1e6),
      new anchor.BN(20 * 1e6),
      new anchor.BN(10_000),
    )
    .accounts({
      creator: creator.publicKey,
      creatorAta: creator_ata,
      player: player.publicKey,
      playerAta: player_ata,
      auth,
      tokenProgram,
      associatedTokenProgram,
      systemProgram: SystemProgram.programId
    })
    .signers(
      [
        creator
      ]
    )
    .rpc()
    await(confirmTx);
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  )
}

const confirmTxs = async (signatures: string[]) => {
  await Promise.all(signatures.map(confirmTx))
}

const newMintToAta = async (connection, minter: Keypair): Promise<{ mint: PublicKey, ata: PublicKey }> => { 
  const mint = await createMint(connection, minter, minter.publicKey, null, 6)
  // await getAccount(connection, mint, commitment)
  const ata = await createAccount(connection, minter, mint, minter.publicKey)
  const signature = await mintTo(connection, minter, mint, ata, minter, 21e8)
  await confirmTx(signature)
  return {
    mint,
    ata
  }
}