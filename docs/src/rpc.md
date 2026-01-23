# RPC (dogecoind)

The module `rpc` provides a minimal JSON-RPC client for a running Dogecoin node.

Supported operations:

- Fetch a prevout script/value with `getrawtransaction` (verbose)
- Broadcast a signed transaction via `sendrawtransaction`

This is the most reliable way to get prevout data for signing, since it reflects your node's view of the chain/mempool.
