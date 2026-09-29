import asyncHandler from "../utils/asyncHandler.js";

import {
  getCreatorProfile,
  getCreatorTokens,
  getCreatorRewards,
} from "../services/creator/creatorService.js";

export const getCreatorProfileController = asyncHandler(async (req, res) => {
  const profile = await getCreatorProfile(req.params.wallet);

  res.status(200).json({
    success: true,
    profile,
  });
});

export const getCreatorTokensController = asyncHandler(async (req, res) => {
  const tokens = await getCreatorTokens(req.params.wallet);

  res.status(200).json({
    success: true,
    tokens,
  });
});

export const getCreatorRewardsController = asyncHandler(async (req, res) => {
  const rewards = await getCreatorRewards(req.params.wallet);

  res.status(200).json({
    success: true,
    rewards,
  });
});
