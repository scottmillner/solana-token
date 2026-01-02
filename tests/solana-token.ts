import { assert } from "chai";
import { setup, initializeMint } from "./utils";

describe("solana-token", () => {
  const decimals = 9;
  const { provider: _provider, program, authority } = setup();

  it("Should initialize token mint", async () => {
    const mintAddress = await initializeMint(program, authority.publicKey, decimals);
    const mintData = await program.account.tokenMint.fetch(mintAddress);

    assert.equal(mintData.authority.toString(), authority.publicKey.toString());
    assert.equal(mintData.totalSupply.toNumber(), 0);
    assert.equal(mintData.decimals, decimals);

    console.log("✅ Token mint initialized successfully");
    console.log("   Mint address:", mintAddress.toString());
  });
});
