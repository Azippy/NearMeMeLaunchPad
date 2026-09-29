import asyncHandler from "../utils/asyncHandler.js";
import { createToken } from "../services/token/tokenService.js";

export const createTokenController = asyncHandler(async (req, res) => {
  const token = await createToken(req.body);

  res.status(201).json({
    success: true,
    message: "Token created successfully",
    token,
  });
});
