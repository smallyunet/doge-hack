# Public Explorer API

If you do not have a local node, Doge-Hack can query a public explorer.

- Chain.so (v2): fetch unspent outputs by address, and broadcast transactions.
- SoChain (v3): fetch transaction details (including output script/value) by txid.

Security note:

- Public APIs can be rate-limited or temporarily inaccurate.
- Do not embed secret API keys in client-side apps.
