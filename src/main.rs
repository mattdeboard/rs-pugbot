use kankyo;
use pugbot;

fn main() {
  // This will load the environment variables located at `./.env`, relative to
  // the CWD. See `./.env.example` for an example on how to structure this.
  kankyo::load().expect("Failed to load .env file");
  // FIXME: needs an async executor to run
  let _ = pugbot::client_setup().await;
}
