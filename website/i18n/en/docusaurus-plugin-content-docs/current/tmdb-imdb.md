---
sidebar_position: 7
---

# TMDB and IMDB support 

This documentation provides information on how to enable and use TMDB and IMDB support in Ygégé.

## Enabling TMDB and IMDB Support

To enable TMDB and IMDB support, you need to set the appropriate environment variable `TMDB_TOKEN` with your TMDB API token *(the jwt one)* or add it to your `config.json` file.

## Why do I need my own TMDB token?

Ygégé does not ship with a built-in TMDB token to avoid potential abuse and rate-limiting issues. By using your own token, you ensure that your usage is tracked under your account, allowing for better reliability and access to TMDB services.

## What about IMDB?

IMDB support is managed via TMDB, so by enabling TMDB, you will also have access to IMDB information when available.

## Obtaining a TMDB API Token

1. Create an account on [TMDB](https://www.themoviedb.org/signup/).
2. Log in to your TMDB account.
3. Create your API token by following the instructions on the [TMDB API documentation](https://developers.themoviedb.org/3/getting-started/introduction).
4. Fill the form to request an API key, providing necessary details about your application.
5. You should immediately receive your API key, but sometimes it may take a few minutes to be approved.
6. Once you have your API key, you can use it to enable TMDB support in Ygégé.