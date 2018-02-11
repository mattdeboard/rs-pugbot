extern crate kankyo;
extern crate pugbot;

fn main() {
  // This will load the environment variables located at `./.env`, relative to
  // the CWD. See `./.env.example` for an example on how to structure this.
  kankyo::load().expect("Failed to load .env file");
  pugbot::client_setup();
}
