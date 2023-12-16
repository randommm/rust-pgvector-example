Playing with Postgres vector search and Google embedding API.

## What this does

Each time you run the executable:

* The message is embedding encoded with the Google embedding API.
* Then we vector search for the closest message available on the database (but excluding messages that are exactly the same).
* And finally add the message to database for future searches.

## Usage instructions

* Create a Postgres instance with pgvector: `docker run -d --name postgresql_pgvector -e POSTGRES_USER=postgres -e POSTGRES_DB=main -e POSTGRES_PASSWORD=your_postgresql_password_here -e PGDATA=/var/lib/postgresql/data/pgdata -p 5432:5432 -v ~/.local/postgresql_pgvector:/var/lib/postgresql/data ankane/pgvector`

* Get an Gemini API key at https://ai.google.dev/pricing

* Create a file called `.env` in the root directory (same folder that `LICENSE` is) with Finnhub API token, the Postgresql database url:

      GEMINI_API_TOKEN=your_token_here
      POSTGRES_PASSWORD=your_postgresql_password_here
      DATABASE_URL=postgres://postgres:${POSTGRES_PASSWORD}@localhost/main
      PGADMIN_DEFAULT_EMAIL=your@email.com
      PGADMIN_DEFAULT_PASSWORD=your_password_for_postgres_webadmin_here

* If you don't have `Rust` installed, see `https://rustup.rs`

* Create the database with: `cargo install sqlx-cli && sqlx database create && sqlx migrate run`

* Run with some messages:

* * `MESSAGE="cat" cargo run`
* * `MESSAGE="hello" cargo run`
* * `MESSAGE="a person is there" cargo run`
* * `MESSAGE="a dog is here" cargo run`
* * `MESSAGE="dog" cargo run`
* * `MESSAGE="a human is somewhere" cargo run`
* * `MESSAGE="a mouse is there" cargo run`
* * `MESSAGE="hi there" cargo run`
