import asyncHandler from "../utils/asyncHandler.js";
import { getBlock } from "../near/rpcClient.js";
import env from "../config/env.js";

export const getNearStatus = asyncHandler(async (req, res) => {
  const block = await getBlock();

  res.status(200).json({
    success: true,

    network: env.near.network,

    blockchain: {
      status: "connected",
      blockHeight: block.header.height,
      blockHash: block.header.hash,
      timestamp: block.header.timestamp,
    },
  });
});
