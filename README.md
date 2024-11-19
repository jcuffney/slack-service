# Slack Service

> This service is responsible for sending/receiving messages to/from Slack.

Rust workspaces are a way to manage multiple Rust projects that are related to each other. This workspace contains the following projects:
- `events/on_message_send`
- `http/webhook`

`cargo run --bin <webhook|on_message_send>`