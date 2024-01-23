use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

declare_id!("5ZPzPodxYJ4ce4n395fR9MsX6HnmY9Cez3181mw5FNY6");

#[program]
pub mod sendsol2024 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> 
    Result<()> {
        ctx.accounts.init(&ctx.bumps, data)?;
        msg!("Changed data to: {}!", data);

        Ok(())

    }
    
    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> 
    {
        ctx.accounts.send_sol(amount)
    }
    pub fn send_sol_pda(ctx: Context<SendSolPda>, amount: u64) -> 
    Result<()> {
        ctx.accounts.send_sol_pda(amount)
    }
}
//Owner Signer way
#[derive(Accounts)]
pub struct SendSol<'info> {
#[account(mut)]
pub sender: Signer<'info>,
// CHECK
pub receiver: AccountInfo<'info>,
pub system_program: Program<'info, System>,
}

impl<'info> SendSol<'info> {    
pub fn send_sol(
    &mut self,
    amount:u64,
) -> Result<()> {       
    let accounts = Transfer {
        from: self.sender.to_account_info(),
        to: self.receiver.to_account_info()
    };

    let ctx = CpiContext::new(
        self.system_program.to_account_info(),
        accounts
    );

    transfer(ctx, amount)
}
}
//PDa way
#[derive(Accounts)]
pub struct SendSolPda<'info> {
#[account(
    seeds=[b"sender"],
    bump = sender.bump
)]
pub sender: Account<'info, SigningAccount>,
pub receiver: AccountInfo<'info>,
pub system_program: Program<'info, System>,
}

impl<'info> SendSolPda<'info> {    
pub fn send_sol_pda(
    &mut self,
    amount:u64,
) -> Result<()> {       
    let accounts = Transfer {
        from: self.sender.to_account_info(),
        to: self.receiver.to_account_info()
    };
    let seeds = &[
        &b"sender"[..],
        &[self.sender.bump],
    ];

    let signer_seeds = &[&seeds[..]];

    let ctx = CpiContext::new_with_signer(
        self.system_program.to_account_info(),
        accounts,
        signer_seeds
    );

    transfer(ctx, amount)
}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
#[account(
      init,
      payer = signer,          
      seeds = [b"sender"], 
      bump,
      space = SigningAccount::LEN
)]
pub new_account: Account<'info, SigningAccount>,
#[account(mut)]
pub signer: Signer<'info>,
pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {

pub fn init(
    &mut self, 
    bumps: &InitializeBumps,
    data: u64,
) -> Result<()> {
    
    self.new_account.set_inner(SigningAccount{ 
        data, 
        bump: bumps.new_account
});
    Ok(())

}

}

#[account]
pub struct SigningAccount {
data: u64,
bump: u8,
}

impl SigningAccount {
    pub const LEN: usize = 8 + 8 + 1;

}
