# Podcast API Rust Library

[![Rust](https://github.com/ListenNotes/podcast-api-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/ListenNotes/podcast-api-rust/actions/workflows/rust.yml) [![Crates.io](https://img.shields.io/crates/v/podcast-api)](https://crates.io/crates/podcast-api)

The official Rust library for the [Listen Notes Podcast API](https://www.listennotes.com/api/).
Search podcasts and episodes, fetch metadata, and manage playlists.
Questions: [hello@listennotes.com](mailto:hello@listennotes.com).

## Installation

Requires Rust 1.88+ and a Tokio runtime. Cargo manages dependencies, builds, tests,
and publishing; no separate package manager is needed.

```toml
[dependencies]
podcast-api = "3.1.0"
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Usage

Path identifiers are separate positional string arguments. Other parameters use
`serde_json::json!`. `Client::new(None)` uses the stateless public mock API;
`Client::new(Some(key))` uses production with your
[Listen API key](https://www.listennotes.com/api/dashboard/#apps).

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_podcast_by_id(
        "4d3fe717742d4963a85562e9f84d8c79", &json!({"sort": "recent_first"}),
    ).await?;
    println!("Status: {}", response.response.status());
    println!("Headers: {:?}", response.response.headers());
    println!("{}", response.json().await?);
    Ok(())
}
```

Reuse a client for connection pooling. Clients keep their own API keys and
configuration. Defaults are a 30-second total timeout, 10-second connection timeout,
and no redirects or automatic retries. Use `Client::http_client_builder()` and
`Client::new_custom()` to adjust the HTTP configuration. Supplying an independently
built reqwest client also supplies its redirect, retry, proxy, and timeout policies.
`with_base_url()` supports a local test server; it sends the client's credentials
to that URL, so only select a server you control.

### Handling errors and upgrading from 1.x

3.0.0 uses Rust 2024 and reqwest 0.13, with Rust 1.88 as the minimum compiler.
Existing method names, positional identifiers, JSON parameters, and the
`Response { response, request }` wrapper remain. New playlist write methods use
the same conventions, including two positional identifiers for item operations.

HTTP error variants now carry response context: match `Error::NotFoundError(_)`
instead of `Error::NotFoundError`. The enum is non-exhaustive; include a fallback
arm. HTTP 403 is `PermissionDeniedError`; all non-2xx responses are errors.
`error.api_error()` exposes status, headers, and the unmodified body. Displaying an
error includes server details. Connection errors preserve their original cause.

Empty descriptions and notes are sent as empty strings; omitted and null fields
are skipped. Path identifiers, query strings, and form bodies are URL-encoded.
The default User-Agent is `podcast-api-rust <version>`.

## Development

```sh
cargo test --locked
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo doc --locked --no-deps
cargo run --locked --example sample
cargo package --locked
```

`cargo test` uses a loopback HTTP fixture, never production or the public mock.
README examples compile as doctests without executing requests. The sample is a
separate, explicit read-only call to the public mock.

Run the opt-in network suite separately:

```sh
cargo test --locked --test mock_integration -- --ignored --test-threads=1
```

It only calls the public mock API without credentials, with proxies, redirects,
and retries disabled. The mock is stateless; these checks do not prove write
persistence or production authorization. CI tests the minimum and latest stable
compiler, and runs mock integration separately.

Commit `Cargo.lock` to reproduce development and CI. Library consumers resolve
compatible dependencies from `Cargo.toml`. Generated methods, the contract,
method dispatch used by tests, and the marked README sections come from the
Listen Notes monorepo's `sync.py rust`; do not edit generated sections directly.
The crate builds and tests independently of that repository. Publication to
crates.io and GitHub release creation are separate review steps.

## Method index

<!-- BEGIN GENERATED METHOD INDEX -->

- [`search`](#search) — `GET /search`
- [`typeahead`](#typeahead) — `GET /typeahead`
- [`search_episode_titles`](#search_episode_titles) — `GET /search_episode_titles`
- [`spellcheck`](#spellcheck) — `GET /spellcheck`
- [`fetch_related_searches`](#fetch_related_searches) — `GET /related_searches`
- [`fetch_trending_searches`](#fetch_trending_searches) — `GET /trending_searches`
- [`fetch_best_podcasts`](#fetch_best_podcasts) — `GET /best_podcasts`
- [`fetch_podcast_by_id`](#fetch_podcast_by_id) — `GET /podcasts/{id}`
- [`delete_podcast`](#delete_podcast) — `DELETE /podcasts/{id}`
- [`fetch_episode_by_id`](#fetch_episode_by_id) — `GET /episodes/{id}`
- [`batch_fetch_episodes`](#batch_fetch_episodes) — `POST /episodes`
- [`batch_fetch_podcasts`](#batch_fetch_podcasts) — `POST /podcasts`
- [`fetch_curated_podcasts_list_by_id`](#fetch_curated_podcasts_list_by_id) — `GET /curated_podcasts/{id}`
- [`fetch_podcast_genres`](#fetch_podcast_genres) — `GET /genres`
- [`fetch_podcast_regions`](#fetch_podcast_regions) — `GET /regions`
- [`fetch_podcast_languages`](#fetch_podcast_languages) — `GET /languages`
- [`just_listen`](#just_listen) — `GET /just_listen`
- [`fetch_curated_podcasts_lists`](#fetch_curated_podcasts_lists) — `GET /curated_podcasts`
- [`fetch_recommendations_for_podcast`](#fetch_recommendations_for_podcast) — `GET /podcasts/{id}/recommendations`
- [`fetch_recommendations_for_episode`](#fetch_recommendations_for_episode) — `GET /episodes/{id}/recommendations`
- [`submit_podcast`](#submit_podcast) — `POST /podcasts/submit`
- [`fetch_playlist_by_id`](#fetch_playlist_by_id) — `GET /playlists/{id}`
- [`fetch_my_playlists`](#fetch_my_playlists) — `GET /playlists`
- [`fetch_audience_for_podcast`](#fetch_audience_for_podcast) — `GET /podcasts/{id}/audience`
- [`fetch_podcasts_by_domain`](#fetch_podcasts_by_domain) — `GET /podcasts/domains/{domain_name}`
- [`create_playlist`](#create_playlist) — `POST /playlists`
- [`update_playlist`](#update_playlist) — `PUT /playlists/{id}`
- [`delete_playlist`](#delete_playlist) — `DELETE /playlists/{id}`
- [`add_playlist_item`](#add_playlist_item) — `POST /playlists/{id}/items`
- [`delete_playlist_item`](#delete_playlist_item) — `DELETE /playlists/{id}/items/{item_id}`
- [`update_playlist_item_notes`](#update_playlist_item_notes) — `PUT /playlists/{id}/items/{item_id}`

<!-- END GENERATED METHOD INDEX -->

## API reference

<!-- BEGIN GENERATED API REFERENCE -->

Methods accept positional path identifiers followed by `&serde_json::Value` parameters and return `Result<Response>`. Examples use the public mock server. Pass an API key to `Client::new(Some(key))` for real requests.

### search

Full-text search

`GET /search`

Full-text search on episodes, podcasts, or curated lists of podcasts.
Use the `offset` parameter to paginate through search results.
The FREE plan allows to see up to 30 search results (or `offset` < 30) per query.
The PRO plan allows to see up to 300 search results (or `offset` < 300) per query.
The ENTERPRISE plan allows to see up to 10,000 search results (or `offset` < 10000) per query.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.search(&json!({"q": "star wars", "sort_by_date": 0, "type": "episode", "offset": 0, "len_min": 10, "len_max": 30, "genre_ids": "68,82", "published_before": 1580172454000i64, "published_after": 0, "only_in": "title,description", "language": "English", "region": "", "safe_mode": 0, "unique_podcasts": 0, "interviews_only": 0, "sponsored_only": 0, "page_size": 10})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-search)

### typeahead

Typeahead search

`GET /typeahead`

Suggest search terms, podcast genres, and podcasts.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.typeahead(&json!({"q": "star wars", "show_podcasts": 1, "show_genres": 1, "safe_mode": 0})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-typeahead)

### search_episode_titles

Find individual episodes by searching for their titles

`GET /search_episode_titles`

Conduct targeted searches for individual episodes by title and refine results using the podcast id such as
Listen Notes Podcast ID, Apple Podcasts ID, Spotify ID, or RSS feed URL.
This endpoint is specially designed to streamline the import of specific episodes from platforms
like Apple Podcasts and Spotify into your application.
Compared to the GET /search endpoint, which performs full-text searches across multiple fields,
this endpoint focuses solely on episode titles for enhanced accuracy and performance.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.search_episode_titles(&json!({"q": "Jerusalem Demsas on The Dispossessed"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-search_episode_titles)

### spellcheck

Spell check on a search term

`GET /spellcheck`

Suggest a list of words that correct the spelling errors of a search term. This endpoint is available only in the PRO/ENTERPRISE plan.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.spellcheck(&json!({"q": "microsft stock"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-spellcheck)

### fetch_related_searches

Fetch related search terms

`GET /related_searches`

Suggest related search terms. The results are more comprehensive than from `GET /typeahead`. This endpoint is available only in the PRO/ENTERPRISE plan.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_related_searches(&json!({"q": "evergrande"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-related_searches)

### fetch_trending_searches

Fetch trending search terms

`GET /trending_searches`

Fetch up to 10 most recent trending search terms on the Listen Notes platform.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_trending_searches(&json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-trending_searches)

### fetch_best_podcasts

Fetch a list of best podcasts by genre

`GET /best_podcasts`

Get a list of curated best podcasts by genre,
which are curated by Listen Notes staffs based on various signals from the Internet, e.g.,
top charts on other podcast platforms, recommendations from mainstream media,
user activities on listennotes.com...
You can get the genre ids from `GET /genres` endpoint.
This endpoint returns same data as https://www.listennotes.com/best-podcasts/

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_best_podcasts(&json!({"genre_id": 93, "page": 2, "region": "us", "sort": "listen_score", "safe_mode": 0})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-best_podcasts)

### fetch_podcast_by_id

Fetch detailed meta data and episodes for a podcast by id

`GET /podcasts/{id}`

Fetch detailed meta data and episodes for a specific podcast (up to 10 episodes each time).
You can use the **next_episode_pub_date** parameter to do pagination and fetch more episodes.
During pagination with **next_episode_pub_date**, an empty **episodes** array in the response signals that no more episodes are available.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_podcast_by_id("4d3fe717742d4963a85562e9f84d8c79", &json!({"next_episode_pub_date": 1479154463000i64, "sort": "recent_first"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-podcasts-id)

### delete_podcast

Request to delete a podcast

`DELETE /podcasts/{id}`

Podcast hosting services can use this endpoint to streamline the process of podcast deletion on behave of their users (podcasters). We will review the deletion request within 12 hours. If the podcast is already deleted, the "status" field in the response will be "deleted". Otherwise, the status field will be "in review". If you want to get a notification once the podcast is deleted, you can configure a webhook url in the dashboard: listennotes.com/api/dashboard/#webhooks

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.delete_podcast("4d3fe717742d4963a85562e9f84d8c79", &json!({"reason": "the podcaster wants to delete it"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#delete-api-v2-podcasts-id)

### fetch_episode_by_id

Fetch detailed meta data for an episode by id

`GET /episodes/{id}`

Fetch detailed meta data for a specific episode.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_episode_by_id("6b6d65930c5a4f71b254465871fed370", &json!({"show_transcript": 1})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-episodes-id)

### batch_fetch_episodes

Batch fetch basic meta data for episodes

`POST /episodes`

Batch fetch basic meta data for up to 10 episodes. This endpoint could be used to implement custom playlists for individual episodes. For detailed meta data of an individual episode, you need to use `GET /episodes/{id}`. This endpoint is available only in the PRO/ENTERPRISE plan.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.batch_fetch_episodes(&json!({"ids": "c577d55b2b2b483c969fae3ceb58e362,0f34a9099579490993eec9e8c8cebb82"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#post-api-v2-episodes)

### batch_fetch_podcasts

Batch fetch basic meta data for podcasts

`POST /podcasts`

Batch fetch basic meta data for up to 10 podcasts.
This endpoint could be used to build something like OPML import,
allowing users to import a bunch of podcasts via rss urls.
For detailed meta data (including episodes) of an individual podcast, you need to use `GET /podcasts/{id}`. This endpoint is available only in the PRO/ENTERPRISE plan.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.batch_fetch_podcasts(&json!({"ids": "3302bc71139541baa46ecb27dbf6071a,68faf62be97149c280ebcc25178aa731,37589a3e121e40debe4cef3d9638932a,9cf19c590ff0484d97b18b329fed0c6a", "rsses": "https://rss.art19.com/recode-decode,https://rss.art19.com/the-daily,https://www.npr.org/rss/podcast.php?id=510331,https://www.npr.org/rss/podcast.php?id=510331", "itunes_ids": "1457514703,1386234384,659155419", "spotify_ids": "3DDfEsKDIDrTlnPOiG4ZF4,4qDNe5Gvl1XxdLinUGEXrC,23NZCM4ik6o3UYkM473Itz", "show_latest_episodes": 1, "next_episode_pub_date": 1557394247000i64})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#post-api-v2-podcasts)

### fetch_curated_podcasts_list_by_id

Fetch a curated list of podcasts by id

`GET /curated_podcasts/{id}`

Get detailed meta data of all podcasts in a specific curated list.
This endpoint returns same data as https://www.listennotes.com/curated-podcasts/

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_curated_podcasts_list_by_id("SDFKduyJ47r", &json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-curated_podcasts-id)

### fetch_podcast_genres

Fetch a list of podcast genres

`GET /genres`

Get a list of podcast genres that are supported in Listen Notes.
The genre id can be passed to other endpoints as a parameter to get podcasts in a specific genre,
e.g., `GET /best_podcasts`, `GET /search`...
You may want to cache the list of genres on the client side.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_podcast_genres(&json!({"top_level_only": 1})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-genres)

### fetch_podcast_regions

Fetch a list of supported countries/regions for best podcasts

`GET /regions`

It returns a dictionary of country codes (e.g., us, gb...) & country names (United States, United Kingdom...). The country code is used in the query parameter **region** of `GET /best_podcasts`.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_podcast_regions(&json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-regions)

### fetch_podcast_languages

Fetch a list of supported languages for podcasts

`GET /languages`

Get a list of languages that are supported in Listen Notes database. You can use the language string as query parameter in `GET /search`.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_podcast_languages(&json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-languages)

### just_listen

Fetch a random podcast episode

`GET /just_listen`

Recently published episodes are more likely to be fetched. Good luck!

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.just_listen(&json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-just_listen)

### fetch_curated_podcasts_lists

Fetch curated lists of podcasts

`GET /curated_podcasts`

A bunch of curated lists from online media. For each list, you'll get basic info of up to 5 podcasts. To get detailed meta data of all podcasts in a specific list, you need to use `GET /curated_podcasts/{id}`. We add new curated lists to the database on a daily basis.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_curated_podcasts_lists(&json!({"page": 2})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-curated_podcasts)

### fetch_recommendations_for_podcast

Fetch recommendations for a podcast

`GET /podcasts/{id}/recommendations`

Fetch up to 8 podcast recommendations based on the given podcast id.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_recommendations_for_podcast("25212ac3c53240a880dd5032e547047b", &json!({"safe_mode": 0})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-podcasts-id-recommendations)

### fetch_recommendations_for_episode

Fetch recommendations for an episode

`GET /episodes/{id}/recommendations`

Fetch up to 8 episode recommendations based on the given episode id.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_recommendations_for_episode("254444fa6cf64a43a95292a70eb6869b", &json!({"safe_mode": 0})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-episodes-id-recommendations)

### submit_podcast

Submit a podcast to Listen Notes database

`POST /podcasts/submit`

Podcast hosting services can use this endpoint to help your users directly submit a new podcast to Listen Notes database. If the podcast doesn't exist in the database, "status" in the response will be "in review", and we'll review it within 12 hours. If the podcast exists, "status" in the response will be "found". If this submission is rejected, "status" in the response will be "rejected". You can use `POST /podcasts` to check if multiple podcasts exist in the database. If you want to get a notification once the podcast is accepted, you can either specify the "email" parameter or configure a webhook url in the dashboard: listennotes.com/api/dashboard/#webhooks

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.submit_podcast(&json!({"rss": "https://feeds.megaphone.fm/committed", "email": "hello@example.com"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#post-api-v2-podcasts-submit)

### fetch_playlist_by_id

Fetch a playlist's info and items (i.e., episodes or podcasts).

`GET /playlists/{id}`

A playlist can contain both episodes and podcasts, shown in separate views,
just like playlists created via listennotes.com/listen/.
This endpoint fetches items from the saved default view unless **type** is specified.
The response type and listennotes_url describe the selected view.
You can use the **last_pub_date_ms** parameter to do pagination and fetch more items.
A playlist can be **public** (discoverable on ListenNotes.com),
**unlisted** (accessible to anyone who knows the playlist id),
or **private** (accessible when the API admin has active playlist membership).
Public and unlisted playlists can also be fetched by ID regardless of their owner.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_playlist_by_id("m1pe7z60bsw", &json!({"type": "episode_list", "last_timestamp_ms": 0, "sort": "recent_added_first"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-playlists-id)

### fetch_my_playlists

Fetch a list of your playlists.

`GET /playlists`

This endpoint lists playlists with an active membership for the API admin, including playlists they created or joined.
Each playlist includes its saved default **type** and a **listennotes_url** for that view.
You can use the **page** parameter to do pagination and fetch more playlists.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_my_playlists(&json!({"sort": "recent_added_first", "page": 1})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-playlists)

### fetch_audience_for_podcast

Fetch audience demographics for a podcast

`GET /podcasts/{id}/audience`

Fetch audience demographics for a podcast - 1) directly measured on the Listen Notes platform; 2) only supports audience breakdown by regions for now; 3) not every podcast has data.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_audience_for_podcast("25212ac3c53240a880dd5032e547047b", &json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-podcasts-id-audience)

### fetch_podcasts_by_domain

Fetch podcasts by a publisher's domain name

`GET /podcasts/domains/{domain_name}`

Fetch podcasts by a publisher's domain name, e.g., nytimes.com, wondery.com, npr.org...
Each request will return up to 10 podcasts. You can use the `page` parameter to paginate.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.fetch_podcasts_by_domain("nytimes.com", &json!({"page": 1})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#get-api-v2-podcasts-domains-domain_name)

### create_playlist

Create a playlist.

`POST /playlists`

Create an empty playlist owned by the API admin. Name is required; description defaults to an empty string, visibility defaults to public, and type defaults to episode_list. Set type to podcast_list to make podcasts the default view. The response includes the saved type and its listennotes_url.

Only playlists owned by your admin API account can be modified; contributor membership does not grant write access.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.create_playlist(&json!({"name": "My favorite podcasts", "description": "Podcasts and episodes to revisit.", "visibility": "public", "type": "episode_list"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#post-api-v2-playlists)

### update_playlist

Update playlist metadata.

`PUT /playlists/{id}`

Update any subset of name, description, visibility, and type. Omitted fields remain unchanged; at least one field is required. Switching to private rotates the playlist RSS secret. Type selects the saved default view (episode_list or podcast_list) and the returned listennotes_url; changing it preserves all existing episodes and podcasts.

Only playlists owned by your admin API account can be modified; contributor membership does not grant write access.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.update_playlist("m1pe7z60bsw", &json!({"name": "My favorite podcasts", "description": "Podcasts and episodes to revisit.", "visibility": "public", "type": "podcast_list"})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#put-api-v2-playlists-id)

### delete_playlist

Delete a playlist.

`DELETE /playlists/{id}`

Permanently delete a playlist, including all episode and podcast references saved in this specific playlist and their notes. The actual episodes and podcasts remain in the Listen Notes podcast database.

**Warning: Deletion cannot be undone. Once deleted, the playlist is gone, regardless of how many episodes or podcasts it contains. You, the developer, are responsible for adding a confirmation step in your app's UI before calling this endpoint to prevent accidental deletion.**

Only playlists owned by your admin API account can be modified; contributor membership does not grant write access.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.delete_playlist("m1pe7z60bsw", &json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#delete-api-v2-playlists-id)

### add_playlist_item

Add an episode or podcast to a playlist.

`POST /playlists/{id}/items`

Provide exactly one non-empty episode_id or podcast_id; an empty unused ID field is ignored. Invalid ID formats return 400 and identify the field. A missing episode or podcast returns 404 with an error such as "Episode not found: {episode_id}." or "Podcast not found: {podcast_id}.". Existing active items are reused (200); new or restored items return 201. Omitted notes preserve existing notes, including when restoring a deleted item; supplied notes replace them.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.add_playlist_item("m1pe7z60bsw", &json!({"episode_id": "e53e6992a5b7492f9ea6fcd85d9ad95f", "notes": "Worth a listen."})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#post-api-v2-playlists-id-items)

### delete_playlist_item

Remove an item from a playlist.

`DELETE /playlists/{id}/items/{item_id}`

Delete a playlist item. Repeating deletion of the same item succeeds. This does not delete the episode or podcast from the podcast database.

Only playlists owned by your admin API account can be modified; contributor membership does not grant write access.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.delete_playlist_item("m1pe7z60bsw", "23", &json!({})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#delete-api-v2-playlists-id-items-item_id)

### update_playlist_item_notes

Update notes for a playlist item.

`PUT /playlists/{id}/items/{item_id}`

Replace item notes, or send an empty string to clear them. The item ID and added_at_ms remain unchanged.

Only playlists owned by your admin API account can be modified; contributor membership does not grant write access.

```rust,no_run
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = podcast_api::Client::new(None);
    let response = client.update_playlist_item_notes("m1pe7z60bsw", "23", &json!({"notes": ""})).await?;
    println!("{}", response.json().await?);
    Ok(())
}
```

[Full API documentation](https://www.listennotes.com/api/docs/#put-api-v2-playlists-id-items-item_id)

<!-- END GENERATED API REFERENCE -->
