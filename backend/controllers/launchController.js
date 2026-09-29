import asyncHandler from "../utils/asyncHandler.js";

import {
  createLaunch,
  getLaunches,
  getLaunchById,
} from "../services/launch/launchService.js";

export const createLaunchController = asyncHandler(async (req, res) => {
  const launch = await createLaunch(req.body);

  res.status(201).json({
    success: true,
    message: "NearMeMePad launch created successfully",
    launch,
  });
});

export const getLaunchesController = asyncHandler(async (req, res) => {
  const { mode, status, creator, page = 1, limit = 20 } = req.query;

  const result = await getLaunches({
    mode,
    status,
    creator,
    page: Number(page),
    limit: Number(limit),
  });

  res.status(200).json({
    success: true,
    ...result,
  });
});

export const getLaunchController = asyncHandler(async (req, res) => {
  const launch = await getLaunchById(req.params.launchId);

  res.status(200).json({
    success: true,
    launch,
  });
});
