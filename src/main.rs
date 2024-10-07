/*
    Made by ThePolishTech

    Credits:
      -
*/

// xxxxxxxxxxxxxxxxxxxxxxxxxx //
// ----====  CRATES  ====---- //
// xxxxxxxxxxxxxxxxxxxxxxxxxx //

#![feature(try_blocks)]

// STANDARD LIB
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

// EXTERN CRATES
use chrono;
use rand::{self, Rng};
use regex::Regex;

// SERENETY
#[allow(unused_braces)]
use serenity::{
    async_trait,
    builder::{
        CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage,
    },
    model::{
        application::{
            Command, CommandDataOption, CommandDataOptionValue, CommandOptionType, Interaction,
        },
        colour::Colour,
        gateway::{GatewayIntents, Ready},
        id::ChannelId,
        Timestamp,
    },
    prelude::*,
};
//  --== CRATES ==--  //

//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //
//  ----==== BESPOKE ENUM DEFINITIONS ====----  //
//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //
enum LogLevel {
    // Most to Least severe
    Fatal,
    Error,
    Warning,
    Info,
}
//  --== BESPOKE ENUM DEFINITIONS ==--  //

//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //
//  ----==== BESPOKE STRUCT DEFINITIONS ====----  //
//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //

//  --== BESPOKE STRUCT DEFINITIONS ==--  //

//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //
//  ----==== BESPOKE FUNCTION DEFINITIONS ====----  //
//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //

/// Log message to console with timestamp and severity flag
fn log_to_console(message: &str, severity: LogLevel) {
    /*
        To remove extra code, rewrite this:

        Make it take an argument name AND value as an input, have it return the inner value of the result out. Poping it off of the vector
        simplifies the code                                                             Consider Vec.remove  # vec must be declared as mut
    */

    let current_time = chrono::offset::Local::now();
    let timestamp = current_time.format("%d-%m-%Y | %H:%M:%S").to_string();

    let log_level_message = match severity {
        LogLevel::Fatal   => "FATAL",
        LogLevel::Error   => "ERROR",
        LogLevel::Warning => " WARN",
        LogLevel::Info    => " INFO",
    };

    println!("[ {} ]  => {}:  {}", timestamp, log_level_message, message);
}

/// Scan provided Vec for CommandDataOption using provided name and type, if found, return a clone of it's value
fn get_option_value(
    inbound_vec: &[CommandDataOption],
    requested_option_name: &str,
    requested_type: CommandOptionType,
) -> Option<CommandDataOptionValue> {

    for (index, command_option) in inbound_vec.iter().enumerate() {
        let option_name = command_option.name.as_str();
        let option_type = command_option.value.kind();

        if requested_option_name == option_name && option_type == requested_type {
            let value = inbound_vec[index].clone().value;
            return Some(value);
        }
    }

    None
}
//  --== BESPOKE FUNCTION DEFINITIONS ==--  //

// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx //
//                                          //
// ----==== SERENETY EVENT HANDLER ====---- //
//                                          //
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx //
struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, _ready: Ready) {

        // Log connection
        log_to_console("Bot Connected!", LogLevel::Info);

        // Attempt to get timestamp
        let embed_timestamp: Result<Timestamp, String> = 'block: {
            // get current timestamp
            let system_timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
            if let Err(why) = system_timestamp {
                break 'block Err(format!(
                    "Failed to get timestamp @`EventHandler::ready`:\n\t{why}"
                ));
            }
            // Can't be an Error
            let system_timestamp = system_timestamp.unwrap().as_secs() as i64;

            // Convert to Serenity timestamp
            let embed_timestamp = Timestamp::from_unix_timestamp(system_timestamp);
            if let Err(why) = embed_timestamp {
                break 'block Err(format!(
                    "Failed to parse timestamp @`EventHandler::ready`:\n\t{why}"
                ));
            }
            // Can't be an Error
            Ok(embed_timestamp.unwrap())
        };

        // Handle any potential errors, if non, attempt to send wakeup message
        match embed_timestamp {
            Err(why) => {
                // Log error
                log_to_console(why.as_str(), LogLevel::Error)
            }

            Ok(timestamp) => {
                // Create embed with appropriate settings and send it in the wakeup_channel. If an error occurs, log it
                let embed_colour = Colour::from_rgb(0, 127, 255);
                let embed = CreateEmbed::new()
                    .title("Up and Running! :>")
                    .colour(embed_colour)
                    .timestamp(timestamp);

                let wakeup_channel = ChannelId::from(1271918404152066048);

                let message = CreateMessage::new().embed(embed);

                if let Err(why) = wakeup_channel.send_message(&ctx.http, message).await {
                    log_to_console(
                        format!("Failure to send wakeup message @`EventHandler::ready`:\n\t{why}")
                            .as_str(),
                        LogLevel::Error,
                    );
                }
            }
        }

        // --== Create Slash Commands ==-- //
        // ------------------------------- //
        let slash_commands = vec![
            CreateCommand::new("greeting")
                .description("Say Hello"),

            CreateCommand::new("echo")
                .description("Return inputed string")
                .set_options(vec![CreateCommandOption::new(
                    CommandOptionType::String,
                    "text",
                    "Text to echo",
                )
                .required(true)]),

            CreateCommand::new("roll")
                .description("Roll a dice. Defaults to d20")
                .set_options(vec![
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "action",
                        "Specifies action to roll for",
                    )
                    .required(false),

                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "dice",
                        "Specifies dice to roll. For example: 2d20, d7, 14d28",
                    )
                    .required(false),
                    CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "sum",
                        "If yes, suffix the message with the sum of all rolls"
                    )
                    .required(false)
                ]),

            CreateCommand::new("coin-flip")
                .description("Flip a coin"),

            CreateCommand::new("get-cat-gif")
                .description("Get a random cat GIF"),

            CreateCommand::new("not-implemented")
                .description("A non implemented command for debug/test reasons"),

            CreateCommand::new("send-msg-to-terminal")
                .description("Send a message to the bot's terminal")
                .set_options(vec![CreateCommandOption::new(
                    CommandOptionType::String,
                    "text",
                    "The text that gets sent to the bot's terminal",
                )
                .required(true)]),
        ];

        if let Err(why) = Command::set_global_commands(&ctx.http, slash_commands).await {
            log_to_console(
                format!("Failure to set global commands @`EventHandler::ready`:\n\t{why}").as_str(),
                LogLevel::Fatal,
            )
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction_data: Interaction) {

        match interaction_data {
            Interaction::Command(inbound_command_data) => {

                let inbound_command_name = inbound_command_data.data.name.as_str();
                let inbound_command_calling_user_id = inbound_command_data.user.id.get();

                let inbound_command_response: CreateInteractionResponseMessage =
                    match inbound_command_name {

                        "greeting" => {
                            CreateInteractionResponseMessage::new().content("Hello World!")
                        }

                        "echo" => {
                            let options = inbound_command_data.data.options.clone();
                            let wrapped_option =
                                get_option_value(&options, "text", CommandOptionType::String);

                            let content: Result<String, String> = try {
                                let unwrapped_option = wrapped_option
                                    .ok_or(String::from("`echo` Command recived without option"))?;

                                let text = &unwrapped_option.as_str().ok_or(String::from(
                                    "Could not fetch option in `echo` command",
                                ))?;

                                let text = text.to_string();
                                text
                            };

                            let text_to_echo = match content {
                                Err(why) => {
                                    log_to_console(
                                        format!("Failed to send `echo` command:\n\t{why}").as_str(),
                                        LogLevel::Warning,
                                    );
                                    why
                                }
                                Ok(text) => text,
                            };
                            CreateInteractionResponseMessage::new().content(text_to_echo.as_str())
                        }


                        "roll" => {
                            let compiled_roll_regex = Regex::new("([0-9]+)?d([0-9]+)")
                                .expect("Error parsing Regex, should be correct");
                            let options = inbound_command_data.data.options.clone();

                            let action: Option<String> = 'action: {
                                let action_option =
                                    get_option_value(&options, "action", CommandOptionType::String);

                                match action_option {
                                    Some(value) => Some(
                                        value
                                            .as_str()
                                            .unwrap_or("This won't ever get returned, logically")
                                            .to_string(),
                                    ),
                                    None => break 'action None,
                                }
                            };

                            let dice = {
                                let sides_str =
                                    get_option_value(&options, "dice", CommandOptionType::String)
                                        .unwrap_or(CommandDataOptionValue::String(String::from(
                                            "d20",
                                        )));
                                let sides_str = sides_str.as_str().expect(
                                    "Logically, this will always work, we already set a default",
                                );

                                sides_str.to_string()
                            };

                            let do_count_sum = 'block: {
                                let flag = get_option_value(&options, "sum", CommandOptionType::Boolean);
                                if flag.is_none() {
                                    break 'block false;
                                }
                                let flag = flag.unwrap(); // We checked if its none already

                                match flag.as_bool() {
                                    None =>        break 'block false,  // fallback
                                    Some(value) => break 'block value
                                };
                            };

                            let output_string: String = match compiled_roll_regex.captures(&dice) {
                                None => String::from(
                                    "Invalid dice supplied, example of valid dice: 2d20, d7, 14d28",
                                ),
                                Some(captures) => {
                                    let sides: u32 = captures
                                        .get(2)
                                        .expect("Logically, this will always contain something") // More specifically, all valid dices have a side count
                                        .as_str()
                                        .parse()
                                        .expect("Only will error if a masive number is put in"); // It'll always be a number

                                    let count = match captures.get(1) {
                                        None => 1,
                                        Some(count) => count
                                            .as_str()
                                            .parse()
                                            .expect("Logically, this will always work"),
                                    }; // If no count is provided, default it to 1

                                    let mut rolls: Vec<u32> = vec![];
                                    for _ in 0..count {
                                        rolls.push(rand::thread_rng().gen_range(1..=sides));
                                    }
                                    rolls.sort();
                                    rolls.reverse();

                                    let mut output_text: String = match action {
                                        None => {
                                            format!("<@{inbound_command_calling_user_id}> is rolling:\n")
                                        }
                                        Some(action) => {
                                            format!("<@{inbound_command_calling_user_id}> is rolling for {action}:\n")
                                        }
                                    };

                                    for roll in &rolls {
                                        output_text.push_str(
                                            format!("\t\t{roll} out of {sides}\n").as_str(),
                                        )
                                    }

                                    if do_count_sum {

                                        let mut sum = 0;

                                        for roll in &rolls {
                                            sum += roll
                                        }

                                        output_text.push_str(
                                            format!("\nSum: {sum}").as_str()
                                        );
                                    }

                                    output_text
                                }
                            };

                            CreateInteractionResponseMessage::new().content(output_string)
                        }

                        "coin-flip" => {
                            let is_heads: bool = rand::random();
                            let is_side = 1 == rand::thread_rng().gen_range(1..=10_000);

                            let mut output_text: String = format!(
                                "<@{inbound_command_calling_user_id}> is flipping a coin:\n\t\t"
                            );

                            output_text.push_str(match is_side {
                                true => "It landed on it's side!",
                                false => match is_heads {
                                    true => "Heads",
                                    false => "Tails",
                                },
                            });

                            CreateInteractionResponseMessage::new().content(output_text)
                        }

                        "get-cat-gif" => {
                            let cat_gifs = [
                                "https://tenor.com/view/cats-cat-cute-cat-cat-kissing-cat-hugging-gif-15786228053719683335",
                                "https://tenor.com/view/cat-blast-off-launch-cat-jumping-into-camera-gif-6639037559618288527",
                                "https://cdn.discordapp.com/attachments/1190927778141446225/1276520800949768223/cat-explode-cat-explosion.gif?ex=66c9d423&is=66c882a3&hm=f8ed2d48ccf0152a3e58b841b5eb5b66b4143e0c04036a8b3aa14b624e762206&",
                                "https://tenor.com/view/loading-screen-cat-gif-18391018",
                                "https://tenor.com/view/stand-cat-gif-9415145969837473423",
                                "https://tenor.com/view/cat-22-cat-22-gif-1269004317513322794",
                                "https://tenor.com/view/water-cat-cat-cat-bath-gif-8375496536506751533",
                                "https://tenor.com/view/santoso-huggie-gif-26020769",
                                "https://tenor.com/view/happy-cat-gif-10369477809722850145",
                                "https://cdn.discordapp.com/attachments/1190927778141446225/1276550190639353899/69872BF1-995F-44AE-9DB3-52122178432B.mp4?ex=66c9ef82&is=66c89e02&hm=0095170638b8313c00b0f15f3387839fdbdba85942ecdc2fed0cba6ee687eaca&",
                               "https://cdn.discordapp.com/attachments/1186768643984470159/1264032157312225371/image0.gif?ex=66c9e0ee&is=66c88f6e&hm=d2c05840652dd87b4778fdf96321e0f4528765f420ae5aaf33b35aeca295a463&",
                                "https://tenor.com/view/stillesque-gif-25542034",
                                "https://tenor.com/view/bleh-cat-silly-cat-cat-sticking-tongue-out-gif-12332973546502369976",
                                "https://tenor.com/view/cat-run-he-coming-scary-gif-19282747",
                                "https://tenor.com/view/cat-flabbergasted-shocked-wink-reaction-gif-25698054",
                                "https://tenor.com/view/flabbergasted-cat-flabbergasted-gif-540770107490810074",
                                "https://tenor.com/view/cta-cat-cat-swimming-yeah-nyxn-gif-22803107",
                                "https://tenor.com/view/cat-keyboard-ginger-cat-gif-27129143",
                                "https://tenor.com/view/cat-hello-cat-peek-cat-door-cat-peek-door-gif-17361470248875227911"
                            ];
                            let cat_gif_index = rand::thread_rng().gen_range(0..cat_gifs.len());

                            CreateInteractionResponseMessage::new().content(format!(
                                "<@{inbound_command_calling_user_id}>\n{}",
                                cat_gifs[cat_gif_index]
                            ))
                        }

                        "send-msg-to-terminal" => {
                            let options = inbound_command_data.data.options.clone();
                            let text_to_echo =
                                get_option_value(&options, "text", CommandOptionType::String);

                            match text_to_echo {
                                None => {
                                    log_to_console(
                                        "`echo` command recived without text",
                                        LogLevel::Warning,
                                    );
                                    CreateInteractionResponseMessage::new().content("ERROR")
                                }
                                Some(command_option) => {
                                    let text = command_option
                                        .as_str()
                                        .expect("eh, this should work @`send-msg-to-terminal`");

                                    log_to_console(
                                    format!( "{inbound_command_calling_user_id} sent message:\n\t{text}" ).as_str(),
                                    LogLevel::Info
                                    );

                                    CreateInteractionResponseMessage::new()
                                        .content("Message sent!")
                                }
                            }
                        }

                        _ => {
                            log_to_console(
                                format!(
                                    "Command `{inbound_command_name}` called but not implemented"
                                )
                                .as_str(),
                                LogLevel::Warning,
                            );
                            CreateInteractionResponseMessage::new().content(
                                format!(
                                    "Command `{inbound_command_name}` called but not implemented"
                                )
                                .as_str(),
                            )
                        }
                    };

                let response_message = CreateInteractionResponse::Message(inbound_command_response);
                if let Err(why) = inbound_command_data
                    .create_response(&ctx.http, response_message)
                    .await
                {
                    log_to_console(
                        format!( "Failure to respond to command @`EventHandler::interaction_create::Command`:\n\t{why}" ).as_str(),
                        LogLevel::Warning
                    )
                }
            }

            Interaction::Ping(_ping_interaction) => {}

            _ => {}
        }
    }
}
//  --== SERENETY EVENT HANDLER ==--  //

//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //
//  ----==== MAIN FUNCTION ====----  //
//  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  //
#[tokio::main]
async fn main() {
    // Display splash and conneting message
    println!("    // xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx //");
    println!("   // ----==== The Admiral is Online! ====---- //");
    println!("  // xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx //");
    println!("\nConnecting...\n");

    // Set up Gateway intents
    let gateway_intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    // Grab bot token from enviroment
    //let bot_token =

    let bot_client: Result<(), String> = 'block: {
        // First of all, check if bot token enviromental variable exists
        let bot_token = env::var("DISCORD_TOKEN");
        if let Err(why) = bot_token {
            break 'block Err(format!(
                "Failed to retrive bot token from enviroment @`main`:\n\t{why}"
            ));
        }
        let bot_token = bot_token.expect("Error already handled");

        let client_builder = Client::builder(bot_token, gateway_intents)
            .event_handler(Handler)
            .await;
        if let Err(why) = client_builder {
            break 'block Err(format!("Failed to create client @`main`:\n\t{why}"));
        }
        let mut client_builder = client_builder.expect("Error already handled");

        let client = client_builder.start().await;
        if let Err(why) = client {
            break 'block Err(format!("Failed to start client @`main`:\n\t{why}"));
        }

        client.expect("Error already handled");
        Ok(())
    };

    match bot_client {
        Err(why) => log_to_console(
            format!("Failed to start client @`main`:\n\t{why}").as_str(),
            LogLevel::Fatal,
        ),

        Ok(()) => log_to_console(
            format!("Client started successfully").as_str(),
            LogLevel::Info,
        ),
    }
}
