finance tracker written in rust as a side project
will track transactions and holdings 
eventually will have a web app to view everything from

## 🦀🤑 Setup on a new machine 🤑🦀

**Requires:** [Git](https://git-scm.com/) and [Rust](https://rustup.rs/) (via rustup)

```sh
# 1. Get the code
git clone https://github.com/andrewSmellz/dorsal-finance
cd dorsal-finance

# 2. Install tooling
rustup update stable
cargo install sqlx-cli --version "^0.8" --no-default-features --features sqlite

# 3. Configure and create the database
cp .env.example .env
sqlx database setup

# 4. Run
cargo run
```

> [!IMPORTANT]
> This creates an empty database. To keep existing data, close the app and copy `dorsal.db` (plus any `-wal` and `-shm` files) into the project folder before `sqlx database setup`.