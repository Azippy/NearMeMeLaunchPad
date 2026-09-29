import express from "express";

import {
  createLaunchController,
  getLaunchesController,
  getLaunchController,
} from "../controllers/launchController.js";

const router = express.Router();

router.post("/", createLaunchController);

router.get("/", getLaunchesController);

router.get("/:launchId", getLaunchController);

export default router;
