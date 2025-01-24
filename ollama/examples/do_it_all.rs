use ollama::api::OllamaImpl;
use ollama::error::OllamaError;
use ollama::models::{
    ChatRequest, ContentEnum, Function, Message, Ollama, Parameter, Property, Tool,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), OllamaError> {
    let mut res = HashMap::new();
    let o = Ollama::new("http://localhost:11434".to_string()).expect("Couldn't open old sdk SDK");
    let skip = 0;
    let take = 1;

    let local_models = o.local_models().await?;

    let tools = get_tools();

    for local_model in local_models.iter().skip(skip).take(take) {
        let mesg = Message {
            role: "user".to_string(),
            content: Some(ContentEnum::AString(
                "How is the weather in San Francisco".to_string(),
            )),
            images: None,
            tool_calls: Some(vec![]),
            tool_call_id: None,
        };

        // let mesg = Message {
        //     role: "user".to_string(),
        //     content: ContentEnum::AString("How much is 23 minus 2".to_string()),
        //     images: None,
        //     tool_calls: Some(vec![]),
        //     tool_call_id: None,
        // };

        let request = ChatRequest {
            model: local_model.name.to_string(),
            prompt: None,
            stream: false,
            options: None,
            messages: Some(vec![mesg]),
            format: None,
            tools: Some(tools.clone()),
        };

        // let request_pretty = serde_json::to_string_pretty(&request).expect("should be a json");
        println!("request.model: {}", request.model);
        println!("request.messages: {:?}", request.messages);

        let mut had_tool_call = false;
        let mut r = o.chat(&request).await;

        let mut cnt = 0;
        loop {
            match r.as_ref() {
                Ok(res) => {
                    // let response_pretty =
                    //     serde_json::to_string_pretty(&res).expect("should be a json response");
                    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n");
                    // println!("response: \n {:?}\n", response_pretty);
                    match res.response.as_ref() {
                        Some(rrr) => println!("property response: {:?}", rrr),
                        None => println!("property response empty"),
                    }
                    match res.message.as_ref() {
                        Some(rrr) => println!("property message: {:?}", rrr),
                        None => println!("property message"),
                    }
                    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n");
                    let mut msg_responses = vec![];

                    if res.message.is_some() {
                        let msg = res.message.clone().unwrap();

                        println!("content {:?}", msg.content);

                        let old_msg = Message {
                            role: msg.role.clone(),
                            content: msg.content.clone(),
                            images: None,
                            tool_calls: msg.tool_calls.clone(),
                            tool_call_id: msg.tool_call_id.clone(),
                        };

                        println!("old_msg {:?}", old_msg);
                        msg_responses.push(old_msg);

                        if msg.tool_calls.is_some() {
                            had_tool_call = true;
                            let tool_calls = msg.tool_calls.as_ref().unwrap();
                            for tool_call in tool_calls {
                                let response_msg = match tool_call.function.name.as_str() {
                                    "get_current_weather" => Some(Message {
                                        role: "tool".to_string(),
                                        content: ContentEnum::AString("23".to_string()),
                                        images: None,
                                        tool_call_id: Some("get_current_weather".to_string()),
                                        tool_calls: None,
                                    }),
                                    "subtract_two_numbers" => Some(Message {
                                        role: "tool".to_string(),
                                        content: ContentEnum::AString("-222222".to_string()),
                                        images: None,
                                        tool_call_id: Some("subtract_two_numbers".to_string()),
                                        tool_calls: None,
                                    }),
                                    _ => None,
                                };

                                if response_msg.is_some() {
                                    msg_responses.push(response_msg.unwrap());
                                }
                            }
                        } else {
                        }
                    }

                    let new_request = ChatRequest {
                        model: local_model.name.to_string(),
                        prompt: None,
                        stream: false,
                        options: None,
                        messages: Some(msg_responses),
                        format: Some("json".to_string()),
                        tools: Some(tools.clone()),
                    };

                    r = o.chat(&new_request).await;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            cnt += 1;

            if cnt > 5 || !had_tool_call {
                println!(
                    "break out of loop. cnt {}, had_tool_call {}",
                    cnt, had_tool_call
                );
                break;
            }
        }
        res.insert(local_model.model.clone(), had_tool_call);
    }

    res.iter().for_each(|(name, result)| {
        println!("{} -> {}", name, result);
    });

    Ok(())
}

fn get_tools() -> Vec<Tool> {
    let property_location = Property {
        typ: "string".to_string(),
        description: "The location to get the weather for, e.g. San Francisco, CA".to_string(),
        enums: None,
    };

    let property_format = Property {
        typ: "string".to_string(),
        description: "The format to return the weather in, e.g. 'celsius' or 'fahrenheit'"
            .to_string(),
        enums: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
    };

    let mut properties = HashMap::new();
    properties.insert("location".to_string(), property_location);
    properties.insert("format".to_string(), property_format);

    let parameters = Parameter {
        typ: "object".to_string(),
        properties,
        required: Some(vec!["location".to_string(), "format".to_string()]),
    };

    let function = Function {
        name: "get_current_weather".to_string(),
        description: "Get the current weather for a location".to_string(),
        parameters,
    };

    let tool = Tool {
        typ: "function".to_string(),
        function,
    };

    let property_a = Property {
        typ: "integer".to_string(),
        description: "the first number".to_string(),
        enums: None,
    };

    let property_b = Property {
        typ: "integer".to_string(),
        description: "the second number".to_string(),
        enums: None,
    };

    let mut properties_sub = HashMap::new();
    properties_sub.insert("a".to_string(), property_a);
    properties_sub.insert("b".to_string(), property_b);

    let parameters_sub = Parameter {
        typ: "object".to_string(),
        properties: properties_sub,
        required: Some(vec!["a".to_string(), "b".to_string()]),
    };

    let function_sub = Function {
        name: "subtract_two_numbers".to_string(),
        description: "subtract two numbers".to_string(),
        parameters: parameters_sub,
    };

    let tool_sub = Tool {
        typ: "function".to_string(),
        function: function_sub,
    };

    vec![tool.clone(), tool_sub.clone()]
}
