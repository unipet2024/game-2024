import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { IDL } from "../target/types/game_2024";
import { Wallet } from "@coral-xyz/anchor";
// import { setTimeout } from "timers/promises";

import { PublicKey, Keypair, Connection, clusterApiUrl } from "@solana/web3.js";
export const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
// export const wallet = Keypair.fromSecretKey(
//   Uint8Array.from([
//     60, 31, 216, 134, 68, 78, 5, 54, 175, 135, 221, 227, 168, 70, 131, 114, 133,
//     65, 139, 93, 195, 126, 28, 32, 17, 15, 252, 196, 1, 237, 44, 57, 8, 134, 50,
//     123, 56, 199, 184, 99, 61, 162, 196, 68, 143, 51, 117, 64, 26, 54, 84, 218,
//     154, 157, 209, 231, 34, 3, 251, 190, 216, 153, 90, 113,
//   ])
// );

// Example usage

// export const wallet = Keypair.fromSecretKey(
//   Buffer.from(
//     "4w99u92THAnLeN5NDURC5SVyxng4apQUxD8kkigrkDqdbGtPMT3C693WWWKxL1Encf7WesVFyjXkPfCHoCjU8aCh",
//     "hex"
//   )
// );

// console.log("Wallet:", wallet.publicKey.toString());

import bs58 from "bs58";

function base58ToKeypair(base58PrivateKey: string): Keypair {
  try {
    const privateKeyBuffer = bs58.decode(base58PrivateKey);
    return Keypair.fromSecretKey(privateKeyBuffer);
  } catch (error) {
    throw new Error("Invalid base58 private key.");
  }
}

// Example usage
const base58PrivateKey =
  "5NyxTRB5L9JLdaA3NRwTvWyG9e3XSvNrjo8agqBaU3RUKCZJ9yDV74r6P2K2bTHWBg1tvFLkG1pCCmSyqD7KLJBX"; // Replace with actual base58 private key
export const wallet = base58ToKeypair(base58PrivateKey);
console.log(`Public Key: ${wallet.publicKey.toBase58()}`); //prints the base58-encoded public key
// console.log(`Private Key (Base58): ${wallet.secretKey.toString()}`); // prints the base58-encoded private key

export const provider = new AnchorProvider(
  connection,
  new Wallet(wallet),
  anchor.AnchorProvider.defaultOptions()
);
const idl = IDL;
// Address of the deployed program.
const programId = "GRxJzSgBKJkgYRtjCzciCf1Pf68iGkygcH5s2RPvxj6Z";
// Generate the program client from IDL.
export const program = new anchor.Program(idl, programId, provider);
