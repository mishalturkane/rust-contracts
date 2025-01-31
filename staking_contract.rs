use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

// Program entry point
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let staking_account = next_account_info(accounts_iter)?;

    // Ensure user is a signer
    if !user_account.is_signer {
        msg!("User must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Parse instruction data
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let instruction = instruction_data[0];

    match instruction {
        1 => { // Stake SOL
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            msg!("Staking {} lamports", amount);

            // Transfer SOL from user to staking account
            solana_program::program::invoke(
                &system_instruction::transfer(user_account.key, staking_account.key, amount),
                &[user_account.clone(), staking_account.clone()],
            )?;

            Ok(())
        }
        2 => { // Unstake SOL
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            msg!("Unstaking {} lamports", amount);

            if **staking_account.lamports.borrow() < amount {
                msg!("Insufficient staking balance");
                return Err(ProgramError::InsufficientFunds);
            }

            // Transfer SOL from staking account back to user
            **staking_account.try_borrow_mut_lamports()? -= amount;
            **user_account.try_borrow_mut_lamports()? += amount;

            Ok(())
        }
        _ => {
            msg!("Invalid instruction");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}
