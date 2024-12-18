
# MANTRA DEX audit details
- Total Prize Pool: $60,000 in USDC
  - HM awards: $47,800 in USDC
  - QA awards: $2,000 in USDC
  - Judge awards: $5,800 in USDC
  - Validator awards: $3,900 in USDC
  - Scout awards: $500 in USDC
- [Read our guidelines for more details](https://docs.code4rena.com/roles/wardens)
- Starts November 29, 2024 20:00 UTC
- Ends January 13, 2025 20:00 UTC

**Note re: risk level upgrades/downgrades**

Two important notes about judging phase risk adjustments: 
- High- or Medium-risk submissions downgraded to Low-risk (QA)) will be ineligible for awards.
- Upgrading a Low-risk finding from a QA report to a Medium- or High-risk finding is not supported.

As such, wardens are encouraged to select the appropriate risk level carefully during the submission phase.

## Automated Findings / Publicly Known Issues


_Note for C4 wardens: Anything included in this `Automated Findings / Publicly Known Issues` section is considered a publicly known issue and is ineligible for awards._


- **[Pool Manager]** - Since the pool creation and the initial liquidity provision are two separate actions, an attacker might attempt to front-run the first liquidity provider in order to set an unfavorable price and take advantage of immediate arbitrage. This is handled by multiple messages being passed in the same tx. If a user creating a pool is concerned by front-running then they should send create and provide liquidity messages in the same transaction. We will consider adding this functionality in the contract in the future.

- **[Farm Manager]** - Since the number of farms that can be created for a given LP denom is limited, the farm creation can be blocked if an attacker creates the maximum allowed number, preventing legitimate farm creators from creating meaningful farms. This is not a concern since the farm creation fee is set relatively high, making it unfeasible for someone to perform such an attack. Additionally, the contract owner has the ability to close spam farms if necessary.

- **[Farm Manager]** - Minimal farm assets amount does not respect different
currencies. The MIN_FARM_AMOUNT constant does not take into account different coins with varying monetary value, despite farms can use different denominations for rewards. The constant was essentially chosen to prevent creating farms with dust rewards. However, considering the farm creation fee is relatively high, that issue is mitigated. To make the MIN_FARM_AMOUNT value dynamic a solution involving oracles must be adopted, which is not in place at the moment.

- **[Farm Manager]** - Magic numbers used when calculating weights. These numbers were calculated using a polynomial. Will potentially be cleaned up in a future iteration. 

- **[Farm Manager]** - Users can potentially lose farm rewards if they don't claim before the farm expires. The farm expires at least 1 month after the reward distribution finishes. 

- **[Epoch Manager]** - The contract owner can alter the epoch configuration, which could change the epoch values derived by the contract when querying the current epoch value, which is used when calculating farm rewards. Although this is possible, it is not intended to happen once the contract is instantiated and the genesis epoch starts.

- **[Fee Collector]** - The fee collector contract doesn't have any functionality as of yet besides collecting fees generated by the protocol. The contract will be migrated once there's a use for those fees.

- Overflow checks not enabled for release profile for individual contracts. It is however enabled at workspace level.


# Overview

# MANTRA DEX

## Resources

1. [Website](https://mantra.zone/)
2. [Docs](https://docs.mantrachain.io/mantra-smart-contracts/mantra_dex)

## Architecture

MANTRA DEX is based on White Whale V2. The protocol is built around singleton contracts, which makes it easier to manage
and integrate with other protocols.

The following is the architecture of MANTRA DEX, and a general description of each contract:

![Mantra Mermaid](https://github.com/code-423n4/2024-11-mantra-dex/blob/main/mantramermaid.png?raw=true)


The direction of the arrows represents the dependencies between the contracts.

### Pool Manager

The Pool Manager is the contract that manages the pools in the DEX. It is responsible for creating pool and handling
swaps. Pool creation is permisionless, meaning anyone can create a pool if the fee is paid. The Pool Manager depends on
the Farm Manager and the Fee Collector.

### Farm Manager

The Farm Manager is the contract that manages the farms in the protocol. It is responsible for creating and
distributing rewards on pools. Farm creation is permissionless, meaning anyone can create a farm if the fee is paid.
The Farm Manager depends on the Epoch Manager, as rewards are distributed based on epochs.

### Fee Collector

The Fee Collector collects the fees accrued by the protocol. Whenever a pool or a farm is created, a fee is sent
to the Fee Collector. As of now, the Fee Collector does not have any other function.

### Epoch Manager

The Epoch Manager is the contract that manages the epochs in the protocol. Its single responsibility is to create the epochs,
which are used by the Farm Manager for distributing rewards.

## Instantiation

Based on the dependencies between the contracts, the instantiation of the contracts is as follows:

- Epoch Manager
- Fee Collector
- Farm Manager
- Pool Manager

---

## Building and Deploying MANTRA DEX

To build and deploy MANTRA DEX's smart contracts, there are a series of deployment scripts under `scripts/`. Alternatively,
there are a few `just` recipes you can take advantage of. You need at least Rust v1.65.0 to compile the contracts.

### Build scripts

- `build_release.sh`: builds the project artifacts, optimized for production.
- `build_schemas.sh`: generates schemas for the contracts.
- `check_artifacts_size.sh`: validates the size of the optimized artifacts. The default maximum size is 600 kB, though
  it is customizable by passing the number of kB to the script. For example `check_artifacts_size.sh 400` verifies the
  artifacts are under 400 kB.

### Just recipes

All recipes are found in the `justfile`. To see all available recipes, run `just` or `just --list`. Here are some of them:

- `build` # Builds the whole project.
- `optimize` # Creates the optimized wasm files for deployment.
- `fmt` # Formats the rust, toml and sh files in the project.
- `schemas` # Generates the schemas for the contracts.


## Links

- **Previous audits:**  The previous audit report is not public.
- **Documentation:** https://docs.mantrachain.io/mantra-smart-contracts/mantra_dex
- **Website:** https://www.mantrachain.io/
- **X:** https://twitter.com/MANTRA_Chain

---


# Scope

*See [scope.txt](https://github.com/code-423n4/2024-11-mantra/blob/main/scope.txt)*

### Files in scope

| File                                      |    nSLOC    | Purpose |
|:----------------------------------------- |:-----------:|:-------:|
| All Rust files in [`/contracts/epoch-manager/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/contracts/epoch-manager)  |    494      |         |
| All Rust files in [`/contracts/farm-manager/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/contracts/farm-manager)   |    10739    |         |
| All Rust files in [`/contracts/fee-collector/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/contracts/fee-collector)  |    138      |         |
| All Rust files in [`/contracts/pool-manager/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/contracts/pool-manager)   |    9435     |         |
| All Rust files in [`/packages/amm/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/packages/amm)             |    1115     |         |
| All Rust files in [`/packages/common-testing/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/packages/common-testing)  |    164      |         |
| All Rust files in [`/packages/utils/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/packages/utils)           |    36       |         |
| All Rust files in [`/xtask/`](https://github.com/code-423n4/2024-11-mantra-dex/tree/main/xtask)                    |    147      |         |
| **Totals**                                | **22268** |         |

### Files out of scope
No files in this repo are out of scope.


## Scoping Q &amp; A

### General questions

| Question                                | Answer                       |
| --------------------------------------- | ---------------------------- |
| ERC20 used by the protocol              |       None            |
| Test coverage                           | Contracts: 87.13% - Packages: 83.83%                          |
| ERC721 used  by the protocol            |           None              |
| ERC777 used by the protocol             |           None                |
| ERC1155 used by the protocol            |              None           |
| Chains the protocol will be deployed on | MANTRA Chain  |



### External integrations (e.g., Uniswap) behavior in scope:


| Question                                                  | Answer |
| --------------------------------------------------------- | ------ |
| Enabling/disabling fees (e.g. Blur disables/enables fees) | No   |
| Pausability (e.g. Uniswap pool gets paused)               |  No   |
| Upgradeability (e.g. Uniswap gets upgraded)               |   No  |


### EIP compliance 
N/A



# Additional context

## Main invariants

Main invariants:

- Genesis epoch and epoch duration, not intended to changed once set up.
- Pool fees to remain immutable once the pool is created.
- Pool asset decimals to remain immutable once the pool is created.
- Pool and Farm creation fee will always be a meaningful value, never set to zero.
- The number of farms for a given LP denom is constant. 
- The number of active and closed farm positions a user can have at any given time is finite.


## Attack ideas (where to focus for bugs)
Areas of concern:

On the Pool Manager:
- Pool swaps and fees calculations with single and multihop operations.
- Pool LP calculations.
- Internal pool bookkeeping. Since the pool manager holds multiple pools, there's an internal pool ledger keeping track of what assets are in which pool. It is critical the balances are updated accurately liquidity is added/removed from pools.
- Is it possible to extract value while creating pools by not paying the right fees, i.e. pool creation fee + token factory fee?
- Potential flaw providing liquidity with a single asset?

On the Farm Manager:
- Farm reward calculation/claim logic.
- Is it possible to maliciously extract funds from the farm manager?
- Can people be locked from claiming pending rewards?
- Can an attacker exploit creating farm positions on behalf of someone else via the pool manager?
- Emergency penalty fee calculation and distribution among farm owners.

Overall:

- Pay attention to the code flows and try finding any flaws in the logic that could lead to unexpected behaviors or vulnerabilities in the contracts.
- Approximation errors when handling very large amounts, especially with assets having 18 decimal places. Think of handling trillions of such assets.



## All trusted roles in the protocol

While the protocol is permissionless, there are a few roles:

- Contract owner: The only role that can update the existing configuration of the contract. This applies to all the contracts.
- Farm owner: The owner of the farm. This role can at any point close the farm. This can also be done by the contract owner of the farm manager contract. Unclaimed funds are sent back to the farm owner.

## Describe any novel or unique curve logic or mathematical models implemented in the contracts:

Standard `xyk` AMM model for constant product pools. 
The curve model for stable pools with 32 Newton iterations instead of 256.


## Running tests


```bash
git clone --recurse https://github.com/code-423n4/2024-11-mantra-dex.git
cd 2024-11-mantra-dex
cargo build
cargo test
# If you don´t have `just`: `cargo install just` before proceeding
just optimize 
```

- For test coverage
```bash
cargo tarpaulin -v
```


## Miscellaneous
Employees of MANTRA and employees' family members are ineligible to participate in this audit.

Code4rena's rules cannot be overridden by the contents of this README. In case of doubt, please check with C4 staff.




