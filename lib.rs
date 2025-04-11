use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("DZVBZGk4xDUKaboJnVPT66Kd468pX1aHg8ajfZkKg3VE");

#[program]
mod hello_anchor {
    use super::*;

    // Create a user profile
    pub fn create_profile(
        ctx: Context<CreateProfile>,
        username: String,
        bio: String,
        avatar_cid: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.username = username;
        profile.bio = bio;
        profile.avatar_cid = avatar_cid;
        profile.authority = ctx.accounts.user.key();
        Ok(())
    }

    // Create a new post (stores metadata, Shadow Drive CID)
    pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
        let post = &mut ctx.accounts.post;
        post.user = ctx.accounts.user.key();
        post.content = content;
        post.timestamp = Clock::get()?.unix_timestamp;
        post.tip_amount = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        space = UserProfile::INIT_SPACE,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    #[max_len(15)]
    pub username: String, // e.g., "alice"
    #[max_len(160)]
    pub bio: String, // Short user bio
    #[max_len(260)]
    pub avatar_cid: String, // Shadow Drive CID for profile picture
    pub authority: Pubkey, // Wallet that owns this profile
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(
        init,
        payer = user, 
        space = Post::INIT_SPACE,
        seeds = [b"post", user.key().as_ref()],
        bump
        )]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Post {
    pub user: Pubkey, // Author's wallet
    #[max_len(260)]
    pub content: String, // Text content
    pub timestamp: i64, // Unix timestamp
    pub tip_amount: u64, // Total tips (lamports)
}
