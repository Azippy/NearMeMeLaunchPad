import express from "express";

import { prepareRewardClaimController } from "../controllers/rewardController.js";

const router = express.Router();

router.post("/claim/prepare", prepareRewardClaimController);

export default router;
