import mongoose from "mongoose";

const pairAssetSchema = new mongoose.Schema(
  {
    symbol: {
      type: String,
      required: true,
      uppercase: true,
      trim: true,
      index: true,
    },

    name: {
      type: String,
      required: true,
      trim: true,
    },

    contractId: {
      type: String,
      required: true,
      unique: true,
      index: true,
    },

    assetType: {
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

    decimals: {
      type: Number,
      required: true,
    },

    logo: {
      type: String,
    },

    enabled: {
      type: Boolean,
      default: true,
      index: true,
    },

    creatorSelectable: {
      type: Boolean,
      default: true,
    },

    isDefault: {
      type: Boolean,
      default: false,
    },

    verified: {
      type: Boolean,
      default: false,
    },

    source: {
      type: String,
      enum: [
        "NEAR_NATIVE",
        "NEAR_ECOSYSTEM",
        "RHEA",
        "NEAR_INTENTS",
        "ONDO",
        "OTHER",
      ],
      required: true,
    },
  },
  {
    timestamps: true,
  },
);

const PairAsset = mongoose.model("PairAsset", pairAssetSchema);

export default PairAsset;
