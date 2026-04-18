import type { CopyBlock, Locale } from "../../../lib/app-types";

type AssemblySectionProps = {
  copy: CopyBlock;
  locale: Locale;
};

export function AssemblySection({ copy, locale }: AssemblySectionProps) {
  return (
    <section className="workspace-section">
      <div className="workspace-section-head">
        <div>
          <h2>{copy.commandTitle}</h2>
          <p>{copy.commandBody}</p>
        </div>
      </div>

      <div className="command-stack">
        <div className="command-mode">
          <span>{copy.commandModeStrict}</span>
          <strong>@owner/library:snippet@version</strong>
        </div>
        <div className="command-mode">
          <span>{copy.commandModeAuto}</span>
          <strong>own libraries - public fallback - generate last</strong>
        </div>
        <div className="command-prompt">
          <span>{copy.commandModePrompt}</span>
          <code>
            {locale === "fr"
              ? "Construis cette feature avec React + Tailwind en résolvant d’abord @moi/ui-kit"
              : "Build this feature with React + Tailwind and resolve @me/ui-kit first"}
          </code>
        </div>
      </div>
    </section>
  );
}
