import express from "express";

import {
  getCreatorProfileController,
  getCreatorTokensController,
  getCreatorRewardsController,
} from "../controllers/creatorController.js";

const router = express.Router();

router.get("/:wallet", getCreatorProfileController);

router.get("/:wallet/tokens", getCreatorTokensController);

router.get("/:wallet/rewards", getCreatorRewardsController);

export default router;
