use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Define the entry point for the program
entrypoint!(process_instruction);

// Main processing function
fn process_instruction(
    _program_id: &Pubkey, // ID of the program being executed
    _accounts: &[AccountInfo], // Accounts involved in the transaction
    instruction_data: &[u8], // Input data
) -> ProgramResult {
    // Parse the input data (2 numbers to add)
    if instruction_data.len() != 8 {
        msg!("Invalid input data length. Expected 8 bytes (two i32 numbers).");
        return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
    }

    let num1 = i32::from_le_bytes(instruction_data[0..4].try_into().unwrap());
    let num2 = i32::from_le_bytes(instruction_data[4..8].try_into().unwrap());

    // Perform the addition
    let sum = num1 + num2;

    // Log the result
    msg!("The sum of {} and {} is {}", num1, num2, sum);

    Ok(())
}
