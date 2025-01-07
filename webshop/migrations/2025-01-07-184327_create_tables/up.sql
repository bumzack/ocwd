CREATE TABLE orders
(
    id                SERIAL PRIMARY KEY,
    order_number      VARCHAR(1000)                                               NOT NULL,
    buyer_id          VARCHAR(1000)                                               NOT NULL,
    buyer_name        VARCHAR(1000),
    erp_order_number  VARCHAR(1000),
    state             VARCHAR(1000),
    additional_info_1 VARCHAR(1000),
    additional_info_2 VARCHAR(1000),
    number_items      INTEGER                                                     NOT NULL,
    blacklisted       BOOLEAN                  DEFAULT false                      NOT NULL,
    order_created     timestamp with time zone default (now() at time zone 'utc') NOT NULL,
    created           timestamp with time zone default (now() at time zone 'utc') NOT NULL,
    updated           timestamp with time zone default (now() at time zone 'utc') NOT NULL,
    UNIQUE (order_number)
);

CREATE TABLE order_items
(
    id                SERIAL PRIMARY KEY,
    order_number      VARCHAR(1000) REFERENCES orders (order_number)              NOT NULL,
    code              VARCHAR(1000)                                               NOT NULL,
    name              VARCHAR(1000),
    description       VARCHAR(1000),
    price             DOUBLE PRECISION                                            NOT NULL,
    state             VARCHAR(1000),
    additional_info_1 VARCHAR(1000),
    additional_info_2 VARCHAR(1000),
    item_created      timestamp with time zone default (now() at time zone 'utc') NOT NULL,
    created           timestamp with time zone default (now() at time zone 'utc') NOT NULL,
    updated           timestamp with time zone default (now() at time zone 'utc') NOT NULL
);
