import mongoose from "mongoose";

const candleSchema = new mongoose.Schema(
  {
    launchId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Launch",
      required: true,
      index: true,
    },

    interval: {
      type: String,
      enum: ["1m", "5m", "15m", "1h", "4h", "1d"],
      required: true,
    },

    startTime: {
      type: Date,
      required: true,
    },

    open: {
      type: String,
      required: true,
    },

    high: {
      type: String,
      required: true,
    },

    low: {
      type: String,
      required: true,
    },

    close: {
      type: String,
      required: true,
    },

    volume: {
      type: String,
      default: "0",
    },

    tradeCount: {
      type: Number,
      default: 0,
    },
  },
  {
    timestamps: true,
  },
);

candleSchema.index(
  {
    launchId: 1,
    interval: 1,
    startTime: 1,
  },
  {
    unique: true,
  },
);

const Candle = mongoose.model("Candle", candleSchema);

export default Candle;
