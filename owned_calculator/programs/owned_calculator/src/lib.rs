use anchor_lang::prelude::*;

declare_id!("641bKqcLjVwpPCbMRhAq2ZZJdJaTvEik28qJ8jjSDHJw");

const OWNER: &str = "4prmvep23UCmikgs6oeY1XmXSasvNp8W1HQRKPkgBiEi"; // my local account

#[program]
pub mod owned_calculator {
    use super::*;

    // currently as it is, the below function can be called as many times as needed.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // the Initialize struct could be called anything, like Empty or anything else
        // usually in mainnet programs this struct has init code.
        msg!("Welcome to Solana!: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        // the below is so that the program will panic if a = u64.max, b = 1, for example.
        msg!("The add of a + b is {:?}", a.checked_add(b).unwrap());
        Ok(()) // here the unit type () is returned.
    }

    pub fn sub(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        // the below is so that the program will panic if a = 5, b = 6, for example.
        msg!("The sub of a - b is {:?}", a.checked_sub(b).unwrap()); 
        Ok(())
    }

    pub fn div(ctx: Context<Initialize>, a:u64, b:u64) -> Result<()> {
        // this is to avoid the program from dividing by zero
        // parenthesis in Rust conditionals are optional like shown below
        if b == 0 { return err!(InvalidArg::DivisionByZero); }
        msg!("The div of a/b is {}", a/b);
        Ok(())
    }

                                                    // there are no arrays in Rust, it's usually a Vec
    pub fn multiple_add(ctx: Context<Initialize>, numbers: Vec<u64>) -> Result<()> {
        let mut sum:u64 = 0;
        // & means borrowing, which I found easier to understand as 'view'ing in Solidity
        for num in &numbers { // basic for loop
            sum += num;
        }
        msg!("The total sum is {}", sum); 
        Ok(())
    }

    pub fn mul_div(ctx: Context<Initialize>, numbers: Vec<u64>) -> Result<()> {
        // anchor helper fn-like macro
        require!(numbers.len() == 3, InvalidArg::WrongQuantity);
        // values can be read from vectors like as they are in arrays in other languages
        // these are 0-indexed here as well
        let mut result = numbers[0] * numbers[1];

        result /= numbers[2];
        
        msg!("mul_div is {:?}", result);
        Ok(())
    }

    #[access_control(check(&ctx))] // attribute macro, it works like a function modifier in Solidity
    pub fn mul(ctx: Context<OnlyOwner>, a:u64, b:u64) -> Result<()> {
        msg!("Owner, your unsafe mul is {}", a * b); // this does not check for overflows
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // helper function from anchor like the famous require in Solidity,
    // to determine whether both params are equal. If not, return an error...
    // Hence, this fn might return an Error or an Ok. That is exactly the Result struct,
    // this is why we use it all over the program.
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerErr::Strange
        );

    Ok(())
}

// errors in Solana Rust programs are actually enum. I found it cool, as in Solidity each error is an error itself.
#[error_code] 
pub enum OnlyOwnerErr {
    #[msg("You are a strange, not the owner!")]
    Strange,
}

#[derive(Accounts)] // custom macro
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>, // the 'info gets the information from the blockchain, like the Signer, for example.
}

#[error_code]
pub enum InvalidArg {
    #[msg("CannotDivideByZero")]
    DivisionByZero,
    #[msg("WrongQuantityOfArgs")]
    WrongQuantity,
    #[msg("Overflow")]
    Overflow,
}

#[derive(Accounts)] // it could have been called anything, like Empty, Dog, etc. Anchor defaults it to Initialize
pub struct Initialize {}
