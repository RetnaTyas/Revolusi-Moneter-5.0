# Contract Map

```
[User Wallet]
    |  (native token)
    v
[MEAT] <----> [GOAT]
   |              ^
   |              |
   +-- withdraw -->|
```

* **MEAT** acts as the gateway: it accepts native coins, mints MEAT, and handles swaps in both directions. The owner may withdraw accumulated native balance.
* **GOAT** receives minted supply from MEAT and provides staking functionality. Rewards and configuration parameters are adjustable by the owner.
* **FailingGOAT** is only for testing; it implements the same interface but allows simulated transfer failures.
* **IGOAT** defines the `mintTo` function which enables MEAT to mint GOAT when necessary.

The MEAT contract relies on GOAT for minting new tokens when swapping MEAT for GOAT. Both share the same owner who can manage rates and enable or disable swapping.
