use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::BaseCollectionV1, 
    instructions::CreateV2CpiBuilder, 
};

declare_id!("D9dejLvL66DFJq52bVE1dNUmdUZkeqNYmcW1sqh4HrVY");

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    name: String,
    uri: String,
}

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        let collection = match &ctx.accounts.collection {
          Some(collection) => Some(collection.to_account_info()),
          None => None,
        };
      
        let authority = match &ctx.accounts.authority {
          Some(authority) => Some(authority.to_account_info()),
          None => None,
        };
      
        let owner = match &ctx.accounts.owner {
          Some(owner) => Some(owner.to_account_info()),
          None => None,
        };
      
        let update_authority = match &ctx.accounts.update_authority {
          Some(update_authority) => Some(update_authority.to_account_info()),
          None => None,
        };
        
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
          .asset(&ctx.accounts.asset.to_account_info())
          .collection(collection.as_ref())
          .authority(authority.as_ref())
          .payer(&ctx.accounts.payer.to_account_info())
          .owner(owner.as_ref())
          .update_authority(update_authority.as_ref())
          .system_program(&ctx.accounts.system_program.to_account_info())
          .name(args.name)
          .uri(args.uri)
          .invoke()?;
      
        Ok(())
          }
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(mut)]
    pub collection: Option<Account<'info, BaseCollectionV1>>,
    pub authority: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: this account will be checked by the mpl_core program
    pub owner: Option<UncheckedAccount<'info>>,
    /// CHECK: this account will be checked by the mpl_core program
    pub update_authority: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}