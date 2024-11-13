use crate::errors::TwitterError;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &ctx.accounts.tweet_reaction;

    // -------------------------------------------------------------------------------------------
    // Check the reaction type (Like or Dislike) and subtract accordingly from tweet's like/dislike count
    match tweet_reaction.reaction {
        ReactionType::Like => {
            // Ensure we don't subtract more than what is available
            require!(tweet.likes > 0, TwitterError::MinLikesReached);
            tweet.likes = tweet
                .likes
                .checked_sub(1)
                .ok_or(TwitterError::MinLikesReached)?; // Ensures no underflow
        }
        ReactionType::Dislike => {
            // Ensure we don't subtract more than what is available
            require!(tweet.dislikes > 0, TwitterError::MinDislikesReached);
            tweet.dislikes = tweet
                .dislikes
                .checked_sub(1)
                .ok_or(TwitterError::MinDislikesReached)?; // Ensures no underflow
        }
    }
    // Close the reaction account as it's removed
    // -------------------------------------------------------------------------------------------
    // Close the tweet_reaction account and transfer any lamports to the reaction_author.
    // This is done automatically by the close flag in the account macro.

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>, // The reaction's author

    #[account(
        mut,
        close=reaction_author,  // This ensures the tweet_reaction account is closed
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref(),
        ],
        bump = tweet_reaction.bump
    )]
    pub tweet_reaction: Account<'info, Reaction>, // The reaction account being removed

    #[account(
        mut,
        seeds = [
            tweet.topic[..tweet.topic_length as usize].as_ref(),
            TWEET_SEED.as_bytes(),
            tweet.tweet_author.key().as_ref(),
        ],
        bump = tweet.bump
    )]
    pub tweet: Account<'info, Tweet>, // The tweet being reacted to
}
