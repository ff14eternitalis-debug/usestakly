import { Link } from "@tanstack/react-router";

import { useT } from "../i18n";
import { authUrl } from "../lib/api-client";

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
  const mcpUrl = authUrl("/mcp");
  const configExample = `{
  "mcpServers": {
    "usestakly": {
      "type": "streamable-http",
      "url": "${mcpUrl}",
      "headers": {
        "Authorization": "Bearer usk_REPLACE_WITH_YOUR_TOKEN"
      }
    }
  }
}`;
  const curlExample = `curl -X POST ${mcpUrl} \\
  -H "Authorization: Bearer usk_REPLACE_WITH_YOUR_TOKEN" \\
  -H "Content-Type: application/json" \\
  -H "Accept: application/json, text/event-stream" \\
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"curl","version":"0"}}}'`;

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
        <div className="grid gap-3 md:justify-items-end">
          <Link
            to="/account"
            className="inline-flex w-fit items-center rounded-[6px] border border-line-strong bg-surface px-4 py-2.5 text-[0.92rem] font-medium text-fg transition-colors hover:border-accent hover:text-accent"
          >
            {t.mcpGuide.createTokenAction}
          </Link>
          <p className="max-w-[34ch] text-[0.84rem] leading-relaxed text-fg-muted md:text-right">
            {t.mcpGuide.createTokenHint}
          </p>
        </div>
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
          <span className="kicker">{t.mcpGuide.clientConfigLabel}</span>
          <h2 className="display-md !text-[1.35rem]">
            {t.mcpGuide.clientConfigTitle}
          </h2>
          <p className="text-[0.94rem] leading-relaxed text-fg-dim">
            {t.mcpGuide.clientConfigBody}
          </p>
        </div>
        <CodeBlock code={configExample} />
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
