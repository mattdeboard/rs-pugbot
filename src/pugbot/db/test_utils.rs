use diesel::prelude::*;
use diesel_migrations::embed_migrations;
use diesel_pg_tester::DieselPgTester;
use once_cell::sync::Lazy;

// TestResult is the type that tests and init_db() return. Any error type is acceptable, but
// diesel::result::Error is convenient for this example.
pub type TestResult<T = ()> = Result<T, diesel::result::Error>;

// PGTEST is the global DieselPgTester that the whole test suite will use.
// The error type argument must match the TestResult error type.
#[allow(dead_code)]
pub(crate) static PGTEST: Lazy<DieselPgTester<diesel::result::Error>> =
  Lazy::new(|| {
    DieselPgTester::start(
      1,
      Some("postgres://pugbot:password@localhost/pugbot_test".to_string()),
      init_db,
    )
  });

// init_db() initializes the temporary schema in the database.
#[allow(unused_must_use)]
pub fn init_db(conn: &PgConnection) -> TestResult {
  embed_migrations!();
  embedded_migrations::run_with_output(conn, &mut std::io::stdout());
  Ok(())
}
