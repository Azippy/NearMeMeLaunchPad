import express from "express";
import cors from "cors";
import helmet from "helmet";
import morgan from "morgan";

import env from "./config/env.js";
import routes from "./routes/index.js";
import apiLimiter from "./middleware/ratelimiter.js";
import errorHandler from "./middleware/errorHandler.js";

const app = express();

// Security
app.use(helmet());

// CORS
app.use(
  cors({
    origin: env.frontendUrl,
    credentials: true,
  }),
);

// Request logging
if (env.nodeEnv === "development") {
  app.use(morgan("dev"));
}

// Body parser
app.use(express.json({ limit: "2mb" }));
app.use(express.urlencoded({ extended: true }));

// Rate limiting
app.use("/api", apiLimiter);

// API routes
app.use("/api/v1", routes);

// Unknown route
app.use((req, res) => {
  res.status(404).json({
    success: false,
    message: `Route not found: ${req.method} ${req.originalUrl}`,
  });
});

// Global error handler
app.use(errorHandler);

export default app;
