# CLI

Show help:

```bash
cargo run -- --help
```

## Key commands

### Generate keypair

```bash
cargo run -- --network testnet gen-key
```

### Derive address

```bash
cargo run -- --network testnet address --secret-key <HEX>
```

### Fetch address UTXOs (public)

```bash
cargo run -- --network testnet fetch-utxos-address --address <ADDR>
```

### Create multisig P2SH address

```bash
cargo run -- --network testnet p2sh-multisig -m 2 -k <PUBKEY1_HEX> -k <PUBKEY2_HEX> -k <PUBKEY3_HEX>
```

### Sign a spendable transaction

The `sign` command needs the previous output (prevout) value + `scriptPubKey` to compute the legacy sighash.

You can provide it via:

- RPC (`dogecoind`) using `--rpc-url ...`
- SoChain v3 using `--api-key ...` or `CHAIN_SO_API_KEY`
- Manual `--prev-value` + `--prev-script-hex`

Example (RPC):

```bash
cargo run -- --network testnet sign \
  --txid <UTXO_TXID> --vout <VOUT> \
  --secret-key <SENDER_PRIVKEY_HEX> \
  --to <DEST_ADDR> --amount <SATOSHIS> \
  --fee 1000000 \
  --rpc-url http://127.0.0.1:44555
```

### Broadcast

Using node RPC:

```bash
cargo run -- broadcast --tx-hex <HEX> --rpc-url http://127.0.0.1:44555
```

Using public API:

```bash
cargo run -- --network testnet broadcast-public --tx-hex <HEX>
```
