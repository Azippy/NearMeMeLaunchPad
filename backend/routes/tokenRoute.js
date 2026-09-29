import express from "express";
import { createTokenController } from "../controllers/tokenController.js";

const router = express.Router();

router.post("/", createTokenController);

export default router;
