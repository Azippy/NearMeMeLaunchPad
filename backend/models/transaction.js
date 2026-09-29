import mongoose from "mongoose";

import { TRANSACTION_TYPES } from "../config/constants.js";

const transactionSchema = new mongoose.Schema(
  {
    txHash: {
      type: String,
      required: true,
      unique: true,
      index: true,
    },

    blockHeight: {
      type: Number,
      required: true,
      index: true,
    },

    signer: {
      type: String,
      required: true,
      index: true,
    },

    receiver: {
      type: String,
    },

    type: {
      type: String,
      enum: Object.values(TRANSACTION_TYPES),
      required: true,
      index: true,
    },

    launchId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Launch",
      index: true,
    },

    tokenId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Token",
      index: true,
    },

    success: {
      type: Boolean,
      default: true,
    },

    failureReason: {
      type: String,
    },

    rawActions: {
      type: mongoose.Schema.Types.Mixed,
    },
  },
  {
    timestamps: true,
  },
);

const Transaction = mongoose.model("Transaction", transactionSchema);

export default Transaction;
