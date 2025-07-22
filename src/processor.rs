use solana_program::{
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    account_info::AccountInfo,
};
use super::{
    pool::PoolConfig,
    instruction::ProgramInstruction
};


pub struct Processor {}

impl Processor {
    pub fn process(
        program: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8] 
    ) -> ProgramResult {
        let instruction: ProgramInstruction = ProgramInstruction::unpack(&data)?;

        match instruction {
            ProgramInstruction::InitializePool(pool_config) => Self::process_initialize_pool_instruction(pool_config)?,
            ProgramInstruction::Stake(amount) => Self::process_stake_instruction(amount)?,
            ProgramInstruction::VoteUnstake(amount) => Self::process_vote_unstake_instruction(amount)?,
            ProgramInstruction::LazyUnstake(amount) => Self::process_lazy_unstake_instruction(amount)?,
            ProgramInstruction::Withdraw(amount) => Self::process_withdraw_instruction(amount)?,
        }

        Ok(())
    }

    fn process_initialize_pool_instruction(pool_config: PoolConfig) -> ProgramResult {
        todo!()
    }

    fn process_stake_instruction(amount: u64) -> ProgramResult {
        todo!()
    }

    fn process_vote_unstake_instruction(amount: u64) -> ProgramResult {
        todo!()
    }

    fn process_lazy_unstake_instruction(amount: u64) -> ProgramResult {
        todo!()
    }

    fn process_withdraw_instruction(amount: u64) -> ProgramResult {
        todo!()
    }
}