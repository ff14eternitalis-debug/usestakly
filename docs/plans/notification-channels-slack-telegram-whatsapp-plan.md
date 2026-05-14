# Notification channels: Slack, Telegram, WhatsApp

Status: proposed
Date: 2026-05-14

## Goal

Extend UseStakly external notifications beyond email and Discord while keeping the beta surface simple, reliable, and low-ops.

Current baseline:

- Email channel: configured from `/account`, SMTP/Brevo delivery, test email, watch alerts, daily digest.
- Discord webhook: configured from `/account`, encrypted secret, test webhook, watch alerts, daily digest.

## Recommendation

Implement channels in this order:

1. Slack webhook
2. Telegram bot
3. WhatsApp later, only if real user demand appears

Slack is the best next step because it is close to the existing Discord webhook model: the user pastes an incoming webhook URL, UseStakly encrypts it, and the backend posts test messages, watch alerts, and digests.

Telegram is useful for personal alerts, but needs a slightly more guided setup because users must provide or discover a chat id.

WhatsApp should stay out of the beta implementation for now. It requires WhatsApp Business Platform, approved templates, stricter opt-in, provider costs, and more legal/ops surface.

## Phase 1: Slack webhook

### Backend

- Add `slack_webhook` to `notification_channel_type`.
- Reuse the existing `notification_channels` table.
- Store the Slack webhook URL encrypted with `APP_NOTIFICATION_SECRET`, like Discord.
- Validate webhook URLs before saving.
- Add Slack delivery functions for:
  - test channel connected
  - critical watch alerts
  - daily digest
- Record delivery failures in `notification_channels.last_error`.

### Frontend

- Add a Slack card in `/account`.
- Fields:
  - Slack webhook URL
  - critical alerts toggle
  - daily digest toggle
- Actions:
  - save webhook
  - send test
  - remove

### Tests

- Validate accepted/rejected Slack webhook URLs.
- Verify URL masking never leaks the secret.
- Verify encrypted webhook roundtrip.
- Verify test payload rendering.
- Verify watch alert delivery path selects `slack_webhook`.
- Verify digest delivery path selects `slack_webhook`.

### Production validation

- Create a Slack test channel and incoming webhook.
- Save it in `/account`.
- Send test.
- Trigger or simulate a watch alert.
- Trigger or wait for a digest.
- Confirm `last_error` stays empty.

## Phase 2: Notification provider refactor

Before or during Slack, split provider-specific delivery code so `notification_channels.rs` does not keep growing.

Suggested shape:

```text
backend/src/services/notification_providers/
  mod.rs
  email.rs
  discord.rs
  slack.rs
  telegram.rs
```

Keep business routing in `notification_channels.rs`, but move provider payload formatting and HTTP/SMTP send logic into provider modules.

Shared message types should represent:

- test channel connected
- watch alert
- daily digest

Each provider can render those messages differently:

- email: HTML + text/plain
- Discord: simple content or embed-like markdown
- Slack: Block Kit or mrkdwn text
- Telegram: safe markdown/plain text

## Phase 3: Telegram

### Backend

- Add `telegram` to `notification_channel_type`.
- Add config env for bot token:
  - `APP_TELEGRAM_BOT_TOKEN`
- Store user destination as chat id.
- Validate chat id format conservatively.
- Add Telegram delivery functions for:
  - test channel connected
  - critical watch alerts
  - daily digest
- Return a clear error when `APP_TELEGRAM_BOT_TOKEN` is missing.

### Frontend

- Add a Telegram card in `/account`.
- Fields:
  - chat id
  - critical alerts toggle
  - daily digest toggle
- Add short setup copy only if needed, preferably not too verbose.

### Tests

- Validate chat id.
- Missing bot token returns a clear error.
- Test payload does not break markdown.
- Watch alert and digest delivery paths select `telegram`.

### Production validation

- Create a UseStakly Telegram bot.
- Save a test chat id.
- Send test.
- Trigger or simulate watch alert and digest.

## Phase 4: WhatsApp parking lot

Do not implement in the beta unless there is strong user demand.

Reasons:

- Requires WhatsApp Business Platform or a provider like Twilio.
- Requires approved message templates for many transactional flows.
- Requires stricter opt-in and unsubscribe handling.
- Adds costs and provider-specific operational risk.
- Less natural for a developer OSS radar than Slack, Discord, email, or Telegram.

If revisited later, first write a dedicated WhatsApp compliance and cost plan.

## Acceptance criteria

Slack can be considered complete when:

- User can save a Slack webhook from `/account`.
- User can send a Slack test message.
- Slack receives a real watch alert.
- Slack receives a daily digest.
- Webhook secret is encrypted and masked.
- Delivery failures are visible through `last_error`.
- Backend tests cover validation, masking, payloads, and routing.

Telegram can be considered complete when:

- User can save a Telegram chat id from `/account`.
- User can send a Telegram test message.
- Telegram receives a real watch alert.
- Telegram receives a daily digest.
- Missing bot token produces a clear actionable error.
- Backend tests cover validation, payloads, and routing.

## Recommended next action

Start with Slack webhook support.

It is the smallest useful extension of the current architecture and gives team users a practical notification channel without taking on the operational weight of WhatsApp.
