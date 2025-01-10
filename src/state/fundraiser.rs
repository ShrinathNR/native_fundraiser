use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

// fundraiser:
// - mint_to_raise: Pubkey
// - maker: Pubkey
// - amount_to_raise: u64
// - end_time: i64
pub struct Fundraiser(*const u8);

impl Fundraiser {
    const LEN: usize = 80;

    pub unsafe fn from_account_info_unchecked(account_info: AccountInfo) -> Self {
            Self(account_info.borrow_mut_data_unchecked().as_ptr())
    }

    pub fn from_account_info(account_info: AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        unsafe {
            Self::from_account_info_unchecked(account_info)
        }
    }

    pub unsafe fn mint_to_raise(&self) -> Pubkey {
        *(self.0 as *const Pubkey)
    }

    pub unsafe fn maker(&self) -> Pubkey {
        *(self.0.add(32) as *const Pubkey)
    }

    pub unsafe fn amount_to_raise(&self) -> u64 {
        *(self.0.add(64) as *const u64)
    }

    pub unsafe fn end_time(&self) -> i64 {
        *(self.0.add(72) as *const i64)
    }
}