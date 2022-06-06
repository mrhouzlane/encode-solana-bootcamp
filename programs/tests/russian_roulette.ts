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

  it("Is initialized!", async () => {
    
    // Add your test here.
    const signer_keypair: Keypair = anchor.web3.Keypair.generate();
    let russian_roulette_key: PublicKey;

    // Give SOL to our account
    airdrop(signer_keypair.publicKey, 100);
    
    //let create_game_tx = await program.methods.createGame(new anchor.BN(2))
    //            .signers([signer_keypair]).rpc();

    // console.log("Your transaction signature", create_game_tx);
  });
  
  async function airdrop(requester: PublicKey, amount: number) {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(requester, amount * LAMPORTS_PER_SOL)
    );
  }

});