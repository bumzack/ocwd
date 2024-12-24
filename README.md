# postgres install mac

```
brew install libpq postgresql@17
brew link postgresql@17
cargo install diesel_cli --no-default-features --features postgres
```

## add to .zshrc

```
echo 'export PATH="/opt/homebrew/opt/postgresql@17/bin:$PATH"' >> ~/.zshrc
```

## postgres commands

see:  https://www.codementor.io/@engineerapart/getting-started-with-postgresql-on-mac-osx-are8jcopb

```
CREATE ROLE ollamachat WITH LOGIN PASSWORD 'ollamachat';
```

### list users

```
\du
```

```
ALTER ROLE ollamachat CREATEDB;
```

```
quit
```

```
\q
```

```
psql postgres -U ollamachat
```

```
GRANT ALL on  SCHEMA public  TO  ollamachat;
```


```
CREATE DATABASE ollamachat ;
```

```
GRANT ALL PRIVILEGES ON DATABASE ollamachat TO ollamachat;
```
```
\connect ollamachat
```
