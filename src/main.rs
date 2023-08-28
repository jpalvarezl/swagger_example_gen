use anyhow::Result;
use dotenv::dotenv;

fn main() -> Result<()>{
    dotenv().expect(".env file could not be read.");

    Ok(())
}
