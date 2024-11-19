# Slack Service

> This service is responsible for sending/receiving messages to/from Slack.

![primary](https://github.com/jcuffney/slack-service/actions/workflows/primary.yml/badge.svg)

Rust workspaces are a way to manage multiple Rust projects that are related to each other. This workspace contains the following projects:
- `events/on_message_send` to subscribe to platform events and send messages to Slack.
- `http/webhook` to receive incoming webhooks from Slack.
- `lib/slack` library to share code between these crates.

`cargo run --bin <webhook|on_message_send>`