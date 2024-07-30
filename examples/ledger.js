const SELF = "mv4TT7ZcSYzs7Pd9aCCqNBwuQBN71vR9sVwv";
const OTHER = "mv4V9bkX1RJwi574StVPtftGpUrbcJYYH1qi";

const logBalance = (address) => {
  console.log(`Balance of "${address}": ${Ledger.balance(address)}`);
};

const doTransfer = (n) => {
  console.log(`Transferring ${n} XMV from ${SELF} to ${OTHER}...`);
  Ledger.transfer("mv4V9bkX1RJwi574StVPtftGpUrbcJYYH1qi", n);
};

const doDemo = () => {
  logBalance(SELF);
  logBalance(OTHER);
  doTransfer(10);
  logBalance(SELF);
  logBalance(OTHER);
  return new Response();
};

console.log("Hello JS 👋");

export default doDemo;
