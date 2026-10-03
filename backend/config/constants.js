// backend/src/config/constants.js

// ============================================================
// PLATFORM
// ============================================================

export const PLATFORM_NAME = "NearMeMePad";

export const DEFAULT_LAUNCH_FEE_NEAR = 0.2;

export const TOKEN_TOTAL_SUPPLY = 1_000_000_000;

export const TOKEN_DECIMALS = 24;

// ============================================================
// TRADING FEES
// ============================================================

// 30% of trading fee → NearMeMePad
// 70% of trading fee → Creator
export const TRADING_FEE_PLATFORM_BPS = 3_000;
export const TRADING_FEE_CREATOR_BPS = 7_000;

// ============================================================
// BUY / SELL TAX
// ============================================================

// Creator chooses the tax percentage for each token.
//
// Tax distribution:
// 40% → NearMeMePad
// 60% → Creator
//
// Example:
// 5% trading tax
//     2% → Platform
//     3% → Creator

export const TRADING_TAX_PLATFORM_BPS = 4_000;
export const TRADING_TAX_CREATOR_BPS = 6_000;

// Maximum creator-configurable buy/sell tax.
// 1,000 BPS = 10%
export const MAX_BUY_TAX_BPS = 1_000;
export const MAX_SELL_TAX_BPS = 1_000;

// ============================================================
// TRADING TAX EXPORT
// ============================================================

export const TRADING_TAX = {
  platformBps: TRADING_TAX_PLATFORM_BPS,
  creatorBps: TRADING_TAX_CREATOR_BPS,

  platformPercentage: TRADING_TAX_PLATFORM_BPS / 100,

  creatorPercentage: TRADING_TAX_CREATOR_BPS / 100,

  maxBuyTaxBps: MAX_BUY_TAX_BPS,

  maxSellTaxBps: MAX_SELL_TAX_BPS,
};

// ============================================================
// TRADING FEE
// ============================================================

export const TRADING_FEE = {
  platformBps: TRADING_FEE_PLATFORM_BPS,
  creatorBps: TRADING_FEE_CREATOR_BPS,

  platformPercentage: TRADING_FEE_PLATFORM_BPS / 100,

  creatorPercentage: TRADING_FEE_CREATOR_BPS / 100,
};

// ============================================================
// LAUNCH MODES
// ============================================================

export const LAUNCH_MODE = {
  DIRECT_MARKET: "direct_market",
  BONDING_CURVE: "bonding_curve",
};

// ============================================================
// LAUNCH STATUS
// ============================================================

export const LAUNCH_STATUS = {
  DRAFT: "DRAFT",
  PENDING: "PENDING",
  ACTIVE: "ACTIVE",
  DEPLOYING_TOKEN: "deploying_token",

  PREPARING_LIQUIDITY: "preparing_liquidity",

  GRADUATION_THRESHOLD_REACHED: "GRADUATION_THRESHOLD_REACHED",

  GRADUATING: "GRADUATING",

  GRADUATED: "GRADUATED",

  COMPLETED: "COMPLETED",

  CANCELLED: "CANCELLED",

  FAILED: "FAILED",
};

// ============================================================
// TRADE TYPES
// ============================================================

export const TRADE_TYPE = {
  BUY: "buy",
  SELL: "sell",
};

export const TRANSACTION_TYPES = {
  TOKEN_CREATED: "token_created",
  LAUNCH_CREATED: "launch_created",
  TOKEN_DEPLOYED: "token_deployed",
  BUY: TRADE_TYPE.BUY,
  SELL: TRADE_TYPE.SELL,
  REWARD_CLAIMED: "reward_claimed",
};

// ============================================================
// ASSET TYPES
// ============================================================

export const ASSET_TYPE = {
  NATIVE: "native",
  FUNGIBLE_TOKEN: "fungible_token",
  STOCK: "stock",
};

// ============================================================
// DEFAULT PAIR
// ============================================================

export const NATIVE_NEAR = {
  assetId: "native.near",
  symbol: "NEAR",
  name: "NEAR",
  decimals: 24,
  assetType: ASSET_TYPE.NATIVE,
  isNativeNear: true,
};

// ============================================================
// BONDING CURVE
// ============================================================

export const BONDING_CURVE = {
  DEFAULT_VIRTUAL_TOKEN_RESERVE: TOKEN_TOTAL_SUPPLY,

  DEFAULT_VIRTUAL_QUOTE_RESERVE: 1_000,

  DEFAULT_GRADUATION_MARKET_CAP: 50_000,

  DEFAULT_GRADUATION_LIQUIDITY: 20_000,
};

// ============================================================
// CANDLE INTERVALS
// ============================================================

export const CANDLE_INTERVAL = {
  FIVE_MINUTES: 300,
  ONE_HOUR: 3600,
  SIX_HOURS: 21600,
  ONE_DAY: 86400,
};

// ============================================================
// TOKEN VALIDATION
// ============================================================

export const TOKEN_LIMITS = {
  NAME_MIN_LENGTH: 1,
  NAME_MAX_LENGTH: 50,

  SYMBOL_MIN_LENGTH: 1,
  SYMBOL_MAX_LENGTH: 20,

  DESCRIPTION_MIN_LENGTH: 1,
  DESCRIPTION_MAX_LENGTH: 2_000,
};

// ============================================================
// SOCIAL MEDIA
// ============================================================

export const SOCIAL_MEDIA = {
  OPTIONAL: true,

  MAX_URL_LENGTH: 500,
};

// ============================================================
// SUPPORTED SOCIAL PLATFORMS
// ============================================================

export const SOCIAL_PLATFORM = {
  WEBSITE: "website",
  TWITTER: "twitter",
  TELEGRAM: "telegram",
  DISCORD: "discord",
  INSTAGRAM: "instagram",
  TIKTOK: "tiktok",
  YOUTUBE: "youtube",
};

// ============================================================
// LOGO / IMAGE
// ============================================================

export const IMAGE_LIMITS = {
  MAX_FILE_SIZE_MB: 5,

  ALLOWED_TYPES: ["image/png", "image/jpeg", "image/webp", "image/gif"],
};

// ============================================================
// REWARDS
// ============================================================

export const REWARD_TYPE = {
  TRADING_FEE: "trading_fee",
  TRADING_TAX: "trading_tax",
  HOLDER_REWARD: "holder_reward",
};

// ============================================================
// CREATOR REWARD DESTINATIONS
// ============================================================

export const CREATOR_REWARD_DESTINATION = {
  CREATOR: "creator",
  HOLDERS: "holders",
  LIQUIDITY: "liquidity",
  BURN: "burn",
};

// ============================================================
// REWARD ALLOCATION
// ============================================================

export const REWARD_ALLOCATION = {
  CREATOR: "creator",
  HOLDERS: "holders",
  LIQUIDITY: "liquidity",
  BURN: "burn",
};

// ============================================================
// BASIS POINTS
// ============================================================

export const BPS_DENOMINATOR = 10_000;

// ============================================================
// TIME
// ============================================================

export const TIME = {
  SECOND: 1_000,

  MINUTE: 60_000,

  HOUR: 3_600_000,

  DAY: 86_400_000,
};

// ============================================================
// INDEXER
// ============================================================

export const INDEXER = {
  DEFAULT_BATCH_SIZE: 100,

  CONFIRMATION_BLOCKS: 2,

  POLL_INTERVAL_MS: 5_000,
};

// ============================================================
// RATE LIMITING
// ============================================================

export const RATE_LIMIT = {
  WINDOW_MS: 60_000,

  MAX_REQUESTS: 100,
};

// ============================================================
// PAGINATION
// ============================================================

export const PAGINATION = {
  DEFAULT_LIMIT: 20,

  MAX_LIMIT: 100,
};

// ============================================================
// TRANSACTION
// ============================================================

export const TRANSACTION_STATUS = {
  PENDING: "pending",

  SUCCESS: "success",

  FAILED: "failed",
};

// ============================================================
// DEPLOYMENT
// ============================================================

export const DEPLOYMENT_STATUS = {
  PENDING: "pending",

  DEPLOYING: "deploying",

  INITIALIZING: "initializing",

  COMPLETED: "completed",

  FAILED: "failed",
};

// ============================================================
// API
// ============================================================

export const API_PREFIX = "/api";

export const LAUNCH_FEED_PREFIX = "/launch-feed-v5";

export const LAUNCH_MODES = {
  DIRECT_MARKET: "DIRECT_MARKET",
  BONDING_CURVE: "BONDING_CURVE",
};

export const BONDING_CURVE_TYPES = {
  LINEAR: "LINEAR",
  EXPONENTIAL: "EXPONENTIAL",
  CONSTANT_PRODUCT: "CONSTANT_PRODUCT",
};

export const PROTOCOL_FEES = {
  DEFAULT_LAUNCH_FEE_NEAR: "0.2",

  TRADING_FEE_PLATFORM_BPS: 3000,
  TRADING_FEE_CREATOR_BPS: 7000,

  TAX_PLATFORM_BPS: 4000,
  TAX_CREATOR_BPS: 6000,

  MAX_BUY_TAX_BPS,
  MAX_SELL_TAX_BPS,
};
