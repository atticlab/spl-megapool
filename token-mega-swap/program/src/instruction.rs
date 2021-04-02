//! Instruction types

use num_derive::ToPrimitive;
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
use solana_program::{
    clock::Slot,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

/// Validate arguments for mega swap pool.
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct Validate {    
    /// is exact amount of AB liquidity tokens user wants to put in the new pool
    pub liquidity_pool_ab_exact: u64,
    /// limit on calculated BC token pool value
    pub liquidity_pool_bc_max: u64,
}

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, ToPrimitive)]
pub enum PoolInstruction {
    ///   Initializes a new mega swap.  
    ///   AB and BC token balances must satisfy constraints. Use math module get satisfying value.
    ///   Must be executed in same transaction `spl_token_swap::instruction::Initialize`
    ///
    ///   Accounts:
    ///   - `[]` liquidity_token_ab_supply Account.
    ///   - `[]` liquidity_token_bc_supply Account.
    ///   - `[]` liquidity_token_ab_total Account.
    ///   - `[]` liquidity_token_ab_b_total Account.
    ///   - `[]` liquidity_token_bc_total Account.
    ///   - `[]` liquidity_token_bc_b_total Account.
    ///   - `[]` program_id
    Validate,

    ///   Swap the tokens in the pool.
    ///
    ///   - `[]` Token-swap
    ///   - `[]` swap authority
    ///   - `[]` user transfer authority
    ///   - `[writable]` token_A SOURCE Account, amount is transferable by user transfer authority,
    ///   - `[writable]` token_AB Base Account to mega swap INTO.
    ///   - `[writable]` token_B TEMP throw away Account user transfer authority.
    ///   - `[writable]` token_BC Base Account to mega swap FROM. 
    ///   - `[writable]` token_C DESTINATION Account assigned to USER as the owner.
    ///   - `[writable]` Pool token mint, to generate trading fees
    ///   - `[writable]` Fee account, to receive trading fees
    ///   - '[]` Token program id
    ///   - `[optional, writable]` Host fee account to receive additional trading fees
    Swap,
}

/// Create `Initialize` instruction
#[allow(clippy::too_many_arguments)]
pub fn initialize(
    program_id: &Pubkey,
    pool: &Pubkey,
    authority: &Pubkey,
    decider: &Pubkey,
    deposit_token_mint: &Pubkey,
    deposit_account: &Pubkey,
    token_pass_mint: &Pubkey,
    token_fail_mint: &Pubkey,
    token_program_id: &Pubkey,
    init_args: Initialize,
) -> Result<Instruction, ProgramError> {
    let init_data = PoolInstruction::InitPool(init_args);
    let data = init_data.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*pool, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*decider, false),
        AccountMeta::new_readonly(*deposit_token_mint, false),
        AccountMeta::new(*deposit_account, false),
        AccountMeta::new(*token_pass_mint, false),
        AccountMeta::new(*token_fail_mint, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

