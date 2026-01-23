# P2SH & Multisig

P2SH outputs lock coins to `HASH160(redeem_script)`.

This repo includes:

- building a standard `m-of-n` multisig redeem script
- generating the P2SH address for that redeem script
- generating the P2SH output script (`OP_HASH160 <hash> OP_EQUAL`)

Multisig signing details:

Legacy P2SH multisig spends require a scriptSig shaped like:

- `OP_0` (due to the CHECKMULTISIG bug)
- signatures
- the redeem script

The codebase includes a helper to construct that scriptSig.
