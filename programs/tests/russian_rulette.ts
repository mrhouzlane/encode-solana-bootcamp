import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RussianRulette } from "../target/types/russian_rulette";

describe("russian_rulette", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RussianRulette as Program<RussianRulette>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createGame().rpc();
    console.log("Your transaction signature", tx);
  });
});
