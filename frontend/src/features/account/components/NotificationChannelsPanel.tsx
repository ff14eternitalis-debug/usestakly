import { Button } from "../../../components/Button";
import { useT } from "../../../i18n";
import { formatRelative } from "../../../lib/format";
import type { DigestTimePreset, NotificationChannelSummary } from "../../../lib/types";

type NotificationChannelsPanelProps = {
  loading: boolean;
  channels: NotificationChannelSummary[];
  email: string;
  webhookUrl: string;
  emailCritical: boolean;
  emailDigest: boolean;
  webhookCritical: boolean;
  webhookDigest: boolean;
  digestTimePreset: DigestTimePreset;
  savingEmail: boolean;
  savingWebhook: boolean;
  savingPreferences: boolean;
  deleting: boolean;
  testingId: string | null;
  message: string | null;
  error: string | null;
  onEmailChange(value: string): void;
  onWebhookUrlChange(value: string): void;
  onEmailCriticalChange(value: boolean): void;
  onEmailDigestChange(value: boolean): void;
  onWebhookCriticalChange(value: boolean): void;
  onWebhookDigestChange(value: boolean): void;
  onDigestTimePresetChange(value: DigestTimePreset): void;
  onSaveEmail(): void;
  onSaveWebhook(): void;
  onSavePreferences(): void;
  onDelete(id: string): void;
  onTest(id: string): void;
};

export function NotificationChannelsPanel({
  loading,
  channels,
  email,
  webhookUrl,
  emailCritical,
  emailDigest,
  webhookCritical,
  webhookDigest,
  digestTimePreset,
  savingEmail,
  savingWebhook,
  savingPreferences,
  deleting,
  testingId,
  message,
  error,
  onEmailChange,
  onWebhookUrlChange,
  onEmailCriticalChange,
  onEmailDigestChange,
  onWebhookCriticalChange,
  onWebhookDigestChange,
  onDigestTimePresetChange,
  onSaveEmail,
  onSaveWebhook,
  onSavePreferences,
  onDelete,
  onTest
}: NotificationChannelsPanelProps) {
  const t = useT();
  const emailChannel = channels.find((channel) => channel.channelType === "email");
  const webhookChannel = channels.find(
    (channel) => channel.channelType === "discord_webhook"
  );

  return (
    <section className="surface grid gap-5 p-5">
      <div className="grid gap-1.5">
        <span className="kicker">{t.account.notificationChannelsEyebrow}</span>
        <h2 className="text-[1.08rem] font-semibold text-fg">
          {t.account.notificationChannelsTitle}
        </h2>
        <p className="max-w-[72ch] text-[0.9rem] leading-relaxed text-fg-dim">
          {t.account.notificationChannelsIntro}
        </p>
      </div>

      {loading ? (
        <div className="py-8 text-center">
          <span className="kicker">{t.common.tuning}</span>
        </div>
      ) : (
        <div className="grid gap-4 md:grid-cols-2">
          <div className="grid gap-4 rounded-[8px] border border-line bg-bg-subtle p-4">
            <div className="grid gap-1">
              <h3 className="text-[0.98rem] font-semibold text-fg">
                {t.account.emailChannelTitle}
              </h3>
              <p className="text-[0.84rem] leading-relaxed text-fg-dim">
                {t.account.emailChannelBody}
              </p>
            </div>
            <label className="grid gap-1.5">
              <span className="kicker">{t.account.emailChannelLabel}</span>
              <input
                type="email"
                value={email}
                onChange={(event) => onEmailChange(event.target.value)}
                placeholder={t.account.emailChannelPlaceholder}
                className="input"
              />
            </label>
            <label className="inline-flex items-center gap-2 text-[0.86rem] text-fg-dim">
              <input
                type="checkbox"
                checked={emailCritical}
                onChange={(event) => onEmailCriticalChange(event.target.checked)}
                className="size-4 accent-[var(--color-accent)]"
              />
              {t.account.criticalAlerts}
            </label>
            <label className="inline-flex items-center gap-2 text-[0.86rem] text-fg-dim">
              <input
                type="checkbox"
                checked={emailDigest}
                onChange={(event) => onEmailDigestChange(event.target.checked)}
                className="size-4 accent-[var(--color-accent)]"
              />
              {t.account.dailyDigest}
            </label>
            {emailChannel ? (
              <ChannelMeta
                channel={emailChannel}
                testable
                deleting={deleting}
                testingId={testingId}
                onDelete={onDelete}
                onTest={onTest}
              />
            ) : null}
            <Button
              type="button"
              variant="outline"
              onClick={onSaveEmail}
              disabled={savingEmail || !email.trim()}
            >
              {savingEmail ? t.account.savingChannel : t.account.saveEmailChannel}
            </Button>
          </div>

          <div className="grid gap-4 rounded-[8px] border border-line bg-bg-subtle p-4">
            <div className="grid gap-1">
              <h3 className="text-[0.98rem] font-semibold text-fg">
                {t.account.discordChannelTitle}
              </h3>
              <p className="text-[0.84rem] leading-relaxed text-fg-dim">
                {t.account.discordChannelBody}
              </p>
            </div>
            <label className="grid gap-1.5">
              <span className="kicker">{t.account.discordWebhookLabel}</span>
              <input
                type="password"
                value={webhookUrl}
                onChange={(event) => onWebhookUrlChange(event.target.value)}
                placeholder={t.account.discordWebhookPlaceholder}
                className="input"
              />
            </label>
            <label className="inline-flex items-center gap-2 text-[0.86rem] text-fg-dim">
              <input
                type="checkbox"
                checked={webhookCritical}
                onChange={(event) => onWebhookCriticalChange(event.target.checked)}
                className="size-4 accent-[var(--color-accent)]"
              />
              {t.account.criticalAlerts}
            </label>
            <label className="inline-flex items-center gap-2 text-[0.86rem] text-fg-dim">
              <input
                type="checkbox"
                checked={webhookDigest}
                onChange={(event) => onWebhookDigestChange(event.target.checked)}
                className="size-4 accent-[var(--color-accent)]"
              />
              {t.account.dailyDigest}
            </label>
            {webhookChannel ? (
              <ChannelMeta
                channel={webhookChannel}
                testable
                deleting={deleting}
                testingId={testingId}
                onDelete={onDelete}
                onTest={onTest}
              />
            ) : null}
            <Button
              type="button"
              variant="outline"
              onClick={onSaveWebhook}
              disabled={savingWebhook || (!webhookUrl.trim() && !webhookChannel)}
            >
              {savingWebhook ? t.account.savingChannel : t.account.saveWebhookChannel}
            </Button>
          </div>
        </div>
      )}

      <div className="grid gap-4 rounded-[8px] border border-line bg-bg-subtle p-4 md:grid-cols-[1fr_auto] md:items-end">
        <label className="grid gap-1.5">
          <span className="kicker">{t.account.digestTimeLabel}</span>
          <select
            className="input"
            value={digestTimePreset}
            onChange={(event) =>
              onDigestTimePresetChange(event.target.value as DigestTimePreset)
            }
          >
            <option value="morning">{t.account.digestMorning}</option>
            <option value="noon">{t.account.digestNoon}</option>
            <option value="evening">{t.account.digestEvening}</option>
            <option value="night">{t.account.digestNight}</option>
          </select>
        </label>
        <Button
          type="button"
          variant="outline"
          onClick={onSavePreferences}
          disabled={savingPreferences}
        >
          {savingPreferences
            ? t.account.savingChannel
            : t.account.saveNotificationPreferences}
        </Button>
      </div>

      {message ? <p className="text-[0.86rem] text-accent">{message}</p> : null}
      {error ? (
        <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
          {error}
        </p>
      ) : null}
    </section>
  );
}

function ChannelMeta({
  channel,
  testable,
  deleting,
  testingId,
  onDelete,
  onTest
}: {
  channel: NotificationChannelSummary;
  testable: boolean;
  deleting: boolean;
  testingId: string | null;
  onDelete(id: string): void;
  onTest(id: string): void;
}) {
  const t = useT();
  return (
    <div className="grid gap-3 rounded-[6px] border border-line bg-surface/60 p-3">
      <div className="grid gap-1">
        <span className="mono text-[0.8rem] text-fg">{channel.destination}</span>
        <span className="text-[0.78rem] text-fg-muted">
          {t.account.createdAt} {formatRelative(channel.createdAt)}
          {channel.lastTestedAt
            ? ` · ${t.account.lastTested} ${formatRelative(channel.lastTestedAt)}`
            : ""}
        </span>
        {channel.lastError ? (
          <span className="text-[0.78rem]" style={{ color: "var(--color-danger)" }}>
            {channel.lastError}
          </span>
        ) : null}
      </div>
      <div className="flex flex-wrap gap-2">
        {testable ? (
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onClick={() => onTest(channel.id)}
            disabled={testingId === channel.id}
          >
            {testingId === channel.id
              ? t.account.testingChannel
              : t.account.testChannel}
          </Button>
        ) : null}
        <Button
          type="button"
          variant="danger"
          size="sm"
          onClick={() => onDelete(channel.id)}
          disabled={deleting}
        >
          {deleting ? t.account.deletingChannel : t.account.deleteChannel}
        </Button>
      </div>
    </div>
  );
}
