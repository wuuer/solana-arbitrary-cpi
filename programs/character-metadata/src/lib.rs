use anchor_lang::prelude::*;

declare_id!("7bWYat9i4VqWooGSM2w3h5Chr5qzEwkSVxFQa1ciALtk");

const DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod character_metadata {
    use super::*;

    pub fn create_metadata(ctx: Context<CreateMetadata>) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;
        let random_health = pseudo_random(Clock::get()?, 20);
        let random_power = pseudo_random(Clock::get()?, 20);

        metadata.health = random_health;
        metadata.power = random_power;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    /// CHECK: manual checks
    pub character: AccountInfo<'info>,
    #[account(
        init,
        payer = authority,
        space = DISCRIMINATOR_SIZE + Metadata::INIT_SPACE,
        seeds = [character.key().as_ref()],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Metadata {
    pub character: Pubkey,
    pub health: u8,
    pub power: u8,
}

fn pseudo_random(clock: Clock, limit: u8) -> u8 {
    let big_limit = limit as i64;
    let random = clock.unix_timestamp.checked_rem(big_limit).unwrap();

    random as u8
}
