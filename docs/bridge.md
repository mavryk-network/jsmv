# 💸 Asset Bridge

`jsmv` maintains a persistent ledger of all accounts and their balances of L2 tez (stored as mumav).

The `jsmv` _bridge_ implements a bridge protocol that allows to transfer fungible tokens (namely [CTEZ](https://ctez.app/)) from Mavryk to the `jsmv` rollup (and back *soon*™️).

::: danger
⚠️ Withdrawals from `jsmv` to Mavryk is not supported ⚠️
:::

## Quick Start

The `jsmv` CLI empowers you to effortlessly transfer assets between a Mavryk address (`mv1`) and a `jsmv` L2 address (`mv4`) using the provided `bridge` commands.

To deposit assets from a Mavryk address to a `jsmv` L2 address, run the following command:

```bash
jsmv bridge deposit --from <MV1_ADDRESS/ALIAS> --to <MV4_ADDRESS/ALIAS> --amount <AMOUNT>
```

Replace `<MV1_ADDRESS/ALIAS>` with the source Mavryk address or alias (managed by `mavkit-client`), `<MV4_ADDRESS/ALIAS>` with the destination `jsmv` address, and `<AMOUNT>` with the quantity of CTEZ to deposit.

For example, running:

```bash
jsmv bridge deposit --from mv1TxMEnmav51G1Hwcib1rBnBeniDMgG8nkJ \
    --to mv4aXtj1qe8kQKcZADtYNALcnG91dEUVmb44 \
    --amount 42
```

sucessfully deposits 42 CTEZ from `mv1TxMEnmav51G1Hwcib1rBnBeniDMgG8nkJ` to the `mv4aXtj1qe8kQKcZADtYNALcnG91dEUVmb44` `jsmv` address, outputting:

```
Deposited 42 CTEZ to mv4aXtj1qe8kQKcZADtYNALcnG91dEUVmb44
```

## How it Works?

::: danger
⚠️ Under construction ⚠️
:::
