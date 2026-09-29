import dotenv from "dotenv";

dotenv.config();

const env = {
  nodeEnv: process.env.NODE_ENV || "development",

  port: Number(process.env.PORT) || 5000,

  mongoUri: process.env.MONGO_URI,

  frontendUrl: process.env.FRONTEND_URL || "http://localhost:5173",

  near: {
    network: process.env.NEAR_NETWORK || "testnet",

    rpcUrl: process.env.NEAR_RPC_URL || "https://rpc.testnet.near.org",

    factoryContractId: process.env.NEAR_FACTORY_CONTRACT_ID,

    lockerContractId: process.env.NEAR_LOCKER_CONTRACT_ID,

    treasuryContractId: process.env.NEAR_TREASURY_CONTRACT_ID,
  },
};

export default env;
