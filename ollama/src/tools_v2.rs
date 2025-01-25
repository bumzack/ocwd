use crate::models::{Function, Parameter, Property, Tool};
use std::collections::HashMap;

pub fn get_tools_v2() -> Vec<Tool> {
    vec![
        get_weather(),
        get_sub_numbers(),
        get_stable_diffusion(),
        get_wuerstchen(),
        get_psql(),
        get_txt2img(),
    ]
}

fn get_txt2img() -> Tool {
    let property_prompt = Property {
        typ: "string".to_string(),
        description: "The prompt which will be used as input to the txt2img model.".to_string(),
        enums: None,
    };

    let property_model = Property {
        typ: "string".to_string(),
        description: "The model that should be used to create the image.".to_string(),
        enums: Some(vec![
            "wuerstchen".to_string(),
            "stable_diffusion_large".to_string(),
            "stable_diffusion_medium".to_string(),
            "kandinsky".to_string(),
        ]),
    };

    let property_size = Property {
        typ: "number".to_string(),
        description: "The width and height of the image that should be created in pixel."
            .to_string(),
        enums: None,
    };

    let mut properties = HashMap::new();
    properties.insert("prompt".to_string(), property_prompt);
    properties.insert("model".to_string(), property_model);
    properties.insert("size".to_string(), property_size);

    let parameters = Parameter {
        typ: "object".to_string(),
        properties,
        required: Some(vec!["prompt".to_string(), "prompt".to_string()]),
    };

    let function = Function {
        name: "get_image".to_string(),
        description: "I can create an image from a prompt using a txt2img diffusion model. The name of the model should be provided \
        in the 'model' parameter. The prompt must be provided in the 'prompt' parameter. The return value is a URL where the image \
        can be retrieved. The width of the quadratic image which should be generated can be specified using the 'size' \
        parameter".to_string(),
        parameters,
    };

    let tool = Tool {
        typ: "function".to_string(),
        function,
    };

    tool
}

fn get_stable_diffusion() -> Tool {
    let property_prompt = Property {
        typ: "string".to_string(),
        description: "The prompt which will be used as input to the txt2img model.".to_string(),
        enums: None,
    };

    let mut properties = HashMap::new();
    properties.insert("prompt".to_string(), property_prompt);

    let parameters = Parameter {
        typ: "object".to_string(),
        properties,
        required: Some(vec!["prompt".to_string()]),
    };

    let function = Function {
        name: "get_image_from_txt2img_stable_diffusion".to_string(),
        description: "I can create an image from a prompt using a txt2img diffusion model. The model is the stable diffusion large model. It takes about 30 seconds to generate an image. As a return value I will provide a URL where the image can be viewed.".to_string(),
        parameters,
    };

    let tool = Tool {
        typ: "function".to_string(),
        function,
    };

    tool
}

fn get_psql() -> Tool {
    let property_prompt = Property {
        typ: "string".to_string(),
        description: "The SQL query that will be executed on the database server.".to_string(),
        enums: None,
    };

    let property_secret = Property {
        typ: "string".to_string(),
        description: "The 'secret' the user has to provide to be allowed to access the data ."
            .to_string(),
        enums: None,
    };

    let mut properties = HashMap::new();
    properties.insert("prompt".to_string(), property_prompt);
    properties.insert("secret".to_string(), property_secret);

    let parameters = Parameter {
        typ: "object".to_string(),
        properties,
        required: Some(vec!["prompt".to_string(), "secret".to_string()]),
    };

    let function = Function {
        name: "get_postgres_access_db".to_string(),
        description: "I have access to a postgres server. I can execute SQL queries on a database. If the user wants \
        access to the data from the database the user has to provide a secret for security reasons. The return \
        value will be in JSON format. I can provide a list of tables I have access to, I can provide \
        a list of databases I have access to and I can provide detailed information about the structure of the \
        databases and their tables.".to_string(),
        parameters,
    };

    let tool = Tool {
        typ: "function".to_string(),
        function,
    };

    tool
}

fn get_wuerstchen() -> Tool {
    let property_prompt = Property {
        typ: "string".to_string(),
        description: "The prompt which will be used as input to the txt2img model.".to_string(),
        enums: None,
    };

    let mut properties = HashMap::new();
    properties.insert("prompt".to_string(), property_prompt);

    let parameters = Parameter {
        typ: "object".to_string(),
        properties,
        required: Some(vec!["prompt".to_string()]),
    };

    let function = Function {
        name: "get_image_from_txt2img_wuerstchen".to_string(),
        description: "I can create an image from a prompt using a txt2img diffusion model. The model is a named 'wuerstchen' and it takes about 3 minutes to generate an image. As a return value I will provide a URL where the image can be viewed.".to_string(),
        parameters,
    };

    let tool = Tool {
        typ: "function".to_string(),
        function,
    };

    tool
}

fn get_sub_numbers() -> Tool {
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

    tool_sub
}

fn get_weather() -> Tool {
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

    tool
}
