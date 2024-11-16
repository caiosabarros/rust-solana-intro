use anchor_lang::prelude::*;

declare_id!("641bKqcLjVwpPCbMRhAq2ZZJdJaTvEik28qJ8jjSDHJw");

#[program]
pub mod owned_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Welcome to Solana!: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        msg!("The add of a + b is {:?}", a.checked_add(b).unwrap());
        Ok(())
    }

    pub fn sub(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        msg!("The sub of a - b is {:?}", a.checked_sub(b).unwrap());
        Ok(())
    }

    pub fn div(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        if b == 0 { return err!(InvalidArg::DivisionByZero); }
        msg!("The div of a/b is {}", a/b);
        Ok(())
    }

    pub fn multipleAdd(ctx: Context<Initialize>, numbers: Vec<u64>) -> Result<()> {
        let mut sum:u64 = 0;
        for num in &numbers {
            sum += num;
        }
        msg!("The total sum is {}", sum);
        Ok(())
    }
}

#[error_code]
pub enum InvalidArg {
    #[msg("CannotDivideByZero")]
    DivisionByZero,
}

#[derive(Accounts)]
pub struct Initialize {}
