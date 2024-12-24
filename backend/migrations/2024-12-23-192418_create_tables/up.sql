-- Your SQL goes here

CREATE TABLE ollama_model
(
    id                        SERIAL PRIMARY KEY,
    name                      VARCHAR(1000)                                               NOT NULL,
    model                     VARCHAR(1000)                                               NOT NULL,
    size                      BIGINT                                                      NOT NULL,
    detail_format             VARCHAR(1000)                                               NOT NULL,
    detail_family             VARCHAR(1000)                                               NOT NULL,
    detail_parameter_size     VARCHAR(1000)                                               NOT NULL,
    detail_quantization_level VARCHAR(1000)                                               NOT NULL,
    created                   timestamp with time zone default (now() at time zone 'utc') NOT NULL,
    UNIQUE (name, model, size)
);

CREATE TABLE ollama_prompt
(
    id      SERIAL PRIMARY KEY,
    prompt  TEXT                                                  NOT NULL,
    created timestamp
                with time zone default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE ollama_chat
(
    id                   SERIAL PRIMARY KEY,
    model_id             INTEGER REFERENCES ollama_model (id)                        NOT NULL,
    prompt_id            INTEGER REFERENCES ollama_prompt (id)                       NOT NULL,
    parent_id            INTEGER REFERENCES ollama_chat (id),
    response             TEXT                                                        NOT NULL,
    ollama_response_json Jsonb                                                       NOT NULL,
    ollama_request_json  Jsonb                                                       NOT NULL,
    num_ctx              BIGINT,
    seed                 BIGINT,
    temperature          DOUBLE PRECISION,
    top_k                DOUBLE PRECISION,
    top_p                DOUBLE PRECISION,
    duration_ms                 BIGINT NOT NULL,
    created              timestamp with time zone default (now() at time zone 'utc') NOT NULL
);


-- Your SQL goes here

CREATE TABLE ollama_chat_queue
(
    id          SERIAL PRIMARY KEY,
    model_id    INTEGER REFERENCES ollama_model (id)                        NOT NULL,
    prompt_id   INTEGER REFERENCES ollama_prompt (id)                       NOT NULL,
    state       VARCHAR(100)                                                NOT NULL,
    num_ctx     BIGINT,
    temperature DOUBLE PRECISION,
    seed        BIGINT,
    top_k       DOUBLE PRECISION,
    top_p       DOUBLE PRECISION,
    created     timestamp with time zone default (now() at time zone 'utc') NOT NULL
);
