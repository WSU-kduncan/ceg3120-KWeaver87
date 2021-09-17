#![allow(non_snake_case)]
use rand::prelude::IteratorRandom;
use serde::Deserialize;
use std::{collections::HashSet, convert::TryInto, env, fs::File, path::Path};

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
    prelude::TypeMapKey,
};

const MAX_RIKER_LINES: i32 = 8;
const RIKER_DATA_PATH: &str = "data/riker.json";
struct RikerData;
impl TypeMapKey for RikerData {
    type Value = HashSet<RikerLine>;
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
struct RikerLine {
    text: String,
    episode: String,
    word_count: u64,
}

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
                    let lines_requested: usize = command
                        .data
                        .options
                        .first()
                        .and_then(|o| {
                            if let Some(ApplicationCommandInteractionDataOptionValue::Integer(
                                lines_req,
                            )) = o.resolved.as_ref()
                            {
                                Some(lines_req)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(&1)
                        .to_owned()
                        .try_into()
                        .expect("Could not parse option from i64 to usize");

                    riker_ipsum(&ctx, lines_requested).await
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

        let mut data = ctx.data.write().await;
        data.insert::<RikerData>(load_riker_data());

        println!("{} is initialized!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
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
    msg.reply(ctx, riker_ipsum(ctx, 1).await).await?;

    Ok(())
}

async fn riker_ipsum(ctx: &Context, num_lines: usize) -> String {
    let data = ctx.data.read().await;
    let rikers = data
        .get::<RikerData>()
        .expect("Expected RikerData in TypeMap");

    let mut rng = &mut rand::thread_rng();

    rikers
        .iter()
        .choose_multiple(&mut rng, num_lines)
        .iter()
        .map(|line| line.text.clone())
        .collect::<Vec<String>>()
        .join(" ")
}

fn load_riker_data() -> HashSet<RikerLine> {
    let file = File::open(Path::new(RIKER_DATA_PATH)).expect("Couldn't load riker.json file");

    serde_json::from_reader(&file).expect("Error reading riker.json data")
}
