use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .prompt("cats on sofa and carpet in living room")
        .n(2)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    let mut choices = vec!["Update Plan", "Execute Plan", "Exit"];
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option:")
            .items(&choices)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                // TODO: Implement functionality for updating plans
            }
            1 => {
                // TODO: Implement functionality for executing plans
            }
            2 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid selection.");
                continue;
            }
        }
    }

    Ok(())
}
