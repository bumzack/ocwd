

# Random Notes for stuff

## where to find fonts on Mac OS
/System/Library/Fonts/Monaco.ttf
/System/Library/Fonts/Supplemental/Tahoma.ttf
/System/Library/Fonts/Supplemental/Arial.ttf


# restore postgres dump with fully specified dbname, user and pass
pg_dump works the same I guess

pg_restore --dbname=postgresql://ollamachat:ollamachat@127.0.0.1:5432/ollamachat


## start ubuntu in textmode

systemctl set-default  multi-user.target

## start ubuntu in graphics mode

systemctl set-default   graphical.target