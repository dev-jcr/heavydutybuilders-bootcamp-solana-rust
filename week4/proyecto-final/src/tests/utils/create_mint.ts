// Add "@solana/spl-token" to javascript package.json at dependencies

import { AnchorProvider, web3 } from "@coral-xyz/anchor";
import {
  createInitializeMintInstruction,
  MintLayout,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';

  //  Create new Token (Mint Account)
  export const createMint = async (
    provider: AnchorProvider,
    decimals = 0 // No decimals
  ): Promise<web3.PublicKey> => {
    // Token key pair creation
    const tokenMint = new web3.Keypair();
    // Calculate rent necessary to create token
    const lamportsForMint =
    await provider.connection.getMinimumBalanceForRentExemption(
      MintLayout.span // Mint Layout
    );

  // Allocate mint and wallet account
  await provider.sendAndConfirm(
    new web3.Transaction()
    .add(
      // Create mint account
      web3.SystemProgram.createAccount({
        programId: TOKEN_PROGRAM_ID, // program_id
        space: MintLayout.span, // space
        fromPubkey: provider.wallet.publicKey, // payer
        newAccountPubkey: tokenMint.publicKey, // token address
        lamports: lamportsForMint, // rent
      })
    )
    .add(
      // Initialize mint account
      createInitializeMintInstruction(
        tokenMint.publicKey, // token address
        decimals, // decimals
        provider.wallet.publicKey, // Mint Authority
        provider.wallet.publicKey // freeze authority
      )
    ),
    [tokenMint] // Signer
  );
  //returns new Token address
  return tokenMint.publicKey;