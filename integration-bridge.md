# Integration Bridge

Integration between the smart contracts and external applications is handled through a small Node/Hardhat backend.

1. **Deployment Script** – `scripts/deploy.js` compiles and deploys GOAT and MEAT then links the two contracts by calling `setMEATAddress`.
2. **ABI Artifacts** – Hardhat outputs the ABI and bytecode which can be imported by backend services or directly by the frontend to construct `ethers.Contract` instances.
3. **API Layer** – An optional Express server can expose REST endpoints wrapping calls such as `swapMEATForGOAT` or `stake`. This keeps private keys on the server while the frontend signs transactions client‑side when necessary.
4. **Event Listening** – Both layers watch for important events (`MintedWithNative`, `Staked`, `Unstaked`, etc.) to keep the UI in sync.

This bridge ensures seamless coordination between on‑chain logic and user interfaces without duplicating business rules.
