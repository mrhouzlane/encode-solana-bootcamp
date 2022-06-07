import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, PublicKey } from "@solana/web3.js"
import { RussianRoulette } from "../target/types/russian_roulette";
const expect = require("chai").expect; 
const LAMPORTS_PER_SOL = 1_000_000_000;




describe("russian_rulette", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider); 
  const user = provider.wallet; //wallet provided to use 
  const program = anchor.workspace.RussianRulette as Program<RussianRoulette>;


  it("Initialized!", async () => {
    const signer_keypair: Keypair = anchor.web3.Keypair.generate();
    let russian_roulette_key: PublicKey;

    console.log("Test print");
    // Add your test here.

    // Give SOL to our account
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(signer_keypair.publicKey, 100 * LAMPORTS_PER_SOL)
    );
    console.log("Test end");

    //let create_game_tx = await program.methods.createGame(new anchor.BN(2)).signers([signer_keypair]).rpc();

   
    // russian_roulette.ticket_price = ticket_price;
  });

  it("Creates a game", async () => {
    //Add your test here.

    //1. Do an airdrop 
    const signer_keypair: Keypair = anchor.web3.Keypair.generate();
    //await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(RussianRoulette.publicKey, 100 * LAMPORTS_PER_SOL))

    //2. function createGame 
    const ticket_price = new anchor.BN(200); 
    const program = anchor.workspace.RussianRulette as Program<RussianRoulette>;

    let tx = await program.rpc.createGame(ticket_price, {
      accounts: {
        russianRoulette: signer_keypair.publicKey,
        owner: signer_keypair.publicKey,
        systemProgram: program.programId
      },
      options: {commitment: "confirmed"},
      signers: [signer_keypair],
      })

      console.log("Your transaction signature", tx);

  })


});

