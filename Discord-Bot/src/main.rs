#![allow(non_snake_case)]
use std::env;

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{
        channel::Message,
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
};

const MAX_RIKER_LINES: i32 = 10;

#[group]
#[commands(riker)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // TODO: Reimplement as webhook
            let content = match command.data.name.as_str() {
                "riker" => {
                    if let Some(first_opt) = command.data.options.first() {
                        if let ApplicationCommandInteractionDataOptionValue::Integer(num_lines) =
                            first_opt.resolved.as_ref().unwrap_or(
                                &ApplicationCommandInteractionDataOptionValue::Integer(1),
                            )
                        {
                            riker_ipsum(&ctx, &num_lines)
                        } else {
                            riker_ipsum(&ctx, &1)
                        }
                    } else {
                        riker_ipsum(&ctx, &1)
                    }
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _commands =
            ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
                commands.create_application_command(|command| {
                    command
                        .name("riker")
                        .description("Generate a Riker Ipsum")
                        .create_option(|option| {
                            option
                                .name("num_lines")
                                .description("The number of ipsum lines to generate")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(false);

                            for i in 1..=MAX_RIKER_LINES {
                                option.add_int_choice(format!("{}", i), i);
                            }

                            option
                        })
                })
            })
            .await;

        println!("{} is initialized!", ready.user.name);
    }
}

fn riker_ipsum(_context: &Context, num_lines: &i64) -> String {
    // TODO: Use dictionary!
    format!("Hello I'm RikerIpsum and you wanted {} lines", num_lines)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let app_id = env::var("DISCORD_APP_ID")
        .expect("app_id")
        .parse()
        .expect("app_id is not a valid id");

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(app_id)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn riker(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, riker_ipsum(ctx, &1)).await?;

    Ok(())
}
