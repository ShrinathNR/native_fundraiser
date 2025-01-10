use instructions::FundraiserInstruction;
use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

pub mod state;
pub mod instructions;

pub const ID: Pubkey = five8_const::decode_32_const("ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa");

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
        FundraiserInstruction::Initialize => todo!(),
        FundraiserInstruction::Contribute => todo!(),
        FundraiserInstruction::Checker => todo!(),
        FundraiserInstruction::Refund => todo!(),
    }

}