import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenLocked } from "../target/types/token_locked";
import { Connection, PublicKey } from "@solana/web3.js";
import fs from "fs";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import dayjs = require("dayjs");
import { BN, min } from "bn.js";
describe("token-locked", () => {
  const payerPath = "./tests/payer-wallet.json";
  const keypairBuffer = fs.readFileSync(payerPath);
  const payerWallet = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(keypairBuffer.toString()))
  );
  // Configure the client to use the local cluster.
  const connection = new Connection(
    "https://little-practical-wildflower.solana-devnet.discover.quiknode.pro/b33c1731ef24950ad5a92445bc1133e16e4271ed/"
  );
  const provider = new anchor.AnchorProvider(
    connection,
    new NodeWallet(payerWallet),
    { commitment: "confirmed" }
  );
  anchor.setProvider(provider);
  const mint = new PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");

  const program = anchor.workspace.TokenLocked as Program<TokenLocked>;

  // it("Is initialized!", async () => {
  //   try {
  //     const [lockAccount,_] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from(anchor.utils.bytes.utf8.encode("guac_lock")),
  //         payerWallet.publicKey.toBuffer(),
  //         mint.toBuffer(),
  //       ],
  //       program.programId
  //     );
  //    const lockAccountAta = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     payerWallet,
  //     mint,
  //     lockAccount,true
  //   );
  //   console.log("Lock Account",lockAccountAta.address.toBase58()); 
  //     const payerAccount = await getOrCreateAssociatedTokenAccount(
  //       connection,
  //       payerWallet,
  //       mint,
  //       payerWallet.publicKey
  //     );
  //     console.log("Payer Account",payerAccount.address.toBase58()); 
  //     const endDate = dayjs(Date.now()).add(7, "day").toDate().getTime();
  //     // Add your test here.
  //     const tx = await program.methods
  //       .initAccount(new BN(150 * Math.pow(10, 6)), new BN(endDate))
  //       .accounts({
  //         lockAccount: lockAccount,
  //         lockAta: lockAccountAta.address,
  //         feeAta: payerAccount.address,
  //         signerAta: payerAccount.address,
  //         signer: payerWallet.publicKey,
  //         mint: mint,
  //         tokenProgram:TOKEN_PROGRAM_ID
  //       }).signers([payerWallet])
  //       .rpc();
  //     console.log("Your transaction signature", tx);
  //   } catch (error) {
  //     console.log("Error",error)
  //   }
  // });
  it("Can Withdraw and close",async ()=>{
      try {
      const [lockAccount,_] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from(anchor.utils.bytes.utf8.encode("guac_lock")),
          payerWallet.publicKey.toBuffer(),
          mint.toBuffer(),
        ],
        program.programId
      );
     const lockAccountAta = await getOrCreateAssociatedTokenAccount(
      connection,
      payerWallet,
      mint,
      lockAccount,true
    );
    console.log("Lock Account",lockAccountAta.address.toBase58()); 
      const payerAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payerWallet,
        mint,
        payerWallet.publicKey
      );
      console.log("Payer Account",payerAccount.address.toBase58()); 
      const endDate = dayjs(Date.now()).add(7, "day").toDate().getTime();
      // Add your test here.
      const tx = await program.methods
        .closeAndWithdraw()
        .accounts({
          lockAccount: lockAccount,
          lockAta: lockAccountAta.address,
 
          signerAta: payerAccount.address,
          signer: payerWallet.publicKey,
          mint: mint,
          tokenProgram:TOKEN_PROGRAM_ID
        }).signers([payerWallet])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error",error)
    }
  })
});
