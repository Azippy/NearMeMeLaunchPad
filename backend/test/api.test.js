import assert from "node:assert/strict";
import { once } from "node:events";
import { after, before, test } from "node:test";

import app from "../app.js";
import {
  BONDING_CURVE_TYPES,
  LAUNCH_MODES,
  MAX_BUY_TAX_BPS,
  MAX_SELL_TAX_BPS,
  PROTOCOL_FEES,
} from "../config/constants.js";
import Launch from "../models/launch.js";
import Candle from "../models/candle.js";
import CreatorReward from "../models/creatorReward.js";
import Holder from "../models/holder.js";
import PairAsset from "../models/pairAsset.js";
import Pool from "../models/pool.js";
import ProtocolConfig from "../models/protocolConfig.js";
import RewardClaim from "../models/rewardClaim.js";
import Token from "../models/token.js";
import Trade from "../models/trade.js";
import Transaction from "../models/transaction.js";
import { queryNear, getBlock, getBlockByHeight } from "../near/rpcClient.js";
import { executeNearTransaction } from "../near/transactions.js";
import { callViewFunction } from "../near/viewMethods.js";
import { getLaunches } from "../services/launch/launchService.js";
import { validateLaunch } from "../services/launch/validateLaunch.js";
import validateTokenCreation from "../services/token/validateToken.js";

let server;
let baseUrl;

before(async () => {
  server = app.listen(0, "127.0.0.1");
  await once(server, "listening");
  baseUrl = `http://127.0.0.1:${server.address().port}`;
});

after(async () => {
  if (server) {
    server.close();
    await once(server, "close");
  }
});

test("health endpoint returns service status", async () => {
  const response = await fetch(`${baseUrl}/api/v1/health`);
  const body = await response.json();

  assert.equal(response.status, 200);
  assert.equal(body.success, true);
  assert.equal(body.application, "NearMeMePad");
  assert.ok(Number.isFinite(Date.parse(body.timestamp)));
});

test("all model and NEAR modules load", () => {
  assert.ok(
    [
      Candle,
      CreatorReward,
      Holder,
      Launch,
      PairAsset,
      Pool,
      ProtocolConfig,
      RewardClaim,
      Token,
      Trade,
      Transaction,
      queryNear,
      getBlock,
      getBlockByHeight,
      executeNearTransaction,
      callViewFunction,
    ].every(Boolean),
  );
});

test("unknown API routes return a structured 404", async () => {
  const response = await fetch(`${baseUrl}/api/v1/does-not-exist`);
  const body = await response.json();

  assert.equal(response.status, 404);
  assert.equal(body.success, false);
});

test("token validation normalizes valid input", () => {
  const token = validateTokenCreation({
    creator: "alice.near",
    name: "Example Token",
    symbol: "exm",
    logo: "https://example.com/token.png",
    description: "A test token",
    creatorPercent: 40,
    holdersPercent: 20,
    liquidityPercent: 20,
    burnPercent: 20,
    buyTaxPercent: 1.5,
    sellTaxPercent: 0,
  });

  assert.equal(token.symbol, "EXM");
  assert.equal(token.buyTaxBps, 150);
  assert.equal(token.sellTaxBps, 0);
});

test("token validation rejects unsafe URLs and invalid account IDs", () => {
  const input = {
    creator: "alice.near",
    name: "Example Token",
    symbol: "EXM",
    logo: "https://example.com/token.png",
    description: "A test token",
    creatorPercent: 40,
    holdersPercent: 20,
    liquidityPercent: 20,
    burnPercent: 20,
    buyTaxPercent: 0,
    sellTaxPercent: 0,
  };

  assert.throws(
    () => validateTokenCreation({ ...input, logo: "javascript:alert(1)" }),
    { statusCode: 400 },
  );
  assert.throws(
    () => validateTokenCreation({ ...input, creator: "Alice.NEAR" }),
    { statusCode: 400 },
  );
});

test("launch validation accepts a valid bonding curve", () => {
  const result = validateLaunch({
    creator: "alice.near",
    tokenContractId: "pending-token",
    tokenId: "507f1f77bcf86cd799439011",
    mode: LAUNCH_MODES.BONDING_CURVE,
    bondingCurve: {
      curveType: BONDING_CURVE_TYPES.CONSTANT_PRODUCT,
      virtualTokenReserve: "1000000",
      virtualQuoteReserve: "1000",
      realTokenReserve: "0",
      realQuoteReserve: "0",
      initialPrice: "1",
      graduationMarketCap: "100000",
      graduationLiquidity: "10000",
      tradingFeeBps: 100,
    },
  });

  assert.deepEqual(result, { valid: true, errors: [] });
});

test("launch validation rejects amounts outside u128", () => {
  const result = validateLaunch({
    creator: "alice.near",
    tokenContractId: "pending-token",
    tokenId: "507f1f77bcf86cd799439011",
    mode: LAUNCH_MODES.BONDING_CURVE,
    bondingCurve: {
      curveType: BONDING_CURVE_TYPES.CONSTANT_PRODUCT,
      virtualTokenReserve: "340282366920938463463374607431768211456",
      virtualQuoteReserve: "1000",
      realTokenReserve: "0",
      realQuoteReserve: "0",
      initialPrice: "1",
      graduationMarketCap: "100000",
      graduationLiquidity: "10000",
    },
  });

  assert.equal(result.valid, false);
  assert.ok(
    result.errors.some((error) => error.includes("virtualTokenReserve")),
  );
});

test("token and launch documents satisfy required MongoDB fields", async () => {
  const token = new Token({
    contractId: "pending-token",
    creator: "alice.near",
    name: "Example Token",
    symbol: "EXM",
    totalSupply: "1000000000",
    metadata: {
      image: "https://example.com/token.png",
      description: "A test token",
    },
    feeDistribution: {
      creatorPercent: 40,
      holdersPercent: 20,
      liquidityPercent: 20,
      burnPercent: 20,
    },
    tradingTax: {
      buyTaxBps: 100,
      sellTaxBps: 100,
    },
  });

  const launch = new Launch({
    launchId: "NMP-test",
    creator: "alice.near",
    tokenId: "507f1f77bcf86cd799439011",
    tokenContractId: "pending-token",
    mode: LAUNCH_MODES.BONDING_CURVE,
    blockchain: {
      network: "testnet",
      quoteAsset: "native.near",
      quoteAssetType: "NATIVE_NEAR",
      quoteAssetDecimals: 24,
    },
  });

  await token.validate();
  await launch.validate();
});

test("public fee limits match validator fee limits", () => {
  assert.equal(PROTOCOL_FEES.MAX_BUY_TAX_BPS, MAX_BUY_TAX_BPS);
  assert.equal(PROTOCOL_FEES.MAX_SELL_TAX_BPS, MAX_SELL_TAX_BPS);
});

test("launch pagination rejects invalid limits before database queries", async () => {
  await assert.rejects(() => getLaunches({ page: 0, limit: 20 }), {
    statusCode: 400,
  });
  await assert.rejects(() => getLaunches({ page: 1, limit: 101 }), {
    statusCode: 400,
  });
  await assert.rejects(
    () => getLaunches({ mode: "unknown", page: 1, limit: 20 }),
    { statusCode: 400 },
  );
  await assert.rejects(
    () => getLaunches({ creator: "Bad.Account", page: 1, limit: 20 }),
    { statusCode: 400 },
  );
});

test("token symbols have a unique database index", () => {
  const symbolIndex = Token.schema
    .indexes()
    .find(([keys]) => keys.symbol === 1);
  assert.ok(symbolIndex);
  assert.equal(symbolIndex[1].unique, true);
});
