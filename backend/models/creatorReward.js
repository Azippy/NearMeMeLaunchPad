import mongoose from "mongoose";

const creatorRewardSchema = new mongoose.Schema(
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
      index: true,
    },

    launchId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Launch",
      index: true,
    },

    quoteTokenId: {
      type: String,
      required: true,
    },

    // Amount accumulated and waiting to be claimed
    pendingAmount: {
      type: String,
      default: "0",
    },

    // Total amount ever earned
    totalEarned: {
      type: String,
      default: "0",
    },

    // Total amount already claimed
    totalClaimed: {
      type: String,
      default: "0",
    },

    lastClaimTxHash: {
      type: String,
    },

    lastClaimedAt: {
      type: Date,
    },
  },
  {
    timestamps: true,
  },
);

creatorRewardSchema.index(
  {
    creator: 1,
    tokenId: 1,
  },
  {
    unique: true,
  },
);

const CreatorReward = mongoose.model("CreatorReward", creatorRewardSchema);

export default CreatorReward;
