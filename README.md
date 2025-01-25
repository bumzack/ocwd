# OCWD - Ollama Chat With Database

Naming things is hard, finding a good project name is even harder. It started out as a Rust backend with 
the ability to call the Ollama API and store the responses in a postgres database. Now it is a little bit more than that.

> This is under heavy development. Breaking changes are to be expected on a daily basis!!

This repository is a playground to get familiar with Ollama, LLMs in general, tool usage, chat protocol and - as it
seems - the most important part will be writing good (system-/tool-) prompts to make stuff work.

The basic idea is to play around with LLMs and (mostly) [Ollama](https://ollama.com/) but
also [candle](https://github.com/huggingface/candle) to check how good LLMs are at writing code - and also to compare
different LLMs.

The backend and frontend are running on a desktop with a Nvidia RTX 3090 Ti (24G). The backend and frontend ports are
exposed using Nginx as reverse proxy. Ollama is installed on the desktop and on the laptop which is used for developing
stuff. Coding is done mostly using RustRover & CodeGPT extension, some  [Zed](https://github.com/zed-industries/zed) (
for me it still has major limitations compared to RustRover).

One near-future goal is to try implementing tools for the following use cases:

- webshop statistics (e.g. how man orders per quarter, total revenue in given time span). The ```generic-tools``` folder
  contains a ```postgres``` tool, which should be used for this. The strict Rust type system does not allow (well - at
  least me) not to use diesel for querying arbitrary database schemas, that's why ```tokio-postgres``` is used for the
  tool impl.
- generate audio from text within the chat (parlor tool can be used for this). This is going to require to "pause" the
  chat for as long as the GPU is used for the parlor task.
- generate images in chat using either ```wuerstchen``` or ```stable-diffusion``` tool. See
  the [candle example](https://github.com/huggingface/candle/tree/main/candle-examples) repo for the source of the tools
  impl.
- try get something like a "personal assistant" working by adding some more tools. but basically, the goal is to have a
  relatively generic database that can store ToDos, appointments and notes. And be queryable via chat.

## tech-stack

- backend: Rust (axum, diesel, reqwest)
- frontend: SvelteKit (a "new" version of sycamore is available, that would be interesting to try again).
- other stuff used:
  [candle](https://github.com/huggingface/candle) (it would be great to replace all the ollama stuff with candle...)

## what you find here

- ```backend```: Axum web server for a SPA implementing different Ollama APIs to "chat" with a LLM. It should support
  tools in the near future.
- ```docs```: just some notes how to get this stuff to run. Esp. setting up PostgreSQL is a - well - not the most
  straight forward thing to do.
- ```fakewebshopserver```: generates some dummy orders and order items. Just a PoC, that order data can be retrieved
  from an API.
- ```frontend```: the SvelteKit front-end for the chat app. (Oh boy - a streaming response containing JSON objects -
  this can't be the way to do this.)
- ```generic-tools```: rust implementation of some tools. No real dependency to LLM tools. Just a set of functions that
  do stuff.
- ```ollama```: implements the Ollama API. List models, create a model, chat with a model, etc...
- ```webshop```: this is a CLI tool, that reads order data from a (arbitrary) web API (ok - it should provide the
  required endpoints, but other than that) and stores the data in a postgre DB using diesel. This database can then be
  used by the ```postgres``` tool in the ```generic-tools``` crate and the LLMwrites/provides the SQL queries to answer
  the users questions.

