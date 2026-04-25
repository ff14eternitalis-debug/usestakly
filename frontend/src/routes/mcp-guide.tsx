import { Link } from "@tanstack/react-router";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useMemo, useState } from "react";

import { useT } from "../i18n";
import { authUrl } from "../lib/api-client";
import { createAgentToken } from "../lib/api/account";
import type { AgentTokenCreated } from "../lib/types";
import { loginSearch } from "../lib/return-to";
import { useAuthStore } from "../state/auth-store";

function CodeBlock({ code }: { code: string }) {
  return (
    <pre className="overflow-x-auto rounded-[8px] border border-line bg-bg-subtle p-4 text-[0.82rem] leading-relaxed text-fg-dim">
      <code>{code}</code>
    </pre>
  );
}

function Step({
  index,
  title,
  body
}: {
  index: string;
  title: string;
  body: string;
}) {
  return (
    <li className="grid gap-2 border-t border-line py-5 md:grid-cols-[88px_1fr] md:gap-6">
      <span className="mono text-[0.74rem] uppercase tracking-[0.14em] text-accent">
        {index}
      </span>
      <div className="grid gap-1.5">
        <h2 className="display-md !text-[1.08rem]">{title}</h2>
        <p className="max-w-[68ch] text-[0.94rem] leading-relaxed text-fg-dim">
          {body}
        </p>
      </div>
    </li>
  );
}

export function McpGuidePage() {
  const t = useT();
  const queryClient = useQueryClient();
  const isAuthed = useAuthStore((s) => s.status === "authenticated");
  const [label, setLabel] = useState("codex-local");
  const [created, setCreated] = useState<AgentTokenCreated | null>(null);
  const [client, setClient] = useState<"codex" | "cursor" | "claude" | "generic">("codex");
  const [copied, setCopied] = useState(false);
  const [testResult, setTestResult] = useState<"idle" | "ok" | "fail">("idle");
  const mcpUrl = authUrl("/mcp");

  const tokenValue = created?.token ?? "usk_REPLACE_WITH_YOUR_TOKEN";
  const clientLabels = {
    codex: t.mcpGuide.clientCodex,
    cursor: t.mcpGuide.clientCursor,
    claude: t.mcpGuide.clientClaude,
    generic: t.mcpGuide.clientGeneric
  };
  const configExample = useMemo(() => {
    const serverName = client === "generic" ? "usestakly" : "usestakly";
    return `{
  "mcpServers": {
    "${serverName}": {
      "type": "streamable-http",
      "url": "${mcpUrl}",
      "headers": {
        "Authorization": "Bearer ${tokenValue}"
      }
    }
  }
}`;
  }, [client, mcpUrl, tokenValue]);
  const curlExample = `curl -X POST ${mcpUrl} \\
  -H "Authorization: Bearer ${tokenValue}" \\
  -H "Content-Type: application/json" \\
  -H "Accept: application/json, text/event-stream" \\
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"curl","version":"0"}}}'`;

  const createToken = useMutation({
    mutationFn: () => createAgentToken(label.trim() || "usestakly-agent"),
    onSuccess: async (token) => {
      setCreated(token);
      setCopied(false);
      setTestResult("idle");
      await queryClient.invalidateQueries({ queryKey: ["agent-tokens"] });
    }
  });

  const testToken = useMutation({
    mutationFn: async () => {
      if (!created?.token) throw new Error("No token");
      const response = await fetch(mcpUrl, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${created.token}`,
          "Content-Type": "application/json",
          Accept: "application/json, text/event-stream"
        },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "initialize",
          params: {
            protocolVersion: "2025-06-18",
            capabilities: {},
            clientInfo: { name: "usestakly-web", version: "1" }
          }
        })
      });
      if (!response.ok) throw new Error(`MCP returned ${response.status}`);
      await response.text();
    },
    onSuccess: () => setTestResult("ok"),
    onError: () => setTestResult("fail")
  });

  async function copyConfig(): Promise<void> {
    await navigator.clipboard.writeText(configExample);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1500);
  }

  return (
    <section className="shell grid gap-10 py-10 md:py-14">
      <header className="grid gap-5 border-b border-line pb-8 md:grid-cols-[1.2fr_0.8fr] md:items-end">
        <div className="grid gap-4">
          <span className="kicker">{t.mcpGuide.eyebrow}</span>
          <h1 className="display-lg max-w-[22ch]">{t.mcpGuide.h1}</h1>
          <p className="max-w-[66ch] text-[0.98rem] leading-relaxed text-fg-dim">
            {t.mcpGuide.intro}
          </p>
        </div>
        <InstallAssistant
          isAuthed={isAuthed}
          label={label}
          onLabelChange={setLabel}
          pending={createToken.isPending}
          created={created}
          copied={copied}
          testPending={testToken.isPending}
          testResult={testResult}
          onCreate={() => createToken.mutate()}
          onCopyConfig={() => void copyConfig()}
          onTest={() => testToken.mutate()}
        />
      </header>

      <section className="grid gap-4">
        <div className="grid gap-2">
          <span className="kicker">{t.mcpGuide.endpointLabel}</span>
          <CodeBlock code={mcpUrl} />
        </div>
        <p className="text-[0.9rem] leading-relaxed text-fg-dim">
          {t.mcpGuide.endpointBody}
        </p>
      </section>

      <section className="grid gap-5 border-t border-line pt-8 md:grid-cols-[0.82fr_1.18fr] md:gap-8">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.mcpGuide.chooseClientLabel}</span>
          <h2 className="display-md !text-[1.35rem]">
            {t.mcpGuide.configReadyTitle}
          </h2>
          <p className="text-[0.94rem] leading-relaxed text-fg-dim">
            {t.mcpGuide.configReadyBody}
          </p>
          <div className="flex flex-wrap gap-2 pt-2">
            {(["codex", "cursor", "claude", "generic"] as const).map((id) => (
              <button
                key={id}
                type="button"
                onClick={() => setClient(id)}
                className={`rounded-[6px] border px-3 py-1.5 text-[0.82rem] font-medium transition-colors ${
                  client === id
                    ? "border-accent bg-[color:var(--color-accent-glow)] text-accent"
                    : "border-line bg-surface text-fg-dim hover:border-accent hover:text-accent"
                }`}
              >
                {clientLabels[id]}
              </button>
            ))}
          </div>
        </div>
        <div className="grid gap-3">
          <CodeBlock code={configExample} />
          <button
            type="button"
            onClick={() => void copyConfig()}
            className="inline-flex w-fit items-center rounded-[6px] border border-line-strong bg-surface px-4 py-2 text-[0.88rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent"
          >
            {copied ? t.mcpGuide.copied : t.mcpGuide.copyConfig}
          </button>
        </div>
      </section>

      <section className="grid gap-3">
        <span className="kicker">{t.mcpGuide.stepsLabel}</span>
        <ol className="grid">
          {t.mcpGuide.steps.map((step, index) => (
            <Step
              key={step.title}
              index={`0${index + 1}`}
              title={step.title}
              body={step.body}
            />
          ))}
        </ol>
      </section>

      <section className="grid gap-5 border-t border-line pt-8 md:grid-cols-[0.82fr_1.18fr] md:gap-8">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.mcpGuide.smokeTestLabel}</span>
          <h2 className="display-md !text-[1.35rem]">
            {t.mcpGuide.smokeTestTitle}
          </h2>
          <p className="text-[0.94rem] leading-relaxed text-fg-dim">
            {t.mcpGuide.smokeTestBody}
          </p>
        </div>
        <CodeBlock code={curlExample} />
      </section>

      <section className="grid gap-5 border-t border-line pt-8 md:grid-cols-[0.82fr_1.18fr] md:gap-8">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.mcpGuide.toolsLabel}</span>
          <h2 className="display-md !text-[1.35rem]">
            {t.mcpGuide.toolsTitle}
          </h2>
          <p className="text-[0.94rem] leading-relaxed text-fg-dim">
            {t.mcpGuide.toolsBody}
          </p>
        </div>
        <ul className="grid gap-3">
          {t.mcpGuide.tools.map((tool) => (
            <li
              key={tool.name}
              className="grid gap-1 rounded-[8px] border border-line bg-surface/35 p-4"
            >
              <span className="mono text-[0.8rem] text-accent">
                {tool.name}
              </span>
              <p className="text-[0.9rem] leading-relaxed text-fg-dim">
                {tool.body}
              </p>
            </li>
          ))}
        </ul>
      </section>

      <section className="grid gap-5 border-t border-line pt-8 md:grid-cols-[0.82fr_1.18fr] md:gap-8">
        <div className="grid content-start gap-3">
          <span className="kicker">{t.mcpGuide.securityLabel}</span>
          <h2 className="display-md !text-[1.35rem]">
            {t.mcpGuide.securityTitle}
          </h2>
        </div>
        <ul className="grid gap-2">
          {t.mcpGuide.securityItems.map((item) => (
            <li
              key={item}
              className="border-l border-line pl-4 text-[0.94rem] leading-relaxed text-fg-dim"
            >
              {item}
            </li>
          ))}
        </ul>
      </section>
    </section>
  );
}

function InstallAssistant({
  isAuthed,
  label,
  onLabelChange,
  pending,
  created,
  copied,
  testPending,
  testResult,
  onCreate,
  onCopyConfig,
  onTest
}: {
  isAuthed: boolean;
  label: string;
  onLabelChange: (value: string) => void;
  pending: boolean;
  created: AgentTokenCreated | null;
  copied: boolean;
  testPending: boolean;
  testResult: "idle" | "ok" | "fail";
  onCreate: () => void;
  onCopyConfig: () => void;
  onTest: () => void;
}) {
  const t = useT();

  if (!isAuthed) {
    return (
      <div className="grid gap-3 rounded-[8px] border border-line bg-surface/45 p-5 md:justify-items-end">
        <p className="max-w-[34ch] text-[0.88rem] leading-relaxed text-fg-dim md:text-right">
          {t.mcpGuide.signInToCreate}
        </p>
        <Link
          to="/login"
          search={loginSearch()}
          className="inline-flex w-fit items-center rounded-[6px] border border-line-strong bg-surface px-4 py-2.5 text-[0.92rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent"
        >
          {t.nav.signIn}
        </Link>
      </div>
    );
  }

  return (
    <div className="grid gap-3 rounded-[8px] border border-line bg-surface/45 p-5">
      <div className="grid gap-1.5">
        <span className="kicker">{t.mcpGuide.installAssistantLabel}</span>
        <p className="text-[0.88rem] leading-relaxed text-fg-dim">
          {t.mcpGuide.installAssistantBody}
        </p>
      </div>
      <label className="grid gap-1.5">
        <span className="kicker">{t.mcpGuide.tokenLabel}</span>
        <input
          type="text"
          value={label}
          onChange={(event) => onLabelChange(event.target.value)}
          placeholder={t.mcpGuide.tokenPlaceholder}
          className="input"
        />
      </label>
      <div className="flex flex-wrap gap-2">
        <button
          type="button"
          onClick={onCreate}
          disabled={pending}
          className="inline-flex items-center rounded-[6px] border border-line-strong bg-surface px-4 py-2 text-[0.88rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent disabled:opacity-40"
        >
          {pending ? t.mcpGuide.creatingToken : t.mcpGuide.createTokenInline}
        </button>
        <button
          type="button"
          onClick={onCopyConfig}
          disabled={!created}
          className="inline-flex items-center rounded-[6px] border border-line bg-transparent px-4 py-2 text-[0.88rem] font-medium text-fg-dim transition-colors hover:border-accent hover:text-accent disabled:opacity-40"
        >
          {copied ? t.mcpGuide.copied : t.mcpGuide.copyConfig}
        </button>
        <button
          type="button"
          onClick={onTest}
          disabled={!created || testPending}
          className="inline-flex items-center rounded-[6px] border border-line bg-transparent px-4 py-2 text-[0.88rem] font-medium text-fg-dim transition-colors hover:border-accent hover:text-accent disabled:opacity-40"
        >
          {testPending ? t.mcpGuide.testingToken : t.mcpGuide.testToken}
        </button>
      </div>
      {created ? (
        <p className="text-[0.84rem] leading-relaxed text-fg-dim">
          {t.mcpGuide.tokenReady}
        </p>
      ) : null}
      {testResult !== "idle" ? (
        <p
          className="text-[0.84rem] leading-relaxed"
          style={{
            color:
              testResult === "ok"
                ? "var(--color-accent)"
                : "var(--color-danger)"
          }}
        >
          {testResult === "ok" ? t.mcpGuide.testOk : t.mcpGuide.testFail}
        </p>
      ) : null}
    </div>
  );
}
