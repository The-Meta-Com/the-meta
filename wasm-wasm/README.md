# Substrate Cumulus Parachain Template

A new [Cumulus](https://github.com/paritytech/cumulus/)-based Substrate node, ready for hacking ☁️..

This project is originally a fork of the
[Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
modified to include dependencies required for registering this node as a **parathread** or
**parachain** to a **relay chain**.

The stand-alone version of this template is hosted on the
[Substrate Devhub Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/)
for each release of Polkadot. It is generated directly to the upstream
[Parachain Template in Cumulus](https://github.com/paritytech/cumulus/tree/master/parachain-template)
at each release branch using the
[Substrate Template Generator](https://github.com/paritytech/substrate-template-generator/).

👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and
parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).


🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).


1. insert key to keystore
./target/release/node-template key insert --base-path /tmp/the-meta-com \
  --chain /the-meta/the-meta-com/plain-the-meta-chainspec.json \
  --scheme Sr25519 \
  --suri "your secret key" \
  --key-type aura

  --chain is the chainspec.json file for the-meta parachain
  Example: https://docs.substrate.io/tutorials/get-started/add-trusted-nodes/

going by this tutorial: https://docs.substrate.io/tutorials/connect-relay-and-parachains/connect-a-local-parachain/

  2. Reserve a unique identifier
  3. Modify the default chain specification
  4. Prepare the parachain collator
  - Start a collator node with a command similar to the following:
  - --base-path should be the same that from point 1.  


  ./target/release/parachain-template-node \
--collator \
--force-authoring \
--chain raw-parachain-chainspec.json \
--base-path /tmp/the-meta-com \
--port 40333 \
--ws-port 8844 \
-- \
--execution wasm \
--chain ../polkadot/raw-local-chainspec.json \
--port 30343 \
--ws-port 9977

   5. Register with the local relay chain.

