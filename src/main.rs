use feed_rs::parser;
use reqwest;
use serde::Serialize;
use std::error::Error;
use tokio::time;

#[derive(Serialize)]
struct Image {
    url: String,
}

#[derive(Serialize)]
struct Embed {
    title: String,
    url: String,
    timestamp: String,
    image: Image,
    color: i32,
}

#[derive(Serialize)]
struct WebhookPayload {
    embeds: Vec<Embed>,
}

fn asset_match(content: &str) -> &str {
    let query = content.to_lowercase();
    if query.contains("ambu") {
        "https://github.com/ThijmenGThN/alarmhook/blob/main/assets/ambulance.png?raw=true"
    } else if query.contains("brand") {
        "https://github.com/ThijmenGThN/alarmhook/blob/main/assets/brandweer.png?raw=true"
    } else if query.contains("politie") {
        "https://github.com/ThijmenGThN/alarmhook/blob/main/assets/politie.png?raw=true"
    } else if query.contains("trauma") {
        "https://github.com/ThijmenGThN/alarmhook/blob/main/assets/trauma.png?raw=true"
    } else {
        "https://github.com/ThijmenGThN/alarmhook/blob/main/assets/ongeval.png?raw=true"
    }
}

async fn poll() -> Result<(), Box<dyn Error>> {
    // Read cache from file, default to empty if file doesn't exist
    let cache_str = tokio::fs::read_to_string("./cache.json")
        .await
        .unwrap_or_default();
    let mut cache: Vec<String> = if cache_str.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&cache_str)?
    };

    // Flag to check if cache was initially empty
    let cache_was_empty = cache.is_empty();

    // Fetch and parse RSS feed
    let rss_url = std::env::var("RSS")?;
    let response = reqwest::get(&rss_url).await?;
    let xml = response.text().await?;
    let feed = parser::parse(xml.as_bytes())?;

    // Process feed entries in reverse order
    for entry in feed.entries.iter().rev() {
        if cache.contains(&entry.id) {
            continue; // Skip if already in cache
        }

        // Skip sending webhooks if cache was initially empty
        if cache_was_empty {
            cache.push(entry.id.clone());
            continue;
        }

        // Extract fields for the embed
        let title = entry
            .summary
            .as_ref()
            .map(|t| t.content.clone())
            .unwrap_or_else(|| {
                entry
                    .title
                    .as_ref()
                    .map(|t| t.content.clone())
                    .unwrap_or_default()
            });
        let link = entry
            .links
            .first()
            .map(|l| l.href.clone())
            .unwrap_or_default();
        let timestamp = entry.updated.map(|dt| dt.to_rfc3339()).unwrap_or_default();
        let title_clone = title.clone();
        let image_url = asset_match(&title_clone);

        // Construct Discord embed
        let embed = Embed {
            title,
            url: link,
            timestamp,
            image: Image {
                url: image_url.to_string(),
            },
            color: 0xe36549, // Hex #e36549 as integer
        };
        let payload = WebhookPayload {
            embeds: vec![embed],
        };

        // Send to Discord webhook
        let webhook_url = std::env::var("WEBHOOK")?;
        reqwest::Client::new()
            .post(&webhook_url)
            .json(&payload)
            .send()
            .await?;

        // Update cache and log
        cache.push(entry.id.clone());
        println!("Sent: {:?}", entry);
    }

    // Write updated cache back to file
    let cache_str = serde_json::to_string(&cache)?;
    tokio::fs::write("./cache.json", cache_str).await?;
    println!("Done, next poll in 15m.\n");

    Ok(())
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initial poll
    if let Err(e) = poll().await {
        eprintln!("Error in initial poll: {}", e);
    }

    // Poll every 15 minutes
    loop {
        time::sleep(time::Duration::from_secs(15 * 60)).await;
        if let Err(e) = poll().await {
            eprintln!("Error in poll: {}", e);
        }
    }
}
