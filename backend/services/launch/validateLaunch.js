import { LAUNCH_MODES, BONDING_CURVE_TYPES } from "../../config/constants.js";

const isPositiveIntegerString = (value) => {
  if (typeof value !== "string") {
    return false;
  }

  if (!/^\d+$/.test(value)) {
    return false;
  }

  return BigInt(value) > 0n;
};

const isNonNegativeIntegerString = (value) => {
  if (typeof value !== "string") {
    return false;
  }

  return /^\d+$/.test(value);
};

export const validateLaunch = (data) => {
  const errors = [];

  const { creator, tokenContractId, tokenId, mode } = data;

  // -------------------------
  // BASIC VALIDATION
  // -------------------------

  if (!creator) {
    errors.push("Creator account ID is required");
  }

  if (!tokenContractId) {
    errors.push("Token contract ID is required");
  }

  if (!tokenId) {
    errors.push("Token ID is required");
  }

  if (!mode) {
    errors.push("Launch mode is required");
  }

  if (mode && !Object.values(LAUNCH_MODES).includes(mode)) {
    errors.push(
      `Invalid launch mode. Allowed modes: ${Object.values(LAUNCH_MODES).join(
        ", ",
      )}`,
    );
  }

  // -------------------------
  // DIRECT MARKET
  // -------------------------

  if (mode === LAUNCH_MODES.DIRECT_MARKET) {
    const { directMarket, bondingCurve } = data;

    if (!directMarket) {
      errors.push("directMarket configuration is required");
    }

    if (bondingCurve) {
      errors.push(
        "bondingCurve configuration must not be supplied for DIRECT_MARKET",
      );
    }

    if (directMarket) {
      if (!directMarket.quoteTokenId) {
        errors.push("directMarket.quoteTokenId is required");
      }

      if (!isPositiveIntegerString(directMarket.baseLiquidity)) {
        errors.push(
          "directMarket.baseLiquidity must be a positive integer string",
        );
      }

      if (!isPositiveIntegerString(directMarket.quoteLiquidity)) {
        errors.push(
          "directMarket.quoteLiquidity must be a positive integer string",
        );
      }

      if (!isPositiveIntegerString(directMarket.initialPrice)) {
        errors.push(
          "directMarket.initialPrice must be a positive integer string",
        );
      }

      if (directMarket.liquidityLockDuration !== undefined) {
        if (
          !Number.isInteger(directMarket.liquidityLockDuration) ||
          directMarket.liquidityLockDuration < 0
        ) {
          errors.push(
            "directMarket.liquidityLockDuration must be a non-negative integer",
          );
        }
      }
    }
  }

  // -------------------------
  // BONDING CURVE
  // -------------------------

  if (mode === LAUNCH_MODES.BONDING_CURVE) {
    const { bondingCurve, directMarket } = data;

    if (!bondingCurve) {
      errors.push("bondingCurve configuration is required");
    }

    if (directMarket) {
      errors.push(
        "directMarket configuration must not be supplied for BONDING_CURVE",
      );
    }

    if (bondingCurve) {
      if (
        !Object.values(BONDING_CURVE_TYPES).includes(bondingCurve.curveType)
      ) {
        errors.push(
          `Invalid bonding curve type. Allowed types: ${Object.values(
            BONDING_CURVE_TYPES,
          ).join(", ")}`,
        );
      }

      if (!isPositiveIntegerString(bondingCurve.virtualTokenReserve)) {
        errors.push(
          "bondingCurve.virtualTokenReserve must be a positive integer string",
        );
      }

      if (!isPositiveIntegerString(bondingCurve.virtualQuoteReserve)) {
        errors.push(
          "bondingCurve.virtualQuoteReserve must be a positive integer string",
        );
      }

      if (!isNonNegativeIntegerString(bondingCurve.realTokenReserve)) {
        errors.push(
          "bondingCurve.realTokenReserve must be a non-negative integer string",
        );
      }

      if (!isNonNegativeIntegerString(bondingCurve.realQuoteReserve)) {
        errors.push(
          "bondingCurve.realQuoteReserve must be a non-negative integer string",
        );
      }

      if (!isPositiveIntegerString(bondingCurve.initialPrice)) {
        errors.push(
          "bondingCurve.initialPrice must be a positive integer string",
        );
      }

      if (!isPositiveIntegerString(bondingCurve.graduationMarketCap)) {
        errors.push(
          "bondingCurve.graduationMarketCap must be a positive integer string",
        );
      }

      if (!isPositiveIntegerString(bondingCurve.graduationLiquidity)) {
        errors.push(
          "bondingCurve.graduationLiquidity must be a positive integer string",
        );
      }

      if (bondingCurve.tradingFeeBps !== undefined) {
        if (
          !Number.isInteger(bondingCurve.tradingFeeBps) ||
          bondingCurve.tradingFeeBps < 0 ||
          bondingCurve.tradingFeeBps > 1000
        ) {
          errors.push("bondingCurve.tradingFeeBps must be between 0 and 1000");
        }
      }
    }
  }

  return {
    valid: errors.length === 0,
    errors,
  };
};
