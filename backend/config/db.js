import mongoose from "mongoose";
import env from "./env.js";

const connectDB = async () => {
  if (typeof env.mongoUri !== "string" || !env.mongoUri.trim()) {
    throw new Error("MONGO_URI is required to start the backend");
  }

  try {
    const connection = await mongoose.connect(env.mongoUri);

    console.log(`MongoDB connected: ${connection.connection.host}`);
    return connection;
  } catch (error) {
    throw new Error(`MongoDB connection failed: ${error.message}`, {
      cause: error,
    });
  }
};

export default connectDB;
