# Asset Manager's Vault

This Solana program implements an asset manager's vault where customers can deposit SPL tokens of their choice. The vault manager cannot withdraw the vault's funds, ensuring the security of deposited assets.

## Program Structure

The program is contained in a single `lib.rs` file and consists of the following main components:

1. **Vault**: The main account that stores information about the vault, including the manager's public key, vault name, and total value of deposited funds.

2. **InvestorPosition**: An account that tracks each investor's position in the vault, including the amount they've deposited.

3. **Instructions**:
   - `create_vault`: Initializes a new vault
   - `deposit_funds`: Allows investors to deposit funds into the vault
   - `withdraw_funds`: Allows investors to withdraw their funds but not the owner of the vault

4. **Error Handling**: Custom errors are defined to handle invalid operations.

5. **Events**: Events are emitted for deposit and withdrawal operations to facilitate off-chain tracking.

## Program Flow

### 1. Creating a Vault

- The vault manager calls the `create_vault` instruction.
- A new `Vault` account is initialized with the manager's public key, vault name, and initial total value of 0.

### 2. Depositing Funds

- An investor calls the `deposit_funds` instruction.
- The program transfers the specified amount of SPL tokens from the investor's token account to the vault's token account.
- The investor's position is updated, increasing their deposited amount.
- The vault's total value is increased.
- A `DepositEvent` is emitted.

### 3. Withdrawing Funds

- An investor (not the manager) calls the `withdraw_funds` instruction.
- The program checks if the investor has sufficient funds.
- If valid, the specified amount of SPL tokens is transferred from the vault's token account to the investor's token account.
- The investor's position is updated, decreasing their deposited amount.
- The vault's total value is decreased.
- A `WithdrawEvent` is emitted.

## Security Features

1. **Manager Restriction**: The vault manager is prevented from withdrawing funds, as checked in the `withdraw_funds` instruction.

2. **PDA-based Accounts**: The vault and token accounts use Program Derived Addresses (PDAs) for added security.

3. **Associated Token Accounts**: The program uses associated token accounts to ensure each investor and the vault have the correct token accounts for each SPL token.

4. **Amount Validation**: All deposit and withdrawal amounts are checked to be greater than zero.

5. **Balance Checks**: The program ensures investors cannot withdraw more than they've deposited.

## Events

The program emits events for deposits and withdrawals, allowing for easy off-chain tracking of vault activities.

## Error Handling

Custom errors are defined to provide clear feedback for invalid operations:
- `InvalidAmount`: When trying to deposit or withdraw zero or negative amounts.
- `InsufficientFunds`: When an investor tries to withdraw more than they've deposited.
- `ManagerCannotWithdraw`: When the vault manager attempts to withdraw funds.

This program provides a secure and transparent way for asset managers to create vaults and for investors to deposit and withdraw SPL tokens, with the assurance that the vault manager cannot access the deposited funds.
