# 💸 Asset Bridge

`jsmv` maintains a persistent ledger of all accounts and their balances of L2 tez (stored as mumav).

The `jsmv` _bridge_ implements a bridge protocol that allows to transfer fungible tokens (namely [CTEZ](https://ctez.app/)) from Mavryk to the `jsmv` rollup (and back *soon*™️).

::: danger
⚠️ Withdrawals from `jsmv` to Mavryk is not supported ⚠️
:::

## Quick Start

The `jsmv` CLI empowers you to effortlessly transfer assets between a Mavryk address (`tz1`) and a `jsmv` L2 address (`tz4`) using the provided `bridge` commands.

To deposit assets from a Mavryk address to a `jsmv` L2 address, run the following command:

```bash
jsmv bridge deposit --from <TZ1_ADDRESS/ALIAS> --to <TZ4_ADDRESS/ALIAS> --amount <AMOUNT>
```

Replace `<TZ1_ADDRESS/ALIAS>` with the source Mavryk address or alias (managed by `mavkit-client`), `<TZ4_ADDRESS/ALIAS>` with the destination `jsmv` address, and `<AMOUNT>` with the quantity of CTEZ to deposit.

For example, running:

```bash
jsmv bridge deposit --from tz1faswCTDciRzE4oJ9jn2Vm2dvjeyA9fUzU \
    --to tz4N7y3T2e2dfCyHB1Ama68jnt3Fps7Ufu6d \
    --amount 42
```

sucessfully deposits 42 CTEZ from `tz1faswCTDciRzE4oJ9jn2Vm2dvjeyA9fUzU` to the `tz4N7y3T2e2dfCyHB1Ama68jnt3Fps7Ufu6d` `jsmv` address, outputting:

```
Deposited 42 CTEZ to tz4N7y3T2e2dfCyHB1Ama68jnt3Fps7Ufu6d
```

## How it Works?

::: danger
⚠️ Under construction ⚠️
:::
