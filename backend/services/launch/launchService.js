import crypto from "crypto";
import Launch from "../../models/launch.js";
import Token from "../../models/token.js";
import PairAsset from "../../models/pairAsset.js";
import AppError from "../../utils/AppError.js";

import {
  LAUNCH_MODES,
  LAUNCH_STATUS,
  NATIVE_NEAR,
} from "../../config/constants.js";
import env from "../../config/env.js";

import { validateLaunch } from "./validateLaunch.js";

const isNearAccountId = (value) =>
  typeof value === "string" &&
  value.length >= 2 &&
  value.length <= 64 &&
  /^(?:[a-z0-9]+(?:[._-][a-z0-9]+)*)$/.test(value);

const createLaunch = async (data) => {
  const validation = validateLaunch(data);

  if (!validation.valid) {
    throw new AppError(validation.errors.join("; "), 400);
  }

  const {
    creator,
    tokenContractId,
    tokenId,
    mode,
    description,
    directMarket,
    bondingCurve,
  } = data;

  // -------------------------
  // VERIFY TOKEN
  // -------------------------

  const token = await Token.findById(tokenId);

  if (!token) {
    throw new AppError("Token record not found", 404);
  }

  if (token.creator !== creator) {
    throw new AppError("Only the token creator can launch this token", 403);
  }

  if (token.contractId !== tokenContractId) {
    throw new AppError("Token contract does not match token record", 400);
  }

  // -------------------------
  // PREVENT DUPLICATE ACTIVE
  // LAUNCHES FOR SAME TOKEN
  // -------------------------

  const existingLaunch = await Launch.findOne({
    tokenContractId,
    status: {
      $in: [
        LAUNCH_STATUS.PENDING,
        LAUNCH_STATUS.ACTIVE,
        LAUNCH_STATUS.GRADUATING,
        LAUNCH_STATUS.GRADUATED,
      ],
    },
  });

  if (existingLaunch) {
    throw new AppError("This token already has an active launch", 409);
  }

  // -------------------------
  // CREATE INTERNAL LAUNCH ID
  // -------------------------

  const launchId = `NMP-${crypto.randomUUID()}`;

  const quoteAssetId =
    mode === LAUNCH_MODES.DIRECT_MARKET
      ? directMarket.quoteTokenId
      : bondingCurve.quoteAssetId || NATIVE_NEAR.assetId;

  const quoteAsset =
    quoteAssetId === NATIVE_NEAR.assetId
      ? {
          assetType: "NATIVE_NEAR",
          decimals: NATIVE_NEAR.decimals,
        }
      : await PairAsset.findOne({
          contractId: quoteAssetId,
          enabled: true,
        }).lean();

  if (!quoteAsset) {
    throw new AppError("Quote asset is not enabled", 400);
  }

  const quoteAssetType =
    quoteAsset.assetType === "native" ? "NATIVE_NEAR" : quoteAsset.assetType;

  // -------------------------
  // BUILD DOCUMENT
  // -------------------------

  const launchData = {
    launchId,

    creator,

    tokenId,

    tokenContractId,

    mode,

    description,

    status: LAUNCH_STATUS.PENDING,

    blockchain: {
      network: env.near.network,
      factoryContractId: env.near.factoryContractId,
      quoteAsset: quoteAssetId,
      quoteAssetType,
      quoteAssetDecimals: quoteAsset.decimals,
    },
  };

  // -------------------------
  // MODE-SPECIFIC DATA
  // -------------------------

  if (mode === LAUNCH_MODES.DIRECT_MARKET) {
    launchData.directMarket = {
      quoteTokenId: directMarket.quoteTokenId,

      baseLiquidity: directMarket.baseLiquidity,

      quoteLiquidity: directMarket.quoteLiquidity,

      initialPrice: directMarket.initialPrice,

      poolId: directMarket.poolId,

      liquidityLockDuration: directMarket.liquidityLockDuration,
    };
  }

  if (mode === LAUNCH_MODES.BONDING_CURVE) {
    launchData.bondingCurve = {
      curveType: bondingCurve.curveType,

      virtualTokenReserve: bondingCurve.virtualTokenReserve,

      virtualQuoteReserve: bondingCurve.virtualQuoteReserve,

      realTokenReserve: bondingCurve.realTokenReserve || "0",

      realQuoteReserve: bondingCurve.realQuoteReserve || "0",

      initialPrice: bondingCurve.initialPrice,

      graduationMarketCap: bondingCurve.graduationMarketCap,

      graduationLiquidity: bondingCurve.graduationLiquidity,

      graduationPoolId: bondingCurve.graduationPoolId,

      tradingFeeBps: bondingCurve.tradingFeeBps ?? 100,
    };
  }

  const launch = await Launch.create(launchData);

  return launch;
};

const getLaunches = async ({ mode, status, creator, page = 1, limit = 20 }) => {
  if (!Number.isSafeInteger(page) || page < 1) {
    throw new AppError("Page must be a positive integer", 400);
  }

  if (!Number.isSafeInteger(limit) || limit < 1 || limit > 100) {
    throw new AppError("Limit must be an integer between 1 and 100", 400);
  }

  if (mode !== undefined && !Object.values(LAUNCH_MODES).includes(mode)) {
    throw new AppError("Invalid launch mode", 400);
  }

  if (status !== undefined && !Object.values(LAUNCH_STATUS).includes(status)) {
    throw new AppError("Invalid launch status", 400);
  }

  if (creator !== undefined && !isNearAccountId(creator)) {
    throw new AppError("Creator must be a valid NEAR account ID", 400);
  }

  const filter = {};

  if (mode) {
    filter.mode = mode;
  }

  if (status) {
    filter.status = status;
  }

  if (creator) {
    filter.creator = creator;
  }

  const skip = (page - 1) * limit;

  const [launches, total] = await Promise.all([
    Launch.find(filter)
      .populate("tokenId")
      .sort({ createdAt: -1 })
      .skip(skip)
      .limit(limit),

    Launch.countDocuments(filter),
  ]);

  return {
    launches,
    pagination: {
      page,
      limit,
      total,
      pages: Math.ceil(total / limit),
    },
  };
};

const getLaunchById = async (launchId) => {
  const launch = await Launch.findOne({
    launchId,
  }).populate("tokenId");

  if (!launch) {
    throw new AppError("Launch not found", 404);
  }

  return launch;
};

export { createLaunch, getLaunches, getLaunchById };
