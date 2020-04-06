# web_sqlx

## for mysql
```bash
cargo run
# or
DATABASE_URL="mysql://app:app@localhost:3306/app" cargo run --no-default-features --features with-mysql
```

## for pg
```bash
DATABASE_URL="postgres://app:app@localhost:5432/app" cargo run --no-default-features --features with-postgres
```