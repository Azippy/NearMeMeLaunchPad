import express from "express";

import { getNearStatus } from "../controllers/nearController.js";

import launchRoutes from "./launchRoute.js";
import tokenRoutes from "./tokenRoutes.js";
import creatorRoutes from "./creatorRoutes.js";
import rewardRoutes from "./rewardRoutes.js";

const router = express.Router();

router.get("/health", (req, res) => {
  res.status(200).json({
    success: true,
    application: "NearMeMePad",
    message: "NearMeMePad API is running",
    timestamp: new Date().toISOString(),
  });
});

router.get("/near/status", getNearStatus);

router.use("/launches", launchRoutes);
router.use("/tokens", tokenRoutes);
router.use("/creators", creatorRoutes);
router.use("/rewards", rewardRoutes);

export default router;
