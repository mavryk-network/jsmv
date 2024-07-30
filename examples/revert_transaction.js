const ADDR_1 = "mv4TT7ZcSYzs7Pd9aCCqNBwuQBN71vR9sVwv";
const ADDR_2 = "mv4V9bkX1RJwi574StVPtftGpUrbcJYYH1qi";
const handler = async () => {
  console.log("Hello");
  const otherAddress = Ledger.selfAddress() == ADDR_1 ? ADDR_2 : ADDR_1;

  await Contract.call(
    otherAddress,
    "export default () => Kv.set('key', 'Hello World')",
  );
  try {
    await Contract.call(
      otherAddress,
      "export default () => { Kv.delete('key') ; throw 'Ha ha ha I deleted your key and threw an error' }",
    );
  } catch (error) {
    console.error("Caught: ", error);
  }
  await Contract.call(
    otherAddress,
    "export default () => console.log(Kv.get('key'))",
  );
};

export default handler;
