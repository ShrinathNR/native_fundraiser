use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

use pinocchio_token::{instructions::Transfer, state::TokenAccount};
use solana_nostd_sha256::hashv;

use crate::{
    state::{Contributor, Fundraiser},
    PDA_MARKER,
};

pub fn contribute(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, fundraiser, contributor_account, contributor_ta, vault, ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (bump, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let pda = hashv(&[
        fundraiser.key().as_ref(),
        &[*bump],
        crate::ID.as_ref(),
        PDA_MARKER,
    ]);
    assert_eq!(pda, vault.key().as_ref());

    let fundraiser_data = Fundraiser::from_account_info(fundraiser);

    let current_time = Clock::get()?.unix_timestamp;
    let amount: u64;

    unsafe {
        assert_eq!(
            &fundraiser_data.mint_to_raise(),
            TokenAccount::from_account_info_unchecked(vault)?.mint()
        );
        assert!(fundraiser_data.end_time() > current_time);

        amount = *(data.as_ptr() as *const u64);
    }

    if contributor_account.data_len() != 0 {
        unsafe {
            // Get a mutable pointer to the account's data once
            let data_ptr = contributor_account.borrow_mut_data_unchecked().as_mut_ptr();

            // Calculate the new amount and store it in the correct position (32-byte offset)
            *(data_ptr.add(32) as *mut [u8; 8]) =
                (Contributor::from_account_info(contributor_account).amount() + amount)
                    .to_le_bytes();
        }
    } else {
        unsafe {
            // Get a mutable pointer to the account's data
            let data_ptr = contributor_account.borrow_mut_data_unchecked().as_mut_ptr();

            // Store the contributor key at the start (32 bytes)
            *(data_ptr as *mut [u8; 32]) = *contributor.key();

            // Store the amount in the next 8 bytes (32-byte offset)
            *(data_ptr.add(32) as *mut [u8; 8]) = amount.to_le_bytes();
        }
    }

    Transfer {
        from: contributor_ta,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    Ok(())
}
