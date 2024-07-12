import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";

import { PublicKey } from "@solana/web3.js";

import { program, provider, connection } from "./helper";
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

  const MINT = new PublicKey("BEPcn67dJMpQRW3JkLxWCLdX9FC6fb8wSevVe5EkeU5x");
  const USDC = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW");
  const user = new PublicKey("CfN9A1tBhC7BxoubNkNuB8CrH6W6hojNhT5kGawNdupy");

  const nft_user = await getAssociatedTokenAddress(MINT, user);
  console.log("NFT user:", nft_user.toString());

  // const nft_game = await getOrCreateAssociatedTokenAccount(
  //   connection,
  //   payer,
  //   MINT,
  //   game_account,
  //   true
  // );

  const nft_game = await getAssociatedTokenAddress(MINT, game_account, true);
  console.log("NFT game:", nft_game.toString());

  const usdc_user = await getAssociatedTokenAddress(USDC, user);
  console.log("USDC user:", usdc_user.toString());

  const usdc_game = await getAssociatedTokenAddress(USDC, game_account, true);
  console.log("USDC game:", usdc_game.toString());

  let user1_account = getUserAccount(nft_user);
  console.log("\tUser account SOL:", user1_account.toString());

  console.log("----------------USER ACTIVE NFT ------------------");
  try {
    await program.methods
      .activeBySpl()
      .accounts({
        game: game_account,
        user: user,
        nftGame: nft_game,
        currencyGame: usdc_game,
        currencyMint: USDC,
        currencyUser: usdc_user,
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
