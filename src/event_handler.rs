use std::fs::File;
use std::io::Write;
use std::process::Command;

use regex::Regex;
use serenity::builder::{CreateAttachment, CreateMessage};
use serenity::model::prelude::Message;
use serenity::{
    async_trait,
    prelude::{Context, EventHandler},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        let regex =
            Regex::new("(^```typst\n(?P<code>(\n|.)+)\n```)|(?P<formula>^\\$(.)+\\$)").unwrap();

        let capture = regex.captures(&message.content);

        if let Some(captures) = capture {
            let code = if let Some(code) = captures.name("code") {
                "#set text(font: \"Noto Serif CJK JP\")\n".to_string() + code.as_str()
            } else {
                "#set page(fill: rgb(\"#313338\"))\n#set text(fill: rgb(\"#ffffff\"))\n#set text(font: \"Noto Serif CJK JP\")\n".to_string() + captures.name("formula").unwrap().as_str()
            };

            let uuid = uuid::Uuid::new_v4().to_string();

            let mut file = File::create(format!("tmp/{uuid}.typ")).unwrap();
            file.write_all(code.as_bytes()).unwrap();

            let output = Command::new("typst")
                .args([
                    "compile",
                    "--ppi",
                    "288",
                    "-f",
                    "png",
                    &format!("tmp/{uuid}.typ"),
                    &format!("tmp/{uuid}.png"),
                ])
                .output();

            if let Ok(output) = output {
                if File::open(&format!("./tmp/{uuid}.png")).is_err() {
                    message
                        .reply(
                            &context.http,
                            format!(
                                "Error\n```\n{}\n```",
                                String::from_utf8(output.stderr).unwrap()
                            ),
                        )
                        .await
                        .unwrap();

                    return;
                }
                let output = Command::new("convert")
                    .args([
                        "convert",
                        &format!("tmp/{uuid}.png"),
                        "-trim",
                        "+repage",
                        "-bordercolor",
                        "#313338",
                        "-border",
                        "10x10",
                        &format!("tmp/{uuid}-output.png"),
                    ])
                    .output();
                if let Ok(_) = output {
                    message
                        .channel_id
                        .send_files(
                            &context.http,
                            CreateAttachment::path(format!("./tmp/{uuid}-output.png")).await,
                            CreateMessage::new(),
                        )
                        .await
                        .unwrap();
                } else {
                    message
                        .reply(&context.http, format!("Error: `{:?}`", output.err()))
                        .await
                        .unwrap();
                }
            } else {
                message
                    .reply(&context.http, format!("Error: `{:?}`", output.err()))
                    .await
                    .unwrap();
            }
        }
    }
}

/*
 * typst compile -f png test.typ test.png
 * convert test.png -trim +repage -bordercolor White -border 10x10 output.png
 * */
