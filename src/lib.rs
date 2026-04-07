use std::env;

use clap::Parser;

use poise::{command, serenity_prelude::{self as serenity, futures::lock::Mutex}};
use tokio_util::sync::CancellationToken;
use windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS;
use windows_sys::Win32::System::Console::AttachConsole;

mod commands;

use crate::commands::{get_commands};


pub struct Data {
    pub stats: Stats,
    pub stop_tokens: StopTokens
} 

pub struct Stats {
    pub startup_time: std::time::Instant,
    pub bot_version: String,
}

pub struct StopTokens {
    pub program_stop_token: CancellationToken,
    pub all_subtask_stop: Mutex<CancellationToken>
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;


#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, default_value = None, default_missing_value = "__all__", require_equals = false, num_args(0..=1), )]
    update: Option<String>,
}

pub async fn start_bot(args: Args, stop_token: CancellationToken) {
    let framework_stop_token = stop_token.clone();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_commands(),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {

                match args.update {
                    Some(mode) if mode == "__all__" => {
                        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                        println!("Updated globally !");
                    },
                    Some(mode) if mode == "def" => {

                        let id = env::var("GUILD_ID")
                            .expect("No GUILD_ID in .env").parse()?;

                        let guild_id = serenity::GuildId::new(id);
                        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                        
                        if let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await {
                            println!("Updated bot on {}", guild.name);
                        }

                    }
                    Some(mode) => {

                        let guild_id = serenity::GuildId::new(mode.parse()?);
                        poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                        
                        if let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await {
                            println!("Updated bot on {}", guild.name);
                        }

                    }
                    None => {
                        println!("Not updated !")
                    }
                }

                println!("All set ! Richaud is ready.");
                
                Ok(
                    Data {
                        stats: Stats {
                            startup_time: std::time::Instant::now(),
                            bot_version: env!("CARGO_PKG_VERSION").to_string()
                        },
                        stop_tokens: StopTokens { 
                            program_stop_token: framework_stop_token,
                            all_subtask_stop: Mutex::new(CancellationToken::new())
                        }
                    }
                )
            })
        })
        .build();

    let bot_token = env::var("BOT_TOKEN")
        .expect("No BOT_TOKEN in .env");

    let intents = serenity::GatewayIntents::non_privileged();

    let mut client = serenity::ClientBuilder::new(bot_token, intents)
        .framework(framework)
        .await
        .unwrap();

    let shard_manager = client.shard_manager.clone();

    println!("Starting Richaud...");
    let bot_future = client.start();


    tokio::select! {
        _ = bot_future => {
            println!("Richaud stopped on his own");
        },
        _ = stop_token.cancelled() => {
            println!("Stopping signal received ! shuting down...");
            shard_manager.shutdown_all().await;
            println!("Shutdowned all shards, Goodbye !");
        }
    }

}

pub fn try_attach_console() {
    unsafe {
        if AttachConsole(ATTACH_PARENT_PROCESS) != 0 {
            let _ = std::io::stdout().lock();
            println!("\nConnected to the console");
        }
    }
}