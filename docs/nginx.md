# Nginx configs for Ubuntu

Use these as an example how to configure Nginx on Ubuntu to serve Ollama, the backend and the frontend from
a desktop machine in your local network.

## Ollama

see [Ollama FAQ](https://github.com/ollama/ollama/blob/main/docs/faq.md) for configuring Ollama to
listen on 0.0.0.0 instead of 127.0.0.1 or localhost, which might be helpful.

assuming your desktop PC has the IP 10.0.0.48 assigned:

```
server {
        listen 11433;
        server_name 10.0.0.48:11433;

        location / {
                proxy_pass http://localhost:11434;
                proxy_set_header Host localhost:11434;
        }

        proxy_connect_timeout 60000s;
        proxy_send_timeout 600000s;
        proxy_read_timeout 60000s;
        send_timeout 60000s;
}
```

## backend

```
server {
        listen 3022;
        server_name 10.0.0.48;

        location / {
                proxy_pass http://127.0.0.1:3023;
                proxy_set_header Host 127.0.0.1:3023;
        }
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
        send_timeout 60s;
}
```

## frontend SvelteKit dev port 5173

The dev frontend will be accessible on http://10.0.0.48:5172

```
server {
        listen 5172;
        server_name 10.0.0.48;

        location / {
                proxy_pass http://localhost:5173;
                proxy_set_header Host localhost:5173;
        }
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
        send_timeout 60s;
}
```

## frontend SvelteKit preview port 4173

The preview frontend will be accessible on http://10.0.0.48

```
server {
        listen 80;
        server_name 10.0.0.48;


        location / {
                proxy_pass http://localhost:4173;
                proxy_set_header Host localhost:4173;
        }
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
        send_timeout 60s;
}
```
