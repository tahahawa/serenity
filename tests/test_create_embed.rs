#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#![cfg(all(feature = "builder", feature = "utils"))]

#[macro_use]
extern crate serde_json;
extern crate serenity;

use serde_json::Value;
use serenity::model::{Embed, EmbedField, EmbedImage};
use serenity::utils::builder::CreateEmbed;
use serenity::utils::Colour;

#[test]
fn test_from_embed() {
    let embed = Embed {
        author: None,
        colour: Colour::new(0xFF0011),
        description: Some("This is a test description".to_string()),
        fields: vec![
            EmbedField {
                inline: false,
                name: "a".to_string(),
                value: "b".to_string(),
            },
            EmbedField {
                inline: true,
                name: "c".to_string(),
                value: "z".to_string(),
            },
        ],
        footer: None,
        image: Some(EmbedImage {
            height: 213,
            proxy_url: "a".to_string(),
            url: "https://i.imgur.com/XfWpfCV.gif".to_string(),
            width: 224,
        }),
        kind: "rich".to_string(),
        provider: None,
        thumbnail: None,
        timestamp: None,
        title: Some("hakase".to_string()),
        url: Some("https://i.imgur.com/XfWpfCV.gif".to_string()),
        video: None,
    };

    let builder = CreateEmbed::from(embed)
        .colour(0xFF0011)
        .description("This is a hakase description")
        .image("https://i.imgur.com/XfWpfCV.gif")
        .title("still a hakase")
        .url("https://i.imgur.com/XfWpfCV.gif");

    let built = Value::Object(builder.0);

    let obj = json!({
        "color": 0xFF0011,
        "description": "This is a hakase description",
        "title": "still a hakase",
        "type": "rich",
        "url": "https://i.imgur.com/XfWpfCV.gif",
        "fields": [
            {
                "inline": false,
                "name": "a",
                "value": "b",
            },
            {
                "inline": true,
                "name": "c",
                "value": "z",
            },
        ],
        "image": {
            "url": "https://i.imgur.com/XfWpfCV.gif",
        },
    });

    assert_eq!(built, obj);
}
