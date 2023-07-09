import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterProgram } from "../target/types/counter_program";
import { Keypair, SystemProgram } from "@solana/web3.js";
import assert from "assert";

describe("counter-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());


  const provider = anchor.AnchorProvider.env()
  const program = anchor.workspace.CounterProgram as Program<CounterProgram>;
  // create counter keypair
  const counter = Keypair.generate()


  it('should initialized!', async () => {
    await program.rpc.create({
      accounts: {
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [counter],
    });
  });



  it('should incrment by 1!', async () => {
    await program.rpc.increment({
      accounts: {
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
      },
      //signers: [counter],
    });
    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    assert.ok(counterAccount.count.toString() === "1");
    console.log('Increment count: ', counterAccount.count.toString());
  });


  it("Decrement counter", async () => {
    const tx = await program.methods.decrement()
      .accounts({
        counter: counter.publicKey,
        authority: provider.wallet.publicKey
      })
      .rpc()

    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    assert.ok(counterAccount.count.toString() === "0");

    console.log('Decrement count: ', counterAccount.count.toString());
  });

});
