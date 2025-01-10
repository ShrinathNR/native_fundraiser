use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

// contributor:
// - contributor: Pubkey
// - amount: u64
pub struct Contributor(*const u8);

impl Contributor {

    const LEN: usize = 40;

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

    pub unsafe fn contributor(&self) -> Pubkey {
        *(self.0 as *const Pubkey)
    }

    pub unsafe fn amount(&self) -> u64 {
        *(self.0.add(32) as *const u64)
    }
}