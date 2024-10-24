use anchor_lang::prelude::*;

declare_id!("4tH7DHxfoeTGGqbgdPdjCwfYf8JwvujdMmD31BD9BswV");

#[program]
pub mod create_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
