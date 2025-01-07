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
CREATE ROLE webshop WITH LOGIN PASSWORD 'webshop';
```

### list users

```
\du
```

```
ALTER ROLE webshop CREATEDB;
```

```
quit
```

```
\q
```

```
psql postgres -U webshop
```

```
GRANT ALL on  SCHEMA public  TO  webshop;
```


```
CREATE DATABASE webshop ;
```

```
GRANT ALL PRIVILEGES ON DATABASE webshop TO webshop;
```
```
\connect webshop
```
