import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { SolanaToken } from "../target/types/solana_token";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

export interface TestContext {
  provider: anchor.AnchorProvider;
  program: Program<SolanaToken>;
  authority: Wallet;
}

// Configure the client to use the local cluster.
export function setup(): TestContext {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaToken as Program<SolanaToken>;
  const authority = provider.wallet as anchor.Wallet;

  return {
    provider,
    program,
    authority,
  };
}

export async function initializeMint(
  program: Program<SolanaToken>,
  authority: PublicKey,
  decimals: number = 9
): Promise<PublicKey> {
  const mint = Keypair.generate();

  await program.methods
    .initialize(decimals)
    .accounts({
      mint: mint.publicKey,
      authority: authority,
      systemProgram: SystemProgram.programId,
    })
    .signers([mint])
    .rpc();

  return mint.publicKey;
}

export async function createTokenAccount(
  program: Program<SolanaToken>,
  mintAddress: PublicKey,
  owner: PublicKey,
  payer: PublicKey
): Promise<PublicKey> {
  // Derive PDA
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("token"), owner.toBuffer(), mintAddress.toBuffer()],
    program.programId
  );

  await program.methods
    .createTokenAccount()
    .accounts({
      mint: mintAddress,
      tokenAccount: pda,
      owner: owner,
      payer: payer,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  return pda;
}
