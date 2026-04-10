use std::time::Duration;

use poise::{serenity_prelude::{self as serenity, Mentionable, Timestamp}};
use tokio_util::sync::CancellationToken;

use crate::{Context, Data, Error};

pub fn get_commands() -> Vec<poise::Command<Data, Box<dyn std::error::Error + std::marker::Send + Sync + 'static>>>{
    vec![ping(), echo(), info(), carriere(), spam(), stopalltasks(), retreat(), op()]
}

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
                .color(0x2F2487)
                .field("Temps de carriere", text, true)
                .field("Serveur", ctx.guild().map_or("DM".to_string(), |g| g.name.clone()), false)
                .field("Version", &ctx.data().stats.bot_version, false)
                .footer(serenity::CreateEmbedFooter::new(format!("Demandé par {}", ctx.author().name)))
                .timestamp(Timestamp::now())
            )
    ).await?;

    Ok(())
}


#[poise::command(slash_command)]
pub async fn spam(
    ctx: Context<'_>,
    #[description = "L'utilisateur à mentionner"] user: serenity::all::User,
    #[description = "Le nombre de fois"] number: u32,
    #[description = "Les secondes entre les messages"] time_between: Option<u64>) -> Result<(), Error> {


    let http = ctx.serenity_context().http.clone();
    let channel_id = ctx.channel_id();
    let user_mention = user.mention().to_string();

    let stop_token = {
        ctx.data().stop_tokens.all_subtask_stop.lock().await.clone()
    };

    ctx.send(poise::CreateReply::default()
        .content("Debut du spam")
        .ephemeral(true)).await?;

    tokio::spawn(async move {
        for _ in 0..number {
            if stop_token.is_cancelled() {
                break;
            }
            let content = format!("Salut {}", user_mention);
            
            if let Err(e) = channel_id.say(&http, content).await {
                eprintln!("Erreur lors de l'envoi : {:?}", e);
                break;
            }
            
            match time_between {
                Some(time) => tokio::time::sleep(std::time::Duration::from_secs(time)).await,
                None => tokio::time::sleep(std::time::Duration::from_secs(1)).await
            }
        }
    });

    Ok(())
}


#[poise::command(slash_command)]
pub async fn stopalltasks(ctx: Context<'_>) -> Result<(), Error> {
    {
        let mut lock = ctx.data().stop_tokens.all_subtask_stop.lock().await;
        lock.cancel();
        *lock = CancellationToken::new();
    }

    ctx.send(poise::CreateReply::default()
        .content("Stopped all tasks !")
        .ephemeral(true)).await?;

    println!("Received stop-all-task signal");
    Ok(())
}


#[poise::command(slash_command)]
pub async fn retreat(ctx: Context<'_>, #[description = "Temps de vie restant"] time: Option<u64>) -> Result<(), Error> {
    
    if let Some(i) = time {
        ctx.say(format!("Je prend ma retraite dans {i} secondes les jeunes")).await?;
        println!("Received retreat. stopping in {i}");
        tokio::time::sleep(Duration::from_secs(i)).await;
    }
    else {
        ctx.say("Je me casse a plus").await?;
        println!("Received retreat, stopping now")
    }

    ctx.data().stop_tokens.program_stop_token.cancel();

    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn op(ctx: Context<'_>, role: serenity::Role, user: Option<serenity::Member>) -> Result<(), Error>  {

    let member: serenity::Member = match user {
        Some(m) => m,
        None => {
            ctx.author_member()
                .await
                .expect("Not a guild")
                .into_owned()
        }
    };

    let add = member.add_role(&ctx.http(), role.id).await;
    match add {
        Ok(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content("Succes !")
                    .reply(false)
                    .ephemeral(true)
            ).await?;
            println!("Given {role} to {member}");
        }
        Err(e) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Fail ! {e}"))
                    .reply(false)
                    .ephemeral(true)
            ).await?;
        }
    }


    Ok(())
}