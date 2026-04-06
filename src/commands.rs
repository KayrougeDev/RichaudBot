use poise::serenity_prelude::{self as serenity, Mentionable, Timestamp};

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content(format!("Pong! 🏓 {}", ctx.author().mention()))
            .ephemeral(true)
    ).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "Le texte à répéter"] message: String,
) -> Result<(), Error> {
    let author_name = ctx.author().display_name();
    let guild_name = ctx.guild().map(|g| g.name.clone()).unwrap_or_else(|| "DM".to_string());

    ctx.send(
        poise::CreateReply::default()
            .content(format!("{author_name} as dit sur {guild_name} : {message}"))
            .reply(true)
            .ephemeral(true)
    ).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "L'utilisateur à mentionner"] user: serenity::all::User,
) -> Result<(), Error> {

    let guild_name = ctx.guild().map(|g| g.name.clone());

    if let Some(name) = guild_name {
        ctx.send(
            poise::CreateReply::default()
                .content(format!("Serv {}, {}", name, user.mention()))
                .reply(false)
                .ephemeral(true)
        ).await?;
    }
    else {
        ctx.send(
            poise::CreateReply::default()
                .content("Pas un serveur ca !")
                .reply(false)
                .ephemeral(true)
        ).await?;
    }

    Ok(())
}


#[poise::command(slash_command)]
pub async fn carriere(ctx: Context<'_>) -> Result<(), Error> {

   
    let uptime = ctx.data().stats.startup_time.elapsed();
    let uptime = std::time::Duration::from_secs(uptime.as_secs());
    let text = humantime::format_duration(uptime).to_string();

    ctx.send(
        poise::CreateReply::default()
            .embed(serenity::CreateEmbed::new()
                .title("La carriere du grand Richaud")
                .description("Quelques stats")
                .color(0xDE2500)
                .field("Temps de carriere", text, true)
                .field("Serveur", ctx.guild().map_or("DM".to_string(), |g| g.name.clone()), false)
                .field("Version", &ctx.data().stats.bot_version, false)
                .footer(serenity::CreateEmbedFooter::new(format!("Demandé par {}", ctx.author().name)))
                .timestamp(Timestamp::now())
            )
    ).await?;

    Ok(())
}