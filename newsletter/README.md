# Newsletter

"Newsletter" project from "Zero to production in Rust" book.

## Implemented features

- As a blog visitor, I want to subscribe to the newsletter, So that I can
  receive email updates when new content is published on the blog.

## Project structure

- `.env` only for development, used by `sqlx` to connect to the database at
  compile-time to check that queries are correct.

## Tests

- `cargo test` to run the tests.
- `TEST_LOG=true cargo test health_check_works | npx bunyan` to run a single
  test
  and see the logs.

### Property-based testing

- If you want to see the random inputs that are being generated, add a
  `dbg!(&valid_email.0);` and run
  `cargo test test_or_module_name -- --nocapture`

## Docker image

To build the docker image, run the following command from project root
directory:

```bash
docker build --tag newsletter --file docker/Dockerfile .
```

To run:

```bash
docker run -p 8000:8000 newsletter
```

## Database

To initialize the local database from scratch, run the following command:

```bash
./scripts/init_db.sh
```

To perform a migration on an existing local instance, run the following command:

```bash
SKIP_DOCKER=true POSTGRES_PORT=5433 ./scripts/init_db.sh
```

`SKIP_DOCKER=true` is used to skip running the new instance in Docker!

To perform a migration on a production instance, run the following command:

```bash
```
