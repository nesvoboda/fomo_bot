use std::env;

fn send_message(token: &String, id: i64, text: String) {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = reqwest::blocking::Client::new();
    client
        .get(url)
        .query(&[("chat_id", id.to_string()), ("text", text)])
        .send()
        .unwrap();
}

fn coded_yesterday(api_key: String) -> serde_json::Value {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get("https://wakatime.com/api/v1/users/current/summaries/")
        .query(&[("api_key", api_key), ("range", "Today".to_string())])
        .send()
        .unwrap();
    res.json::<serde_json::Value>().unwrap()
}

fn main() {
    let api_key = env::var("ACTIVITY_API_KEY").unwrap();
    let bot_token = env::var("BOT_TOKEN").unwrap();
    let telegram_id = env::var("TELEGRAM_ID").unwrap().parse::<i64>().unwrap();

    let json = coded_yesterday(api_key);

    let yesterday_seconds = json["cummulative_total"]["seconds"].as_f64().unwrap();

    if yesterday_seconds == 0.0 {
        send_message(
            &bot_token,
            telegram_id,
            String::from("Seems that you haven't coded yesterday. Let's try to code today!"),
        );
    } else {
        send_message(
            &bot_token,
            telegram_id,
            format!(
                "Great work! Yesterday you've coded: {}. Keep it up!",
                json["cummulative_total"]["text"].as_str().unwrap()
            ),
        );
    }
}
