import mongoose from "mongoose";

const tradeSchema = new mongoose.Schema(
  {
    txHash: {
      type: String,
      required: true,
      index: true,
    },

    blockHeight: {
      type: Number,
      required: true,
      index: true,
    },

    launchId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Launch",
      required: true,
      index: true,
    },

    poolId: {
      type: String,
      index: true,
    },

    trader: {
      type: String,
      required: true,
      index: true,
    },

    side: {
      type: String,
      enum: ["BUY", "SELL"],
      required: true,
    },

    tokenAmount: {
      type: String,
      required: true,
    },

    quoteAmount: {
      type: String,
      required: true,
    },

    price: {
      type: String,
      required: true,
    },

    fee: {
      type: String,
      default: "0",
    },

    timestamp: {
      type: Date,
      required: true,
      index: true,
    },
  },
  {
    timestamps: true,
  },
);

tradeSchema.index({
  launchId: 1,
  timestamp: -1,
});

const Trade = mongoose.model("Trade", tradeSchema);

export default Trade;
