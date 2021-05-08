use kankyo;
use pugbot;
use tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
  // This will load the environment variables located at `./.env`, relative to
  // the CWD. See `./.env.example` for an example on how to structure this.
  kankyo::load().expect("Failed to load .env file");
  // FIXME: needs an async executor to run
  pugbot::client_setup().await;
}
