import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { ApiError } from "../lib/api-client";
import { getMcpMetrics, getPendingRepoSignals, reviewPendingRepoSignal } from "../lib/api/admin";
import {
  createAgentToken,
  deleteNotificationChannel,
  getAccountSummary,
  getAgentTokens,
  getNotificationChannels,
  getNotificationPreferences,
  revokeAgentToken,
  testNotificationChannel,
  updateNotificationPreferences,
  upsertNotificationChannel
} from "../lib/api/account";
import { useT } from "../i18n";
import type { AgentTokenCreated, DigestTimePreset, McpMetricsWindow } from "../lib/types";
import { useAuthStore } from "../state/auth-store";
import { useLocaleStore } from "../state/locale-store";
import { AccountIdentityCard } from "../features/account/components/AccountIdentityCard";
import { AdminMcpObservabilityPanel } from "../features/account/components/AdminMcpObservabilityPanel";
import { AdminModerationPanel } from "../features/account/components/AdminModerationPanel";
import { AgentTokensPanel } from "../features/account/components/AgentTokensPanel";
import { NotificationChannelsPanel } from "../features/account/components/NotificationChannelsPanel";
import { ReputationCard } from "../features/account/components/ReputationCard";

function detectTimezone(): string {
  return Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
}

export function AccountPage() {
  const t = useT();
  const queryClient = useQueryClient();
  const user = useAuthStore((s) => s.user);
  const locale = useLocaleStore((s) => s.locale);
  const [label, setLabel] = useState("");
  const [created, setCreated] = useState<AgentTokenCreated | null>(null);
  const [copied, setCopied] = useState(false);
  const [adminToken, setAdminToken] = useState("");
  const [mcpWindow, setMcpWindow] = useState<McpMetricsWindow>("7d");
  const [notificationEmail, setNotificationEmail] = useState("");
  const [notificationWebhookUrl, setNotificationWebhookUrl] = useState("");
  const [emailCritical, setEmailCritical] = useState(true);
  const [emailDigest, setEmailDigest] = useState(false);
  const [webhookCritical, setWebhookCritical] = useState(true);
  const [webhookDigest, setWebhookDigest] = useState(false);
  const [digestTimePreset, setDigestTimePreset] = useState<DigestTimePreset>("morning");
  const [channelMessage, setChannelMessage] = useState<string | null>(null);

  const summary = useQuery({
    queryKey: ["account-summary"],
    queryFn: ({ signal }) => getAccountSummary(signal)
  });

  const tokens = useQuery({
    queryKey: ["agent-tokens"],
    queryFn: ({ signal }) => getAgentTokens(signal)
  });

  const notificationChannels = useQuery({
    queryKey: ["notification-channels"],
    queryFn: ({ signal }) => getNotificationChannels(signal)
  });

  const notificationPreferences = useQuery({
    queryKey: ["notification-preferences"],
    queryFn: ({ signal }) => getNotificationPreferences(signal)
  });

  const pendingSignals = useQuery({
    queryKey: ["admin-pending-signals"],
    queryFn: () => getPendingRepoSignals(adminToken),
    enabled: adminToken.trim().length > 0
  });

  const mcpMetrics = useQuery({
    queryKey: ["admin-mcp-metrics", mcpWindow],
    queryFn: () => getMcpMetrics(adminToken, mcpWindow),
    enabled: adminToken.trim().length > 0
  });

  const createToken = useMutation({
    mutationFn: () => createAgentToken(label.trim()),
    onSuccess: async (token) => {
      setCreated(token);
      setCopied(false);
      setLabel("");
      await queryClient.invalidateQueries({ queryKey: ["agent-tokens"] });
    }
  });

  const revokeToken = useMutation({
    mutationFn: (id: string) => revokeAgentToken(id),
    onSuccess: async (_, id) => {
      if (created?.id === id) {
        setCreated(null);
        setCopied(false);
      }
      await queryClient.invalidateQueries({ queryKey: ["agent-tokens"] });
    }
  });

  const saveEmailChannel = useMutation({
    mutationFn: () =>
      upsertNotificationChannel({
        channelType: "email",
        email: notificationEmail.trim(),
        label: "Email",
        criticalAlertsEnabled: emailCritical,
        dailyDigestEnabled: emailDigest,
        emailLocale: locale
      }),
    onSuccess: async () => {
      setChannelMessage(t.account.channelSaved);
      await queryClient.invalidateQueries({ queryKey: ["notification-channels"] });
    }
  });

  const saveWebhookChannel = useMutation({
    mutationFn: () =>
      upsertNotificationChannel({
        channelType: "discord_webhook",
        webhookUrl: notificationWebhookUrl.trim(),
        label: "Discord",
        criticalAlertsEnabled: webhookCritical,
        dailyDigestEnabled: webhookDigest,
        emailLocale: locale
      }),
    onSuccess: async () => {
      setNotificationWebhookUrl("");
      setChannelMessage(t.account.channelSaved);
      await queryClient.invalidateQueries({ queryKey: ["notification-channels"] });
    }
  });

  const deleteChannel = useMutation({
    mutationFn: (id: string) => deleteNotificationChannel(id),
    onSuccess: async () => {
      setChannelMessage(t.account.channelDeleted);
      await queryClient.invalidateQueries({ queryKey: ["notification-channels"] });
    }
  });

  const testChannel = useMutation({
    mutationFn: (id: string) => testNotificationChannel(id, locale),
    onSuccess: async () => {
      setChannelMessage(t.account.channelTestSent);
      await queryClient.invalidateQueries({ queryKey: ["notification-channels"] });
    }
  });

  const saveNotificationPreferences = useMutation({
    mutationFn: (next?: {
      digestTimePreset?: DigestTimePreset;
    }) =>
      updateNotificationPreferences({
        digestTimePreset: next?.digestTimePreset ?? digestTimePreset,
        timezone: detectTimezone(),
        emailLocale: locale
      }),
    onSuccess: async (preferences) => {
      setDigestTimePreset(preferences.digestTimePreset);
      setChannelMessage(t.account.notificationPreferencesSaved);
      await queryClient.invalidateQueries({ queryKey: ["notification-preferences"] });
    }
  });

  const reviewSignal = useMutation({
    mutationFn: ({ id, action }: { id: string; action: "approve" | "reject" }) =>
      reviewPendingRepoSignal(id, action, adminToken),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: ["admin-pending-signals"] });
    }
  });

  const error =
    createToken.error instanceof ApiError
      ? createToken.error.message
      : revokeToken.error instanceof ApiError
        ? revokeToken.error.message
        : null;
  const channelError =
    saveEmailChannel.error instanceof ApiError
      ? saveEmailChannel.error.message
      : saveWebhookChannel.error instanceof ApiError
        ? saveWebhookChannel.error.message
        : deleteChannel.error instanceof ApiError
          ? deleteChannel.error.message
          : testChannel.error instanceof ApiError
            ? testChannel.error.message
            : saveNotificationPreferences.error instanceof ApiError
              ? saveNotificationPreferences.error.message
              : null;

  useEffect(() => {
    const emailChannel = notificationChannels.data?.find(
      (channel) => channel.channelType === "email"
    );
    if (!emailChannel) return;
    setNotificationEmail((current) => current || emailChannel.destination);
    setEmailCritical(emailChannel.criticalAlertsEnabled);
    setEmailDigest(emailChannel.dailyDigestEnabled);
  }, [notificationChannels.data]);

  useEffect(() => {
    const webhookChannel = notificationChannels.data?.find(
      (channel) => channel.channelType === "discord_webhook"
    );
    if (!webhookChannel) return;
    setWebhookCritical(webhookChannel.criticalAlertsEnabled);
    setWebhookDigest(webhookChannel.dailyDigestEnabled);
  }, [notificationChannels.data]);

  useEffect(() => {
    if (!notificationPreferences.data) return;
    setDigestTimePreset(notificationPreferences.data.digestTimePreset);
  }, [notificationPreferences.data]);

  async function copyToken(): Promise<void> {
    if (!created?.token) return;
    await navigator.clipboard.writeText(created.token);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1500);
  }

  return (
    <section className="shell grid gap-8 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.account.eyebrow}</span>
        <div className="grid gap-4 md:grid-cols-[1.2fr_1fr] md:items-end">
          <div className="grid gap-3">
            <h1 className="display-lg max-w-[24ch]">
              {t.account.h1Part1}
              <br />
              <span className="accent">{t.account.h1Accent}</span>
            </h1>
            <p className="max-w-[62ch] text-[0.98rem] leading-relaxed text-fg-dim">
              {t.account.intro}
            </p>
          </div>
          <AccountIdentityCard user={user} />
        </div>
      </header>

      <div className="grid gap-4 md:grid-cols-[1.1fr_0.9fr]">
        <AgentTokensPanel
          label={label}
          onLabelChange={setLabel}
          createPending={createToken.isPending}
          revokePending={revokeToken.isPending}
          created={created}
          copied={copied}
          error={error}
          tokensLoading={tokens.isLoading}
          tokens={tokens.data ?? []}
          onCreate={() => createToken.mutate()}
          onCopy={() => void copyToken()}
          onRevoke={(id) => revokeToken.mutate(id)}
          tokenLabel={t.account.tokenLabel}
          tokenPlaceholder={t.account.tokenPlaceholder}
          creatingLabel={t.account.creating}
          createLabel={t.account.create}
          tokenShownOnceLabel={t.account.tokenShownOnce}
          tokenShownOnceHint={t.account.tokenShownOnceHint}
          copiedLabel={t.account.copied}
          copyLabel={t.account.copy}
          createdNowLabel={t.account.createdNow}
          activeTokensLabel={t.account.activeTokens}
          emptyTitle={t.account.emptyTitle}
          emptyBody={t.account.emptyBody}
          createdAtLabel={t.account.createdAt}
          lastUsedLabel={t.account.lastUsed}
          lastUsedNeverLabel={t.account.lastUsedNever}
          revokingLabel={t.account.revoking}
          revokeLabel={t.account.revoke}
          loadingLabel={t.common.tuning}
        />

        <ReputationCard
          summary={summary.data}
          quotaTitle={t.account.quotaTitle}
          quotaBody={t.account.quotaBody}
          reputationLabel={t.account.reputation}
          tierLabel={t.account.tier}
          usageSignalsLabel={t.account.usageSignals}
          successRatioLabel={t.account.successRatio}
          buildReliabilityLabel={t.account.buildReliability}
          regretRatioLabel={t.account.regretRatio}
          passiveSignalsLabel={t.account.passiveSignals}
          eligibilityLabel={t.account.eligibility}
          eligibleLabel={t.account.eligible}
          notEligibleLabel={t.account.notEligible}
        />
      </div>

      <NotificationChannelsPanel
        loading={notificationChannels.isLoading}
        channels={notificationChannels.data ?? []}
        email={notificationEmail}
        webhookUrl={notificationWebhookUrl}
        emailCritical={emailCritical}
        emailDigest={emailDigest}
        webhookCritical={webhookCritical}
        webhookDigest={webhookDigest}
        digestTimePreset={digestTimePreset}
        savingEmail={saveEmailChannel.isPending}
        savingWebhook={saveWebhookChannel.isPending}
        savingPreferences={saveNotificationPreferences.isPending}
        deleting={deleteChannel.isPending}
        testingId={testChannel.isPending ? testChannel.variables ?? null : null}
        message={channelMessage}
        error={channelError}
        onEmailChange={setNotificationEmail}
        onWebhookUrlChange={setNotificationWebhookUrl}
        onEmailCriticalChange={setEmailCritical}
        onEmailDigestChange={setEmailDigest}
        onWebhookCriticalChange={setWebhookCritical}
        onWebhookDigestChange={setWebhookDigest}
        onDigestTimePresetChange={(value) => {
          setDigestTimePreset(value);
          saveNotificationPreferences.mutate({ digestTimePreset: value });
        }}
        onSaveEmail={() => saveEmailChannel.mutate()}
        onSaveWebhook={() => saveWebhookChannel.mutate()}
        onSavePreferences={() =>
          saveNotificationPreferences.mutate({ digestTimePreset })
        }
        onDelete={(id) => deleteChannel.mutate(id)}
        onTest={(id) => testChannel.mutate(id)}
      />

      <AdminModerationPanel
        adminToken={adminToken}
        onAdminTokenChange={setAdminToken}
        loading={pendingSignals.isLoading}
        items={pendingSignals.data}
        reviewPending={reviewSignal.isPending}
        onReview={(id, action) => reviewSignal.mutate({ id, action })}
        title={t.account.adminTitle}
        adminTokenLabel={t.account.adminTokenLabel}
        adminTokenPlaceholder={t.account.adminTokenPlaceholder}
        loadingLabel={t.common.tuning}
        approveLabel={t.account.adminApprove}
        rejectLabel={t.account.adminReject}
        reviewingLabel={t.account.adminReviewing}
        emptyLabel={t.account.adminEmpty}
      />

      <AdminMcpObservabilityPanel
        adminToken={adminToken}
        window={mcpWindow}
        onWindowChange={setMcpWindow}
        loading={mcpMetrics.isLoading}
        report={mcpMetrics.data}
        labels={{
          title: t.account.mcpObservabilityTitle,
          intro: t.account.mcpObservabilityIntro,
          windowLabel: t.account.mcpWindowLabel,
          window24h: t.account.mcpWindow24h,
          window7d: t.account.mcpWindow7d,
          window30d: t.account.mcpWindow30d,
          loading: t.account.mcpLoading,
          totalLogUsage: t.account.mcpTotalLogUsage,
          totalWatchRepo: t.account.mcpTotalWatchRepo,
          totalRejections: t.account.mcpTotalRejections,
          distinctTokens: t.account.mcpDistinctTokens,
          distinctUsers: t.account.mcpDistinctUsers,
          distinctRepos: t.account.mcpDistinctRepos,
          outcomeTitle: t.account.mcpOutcomeTitle,
          rejectionTitle: t.account.mcpRejectionTitle,
          topReposTitle: t.account.mcpTopReposTitle,
          topUsersTitle: t.account.mcpTopUsersTitle,
          dailyTitle: t.account.mcpDailyTitle,
          empty: t.account.mcpEmpty
        }}
      />
    </section>
  );
}
