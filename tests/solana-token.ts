import { assert } from "chai";
import {
  setup,
  initializeMint,
  createTokenAccount,
  mintTokens,
  transferTokens,
  burnTokens,
} from "./utils";
import { Keypair } from "@solana/web3.js";

describe("solana-token", () => {
  const decimals = 9;
  const { provider: _provider, program, authority } = setup();

  // Happy path test cases
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
      1000
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

  it("Should transfer tokens between users", async () => {
    // Setup
    const mintAddress = await initializeMint(
      program,
      authority.publicKey,
      decimals
    );
    const user1 = Keypair.generate();
    const user2 = Keypair.generate();

    const user1TokenAccount = await createTokenAccount(
      program,
      mintAddress,
      user1.publicKey,
      authority.publicKey
    );
    const user2TokenAccount = await createTokenAccount(
      program,
      mintAddress,
      user2.publicKey,
      authority.publicKey
    );

    // Mint tokens
    const mintAmount = 1000;
    await mintTokens(
      program,
      mintAddress,
      user1TokenAccount,
      authority.publicKey,
      mintAmount
    );

    // Transfer tokens from user1 to user2
    const transferAmount = 300;
    await transferTokens(
      program,
      user1TokenAccount,
      user2TokenAccount,
      user1,
      transferAmount
    );

    // Verify balances
    const user1Data = await program.account.tokenAccount.fetch(
      user1TokenAccount
    );
    const user2Data = await program.account.tokenAccount.fetch(
      user2TokenAccount
    );

    assert.equal(user1Data.amount.toNumber(), mintAmount - transferAmount);
    assert.equal(user2Data.amount.toNumber(), transferAmount);

    console.log("✅ Transferred tokens");
    console.log("   User1 balance:", user1Data.amount.toNumber());
    console.log("   User2 balance:", user2Data.amount.toNumber());
  });

  it("Should burn tokens from user", async () => {
    // Setup
    const mintAddress = await initializeMint(
      program,
      authority.publicKey,
      decimals
    );
    const user = Keypair.generate();
    const userTokenAccount = await createTokenAccount(
      program,
      mintAddress,
      user.publicKey,
      authority.publicKey
    );

    // Mint tokens for user
    const mintAmount = 2000;
    await mintTokens(
      program,
      mintAddress,
      userTokenAccount,
      authority.publicKey,
      mintAmount
    );

    // Burn tokens for user
    const burnAmount = 500;
    await burnTokens(program, mintAddress, userTokenAccount, user, burnAmount);

    // Verify user balance
    const accountData = await program.account.tokenAccount.fetch(
      userTokenAccount
    );
    assert.equal(accountData.amount.toNumber(), mintAmount - burnAmount);

    // Very mint token balance
    const mintData = await program.account.tokenMint.fetch(mintAddress);
    assert.equal(mintData.totalSupply.toNumber(), mintAmount - burnAmount);

    console.log("✅ Burned tokens");
    console.log("   Remaining balance:", accountData.amount.toNumber());
    console.log("   Total supply:", mintData.totalSupply.toNumber());
  });

  // Error test cases
  it("Should not transfer more than balance", async () => {
    const mintAddress = await initializeMint(
      program,
      authority.publicKey,
      decimals
    );
    const user1 = Keypair.generate();
    const user2 = Keypair.generate();

    const user1TokenAccount = await createTokenAccount(
      program,
      mintAddress,
      user1.publicKey,
      authority.publicKey
    );
    const user2TokenAccount = await createTokenAccount(
      program,
      mintAddress,
      user2.publicKey,
      authority.publicKey
    );

    // Mint tokens
    const mintAmount = 100;
    await mintTokens(
      program,
      mintAddress,
      user1TokenAccount,
      authority.publicKey,
      mintAmount
    );

    // Try and transfer more than mint amount (user 1 balance).
    try {
      await transferTokens(
        program,
        user1TokenAccount,
        user2TokenAccount,
        user1,
        mintAmount + 1
      );
      assert.fail("Expected error was not thrown");
    } catch (error) {
      assert.include(error.toString(), "InsufficientFunds");
      console.log(
        "Attempted transfer greater than balance failed."
      );
    }
  });

  it("Should not burn more than the balance", async () => {
    const mintAddress = await initializeMint(
      program,
      authority.publicKey,
      decimals
    );
    const user = Keypair.generate();
    const tokenAccount = await createTokenAccount(program, mintAddress, user.publicKey, authority.publicKey);

    // Mint tokens
    const mintAmount = 100;
    await mintTokens(program, mintAddress, tokenAccount, authority.publicKey, mintAmount);

    // Try and burn more than mintAmount (user balance).
    try {
      await burnTokens(program, mintAddress, tokenAccount, user, mintAmount + 1);
      assert.fail("Attempted burn more than balance failed.");
    } catch (error) {
      assert.include(error.toString(), "InsufficientFunds");
      console.log("Attempted burn more than balance failed.");
    }
  });
});
