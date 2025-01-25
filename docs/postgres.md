# How to install postgres and create an user and database

## Install postgres 17 on ubuntu 24.10

See https://dev.to/johndotowl/postgresql-17-installation-on-ubuntu-2404-5bfi for details.

```
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
```

```
curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/postgresql.gpg
```

```
sudo apt update
```

```
sudo apt install postgresql-17
```

```
sudo systemctl start postgresql
```

```
sudo systemctl enable postgresql
```

### if you have a firewall running (ufw) open the port

```
sudo ufw allow 5432/tcp
```


### diesel needs the development libraries for postgres

```
sudo apt-get install libpq-dev
```

## create user and database 

Login in to postgres
```
sudo -u postgres psql
```


Create a user
```
CREATE ROLE ollamachat WITH LOGIN PASSWORD 'ollamachat' ;
```

add permission to create a database to the user
```
ALTER ROLE ollamachat CREATEDB; 
```

```
CREATE DATABASE ollamachat;
```

Grant access/privileges to the user
```
GRANT ALL PRIVILEGES ON DATABASE ollamachat TO ollamachat;
```

if you run into a problem ```Failed to run migrations: permission denied for schema public``` error when running
```diesel migration run```

then either 
```
GRANT ALL ON SCHEMA public TO ollamachat;
```

or this should help
```
ALTER DATABASE ollamachat OWNER TO ollamachat;
```


Login and connect to database
```
psql -U ollamachat -h 127.0.0.1 ollamachat
```


## postgres install Mac OSX

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
or this should help
```
ALTER DATABASE webshop OWNER TO webshop;
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

