use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

use pinocchio_token::instructions::InitializeAccount3;

use crate::state::fundraiser;

pub fn initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [fundraiser, mint_to_raise, vault, ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(fundraiser.is_signer());

    let (bump, data) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    unsafe {
        let fundraiser_data_ptr = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();

        *(fundraiser_data_ptr as *mut Pubkey) = *mint_to_raise.key();
        // fill the remaining data after mint_to_raise(first 32 bytes) 
        *(fundraiser_data_ptr.add(32) as *mut [u8; 48]) = *(data.as_ptr() as *const [u8; 48]);
    }

    let binding = bump.to_le_bytes();
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&binding)];

    let signer = [Signer::from(&seeds)];

    InitializeAccount3 {
        account: vault,
        owner: fundraiser.key(),
        mint: mint_to_raise,
    }
    .invoke_signed(&signer)?;

    Ok(())

}