import { useT } from "../i18n";

export function McpGuidePage() {
  const t = useT();
  return (
    <section className="shell grid gap-8 py-10 md:py-14">
      <header className="grid gap-4">
        <span className="kicker">{t.mcpGuide.eyebrow}</span>
        <h1 className="display-lg max-w-[24ch]">{t.mcpGuide.h1}</h1>
      </header>

      <div className="surface grid place-items-center p-10 md:p-14">
        <p className="mono text-[0.9rem] uppercase tracking-[0.16em] text-fg-dim">
          {t.mcpGuide.wip}
        </p>
      </div>
    </section>
  );
}
