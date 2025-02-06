use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    native_token::LAMPORTS_PER_SOL,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

/// Define the program entrypoint
entrypoint!(process_instruction);

/// Process the airdrop instruction
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Extract accounts from the list
    let funder = next_account_info(accounts_iter)?; // Account sending SOL
    let recipient = next_account_info(accounts_iter)?; // Account receiving SOL
    let system_program = next_account_info(accounts_iter)?; // System program reference

    // Ensure the funder account is a signer
    if !funder.is_signer {
        msg!("Funder must sign the transaction!");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Parse amount from instruction data (assuming 8-byte u64 amount)
    if instruction_data.len() != 8 {
        msg!("Invalid instruction data length");
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes(instruction_data.try_into().unwrap());

    // Ensure a minimum amount is being airdropped
    if amount < LAMPORTS_PER_SOL / 100 {
        msg!("Airdrop amount is too small");
        return Err(ProgramError::InvalidArgument);
    }

    msg!(
        "Airdropping {} lamports from {} to {}",
        amount,
        funder.key,
        recipient.key
    );

    // Create the transfer instruction
    let transfer_instruction =
        system_instruction::transfer(funder.key, recipient.key, amount);

    // Execute the transfer instruction
    invoke(
        &transfer_instruction,
        &[funder.clone(), recipient.clone(), system_program.clone()],
    )?;

    msg!("Airdrop successful!");

    Ok(())
}
