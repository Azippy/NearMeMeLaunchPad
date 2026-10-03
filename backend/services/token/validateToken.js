import AppError from "../../utils/AppError.js";

const isNearAccountId = (value) => {
  if (typeof value !== "string" || value.length < 2 || value.length > 64) {
    return false;
  }

  return /^(?:[a-z0-9]+(?:[._-][a-z0-9]+)*)$/.test(value);
};

const isValidUrl = (value) => {
  if (!value) return true;

  try {
    const url = new URL(value);
    return url.protocol === "https:" || url.protocol === "http:";
  } catch {
    return false;
  }
};

const validatePercentage = (value, fieldName) => {
  const roundedBps = Math.round(value * 100);
  if (
    typeof value !== "number" ||
    !Number.isFinite(value) ||
    value < 0 ||
    value > 10 ||
    Math.abs(value * 100 - roundedBps) > 1e-9
  ) {
    throw new AppError(`${fieldName} must be between 0% and 10%`, 400);
  }
};

const validateTokenCreation = (data) => {
  if (!data || typeof data !== "object" || Array.isArray(data)) {
    throw new AppError("Token request body must be an object", 400);
  }

  const {
    creator,
    name,
    symbol,
    logo,
    description,

    website,
    twitter,
    telegram,
    discord,

    creatorPercent,
    holdersPercent,
    liquidityPercent,
    burnPercent,

    buyTaxPercent,
    sellTaxPercent,
  } = data;

  // -------------------------
  // REQUIRED TOKEN INFORMATION
  // -------------------------

  if (!isNearAccountId(creator)) {
    throw new AppError("Creator must be a valid NEAR account ID", 400);
  }

  if (typeof name !== "string" || !name.trim()) {
    throw new AppError("Token name is required", 400);
  }

  if (typeof symbol !== "string" || !symbol.trim()) {
    throw new AppError("Token symbol is required", 400);
  }

  if (typeof logo !== "string" || !isValidUrl(logo.trim())) {
    throw new AppError("Token logo must be a valid HTTP(S) URL", 400);
  }

  if (typeof description !== "string" || !description.trim()) {
    throw new AppError("Token description is required", 400);
  }

  if (name.trim().length > 50) {
    throw new AppError("Token name cannot exceed 50 characters", 400);
  }

  if (symbol.trim().length > 20) {
    throw new AppError("Token symbol cannot exceed 20 characters", 400);
  }

  if (!/^[a-zA-Z0-9]+$/.test(symbol.trim())) {
    throw new AppError(
      "Token symbol can only contain letters and numbers",
      400,
    );
  }

  if (description.trim().length > 2000) {
    throw new AppError("Token description cannot exceed 2000 characters", 400);
  }

  // -------------------------
  // OPTIONAL SOCIAL LINKS
  // -------------------------

  const optionalLinks = {
    website,
    twitter,
    telegram,
    discord,
  };

  for (const [field, value] of Object.entries(optionalLinks)) {
    if (value !== undefined && value !== null && typeof value !== "string") {
      throw new AppError(`${field} must be a URL string`, 400);
    }

    if (value && !isValidUrl(value.trim())) {
      throw new AppError(`${field} must be a valid URL`, 400);
    }
  }

  // -------------------------
  // CREATOR 70% DISTRIBUTION
  // -------------------------

  const creatorSidePercentages = [
    creatorPercent,
    holdersPercent,
    liquidityPercent,
    burnPercent,
  ];

  for (const percentage of creatorSidePercentages) {
    if (
      typeof percentage !== "number" ||
      !Number.isFinite(percentage) ||
      percentage < 0 ||
      percentage > 100
    ) {
      throw new AppError(
        "Creator distribution percentages must be between 0 and 100",
        400,
      );
    }
  }

  const distributionTotal =
    creatorPercent + holdersPercent + liquidityPercent + burnPercent;

  if (distributionTotal !== 100) {
    throw new AppError("Creator distribution must total exactly 100%", 400);
  }

  // -------------------------
  // BUY / SELL TAX
  // -------------------------

  validatePercentage(buyTaxPercent, "Buy tax");

  validatePercentage(sellTaxPercent, "Sell tax");

  return {
    creator,
    name: name.trim(),
    symbol: symbol.trim().toUpperCase(),
    logo: logo.trim(),
    description: description.trim(),

    website: website?.trim() || undefined,
    twitter: twitter?.trim() || undefined,
    telegram: telegram?.trim() || undefined,
    discord: discord?.trim() || undefined,

    creatorPercent,
    holdersPercent,
    liquidityPercent,
    burnPercent,

    buyTaxBps: Math.round(buyTaxPercent * 100),

    sellTaxBps: Math.round(sellTaxPercent * 100),
  };
};

export default validateTokenCreation;
