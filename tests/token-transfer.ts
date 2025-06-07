import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenTransfer } from "../target/types/token_transfer";
import {assert} from "chai";
import {createMint , createAssociatedTokenAccount , mintTo , getAccount } from "@solana/spl-token"

describe("token-transfer", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.tokenTransfer as Program<TokenTransfer>;
  
  let mint = null;
  let senderTokenAccount = null;

  let receiverTokenAccount = null;
  let sender = provider.wallet;
let receiver = anchor.web3.Keypair.generate();
  it("Transfers tokens from one account to another", async () => {
  
    mint = await createMint(
      provider.connection,
      sender.payer,
      sender.publicKey,
      null, // No freeze authority
      6, // Decimals
    )
    console.log("Mint created:", mint.toBase58());
    senderTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      sender.payer,
      mint,
      sender.publicKey
    );

    console.log("Sender token account created:", senderTokenAccount.toBase58());
    receiverTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      sender.payer,
      mint,
      receiver.publicKey
    );
    console.log("Receiver token account created:", receiverTokenAccount.toBase58());

    // Mint some tokens to the sender's token account
    const amountToMint = 1000000; // 1 token with 6 decimals
    await mintTo(
      provider.connection,
      sender.payer,
      mint,
      senderTokenAccount,
      sender.publicKey,
      amountToMint
    );

    console.log("Minted tokens to sender's account:", amountToMint);
    // Check the balance of the sender's token account
    const senderBalance = await getAccount(provider.connection, senderTokenAccount);
    console.log("Sender's token account balance:", senderBalance.amount.toString());
    // Check the balance of the receiver's token account
    const receiverBalance = await getAccount(provider.connection, receiverTokenAccount);
    console.log("Receiver's token account balance before transfer:", receiverBalance.amount.toString());
    // Transfer tokens from sender's token account to receiver's token account
    const transferAmount = 500000; // 0.5 token with 6 decimals

    await program.methods
      .initialize(new anchor.BN(transferAmount)).accounts
      ({
        sender: sender.publicKey,
        sendersTokenAccount: senderTokenAccount,
        receiverTokenAccount: receiverTokenAccount,
        mint: mint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,

    }).signers([]).rpc();

    
    const senderBalance1 = await getAccount(provider.connection, senderTokenAccount);
    console.log("Sender's token account balance:", senderBalance1.amount.toString());
    // Check the balance of the receiver's token account
    const receiverBalance1 = await getAccount(provider.connection, receiverTokenAccount);
    console.log("Receiver's token account balance before transfer:", receiverBalance1.amount.toString());



  });
});
