

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



## tool calls

https://huggingface.co/mistralai/Mistral-7B-Instruct-v0.3

Note that, for reasons of space, this example does not show a complete cycle of calling a tool and adding the tool call and tool results to the chat history so that the model can use them in its next generation. For a full tool calling example, please see the function calling guide, and note that Mistral does use tool call IDs, so these must be included in your tool calls and tool results. They should be exactly 9 alphanumeric characters.



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
```

### command-r7b:latest

returns the tool call in the content of the message 

```
"{ "location": "San Francisco", "format": "celsius" } 
```


### granite3.1-dense:8b

nope, does not get the tool_call


###  mistral:latest

works

### qwen2.5-coder:32b 
works

