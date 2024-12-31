/System/Library/Fonts/Monaco.ttf
/System/Library/Fonts/Supplemental/Tahoma.ttf
/System/Library/Fonts/Supplemental/Arial.ttf





For a one-liner, like migrating a database you can use --dbname followed by a connection string (including the password) as stated in the pg_dump manual

In essence.

pg_restore --dbname=postgresql://ollamachat:ollamachat@127.0.0.1:5432/ollamachat