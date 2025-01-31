use instructions::{
    checker::checker, contribute::contribute, initialize::initialize, refund::refund,
    FundraiserInstruction,
};
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

pub mod errors;
pub mod instructions;
pub mod state;

pub const ID: Pubkey = five8_const::decode_32_const("ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa");
pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(discriminator)? {
        FundraiserInstruction::Initialize => initialize(accounts, data),
        FundraiserInstruction::Contribute => contribute(accounts, data),
        FundraiserInstruction::Checker => checker(accounts, data),
        FundraiserInstruction::Refund => refund(accounts, data),
    }
}
