// add_reaction.rs
use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction: ReactionType) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;

    // -------------------------------------------------------------------------------------------
    // Based on the reaction type (Like or Dislike), update the corresponding count in the tweet.
    match reaction {
        ReactionType::Like => {
            tweet.likes = tweet
                .likes
                .checked_add(1)
                .ok_or(TwitterError::MaxLikesReached)?; // Ensures no overflow of likes
        }
        ReactionType::Dislike => {
            tweet.dislikes = tweet
                .dislikes
                .checked_add(1)
                .ok_or(TwitterError::MaxDislikesReached)?; // Ensures no overflow of dislikes
        }
    }

    tweet_reaction.reaction = reaction;

    // -------------------------------------------------------------------------------------------
    // Update the tweet_reaction account with necessary information.
    tweet_reaction.reaction_author = ctx.accounts.reaction_author.key();  // Store the reaction author's public key
    tweet_reaction.parent_tweet = ctx.accounts.tweet.key();               // Link to the tweet this reaction belongs to

    // Fix here: Access the bump for tweet_reaction directly from ctx.bumps
    tweet_reaction.bump = ctx.bumps.tweet_reaction;      // Get bump from context

    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>, // The person who is reacting

    // Initialize the tweet_reaction account using a unique seed derived from reaction_author and tweet
    #[account(
        init,
        payer = reaction_author, 
        space = 8 + Reaction::LEN,  // Allocate space for the reaction data (you'll want to adjust this size)
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref(),
        ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,  // The reaction being added

    // Ensure tweet account is mutable and provide seeds for tweet address
    #[account(
        mut,
        seeds = [
            tweet.topic[..tweet.topic_length as usize].as_ref(),
            TWEET_SEED.as_bytes(),
            tweet.tweet_author.key().as_ref(),
        ],
        bump = tweet.bump
    )]
    pub tweet: Account<'info, Tweet>,  // The tweet being reacted to

    pub system_program: Program<'info, System>,  // System program to initialize the account
}
