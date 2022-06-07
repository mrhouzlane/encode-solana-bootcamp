import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, PublicKey } from "@solana/web3.js"
import { RussianRoulette } from "../target/types/russian_roulette";

const LAMPORTS_PER_SOL = 1_000_000_000;




describe("russian_rulette", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RussianRulette as Program<RussianRoulette>;
  const user = provider.wallet;

  it("Initialized!", async () => {
    const signer_keypair: Keypair = anchor.web3.Keypair.generate();
    const tx =  await program.methods.createGame(new anchor.BN(2)).signers([signer_keypair]).rpc();
  });

    // console.log("Test print");
    // // Add your test here.
    // //const tx = await program.methods.execute("").rpc();
    // const signer_keypair: Keypair = anchor.web3.Keypair.generate();
    // let russian_roulette_key: PublicKey;

    // // Give SOL to our account
    // await provider.connection.confirmTransaction(
    //   await provider.connection.requestAirdrop(signer_keypair.publicKey, 100 * LAMPORTS_PER_SOL)
    // );
    // const program = anchor.workspace.RussianRulette as Program<RussianRoulette>;
    // console.log("Test end");

    // //Create game test : 
    // const ticket_price = 20;
    // let create_game_tx = await program.methods.createGame(new anchor.BN(2)).signers([signer_keypair]).rpc();

   
    // // russian_roulette.ticket_price = ticket_price;

});

