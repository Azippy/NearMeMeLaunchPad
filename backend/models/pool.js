import mongoose from "mongoose";

const poolSchema = new mongoose.Schema(
  {
    poolId: {
      type: String,
      required: true,
      unique: true,
      index: true
    },

    launchId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Launch",
      required: true,
      index: true
    },

    token0: {
      type: String,
      required: true
    },

    token1: {
      type: String,
      required: true
    },

    dex: {
      type: String,
      required: true
    },

    feeBps: {
      type: Number,
      default: 30
    },

    reserve0: {
      type: String,
      default: "0"
    },

    reserve1: {
      type: String,
      default: "0"
    },

    liquidity: {
      type: String,
      default: "0"
    },

    active: {
      type: Boolean,
      default: true,
      index: true
    },

    createdAtBlock: {
      type: Number
    }
  },
  {
    timestamps: true
  }
);

const Pool = mongoose.model("Pool", poolSchema);

export default Pool;