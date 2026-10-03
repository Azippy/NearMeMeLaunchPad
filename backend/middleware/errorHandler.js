const errorHandler = (err, req, res, next) => {
  if (res.headersSent) {
    return next(err);
  }

  let statusCode = Number(err.statusCode || err.status) || 500;

  if (err.name === "ValidationError" || err.name === "CastError") {
    statusCode = 400;
  } else if (err.code === 11000) {
    statusCode = 409;
  }

  if (statusCode >= 500) {
    console.error(err.message || "Internal server error");
  }

  res.status(statusCode).json({
    success: false,
    status: statusCode >= 500 ? "error" : "fail",
    message:
      statusCode >= 500
        ? "Internal server error"
        : err.message || "Request failed",

    ...(process.env.NODE_ENV === "development" && {
      stack: err.stack,
    }),
  });
};

export default errorHandler;
