#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, start: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = *ctx.accounts.authority.key;
        counter.count = start;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }

    pub fn initialize2(ctx: Context<Initialize2>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        // ctx.accounts.initialize2(&ctx.bumps)?; //During initialization it will create bump and here we are storing inside chain . so it will not call again and again
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize2<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        // seeds = [b"userstate", user.key().as_ref()], 
        // seeds= (b"userstate",user.key().as_ref()),
        seeds= {b"userstate" ,user.key().as_ref()},
        bump,
        space = VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        // seeds = [b"vault", vault_state.key().as_ref()],
        seeds= {b"vault", vault_state.key().as_ref()},
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub user_state_bump: u8,
    pub user_vault_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}
pub struct MyData1 {
    pub data: u64,
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 48)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

// // #[derive(Accounts)]
// // pub struct UserPda<'info>{

// // #[account(
// //     seeds = ["user".as_bytes()]
// // )]
// // pub vault : SystemAccount<'info>,
// // }

// #[program]
// mod test_macro {
//     use super::*;
//     pub fn call(ctx: Context<DoStuff>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct DoStuff<'info> {
//     #[account(
//         seeds = ["abc".as_bytes()],
//         // seeds = pda_seeds(("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")),
//         bump
//     )]
//     pub my_pda: AccountInfo<'info>,
// }

// fn calculate_seed(prefix: &str) -> &[u8] {
//     prefix.as_bytes()
// }

// // const PREFIX: &[u8] = b"prefix";

// // fn pda_seeds(arg: &Pubkey) -> &[&[u8]] {
// //     [PREFIX, arg.as_ref()]
// // }
// // const PREFIX: &[u8] = b"my_seed";

// // #[derive(Accounts)]
// // pub struct MyContext<'info> {
// //     #[account(
// //         seeds = my_seed(&user.key),
// //         bump
// //     )]
// //     pub data_account: Account<'info, MyData>,

// //     pub user: Signer<'info>,
// // }

// // fn my_seed(user: &Pubkey) -> [&[u8]; 2] {
// //     [PREFIX, user.as_ref()]
// // }

// // #[account]
// // pub struct MyData {
// //     pub data: u64,
// // }

// // const PREFIX: &[u8] = b"my_seed";

// // fn my_seed(user: &Pubkey) -> &[u8] {
// //     [PREFIX]
// // }

// // #[derive(Accounts)]
// // pub struct MyContext<'info> {
// //     #[account(
// //         seeds = my_seed(&user.key),
// //         bump
// //     )]
// //     pub data_account: Account<'info, MyData>,

// //     pub user: Signer<'info>,
// // }

// // #[account]
// // pub struct MyData {
// //     pub data: u64,
// // }
