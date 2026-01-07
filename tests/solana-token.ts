import { assert } from "chai";
import { setup, initializeMint, createTokenAccount, mintTokens } from "./utils";
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
    const accountData = await program.account.tokenAccount.fetch(
      tokenAccountAddress
    );

    assert.equal(accountData.owner.toString(), user.publicKey.toString());
    assert.equal(accountData.mint.toString(), mintAddress.toString());
    assert.equal(accountData.amount.toNumber(), 0);
  });

  it("Should mint tokens to user", async () => {
    // Setup
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

    // Mint 1000 tokens
    const mintAmount = 1000;
    await mintTokens(
      program,
      mintAddress,
      tokenAccountAddress,
      authority.publicKey,
      mintAmount
    );

    // Verify token account balance
    const accountData = await program.account.tokenAccount.fetch(
      tokenAccountAddress
    );
    assert.equal(accountData.amount.toNumber(), mintAmount);

    // Verify total supply
    const mintData = await program.account.tokenMint.fetch(mintAddress);
    assert.equal(mintData.totalSupply.toNumber(), mintAmount);

    console.log("✅ Minted tokens");
    console.log("   Token account balance:", accountData.amount.toNumber());
    console.log("   Total supply:", mintData.totalSupply.toNumber());
  });
});
