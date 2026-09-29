import mongoose from "mongoose";

const protocolConfigSchema = new mongoose.Schema(
  {
    network: {
      type: String,
      enum: ["testnet", "mainnet"],
      required: true,
      unique: true,
    },

    launchFeeYocto: {
      type: String,
      required: true,
    },

    launchFeeNear: {
      type: String,
      required: true,
    },

    tradingTaxPlatformShareBps: {
      type: Number,
      default: 4000,
      immutable: true,
    },

    tradingTaxCreatorShareBps: {
      type: Number,
      default: 6000,
      immutable: true,
    },

    launchFeePlatformSharePercent: {
      type: Number,
      default: 30,
    },

    launchFeeCreatorSharePercent: {
      type: Number,
      default: 70,
    },

    factoryContractId: {
      type: String,
      required: true,
    },

    treasuryContractId: {
      type: String,
      required: true,
    },

    active: {
      type: Boolean,
      default: true,
    },
  },
  {
    timestamps: true,
  },
);

const ProtocolConfig = mongoose.model("ProtocolConfig", protocolConfigSchema);

export default ProtocolConfig;
