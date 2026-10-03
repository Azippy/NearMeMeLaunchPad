import CreatorReward from "../../models/creatorReward.js";
import AppError from "../../utils/AppError.js";

export const prepareRewardClaim = async ({ creator, tokenId }) => {
  if (!creator) {
    throw new AppError("Creator wallet is required", 400);
  }

  if (!tokenId) {
    throw new AppError("Token ID is required", 400);
  }

  const reward = await CreatorReward.findOne({
    creator,
    tokenId,
  }).populate("tokenId", "name symbol contractId");

  if (!reward) {
    throw new AppError("No reward account found for this token", 404);
  }

  if (BigInt(reward.pendingAmount) <= 0n) {
    throw new AppError("No rewards available to claim", 400);
  }

  /*
   * IMPORTANT:
   *
   * We do NOT transfer funds here.
   *
   * The NEAR smart contract will eventually
   * perform the actual claim.
   */

  return {
    creator,
    tokenId: reward.tokenId._id,
    tokenContractId: reward.tokenContractId,

    quoteTokenId: reward.quoteTokenId,

    amount: reward.pendingAmount,

    contractMethod: "claim_rewards",
    contractArgs: {
      token_contract_id: reward.tokenContractId,
      quote_asset_id: reward.quoteTokenId,
    },
  };
};
