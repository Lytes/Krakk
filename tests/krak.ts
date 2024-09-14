import * as anchor from "@coral-xyz/anchor";
import { config } from "dotenv";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  Transaction,
  PublicKey,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as bs58 from "bs58";
import * as crypto from "crypto";
import { keccak256 } from "js-sha3";
import { HashBounty } from "../target/types/hash_bounty";
import * as idl from "../target/idl/hash_bounty.json";
import * as blake3 from "blake3";

config();

describe("krak", () => {
  // Configure the client to use the local cluster.
  const connection = new Connection(clusterApiUrl("devnet"), {
    commitment: "confirmed",
  });

  const playerOneSecretKey: string | undefined =
    process.env.PLAYER_ONE_SECRET_KEY;
  const playerTwoSecretKey: string | undefined =
    process.env.PLAYER_TWO_SECRET_KEY;

  if (!playerOneSecretKey || !playerTwoSecretKey) {
    throw new Error("Missing secret keys in the environment variables.");
  }

  const {
    provider: playerOneProvider,
    keypair: playerOneKP,
    program: playerOneProgram,
  } = getProviderAndProgram(playerOneSecretKey);

  const {
    provider: playerTwoProvider,
    keypair: playerTwoKP,
    program: playerTwoProgram,
  } = getProviderAndProgram(playerTwoSecretKey);

  const bountyAmt = new anchor.BN(100);
  const extraInfo = "It is a 10 char password";
  const cleartext = "password" + generateRandomNumber();
  let gameAccount: PublicKey;
  it("Place Bounty On SHA256 Hash!", async () => {
    const hash = crypto.createHash("sha256").update(cleartext).digest();

    const data = Buffer.from(cleartext, "utf-8");
    const newhash = crypto.createHash("sha256").update(data).digest();

    const ix = await playerOneProgram.methods
      .placeBounty(bountyAmt, Array.from(hash), extraInfo)
      .instruction();
    const tx = new Transaction();
    tx.add(ix);

    gameAccount = new PublicKey(ix.keys[1]["pubkey"].toString());

    const _txhash = await sendAndConfirmTransaction(connection, tx, [
      playerOneKP,
    ]);
    console.log(`Placed bounty ${_txhash} using SHA256`);
  });

  it("Claim Bounty On SHA256 Hash!", async () => {
    const hashType = { sha256: {} };
    const ix = await playerTwoProgram.methods
      .claimBounty(cleartext, hashType)
      .accountsPartial({
        bountyAcct: gameAccount,
        creator: playerOneKP.publicKey,
      })
      .instruction();
    const tx = new Transaction();
    tx.add(ix);

    const _txhash = await sendAndConfirmTransaction(connection, tx, [
      playerTwoKP,
    ]);
    console.log(`Claimed bounty ${_txhash} using SHA256`);
  });

  it("Place Bounty On SHA3 Hash!", async () => {
    const hash = Buffer.from(keccak256(cleartext), "hex");
    const ix = await playerOneProgram.methods
      .placeBounty(bountyAmt, Array.from(hash), extraInfo)
      .instruction();
    const tx = new Transaction();
    tx.add(ix);

    gameAccount = new PublicKey(ix.keys[1]["pubkey"].toString());

    const _txhash = await sendAndConfirmTransaction(connection, tx, [
      playerOneKP,
    ]);
    console.log(`Placed bounty ${_txhash} using SHA3`);
  });

  it("Claim Bounty On SHA3 Hash!", async () => {
    const hashType = { sha3: {} };
    const ix = await playerTwoProgram.methods
      .claimBounty(cleartext, hashType)
      .accountsPartial({
        bountyAcct: gameAccount,
        creator: playerOneKP.publicKey,
      })
      .instruction();
    const tx = new Transaction();
    tx.add(ix);

    const _txhash = await sendAndConfirmTransaction(connection, tx, [
      playerTwoKP,
    ]);
    console.log(`Claimed bounty ${_txhash} using SHA3`);
  });

  it("Place Bounty On BLAKE3 Hash!", async () => {
    const hash = blake3.hash(cleartext);
    const ix = await playerOneProgram.methods
      .placeBounty(bountyAmt, Array.from(hash), extraInfo)
      .instruction();
    const tx = new Transaction();
    tx.add(ix);

    gameAccount = new PublicKey(ix.keys[1]["pubkey"].toString());

    const _txhash = await sendAndConfirmTransaction(connection, tx, [
      playerOneKP,
    ]);
    console.log(`Placed bounty ${_txhash} using BLAKE3`);
  });

  it("Claim Bounty On BLAKE3 Hash!", async () => {
    const hashType = { blake3: {} };
    const ix = await playerTwoProgram.methods
      .claimBounty(cleartext, hashType)
      .accountsPartial({
        bountyAcct: gameAccount,
        creator: playerOneKP.publicKey,
      })
      .instruction();
    const tx = new Transaction();
    tx.add(ix);

    const _txhash = await sendAndConfirmTransaction(connection, tx, [
      playerTwoKP,
    ]);
    console.log(`Claimed bounty ${_txhash} using BLAKE3`);
  });

  function generateRandomNumber() {
    return Math.random();
  }

  function getProviderAndProgram(secretKey: string): {
    provider: anchor.Provider;
    keypair: anchor.web3.Keypair;
    program: anchor.Program<HashBounty>;
  } {
    const keypair = Keypair.fromSecretKey(bs58.decode(secretKey));
    const wallet = new anchor.Wallet(keypair);
    const provider = new anchor.AnchorProvider(connection, wallet, {
      skipPreflight: true,
      commitment: "confirmed",
    });
    const program = new anchor.Program(idl as unknown as HashBounty, provider);
    return { provider, keypair, program };
  }
});
