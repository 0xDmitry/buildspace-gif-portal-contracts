use anchor_lang::prelude::*;

declare_id!("2M6m7ke3fvzGFYMSYUx5xxDpcR6GaByhgoymjPNGHnP2");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      votes: 0
    };
		
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn upvote_gif(ctx: Context<UpvoteGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
		let index = base_account.gif_list.iter().position(|x| x.gif_link == gif_link).unwrap();
    base_account.gif_list[index].votes += 1;
    Ok(())
  }

  pub fn tip_user(ctx: Context<TipUser>, amount: String) -> ProgramResult {
    let amount_as_num: u64 = amount.parse().unwrap();
    let ix = anchor_lang::solana_program::system_instruction::transfer(
      &ctx.accounts.from.key(),
      &ctx.accounts.to.key(),
      amount_as_num,
    );
    anchor_lang::solana_program::program::invoke(
      &ix,
      &[
        ctx.accounts.from.to_account_info(),
        ctx.accounts.to.to_account_info(),
      ],
    );
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpvoteGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>
}

#[derive(Accounts)]
pub struct TipUser<'info> {
  #[account(mut)]
  pub from: Signer<'info>,
  #[account(mut)]
  pub to: AccountInfo<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
  pub gif_link: String,
  pub user_address: Pubkey,
  pub votes: i64,
}

#[account]
pub struct BaseAccount {
  pub total_gifs: u64,
  pub gif_list: Vec<ItemStruct>,
}