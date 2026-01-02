import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaToken } from "../target/types/solana_token";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("solana-token", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaToken as Program<SolanaToken>;

  // Test accounts
  const authority = provider.wallet as anchor.Wallet;
  const decimals = 9;

  it("Should initialize token mint", async () => {
    // Generate a new keypair for the mint.
    const mint = Keypair.generate();

    const tx = await program.methods
      .initialize(decimals)
      .accounts({
        mint: mint.publicKey,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([mint])
      .rpc();

    console.log("Initialize transaction signature:", tx);

    // Fetch the mint account and verify.
    const mintData = await program.account.tokenMint.fetch(mint.publicKey);

    assert.equal(mintData.authority.toString(), authority.publicKey.toString());
    assert.equal(mintData.totalSupply.toNumber(), 0);
    assert.equal(mintData.decimals, decimals);

    console.log("✅ Token mint initialized successfully");
    console.log("   Mint address:", mint.publicKey.toString());
    console.log("   Authority:", mintData.authority.toString());
    console.log("   Decimals:", mintData.decimals);
  });
});
