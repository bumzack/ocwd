// @generated automatically by Diesel CLI.

diesel::table! {
    ollama_chat (id) {
        id -> Int4,
        model_id -> Int4,
        prompt_id -> Int4,
        parent_id -> Nullable<Int4>,
        response -> Text,
        ollama_response_json -> Jsonb,
        ollama_request_json -> Jsonb,
        num_ctx -> Int8,
        seed -> Int8,
        temperature -> Float8,
        top_k -> Float8,
        top_p -> Float8,
        duration_ms -> Int8,
        #[max_length = 500]
        result -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    ollama_chat_queue (id) {
        id -> Int4,
        model_id -> Int4,
        prompt_id -> Int4,
        #[max_length = 100]
        state -> Varchar,
        num_ctx -> Int8,
        temperature -> Float8,
        seed -> Int8,
        top_k -> Float8,
        top_p -> Float8,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    ollama_model (id) {
        id -> Int4,
        #[max_length = 1000]
        name -> Varchar,
        #[max_length = 1000]
        model -> Varchar,
        size -> Int8,
        #[max_length = 1000]
        detail_format -> Varchar,
        #[max_length = 1000]
        detail_family -> Varchar,
        #[max_length = 1000]
        detail_parameter_size -> Varchar,
        #[max_length = 1000]
        detail_quantization_level -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    ollama_prompt (id) {
        id -> Int4,
        prompt -> Text,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::joinable!(ollama_chat -> ollama_model (model_id));
diesel::joinable!(ollama_chat -> ollama_prompt (prompt_id));
diesel::joinable!(ollama_chat_queue -> ollama_model (model_id));
diesel::joinable!(ollama_chat_queue -> ollama_prompt (prompt_id));

diesel::allow_tables_to_appear_in_same_query!(
    ollama_chat,
    ollama_chat_queue,
    ollama_model,
    ollama_prompt,
);
