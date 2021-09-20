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
    http::{CacheHttp, Http},
    model::{
        channel::Message,
        gateway::Ready,
        id::ChannelId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
        },
        webhook::Webhook,
    },
    prelude::TypeMapKey,
    Error,
};

const MAX_RIKER_LINES: i32 = 8;
const RIKER_DATA_PATH: &str = "data/riker.json";
const RIKER_NAME: &str = "Commander William T. Riker";
const RIKER_AVATAR_PATH: &str = "data/riker_avatar.jpg";
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

// Handles all events coming from Discord
#[async_trait]
impl EventHandler for Handler {
	// Event fired after an Interaction is used.
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Application Command is a Slash Command.
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "riker" => {
                    let lines_to_quote: usize = command
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

                    send_riker_msg(&ctx, command.channel_id, lines_to_quote)
                        .await
                        .expect("Expected send Riker msg");

                    // Discord expects some sort of response to the original
					// command, or it'll complain that the command failed.
                    command
                        .create_interaction_response(&ctx.http, |res| {
                            res.kind(InteractionResponseType::ChannelMessageWithSource);
                            res.interaction_response_data(|res_data| {
                                res_data.content(format!(
                                    "Sent {} line{}.",
                                    lines_to_quote,
                                    if lines_to_quote > 1 { "s" } else { "" }
                                ));
                                res_data.flags(
                                    InteractionApplicationCommandCallbackDataFlags::EPHEMERAL,
                                );

                                res_data
                            });

                            res
                        })
                        .await
                        .expect("Expected Interaction Response");
                }
                _ => println!("Invalid slash command: {}", command.data.name.as_str()),
            };
        }
    }

	// Event fired after bot's connection to Discord API is ready.
	// Generates Application Commandsand loads Riker data into `Context.data`.
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
                                .description("The number of Riker lines to send in one message")
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
    send_riker_msg(&ctx, msg.channel_id, 1).await?;

    Ok(())
}

// Returns the number of requested lines as one big string, joined with a space.
async fn riker_ipsum(ctx: &Context, lines_to_quote: usize) -> String {
    let data = ctx.data.read().await;
    let rikers = data
        .get::<RikerData>()
        .expect("Expected RikerData in TypeMap");

    rikers
        .iter()
        .choose_multiple(&mut rand::thread_rng(), lines_to_quote)
        .iter()
        .map(|line| line.text.clone())
        .collect::<Vec<String>>()
        .join(" ")
}

// Reads from file at RIKER_DATA_PATH
fn load_riker_data() -> HashSet<RikerLine> {
    let file = File::open(Path::new(RIKER_DATA_PATH)).expect("Error opening riker.json file");

    serde_json::from_reader(&file).expect("Error parsing riker.json data")
}

// Queries for and returns the Webhook for given ChannelId, or calls
// create_riker_webhook() to build one.
async fn get_bot_webhook(ctx: &Context, chan: ChannelId) -> Webhook {
    let http = ctx.http();
    let hooks = chan
        .webhooks(http)
        .await
        .expect("Expected channel webhooks");
    let bot_name = http
        .get_current_user()
        .await
        .expect("Expected bot current user")
        .name;
    let channel_name = chan
        .name(ctx.cache().unwrap())
        .await
        .expect("Expected channel name");
    let hook_name = format!("{}:{}", bot_name, channel_name);

    if let Some(hook) = hooks
        .iter()
        .find(|h| h.name.as_ref().unwrap_or(&"None".to_string()) == &hook_name)
    {
        hook.clone()
    } else {
        create_riker_webhook(chan, http, hook_name).await
    }
}

// Creates a new Webhook for given ChannelId, using RIKER_AVATAR_PATH for avatar.
async fn create_riker_webhook(chan_id: ChannelId, http: &Http, hook_name: String) -> Webhook {
    // TODO: Query to find and reuse Discord CDN URL for avatar that doesn't
	// use async within a closure.
    // let guilds = http
    //     .get_guilds(&GuildPagination::After(GuildId(0)), 100)
    //     .await
    //     .expect("Expected guilds");
    // let y = guilds.iter().find(|g| {
    //     let h = g.id.webhooks(http);
    //     todo!()
    // });
    let avatar = Path::new(RIKER_AVATAR_PATH);

    chan_id
        .create_webhook_with_avatar(http, hook_name, avatar)
        .await
        .expect("Expected created Webhook")
}

// Generates and sends a `lines` number of lines in one messagem using webhook.
async fn send_riker_msg(
    ctx: &Context,
    chan: ChannelId,
    lines: usize,
) -> Result<Option<Message>, Error> {
    let content = riker_ipsum(&ctx, lines).await;
    let hook = get_bot_webhook(&ctx, chan).await;

    hook.execute(&ctx.http, false, |wh| {
        wh.content(content);
        wh.username(RIKER_NAME);

        wh
    })
    .await
}
