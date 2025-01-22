use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

/// Program entrypoint
entrypoint!(process_instruction);

/// Process the instruction
pub fn process_instruction(
    _program_id: &Pubkey,      // Program ID
    accounts: &[AccountInfo], // Accounts passed in
    _instruction_data: &[u8], // Instruction data
) -> ProgramResult {
    // Parse the account information
    let account_info_iter = &mut accounts.iter();
    let account_to_check = next_account_info(account_info_iter)?;

    // Ensure the account is a valid signer
    if !account_to_check.is_signer {
        msg!("The provided account is not a signer.");
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    // Get the account's balance
    let lamports = account_to_check.lamports();
    msg!("The account balance is: {} lamports", lamports);

    Ok(())
}
