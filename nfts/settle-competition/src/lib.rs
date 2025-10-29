use bolt_lang::*;
use competition::Competition;
use anchor_spl::{
    token::{Token, Mint, TokenAccount, mint_to, MintTo},
    associated_token::AssociatedToken,
};
use mpl_token_metadata::{
    instructions::{
        CreateMetadataAccountV3Cpi,
        CreateMetadataAccountV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{DataV2, Creator},
};

declare_id!("32S5nHLK93PNVJQZgd4PQY4v9tkiLU2j9bEbHhJN4CuL");

#[system]
pub mod settle_competition {

    pub fn execute(ctx: Context<Components>, args: Vec<u8>) -> Result<Components> {
        let clock = Clock::get()?;
        let competition = &mut ctx.accounts.competition;

        require!(competition.is_active, ErrorCode::CompetitionNotActive);
        require!(clock.unix_timestamp >= competition.end_time, ErrorCode::CompetitionNotEnded);

        competition.is_active = false;

        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub competition: Competition,
    }

}

#[derive(Accounts)]
pub struct MintTrophyNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub winner: SystemAccount<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = winner,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub metadata: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: SystemAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintTrophyArgs {
    pub rank: u8,
    pub competition_id: String,
    pub final_pnl: i64,
    pub total_trades: u64,
}

pub fn mint_trophy_nft(
    ctx: Context<MintTrophyNFT>,
    args: MintTrophyArgs,
) -> Result<()> {
    let (name, symbol, uri) = match args.rank {
        1 => (
            "Gold Trophy - 1st Place".to_string(),
            "GOLD".to_string(),
            "https://arweave.net/gold-trophy.json".to_string(),
        ),
        2 => (
            "Silver Trophy - 2nd Place".to_string(),
            "SILVER".to_string(),
            "https://arweave.net/silver-trophy.json".to_string(),
        ),
        3 => (
            "Bronze Trophy - 3rd Place".to_string(),
            "BRONZE".to_string(),
            "https://arweave.net/bronze-trophy.json".to_string(),
        ),
        _ => return Err(ErrorCode::InvalidRank.into()),
    };

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    let metadata_account = CreateMetadataAccountV3Cpi::new(
        &ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountV3CpiAccounts {
            metadata: &ctx.accounts.metadata.to_account_info(),
            mint: &ctx.accounts.mint.to_account_info(),
            mint_authority: &ctx.accounts.authority.to_account_info(),
            payer: &ctx.accounts.authority.to_account_info(),
            update_authority: (&ctx.accounts.authority.to_account_info(), true),
            system_program: &ctx.accounts.system_program.to_account_info(),
            rent: Some(&ctx.accounts.rent.to_account_info()),
        },
        CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.authority.key(),
                    verified: true,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            is_mutable: false,
            collection_details: None,
        },
    );

    metadata_account.invoke()?;

    msg!("Trophy NFT minted for rank {} to {}", args.rank, ctx.accounts.winner.key());
    msg!("Competition: {} | Final P&L: {} | Total Trades: {}",
        args.competition_id, args.final_pnl, args.total_trades);

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Competition is not active")]
    CompetitionNotActive,
    #[msg("Competition has not ended yet")]
    CompetitionNotEnded,
    #[msg("Invalid rank for trophy (must be 1, 2, or 3)")]
    InvalidRank,
}