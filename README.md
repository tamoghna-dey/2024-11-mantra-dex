# Repo setup

## ⭐️ Sponsor: Add code to this repo

- [ ] Create a PR to this repo with the below changes:
- [ ] Confirm that this repo is a self-contained repository with working commands that will build (at least) all in-scope contracts, and commands that will run tests producing gas reports for the relevant contracts.
- [ ] Please have final versions of contracts and documentation added/updated in this repo **no less than 48 business hours prior to audit start time.**
- [ ] Be prepared for a 🚨code freeze🚨 for the duration of the audit — important because it establishes a level playing field. We want to ensure everyone's looking at the same code, no matter when they look during the audit. (Note: this includes your own repo, since a PR can leak alpha to our wardens!)

## ⭐️ Sponsor: Repo checklist

- [ ] Modify the [Overview](#overview) section of this `README.md` file. Describe how your code is supposed to work with links to any relevent documentation and any other criteria/details that the auditors should keep in mind when reviewing. (Here are two well-constructed examples: [Ajna Protocol](https://github.com/code-423n4/2023-05-ajna) and [Maia DAO Ecosystem](https://github.com/code-423n4/2023-05-maia))
- [ ] Review the Gas award pool amount, if applicable. This can be adjusted up or down, based on your preference - just flag it for Code4rena staff so we can update the pool totals across all comms channels.
- [ ] Optional: pre-record a high-level overview of your protocol (not just specific smart contract functions). This saves wardens a lot of time wading through documentation.
- [ ] [This checklist in Notion](https://code4rena.notion.site/Key-info-for-Code4rena-sponsors-f60764c4c4574bbf8e7a6dbd72cc49b4#0cafa01e6201462e9f78677a39e09746) provides some best practices for Code4rena audit repos.

## ⭐️ Sponsor: Final touches
- [ ] Review and confirm the pull request created by the Scout (technical reviewer) who was assigned to your contest. *Note: any files not listed as "in scope" will be considered out of scope for the purposes of judging, even if the file will be part of the deployed contracts.*
- [ ] Check that images and other files used in this README have been uploaded to the repo as a file and then linked in the README using absolute path (e.g. `https://github.com/code-423n4/yourrepo-url/filepath.png`)
- [ ] Ensure that *all* links and image/file paths in this README use absolute paths, not relative paths
- [ ] Check that all README information is in markdown format (HTML does not render on Code4rena.com)
- [ ] Delete this checklist and all text above the line below when you're ready.

---

# MANTRA audit details
- Total Prize Pool: $60,000 in USDC
  - HM awards: $47,800 in USDC
  - QA awards: $2,000 in USDC
  - Judge awards: $5,800 in USDC
  - Validator awards: $3,900 in USDC
  - Scout awards: $500 in USDC
- [Read our guidelines for more details](https://docs.code4rena.com/roles/wardens)
- Starts November 27, 2024 20:00 UTC
- Ends January 13, 2025 20:00 UTC

**Note re: risk level upgrades/downgrades**

Two important notes about judging phase risk adjustments: 
- High- or Medium-risk submissions downgraded to Low-risk (QA)) will be ineligible for awards.
- Upgrading a Low-risk finding from a QA report to a Medium- or High-risk finding is not supported.

As such, wardens are encouraged to select the appropriate risk level carefully during the submission phase.

## Automated Findings / Publicly Known Issues

The 4naly3er report can be found [here](https://github.com/code-423n4/2024-11-mantra/blob/main/4naly3er-report.md).

_Note for C4 wardens: Anything included in this `Automated Findings / Publicly Known Issues` section is considered a publicly known issue and is ineligible for awards._

## 🐺 C4: Begin Gist paste here (and delete this line)

- [Pool Manager] Since the pool creation and the initial liquidity provision are two separate actions, an attacker might attempt to front-run the first liquidity provider in order to set an unfavorable price and take advantage of immediate arbitrage. This is handled by multiple messages being passed in the same tx. If a user creating a pool is concerned by front-running then they should send create and provide liquidity messages in the same transaction. We will consider adding this functionality in the contract in the future.

- [Farm Manager] Since the number of farms that can be created for a given LP denom is limited, the farm creation can be blocked if an attacker creates the maximum allowed number, preventing legitimate farm creators from creating meaningful farms. This is not a concern since the farm creation fee is set relatively high, making it unfeasible for someone to perform such an attack. Additionally, the contract owner has the ability to close spam farms if necessary.

- [Farm Manager] Minimal farm assets amount does not respect different
currencies. The MIN_FARM_AMOUNT constant does not take into account different coins with varying monetary value, despite farms can use different denominations for rewards. The constant was essentially chosen to prevent creating farms with dust rewards. However, considering the farm creation fee is relatively high, that issue is mitigated. To make the MIN_FARM_AMOUNT value dynamic a solution involving oracles must be adopted, which is not in place at the moment.

- [Farm Manager] Magic numbers used when calculating weights. These numbers were calculated using a polynomial. Will potentially be cleaned up in a future iteration. 

- [Farm Manager] Users can potentially lose farm rewards if they don't claim before the farm expires. The farm expires at least 1 month after the reward distribution finishes. 

- [Epoch Manager] The contract owner can alter the epoch configuration, which could change the epoch values derived by the contract when querying the current epoch value, which is used when calculating farm rewards. Although this is possible, it is not intended to happen once the contract is instantiated and the genesis epoch starts.

- [Fee Collector] The fee collector contract doesn't have any functionality as of yet besides collecting fees generated by the protocol. The contract will be migrated once there's a use for those fees.

- Overflow checks not enabled for release profile for individual contracts. It is however enabled at workspace level.

✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

# Overview

[ ⭐️ SPONSORS: add info here ]

## Links

- **Previous audits:**  The previous audit report is ready but hasn't been published yet. It will be added to this repo ASAP.
- **Documentation:** https://docs.mantrachain.io/mantra-smart-contracts/mantra_dex
- **Website:** https://www.mantrachain.io/
- **X:** https://twitter.com/MANTRA_Chain

---

# Scope

[ ✅ SCOUTS: add scoping and technical details here ]

### Files in scope
- ✅ This should be completed using the `metrics.md` file
- ✅ Last row of the table should be Total: SLOC
- ✅ SCOUTS: Have the sponsor review and and confirm in text the details in the section titled "Scoping Q amp; A"

*For sponsors that don't use the scoping tool: list all files in scope in the table below (along with hyperlinks) -- and feel free to add notes to emphasize areas of focus.*

| Contract | SLOC | Purpose | Libraries used |  
| ----------- | ----------- | ----------- | ----------- |
| [contracts/folder/sample.sol](https://github.com/code-423n4/repo-name/blob/contracts/folder/sample.sol) | 123 | This contract does XYZ | [`@openzeppelin/*`](https://openzeppelin.com/contracts/) |

### Files out of scope
✅ SCOUTS: List files/directories out of scope

## Scoping Q &amp; A

### General questions
### Are there any ERC20's in scope?: No

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".


### Are there any ERC777's in scope?: 

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".



### Are there any ERC721's in scope?: No

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".



### Are there any ERC1155's in scope?: No

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".



✅ SCOUTS: Once done populating the table below, please remove all the Q/A data above.

| Question                                | Answer                       |
| --------------------------------------- | ---------------------------- |
| ERC20 used by the protocol              |       🖊️             |
| Test coverage                           | ✅ SCOUTS: Please populate this after running the test coverage command                          |
| ERC721 used  by the protocol            |            🖊️              |
| ERC777 used by the protocol             |           🖊️                |
| ERC1155 used by the protocol            |              🖊️            |
| Chains the protocol will be deployed on | OtherMANTRA Chain  |

### ERC20 token behaviors in scope

| Question                                                                                                                                                   | Answer |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| [Missing return values](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#missing-return-values)                                                      |    |
| [Fee on transfer](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#fee-on-transfer)                                                                  |   |
| [Balance changes outside of transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#balance-modifications-outside-of-transfers-rebasingairdrops) |    |
| [Upgradeability](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#upgradable-tokens)                                                                 |    |
| [Flash minting](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#flash-mintable-tokens)                                                              |    |
| [Pausability](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#pausable-tokens)                                                                      |    |
| [Approval race protections](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#approval-race-protections)                                              |    |
| [Revert on approval to zero address](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-approval-to-zero-address)                            |    |
| [Revert on zero value approvals](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-approvals)                                    |    |
| [Revert on zero value transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-transfers)                                    |    |
| [Revert on transfer to the zero address](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-transfer-to-the-zero-address)                    |    |
| [Revert on large approvals and/or transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-large-approvals--transfers)                  |    |
| [Doesn't revert on failure](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#no-revert-on-failure)                                                   |    |
| [Multiple token addresses](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-transfers)                                          |    |
| [Low decimals ( < 6)](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#low-decimals)                                                                 |    |
| [High decimals ( > 18)](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#high-decimals)                                                              |    |
| [Blocklists](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#tokens-with-blocklists)                                                                |    |

### External integrations (e.g., Uniswap) behavior in scope:


| Question                                                  | Answer |
| --------------------------------------------------------- | ------ |
| Enabling/disabling fees (e.g. Blur disables/enables fees) | No   |
| Pausability (e.g. Uniswap pool gets paused)               |  No   |
| Upgradeability (e.g. Uniswap gets upgraded)               |   No  |


### EIP compliance checklist
N/A

✅ SCOUTS: Please format the response above 👆 using the template below👇

| Question                                | Answer                       |
| --------------------------------------- | ---------------------------- |
| src/Token.sol                           | ERC20, ERC721                |
| src/NFT.sol                             | ERC721                       |


# Additional context

## Main invariants

Main invariants:

- Genesis epoch and epoch duration, not intended to changed once set up.
- Pool fees to remain immutable once the pool is created.
- Pool asset decimals to remain immutable once the pool is created.
- Pool and Farm creation fee will always be a meaningful value, never set to zero.
- The number of farms for a given LP denom is constant. 
- The number of active and closed farm positions a user can have at any given time is finite.


✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

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




✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

## All trusted roles in the protocol

While the protocol is permissionless, there are a few roles:

- Contract owner: The only role that can update the existing configuration of the contract. This applies to all the contracts.
- Farm owner: The owner of the farm. This role can at any point close the farm. This can also be done by the contract owner of the farm manager contract. Unclaimed funds are sent back to the farm owner.

✅ SCOUTS: Please format the response above 👆 using the template below👇

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Owner                          | Has superpowers                |
| Administrator                             | Can change fees                       |

## Describe any novel or unique curve logic or mathematical models implemented in the contracts:

Standard xyk AMM model for constant product pools. 
The curve model for stable pools with 32 Newton iterations instead of 256.

✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

## Running tests

git clone https://github.com/MANTRA-Chain/mantra-dex
cd mantra-dex
cargo build
cargo test
just optimize

✅ SCOUTS: Please format the response above 👆 using the template below👇

```bash
git clone https://github.com/code-423n4/2023-08-arbitrum
git submodule update --init --recursive
cd governance
foundryup
make install
make build
make sc-election-test
```
To run code coverage
```bash
make coverage
```
To run gas benchmarks
```bash
make gas
```

✅ SCOUTS: Add a screenshot of your terminal showing the gas report
✅ SCOUTS: Add a screenshot of your terminal showing the test coverage

## Miscellaneous
Employees of MANTRA and employees' family members are ineligible to participate in this audit.

Code4rena's rules cannot be overridden by the contents of this README. In case of doubt, please check with C4 staff.


# Scope

*See [scope.txt](https://github.com/code-423n4/2024-11-mantra/blob/main/scope.txt)*

### Files in scope


| File   | Logic Contracts | Interfaces | nSLOC | Purpose | Libraries used |
| ------ | --------------- | ---------- | ----- | -----   | ------------ |
| /contracts/farm-manager/src/manager/mod.rs | ****| **** | 1 | ||
| /contracts/fee-collector/src/state.rs | ****| **** | **** | ||
| /contracts/pool-manager/sim/src/lib.rs | ****| **** | 185 | ||
| /contracts/pool-manager/src/liquidity/mod.rs | ****| **** | 1 | ||
| /contracts/pool-manager/src/router/mod.rs | ****| **** | 1 | ||
| /contracts/pool-manager/src/tests/mod.rs | ****| **** | 2 | ||
| /packages/amm/src/epoch_manager.rs | ****| **** | 82 | ||
| /packages/amm/src/tokenfactory/mod.rs | ****| **** | 6 | ||
| /packages/common-testing/src/lib.rs | ****| **** | 1 | ||
| /packages/common-testing/src/multi_test/mod.rs | ****| **** | 1 | ||
| /packages/utils/src/lib.rs | ****| **** | 2 | ||
| **Totals** | **** | **** | **282** | | |

### Files out of scope

*See [out_of_scope.txt](https://github.com/code-423n4/2024-11-mantra/blob/main/out_of_scope.txt)*

| File         |
| ------------ |
| Totals: 0 |

