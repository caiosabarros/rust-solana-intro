import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OwnedCalculator } from "../target/types/owned_calculator";
import { assert } from "chai";

describe("owned_calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OwnedCalculator as Program<OwnedCalculator>;
  let keyPair = anchor.web3.Keypair.generate(); // generate random keypair

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Simple add two numbers", async () => {
    // add()
    const tx = await program.methods.add(new anchor.BN(1), new anchor.BN(2)).rpc();
    console.log("tx sig", tx);
  });

  it("Owner multiplication", async () => {
    // mul
    const tx = await program.methods.mul(new anchor.BN(2), new anchor.BN(3)).accounts({ signerAccount: program.provider.publicKey }).rpc();
    console.log("tx sig", tx);
  });

  it("Not owner cannot multiply", async () => {
    try {
      const tx = await program.methods.mul(new anchor.BN(2), new anchor.BN(3)).accounts({
        signerAccount: keyPair.publicKey
      })
        .signers([keyPair])
        .rpc();
    } catch (_err) {
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: anchor.AnchorError = _err;
      const message = "You are a strange, not the owner!";
      assert.strictEqual(err.error.errorMessage, message);
    }
  });

  it("Add multiple", async () => {
    const tx = await program.methods.multipleAdd([new anchor.BN(1), new anchor.BN(2), new anchor.BN(1e3)]).rpc();
    console.log("tx sig", tx);
  });

});
