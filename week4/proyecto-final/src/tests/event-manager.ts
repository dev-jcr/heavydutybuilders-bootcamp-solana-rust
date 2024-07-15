import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EventManager } from "../target/types/event_manager";
import { BN } from "bn.js";
import { Keypair, PublicKey } from "@solana/web3.js";
import { createMint } from "./utils";

describe("event-manager", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EventManager as Program<EventManager>;

  // Event account addresses. Setting as public keys
  let acceptedMint: PublicKey; // Already existend address

  // PDA's
  let eventPublicKey: PublicKey;
  let eventMint: PublicKey;
  let treasuryVault: PublicKey;
  let gainVault: PublicKey;

  before(async () => {

    // find event account PDA
    [eventPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event","utf-8"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // find event mint account PDA
    [eventPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event_mint","utf-8"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // find treasury vault account PDA
    [eventPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury_vault","utf-8"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // find gain vault account PDA
    [eventPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("gain_vault","utf-8"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    
  });

  it ("Is initialized!", async () => {
    const name:string = "my_event";
    const ticketPrice = new BN(1);

    const tx = await program.methods.initialize().rpc();
    .accounts({
      event: eventPublicKey,
      acceptedMint: acceptedMint,
      eventMint:eventMint,
      treasuryVault: treasuryVault,
      gainVault: gainVault,
      authority: provider.wallet.publicKey,
    })
    .rpc();

    // Show new event info
    const eventAccount = await program.account.event.fetch(eventPublicKey);
    console.log("Event info: ", eventAccount);
  });

});