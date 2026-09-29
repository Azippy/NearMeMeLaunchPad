import mongoose from "mongoose";

const holderSchema = new mongoose.Schema(
  {
    launchId: {
      type: mongoose.Schema.Types.ObjectId,
      ref: "Launch",
      required: true,
      index: true,
    },

    tokenContractId: {
      type: String,
      required: true,
    },

    accountId: {
      type: String,
      required: true,
      index: true,
    },

    balance: {
      type: String,
      required: true,
      default: "0",
    },

    percentage: {
      type: String,
      default: "0",
    },

    firstSeenBlock: {
      type: Number,
    },

    lastSeenBlock: {
      type: Number,
    },
  },
  {
    timestamps: true,
  },
);

holderSchema.index(
  {
    launchId: 1,
    accountId: 1,
  },
  {
    unique: true,
  },
);

const Holder = mongoose.model("Holder", holderSchema);

export default Holder;
