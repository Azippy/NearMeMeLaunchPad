import asyncHandler from "../utils/asyncHandler.js";

import { prepareRewardClaim } from "../services/reward/rewardService.js";

export const prepareRewardClaimController = asyncHandler(async (req, res) => {
  const result = await prepareRewardClaim(req.body);

  res.status(200).json({
    success: true,
    message:
      "Reward claim prepared. Please sign the transaction with your NEAR wallet.",
    claim: result,
  });
});
