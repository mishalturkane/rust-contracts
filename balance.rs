use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Entry point of the program
entrypoint!(process_instruction);

// The main processing function
fn process_instruction(
    program_id: &Pubkey,          // Program ID of the current program
    accounts: &[AccountInfo],    // List of accounts passed to the program
    _instruction_data: &[u8],    // Data passed to the program
) -> ProgramResult {
    // Logging the program execution
    msg!("Solana Balance Display Program Started");

    // Check that at least one account was passed
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Ensure the account is owned by the system program
    if account.owner != &solana_program::system_program::ID {
        msg!("The provided account is not owned by the System Program.");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Fetch the account balance
    let balance = account.lamports();

    // Log the balance (in lamports)
    msg!("Account Address: {}", account.key);
    msg!("Account Balance: {} lamports", balance);

    Ok(())
}
