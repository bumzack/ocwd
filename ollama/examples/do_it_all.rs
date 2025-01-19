use ollama::api::OllamaImpl;
use ollama::error::OllamaError;
use ollama::models::{
    ChatRequest, ContentEnum, Function, Message, Ollama, Parameter, Property, Tool,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), OllamaError> {
    let model = "llama3-groq-tool-use:8b";

    let o = Ollama::new("http://localhost:11434".to_string()).expect("Couldn't open old sdk SDK");

    let local_models = o.local_models().await?;

    local_models
        .iter()
        .for_each(|lm| println!("local models: {:?}", lm));

    let loaded_models = o.local_models().await?;

    loaded_models
        .iter()
        .for_each(|lm| println!("loaded model: {:?}", lm));

    let model = local_models
        .iter()
        .find(|m| m.model.eq(model))
        .expect("No model found");

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

    let mesg = Message {
        role: "user".to_string(),
        content: ContentEnum::AString("How is the weather in San Francisco".to_string()),
        images: None,
        tool_calls: Some(vec![]),
        tool_call_id: None,
    };

    let mesg = Message {
        role: "user".to_string(),
        content: ContentEnum::AString("How much is 23 minus 2".to_string()),
        images: None,
        tool_calls: Some(vec![]),
        tool_call_id: None,
    };

    let request = ChatRequest {
        model: model.name.to_string(),
        prompt: None,
        stream: false,
        options: None,
        messages: Some(vec![mesg]),
        format: Some("json".to_string()),
        tools: Some(vec![tool.clone(), tool_sub.clone()]),
    };

    let request_pretty = serde_json::to_string_pretty(&request).expect("should be a json");

    println!("request: \n{}\n", request_pretty);

    match o.chat(&request).await {
        Ok(res) => {
            let response_pretty =
                serde_json::to_string_pretty(&res).expect("should be a json response");
            println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n");
            println!("response: \n {:?}\n", response_pretty);
            println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\n");
            let mut msg_responses = vec![];

            if res.message.is_some() {
                let msg = res.message.unwrap();
                let new_req_msg = Message {
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                    images: None,
                    tool_calls: msg.tool_calls.clone(),
                    tool_call_id: msg.tool_call_id.clone(),
                };

                println!("new_req_msg {:?}", new_req_msg);
                msg_responses.push(new_req_msg);

                if msg.tool_calls.is_some() {
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
                }
            }

            let new_request = ChatRequest {
                model: model.name.to_string(),
                prompt: None,
                stream: false,
                options: None,
                messages: Some(msg_responses),
                format: Some("json".to_string()),
                tools: Some(vec![tool.clone(), tool_sub.clone()]),
            };

            println!("------------------------------------------------------------------------------------------------------");
            println!("new_request {:?}", new_request);
            println!("------------------------------------------------------------------------------------------------------");

            match o.chat(&new_request).await {
                Ok(new_res) => {
                    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                    println!("new_response {:?}", new_res);
                    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                    println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                }
                Err(e) => {
                    println!("new error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    if loaded_models.len() > 0 {
        let model = loaded_models.first().expect("no model    found    ");
        let model = &model.name;
        o.unload(model).await?;
    }

    Ok(())
}
