# Random Notes for stuff

## where to find fonts on Mac OS

/System/Library/Fonts/Monaco.ttf
/System/Library/Fonts/Supplemental/Tahoma.ttf
/System/Library/Fonts/Supplemental/Arial.ttf

# restore postgres dump with fully specified dbname, user and pass

## pg_restore a database

```pg_restore --dbname=postgresql://ollamachat:ollamachat@127.0.0.1:5432/ollamachat```

## create a dump

```
pg_dump --dbname=postgresql://webshop:webshop@127.0.0.1:5432/webshop
```

## start ubuntu in textmode

systemctl set-default multi-user.target

## start ubuntu in graphics mode

systemctl set-default graphical.target

## tool calls - a collection of remarks

Some LLMs do not return a ```tool_call``` property, but put this data in the ```content``` property of a ```Message```.
This might be okayish for python or typescript servers, but using Rust things get ugly pretty fast.
LLMs which are not using the "common" way of requesting a tool_call, will probably be not/never supported.

### mistral:latest

returns the tool_call in the content of the message

```
{
    "model": "mistral:latest",
    "created_at": "2025-01-19T21:30:16.575599Z",
    "message": {
    "role": "assistant",
    "content": "{ "
        "get_current_weather": {
             "arguments": {
                "location": "San Francisco",
                "format": "celsius"
            }
        } 
    }
}
```

### command-r7b:latest

returns the tool call in the content of the message

```
"{ "location": "San Francisco", "format": "celsius" } 
```


