import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { Game2024 } from "../target/types/game_2024";
import { setTimeout } from "timers/promises";

import {
  SystemProgram,
  LAMPORTS_PER_SOL,
  sendAndConfirmRawTransaction,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import {
  createMint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
  createAccount,
  approve,
} from "@solana/spl-token";
import { assert } from "chai";

const address0 = new PublicKey("11111111111111111111111111111111");

describe("game-2024", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Game2024 as Program<Game2024>;

  const owner = provider.wallet as Wallet;
  const payer = owner.payer;
  let conn = program.provider.connection;

  it("Is initialized!", async () => {
    const game_account = getGameAccount();
    const admin_account = getAdminAccount();
    const operator_account = getOperatorAccount();

    let fees = [
      {
        currency: address0,
        amount: new anchor.BN(10),
        totalCollect: new anchor.BN(0),
        totalWithdraw: new anchor.BN(0),
      },
      {
        currency: new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW"),
        amount: new anchor.BN(10 ** 6),
        totalCollect: new anchor.BN(0),
        totalWithdraw: new anchor.BN(0),
      },
    ];

    try {
      await program.methods
        .init(new anchor.BN(600), fees)
        .accounts({
          game: game_account,
          adminAccount: admin_account,
          operatorAccount: operator_account,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }

    let game_account_info = await program.account.game.fetch(game_account);

    assert.equal(game_account_info.admin.toString(), admin_account.toString());
    assert.equal(game_account_info.durationActive.toNumber(), 600);
    assert.equal(
      game_account_info.operator.toString(),
      operator_account.toString()
    );

    let admin_account_info = await program.account.authorityRole.fetch(
      admin_account
    );
    assert.deepEqual(admin_account_info.authorities, [owner.publicKey]);

    let operator_account_info = await program.account.authorityRole.fetch(
      operator_account
    );
    assert.deepEqual(operator_account_info.authorities, [owner.publicKey]);

    /*
    console.log("-------------SET FEE-----------------");
    let usdcMint = await createMint(
      program.provider.connection,
      owner.payer,
      owner.publicKey,
      null,
      0
    );
    console.log("USDC MINT: ", usdcMint.toString());

    fees = [
      {
        currency: address0,
        amount: new anchor.BN(1000),
        totalCollect: new anchor.BN(0),
        totalWithdraw: new anchor.BN(0),
      },
      {
        currency: usdcMint,
        amount: new anchor.BN(200),
        totalCollect: new anchor.BN(0),
        totalWithdraw: new anchor.BN(0),
      },
    ];

    try {
      await program.methods
        .setFee(fees)
        .accounts({
          game: game_account,
          operatorAccount: operator_account,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }

    game_account_info = await program.account.game.fetch(game_account);

    for (let i = 0; i < fees.length; i++) {
      assert.equal(
        game_account_info.fees[i].currency.toString(),
        fees[i].currency.toString()
      );
      assert.equal(
        game_account_info.fees[i].amount.toNumber(),
        fees[i].amount.toNumber()
      );
      assert.equal(
        game_account_info.fees[i].totalCollect.toNumber(),
        fees[i].totalCollect.toNumber()
      );
      assert.equal(
        game_account_info.fees[i].totalWithdraw.toNumber(),
        fees[i].totalWithdraw.toNumber()
      );
    }

    console.log("-----------------USER 1 ACTIVE NFT 1--------------------");
    const user1 = await await create_user();
    console.log("\tUser 1 : ", user1.publicKey.toString());

    console.log("\tMint NFT-1 to User-1");

    let nft1 = await createMint(
      program.provider.connection,
      owner.payer,
      owner.publicKey,
      null,
      0
    );
    console.log("\tMINT 1: ", nft1.toString());

    let nft1_user1 = await getOrCreateAta(conn, payer, nft1, user1.publicKey);
    console.log("\tNFT-1 USER-1: ", nft1_user1.toString());

    await mintTo(conn, owner.payer, nft1, nft1_user1.address, payer, 1);

    let nft1_game = await getOrCreateAta(conn, payer, nft1, game_account);
    console.log("\tNFT-1 game: ", nft1_game.address.toString());

    let user1_account = getUserAccount(nft1_user1.address);
    console.log("\tUser-1 account SOL:", user1_account.toString());

    let nft1_game_balance = await conn.getTokenAccountBalance(
      nft1_game.address
    );
    console.log(nft1_game_balance.value.amount.toString());

    let nft1_user1_balance = await conn.getTokenAccountBalance(
      nft1_user1.address
    );
    console.log(nft1_user1_balance.value.amount.toString());

    let current = Math.floor(new Date().getTime() / 1000);
    console.log("\tCurrent: ", current);

    try {
      await program.methods
        .activeBySol()
        .accounts({
          game: game_account,
          user: user1.publicKey,
          nftGame: nft1_game.address,
          userAccount: user1_account,
          nftUser: nft1_user1.address,
          mint: nft1,
        })
        .signers([user1])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    console.log("\tCheck NFT1 game owner");
    nft1_game_balance = await conn.getTokenAccountBalance(nft1_game.address);
    console.log(nft1_game_balance.value.amount.toString());

    nft1_user1_balance = await conn.getTokenAccountBalance(nft1_user1.address);
    console.log(nft1_user1_balance.value.amount.toString());

    console.log("\tCheck game fee value");
    game_account_info = await program.account.game.fetch(game_account);
    for (let i = 0; i < game_account_info.fees.length; i++) {
      console.log(
        game_account_info.fees[i].currency +
          " - " +
          game_account_info.fees[i].amount +
          " - " +
          game_account_info.fees[i].totalCollect +
          " - " +
          game_account_info.fees[i].totalWithdraw
      );
    }

    let user1_account_info = await program.account.user.fetch(user1_account);
    // console.log(user1_account_info);
    assert.equal(
      user1_account_info.owner.toString(),
      user1.publicKey.toString()
    );
    // console.log(user1_account_info.time.toNumber())

    await setTimeout(2000);

    // console.log("----------------USER 1 DEACTIVE NFT 1------------------");
    // try {
    //   await program.methods
    //     .deactive()
    //     .accounts({
    //       game: game_account,
    //       user: user1.publicKey,
    //       nftGame: nft1_game.address,
    //       userAccount: user1_account,
    //       nftUser: nft1_user1.address,
    //       mint: nft1,
    //     })
    //     .signers([user1])
    //     .rpc();
    // } catch (error) {
    //   console.log(error);
    // }

    console.log("-----------------USER 2 ACTIVE NFT 2--------------------");
    const user2 = await await create_user();
    console.log("\tUser 2 : ", user2.publicKey.toString());

    console.log("\tMint NFT-2 to User-2");

    let nft2 = await createMint(
      program.provider.connection,
      owner.payer,
      owner.publicKey,
      null,
      0
    );
    console.log("\tMINT 2: ", nft1.toString());

    let nft2_user2 = await getOrCreateAta(conn, payer, nft2, user2.publicKey);
    console.log("\tNFT-2 USER-2: ", nft2_user2.address.toString());

    let usdc_user2 = await getOrCreateAta(
      conn,
      payer,
      usdcMint,
      user2.publicKey
    );
    console.log("\tUSDC USER-2: ", usdc_user2.address.toString());

    await mintTo(conn, owner.payer, nft2, nft2_user2.address, payer, 1);
    await mintTo(conn, owner.payer, usdcMint, usdc_user2.address, payer, 3000);

    let nft2_game = await getOrCreateAta(conn, payer, nft2, game_account);
    console.log("\tNFT-2 game: ", nft2_game.address.toString());

    let usdc_game = await getOrCreateAta(conn, payer, usdcMint, game_account);
    console.log("\tUSDC game: ", usdc_game.address.toString());

    let user2_account = getUserAccount(nft2_user2.address);
    console.log("\tUser-2 account :", user2_account.toString());

    let nft2_game_balance = await conn.getTokenAccountBalance(
      nft2_game.address
    );
    console.log(nft1_game_balance.value.amount.toString());

    let nft2_user2_balance = await conn.getTokenAccountBalance(
      nft2_user2.address
    );
    console.log(nft2_user2_balance.value.amount.toString());

    try {
      await program.methods
        .activeBySpl()
        .accounts({
          game: game_account,
          user: user2.publicKey,
          nftGame: nft2_game.address,
          userAccount: user2_account,
          nftUser: nft2_user2.address,
          mint: nft2,
          currencyGame: usdc_game.address,
          currencyMint: usdcMint,
          currencyUser: usdc_user2.address,
        })
        .signers([user2])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    console.log("\tCheck NFT2 game owner");
    nft2_game_balance = await conn.getTokenAccountBalance(nft2_game.address);
    console.log(nft2_game_balance.value.amount.toString());

    nft2_user2_balance = await conn.getTokenAccountBalance(nft2_user2.address);
    console.log(nft2_user2_balance.value.amount.toString());

    console.log("\tCheck game fee value");
    game_account_info = await program.account.game.fetch(game_account);
    for (let i = 0; i < game_account_info.fees.length; i++) {
      console.log(
        game_account_info.fees[i].currency +
          " - " +
          game_account_info.fees[i].amount +
          " - " +
          game_account_info.fees[i].totalCollect +
          " - " +
          game_account_info.fees[i].totalWithdraw
      );
    }

    let user2_account_info = await program.account.user.fetch(user2_account);
    // console.log(user1_account_info);
    assert.equal(
      user1_account_info.owner.toString(),
      user1.publicKey.toString()
    );
    */
  });

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

  async function create_user() {
    const buyer1 = new anchor.web3.Keypair();
    // console.log("Buyer : ", buyer1.publicKey.toString());

    await airdrop(conn, owner, buyer1.publicKey);

    return buyer1;
  }
});

async function airdrop(con, from, to) {
  let transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: LAMPORTS_PER_SOL,
    })
  );

  // Sign transaction, broadcast, and confirm
  await sendAndConfirmTransaction(con, transaction, [from.payer]);
}

async function getAta(mint, user) {
  return await getAssociatedTokenAddress(mint, user);
}

async function createAta(conn, payer, mint, to) {
  return await createAssociatedTokenAccount(conn, payer, mint, to);
}

async function getOrCreateAta(conn, payer, mint1, acc) {
  return await getOrCreateAssociatedTokenAccount(conn, payer, mint1, acc, true);
}
