use solana_program::{
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    account_info::AccountInfo,
};
use super::processor::Processor;


pub fn process_instruction(
    program: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    Processor::process(program, accounts, data)?;

    Ok(())
}