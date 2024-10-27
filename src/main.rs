use iced::widget::{column, text, button, text_input, Column};
use iced::Theme;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::runtime::Runtime;

pub fn main() -> iced::Result {
    iced::application("Currency Converter", update, view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Convert,
    UpdateAmount(String),
    UpdateFromCurrency(String),
    UpdateToCurrency(String),
}

#[derive(Default)]
struct ConverterState {
    amount: String,
    from_currency: String,
    to_currency: String,
    converted_amount: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    conversion_rates: HashMap<String, f64>,
}

fn update(state: &mut ConverterState, message: Message) {
    match message {
        Message::Convert => {
            let amount = state.amount.parse::<f64>().unwrap_or(0.0);
            let from = state.from_currency.clone();
            let to = state.to_currency.clone();
            let api_key = "YOUR_API_KEY"; // Your api key

            let rt = Runtime::new().unwrap();
            let result = rt.block_on(fetch_conversion(amount, from, to, api_key));
            state.converted_amount = match result {
                Ok(converted) => Some(format!("{:.2}", converted)),
                Err(error) => Some(format!("Error: {}", error)),
            };
        },
        Message::UpdateAmount(amount) => {
            state.amount = amount;
        },
        Message::UpdateFromCurrency(currency) => {
            state.from_currency = currency;
        },
        Message::UpdateToCurrency(currency) => {
            state.to_currency = currency;
        },
    }
}

fn view(state: &ConverterState) -> Column<Message> {
    column![
        text("Currency Converter"),
        text_input("Amount", &state.amount)
            .on_input(Message::UpdateAmount),
        text_input("From Currency (e.g., USD)", &state.from_currency)
            .on_input(Message::UpdateFromCurrency),
        text_input("To Currency (e.g., EUR)", &state.to_currency)
            .on_input(Message::UpdateToCurrency),
        button("Convert").on_press(Message::Convert),
        text(state.converted_amount.as_deref().unwrap_or("Enter details to convert")),
    ]
}

async fn fetch_conversion(
    amount: f64, 
    from: String, 
    to: String, 
    api_key: &str
) -> Result<f64, String> {
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", api_key, from);

    let response = reqwest::get(&url).await.map_err(|_| "Request failed".to_string())?;
    let rates: ApiResponse = response.json().await.map_err(|_| "Invalid response format".to_string())?;

    let to_rate = rates.conversion_rates.get(&to).ok_or("To currency not found")?;

    Ok(amount * to_rate)
}
