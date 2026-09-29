import { JsonRpcProvider } from "near-api-js";

import env from "../config/env.js";

const provider = new JsonRpcProvider({
  url: env.near.rpcUrl,
});

export const queryNear = async (request) => {
  return provider.query(request);
};

export const getBlock = async (blockId = "final") => {
  return provider.viewBlock({
    finality: blockId,
  });
};

export const getBlockByHeight = async (height) => {
  return provider.viewBlock({
    blockId: height,
  });
};

export default provider;
