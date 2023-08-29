# Toot Worker

Toot Worker is a Cloudflare Workers application that enables posting to Mastodon using a Webhook-like mechanism.

## Description

Toot Worker is a lightweight and efficient application built with Rust and Cloudflare Workers. It provides a simple and secure way to post content directly to your Mastodon instance using a POST request.

## Features

- **Webhook-like functionality**: Toot Worker acts as a Webhook, allowing you to send POST requests with the desired content to a specified URL, which will then be posted to your Mastodon instance.
- **Cloudflare Workers**: Powered by Cloudflare Workers, Toot Worker ensures scalability, reliability, and low-latency performance.
- **Rust implementation**: Toot Worker is implemented in Rust, a fast and memory-safe programming language, ensuring high performance and security.
- **Post-only functionality**: Toot Worker focuses solely on the posting feature, making it lightweight and straightforward.

## Setup

1. Click the button below and follow the on-screen instructions to proceed with the setup.

   [![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/bridge-y/toot-worker)

2. On the Cloudflare Workers dashboard, register the following secrets as variables.

   - `BASE_URL`: The URL of your Mastodon instance (e.g. https://mstdn.example.com).
   - `TOOT_PATH`: A randomly generated value.
   - `TOKEN`: The access token of your mastodon account.  
      Please refer to this page: [Obtaining client app access - Mastodon documentation](https://docs.joinmastodon.org/client/token/)

## Usage

```bash
curl -X POST -d '{"text": "test"}' -H "Content-Type: application/json" https://<your worker domain>/<TOOT_PATH>
```

## License

This project is licensed under the MIT License.

## Disclaimer

Toot Worker is a third-party application and is not affiliated with Mastodon or Cloudflare. Use it at your own risk.
