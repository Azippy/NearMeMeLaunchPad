import mongoose from "mongoose";

const tokenSchema = new mongoose.Schema(
  {
    contractId: {
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

    name: {
      type: String,
      required: true,
      trim: true,
      maxlength: 100,
    },

    symbol: {
      type: String,
      required: true,
      uppercase: true,
      trim: true,
      maxlength: 20,
    },

    decimals: {
      type: Number,
      default: 24,
      immutable: true,
    },

    // Every NearMeMePad token has exactly 1 billion tokens
    totalSupply: {
      type: String,
      required: true,
      immutable: true,
      default: "1000000000",
    },

    metadata: {
      image: {
        type: String,
        required: true,
      },

      description: {
        type: String,
        required: true,
        maxlength: 2000,
      },

      website: {
        type: String,
        trim: true,
      },

      twitter: {
        type: String,
        trim: true,
      },

      telegram: {
        type: String,
        trim: true,
      },

      discord: {
        type: String,
        trim: true,
      },
    },

    feeDistribution: {
      platformPercent: {
        type: Number,
        default: 30,
        immutable: true,
      },

      creatorAllocationPercent: {
        type: Number,
        default: 70,
        immutable: true,
      },

      creatorPercent: {
        type: Number,
        required: true,
        min: 0,
        max: 100,
      },

      holdersPercent: {
        type: Number,
        required: true,
        min: 0,
        max: 100,
      },

      liquidityPercent: {
        type: Number,
        required: true,
        min: 0,
        max: 100,
      },

      burnPercent: {
        type: Number,
        required: true,
        min: 0,
        max: 100,
      },
    },
    tradingTax: {
      buyTaxBps: {
        type: Number,
        required: true,
        min: 0,
        max: 1000,
      },

      sellTaxBps: {
        type: Number,
        required: true,
        min: 0,
        max: 1000,
      },
    },

    verified: {
      type: Boolean,
      default: false,
      index: true,
    },

    createdAtBlock: {
      type: Number,
    },

    creationTxHash: {
      type: String,
    },
  },
  {
    timestamps: true,
  },
);

const Token = mongoose.model("Token", tokenSchema);

export default Token;
