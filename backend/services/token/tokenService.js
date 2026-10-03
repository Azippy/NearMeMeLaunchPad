import crypto from "crypto";

import Token from "../../models/token.js";
import AppError from "../../utils/AppError.js";
import validateTokenCreation from "./validateToken.js";

const TOTAL_SUPPLY = "1000000000";
const DECIMALS = 24;

export const createToken = async (data) => {
  const validatedData = validateTokenCreation(data);

  // Check whether the creator already has a token
  // with the same symbol.
  const existingToken = await Token.findOne({
    symbol: validatedData.symbol,
  });

  if (existingToken) {
    throw new AppError(
      `Token symbol ${validatedData.symbol} is already in use`,
      409,
    );
  }

  /*
   * Temporary contract ID.
   *
   * The real contract account will be created
   * later through the NEAR token factory.
   */
  const temporaryContractId = `pending-${crypto.randomUUID()}`;

  const token = await Token.create({
    contractId: temporaryContractId,

    creator: validatedData.creator,

    name: validatedData.name,

    symbol: validatedData.symbol,

    decimals: DECIMALS,

    totalSupply: TOTAL_SUPPLY,

    metadata: {
      image: validatedData.logo,
      description: validatedData.description,
      website: validatedData.website,
      twitter: validatedData.twitter,
      telegram: validatedData.telegram,
      discord: validatedData.discord,
    },

    feeDistribution: {
      platformPercent: 30,

      creatorAllocationPercent: 70,

      creatorPercent: validatedData.creatorPercent,

      holdersPercent: validatedData.holdersPercent,

      liquidityPercent: validatedData.liquidityPercent,

      burnPercent: validatedData.burnPercent,
    },

    tradingTax: {
      buyTaxBps: validatedData.buyTaxBps,
      sellTaxBps: validatedData.sellTaxBps,
    },
  });

  return token;
};
