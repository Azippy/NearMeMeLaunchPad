import mongoose from "mongoose";

const rewardClaimSchema = new mongoose.Schema(
  {
    creator: {
      type: String,
      required: true,
      index: true,
    },

    tokenId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Token",
      required: true,
      index: true,
    },

    tokenContractId: {
      type: String,
      required: true,
    },

    amount: {
      type: String,
      required: true,
    },

    quoteTokenId: {
      type: String,
      required: true,
    },

    txHash: {
      type: String,
      unique: true,
      sparse: true,
      index: true,
    },

    status: {
      type: String,
      enum: ["PREPARED", "PENDING", "SUCCESS", "FAILED"],
      default: "PREPARED",
      index: true,
    },

    failureReason: {
      type: String,
    },

    claimedAt: {
      type: Date,
    },
  },
  {
    timestamps: true,
  },
);

const RewardClaim = mongoose.model("RewardClaim", rewardClaimSchema);

export default RewardClaim;
