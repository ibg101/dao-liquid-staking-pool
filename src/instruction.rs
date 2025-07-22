use solana_program::program_error::ProgramError;
use super::pool::PoolConfig;


pub enum ProgramInstruction {
    InitializePool(PoolConfig),

    Stake(u64),

    VoteUnstake(u64),

    LazyUnstake(u64),

    Withdraw(u64)
}

type InstructionError = Result<ProgramInstruction, ProgramError>;

impl ProgramInstruction {
    pub fn unpack(data: &[u8]) -> InstructionError {
        let (discriminator, data) = data.split_at(1);

        Ok(match discriminator[0] {
            0 => Self::unpack_initialize_pool_instruction(data)?,
            1 => Self::unpack_stake_instruction(data)?,
            2 => Self::unpack_vote_unstake_instruction(data)?,
            3 => Self::unpack_lazy_unstake_instruction(data)?,
            4 => Self::unpack_withdraw_instruction(data)?,
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }

    fn unpack_initialize_pool_instruction(data: &[u8]) -> InstructionError {
        todo!()
    }

    fn unpack_stake_instruction(data: &[u8]) -> InstructionError {
        todo!()
    }

    fn unpack_vote_unstake_instruction(data: &[u8]) -> InstructionError {
        todo!()
    }

    fn unpack_lazy_unstake_instruction(data: &[u8]) -> InstructionError {
        todo!()
    }

    fn unpack_withdraw_instruction(data: &[u8]) -> InstructionError {
        todo!()
    } 
}

#[cfg(feature = "instruction")]
pub use builders::*;

#[cfg(feature = "instruction")]
/// Enables ergonomic instruction builders.
/// 
/// Decided to exclude from the program's binary, since it's not used inside of the program.
mod builders {
    // todo!
}