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

## Docker image

To build the docker image, run the following command from project root
directory:

```bash
docker build --tag newsletter --file docker/Dockerfile .
```
