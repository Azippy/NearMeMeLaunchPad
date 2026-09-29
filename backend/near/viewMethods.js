import { queryNear } from "./rpcClient.js";

export const callViewFunction = async ({
  contractId,
  methodName,
  args = {},
}) => {
  const result = await queryNear({
    request_type: "call_function",
    finality: "final",
    account_id: contractId,
    method_name: methodName,
    args_base64: Buffer.from(JSON.stringify(args)).toString("base64"),
  });

  const rawResult = Buffer.from(result.result).toString();

  try {
    return JSON.parse(rawResult);
  } catch {
    return rawResult;
  }
};
