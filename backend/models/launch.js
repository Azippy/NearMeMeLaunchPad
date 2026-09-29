import mongoose from "mongoose";

import {
  LAUNCH_MODES,
  LAUNCH_STATUS,
  BONDING_CURVE_TYPES,
} from "../config/constants.js";

const launchSchema = new mongoose.Schema(
  {
    launchId: {
      type: String,
      required: true,
      unique: true,
      index: true,
    },

    creator: {
      type: String,
      required: true,
      index: true,
    },

    tokenId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Token",
      required: true,
    },

    tokenContractId: {
      type: String,
      required: true,
      index: true,
    },

    mode: {
      type: String,
      enum: Object.values(LAUNCH_MODES),
      required: true,
      index: true,
    },

    status: {
      type: String,
      enum: Object.values(LAUNCH_STATUS),
      default: LAUNCH_STATUS.PENDING,
      index: true,
    },

    description: {
      type: String,
      trim: true,
      maxlength: 2000,
    },

    // --------------------------------
    // DIRECT MARKET CONFIGURATION
    // --------------------------------

    directMarket: {
      quoteTokenId: {
        type: String,
      },

      baseLiquidity: {
        type: String,
      },

      quoteLiquidity: {
        type: String,
      },

      initialPrice: {
        type: String,
      },

      poolId: {
        type: String,
      },

      liquidityLockDuration: {
        type: Number,
      },

      liquidityLockUntil: {
        type: Date,
      },
    },

    // --------------------------------
    // BONDING CURVE CONFIGURATION
    // --------------------------------

    bondingCurve: {
      curveType: {
        type: String,
        enum: Object.values(BONDING_CURVE_TYPES),
      },

      virtualTokenReserve: {
        type: String,
      },

      virtualQuoteReserve: {
        type: String,
      },

      realTokenReserve: {
        type: String,
      },

      realQuoteReserve: {
        type: String,
      },

      initialPrice: {
        type: String,
      },

      graduationMarketCap: {
        type: String,
      },

      graduationLiquidity: {
        type: String,
      },

      graduationPoolId: {
        type: String,
      },

      tradingFeeBps: {
        type: Number,
      },
    },

    // --------------------------------
    // FEES
    // --------------------------------

    fees: {
      creationFee: {
        type: String,
      },

      tradingFeeBps: {
        type: Number,
      },

      graduationFeeBps: {
        type: Number,
      },
    },

    // --------------------------------
    // BLOCKCHAIN DATA
    // --------------------------------

    blockchain: {
      network: {
        type: String,
        required: true,
      },

      factoryContractId: {
        type: String,
      },

      creationTxHash: {
        type: String,
      },

      creationBlockHeight: {
        type: Number,
      },

      graduationTxHash: {
        type: String,
      },

      graduationBlockHeight: {
        type: Number,
      },
      quoteAsset: {
        type: String,
        required: true,
        index: true,
      },

      quoteAssetType: {
        type: String,
        enum: [
          "NATIVE_NEAR",
          "NEAR_TOKEN",
          "STABLECOIN",
          "RWA",
          "STOCK",
          "ETF",
          "OTHER",
        ],
        required: true,
      },

      quoteAssetDecimals: {
        type: Number,
        required: true,
      },
    },
  },
  {
    timestamps: true,
  },
);

const Launch = mongoose.model("Launch", launchSchema);

export default Launch;
