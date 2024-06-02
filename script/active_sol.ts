import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";

import { program, provider, connection, wallet } from "./helper";
import {
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { min } from "bn.js";

let owner = provider.wallet as Wallet;
const payer = owner.payer;

async function active_sol() {
  anchor.setProvider(provider);
  const game_account = getGameAccount();
  const admin_account = getAdminAccount();
  const operator_account = getOperatorAccount();

  const MINT = new PublicKey("VxaieiYkWUkXL6NFqAhp8aVwDVq3xgt7cvxtXMju4jo");
  const user = wallet.publicKey;

  const nft_user = await getAssociatedTokenAddress(MINT, user);
  console.log("NFT user:", nft_user.toString());

  const nft_game = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    MINT,
    game_account,
    true
  );
  console.log("NFT game:", nft_game.address.toString());

  let user1_account = getUserAccount(nft_user);
  console.log("\tUser account SOL:", user1_account.toString());

  console.log("----------------USER ACTIVE NFT ------------------");
  try {
    await program.methods
      .activeBySol()
      .accounts({
        game: game_account,
        user: user,
        nftGame: nft_game.address,
        userAccount: user1_account,
        nftUser: nft_user,
        mint: MINT,
      })
      // .signers([user1])
      .rpc();
  } catch (error) {
    console.log(error);
  }
}

const getGameAccount = () => {
  const GAME_ACCOUNT = "GAME_ACCOUNT";
  const [game] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(GAME_ACCOUNT)],
    program.programId
  );
  console.log("Game Account: ", game.toString());
  return game;
};

const getOperatorAccount = () => {
  const OPERATOR_ROLE = "OPERATOR_ROLE";
  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(OPERATOR_ROLE)],
    program.programId
  );
  console.log("operator_account: ", mint.toString());
  return mint;
};

const getAdminAccount = () => {
  const ADMIN_ROLE = "ADMIN_ROLE";
  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(ADMIN_ROLE)],
    program.programId
  );
  console.log("admin_account: ", mint.toString());

  return mint;
};

const getUserAccount = (user_ata) => {
  const USER_ACCOUNT = "USER_ACCOUNT";
  const [user_account] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(USER_ACCOUNT), user_ata.toBuffer()],
    program.programId
  );
  // console.log("User account: ", user_account);

  return user_account;
};

active_sol();
