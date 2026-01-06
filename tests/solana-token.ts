import { assert } from "chai";
import { setup, initializeMint, createTokenAccount } from "./utils";
import { Keypair } from "@solana/web3.js";

describe("solana-token", () => {
  const decimals = 9;
  const { provider: _provider, program, authority } = setup();

  it("Should initialize token mint", async () => {
    const mintAddress = await initializeMint(
      program,
      authority.publicKey,
      decimals
    );
    const mintData = await program.account.tokenMint.fetch(mintAddress);

    assert.equal(mintData.authority.toString(), authority.publicKey.toString());
    assert.equal(mintData.totalSupply.toNumber(), 0);
    assert.equal(mintData.decimals, decimals);

    console.log("✅ Token mint initialized successfully");
    console.log("   Mint address:", mintAddress.toString());
  });

  it("Should create token account for user", async () => {
    const mintAddress = await initializeMint(
      program,
      authority.publicKey,
      decimals
    );
    const user = Keypair.generate();

    const tokenAccountAddress = await createTokenAccount(
      program,
      mintAddress,
      user.publicKey,
      authority.publicKey
    );
    const accountData = await program.account.tokenAccount.fetch(tokenAccountAddress);

    assert.equal(accountData.owner.toString(), user.publicKey.toString());
    assert.equal(accountData.mint.toString(), mintAddress.toString());
    assert.equal(accountData.amount.toNumber(), 0);
  });
});
