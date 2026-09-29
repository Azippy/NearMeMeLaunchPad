export const executeNearTransaction = async ({
  signerAccount,
  receiverId,
  actions,
}) => {
  if (!signerAccount) {
    throw new Error("Signer account is required");
  }

  if (!receiverId) {
    throw new Error("Receiver account ID is required");
  }

  if (!actions || actions.length === 0) {
    throw new Error("At least one transaction action is required");
  }

  return signerAccount.signAndSendTransaction({
    receiverId,
    actions,
  });
};
