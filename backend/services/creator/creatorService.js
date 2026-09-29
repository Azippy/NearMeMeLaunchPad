import Token from "../../models/Token.js";
import CreatorReward from "../../models/CreatorReward.js";
import AppError from "../../utils/AppError.js";

export const getCreatorTokens = async (wallet) => {
  if (!wallet) {
    throw new AppError("Creator wallet is required", 400);
  }

  const tokens = await Token.find({
    creator: wallet,
  })
    .sort({ createdAt: -1 })
    .lean();

  return tokens;
};

export const getCreatorRewards = async (wallet) => {
  if (!wallet) {
    throw new AppError("Creator wallet is required", 400);
  }

  const rewards = await CreatorReward.find({
    creator: wallet,
  })
    .populate("tokenId", "name symbol metadata totalSupply")
    .populate("launchId", "launchId mode status")
    .sort({ updatedAt: -1 })
    .lean();

  return rewards;
};

export const getCreatorProfile = async (wallet) => {
  if (!wallet) {
    throw new AppError("Creator wallet is required", 400);
  }

  const [tokens, rewards] = await Promise.all([
    getCreatorTokens(wallet),
    getCreatorRewards(wallet),
  ]);

  return {
    wallet,
    tokenCount: tokens.length,
    tokens,
    rewards,
  };
};
