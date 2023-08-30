use std::time::Duration;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{ReactionType, Context, GuildId};
use serenity::model::guild::Member;
use serenity::model::guild::Guild;

pub async fn choose_activities(ctx: &Context, member: &mut Member, guild_id: &GuildId) {
    let msg = member.user.direct_message(&ctx, |m| {
            m.content("Vad vill du vara med på? Det kan ta upp till 30sekunder att få rollerna.")
                .reactions(vec![
                    ReactionType::Unicode("🥏".to_string()),
                    ReactionType::Unicode("🎲".to_string()),
                ])
        })
        .await
        .unwrap();

    let guild = Guild::get(&ctx, guild_id)
        .await
        .expect("Invalid guild id");

    let discgolf_role = guild.role_by_name("discogolfer").expect("No discgolf found");
    let boardgame_role = guild.role_by_name("brödgamer").expect("No brödgamer role");

    let mut collector = msg.await_reactions(&ctx)
        .timeout(Duration::from_secs(60 * 3))
        .build();

    loop {
        let next = collector.next().await;

        if let Some(reaction) = next {
            let reaction_type = reaction.as_inner_ref();

            if reaction_type.emoji.unicode_eq("🥏") {
                if reaction.is_added() {
                    member.add_role(&ctx, discgolf_role.id).await.unwrap();
                } else if reaction.is_removed() {
                    member.remove_role(&ctx, discgolf_role.id).await.unwrap();
                }
            } else if reaction_type.emoji.unicode_eq("🎲") {
                if reaction.is_added() {
                    member.add_role(&ctx, boardgame_role.id).await.unwrap();
                } else if reaction.is_removed() {
                    member.remove_role(&ctx, boardgame_role.id).await.unwrap();
                }
            }
        }
    }
}

