import app from "./app.js";
import connectDB from "./config/db.js";
import env from "./config/env.js";

const startServer = async () => {
  await connectDB();

  app.listen(env.port, () => {
    console.log(`NEARMEMEPad API running on port ${env.port}`);

    console.log(`Environment: ${env.nodeEnv}`);

    console.log(`NEAR network: ${env.nearNetwork}`);
  });
};

startServer();
