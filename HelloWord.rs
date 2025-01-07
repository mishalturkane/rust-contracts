use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Define the program's entry point
entrypoint!(process_instruction);

// Process instruction function
fn process_instruction(
    program_id: &Pubkey,        // Public key of the program
    accounts: &[AccountInfo],  // Account info array
    instruction_data: &[u8],   // Instruction data
) -> ProgramResult {
    // Log a message to the Solana blockchain
    msg!("Hello, Solana World!");

    // Additional logic can be added here

    Ok(())
}
