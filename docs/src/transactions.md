# Transactions

## Model

Dogecoin uses Bitcoin-style transactions. This project constructs legacy (non-SegWit) transactions using `bitcoin::Transaction`.

## Signing

For legacy P2PKH, the signature hash (sighash) must be computed against the **previous output's** `scriptPubKey`.

That is why the CLI `sign` command requires the prevout script+value to be provided (or fetched).

## Fee and change

The `sign` command produces:

- 1 input: the specified `txid:vout`
- 1 output: destination `--to` with `--amount`
- optional change output: `input_value - amount - fee` to `--change-address` (default: sender address)
