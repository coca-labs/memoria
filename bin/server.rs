#[ntex::main]
async fn main() -> std::io::Result<()> {
    memoria::start().await
}
