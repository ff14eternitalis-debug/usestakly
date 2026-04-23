import { Button } from "../../../components/Button";

type ReportSignalFormProps = {
  title: string;
  hint: string;
  signal: string;
  evidenceUrl: string;
  evidenceDescription: string;
  isPending: boolean;
  isSuccess: boolean;
  error: string | null;
  onSignalChange(value: string): void;
  onEvidenceUrlChange(value: string): void;
  onEvidenceDescriptionChange(value: string): void;
  onSubmit(): void;
  signalLabel: string;
  evidenceUrlLabel: string;
  evidenceDescriptionLabel: string;
  submittingLabel: string;
  submitLabel: string;
  successLabel: string;
};

export function ReportSignalForm({
  title,
  hint,
  signal,
  evidenceUrl,
  evidenceDescription,
  isPending,
  isSuccess,
  error,
  onSignalChange,
  onEvidenceUrlChange,
  onEvidenceDescriptionChange,
  onSubmit,
  signalLabel,
  evidenceUrlLabel,
  evidenceDescriptionLabel,
  submittingLabel,
  submitLabel,
  successLabel
}: ReportSignalFormProps) {
  return (
    <section className="grid gap-4">
      <h2 className="display-md">{title}</h2>
      <p className="max-w-[66ch] text-[0.94rem] leading-relaxed text-fg-dim">{hint}</p>
      <div className="surface grid gap-4 p-5 md:grid-cols-2">
        <label className="grid gap-1.5">
          <span className="kicker">{signalLabel}</span>
          <select value={signal} onChange={(e) => onSignalChange(e.target.value)} className="input">
            <option value="deprecated">deprecated</option>
            <option value="broken">broken</option>
            <option value="security_issue">security_issue</option>
            <option value="doesnt_match_claim">doesnt_match_claim</option>
          </select>
        </label>
        <label className="grid gap-1.5">
          <span className="kicker">{evidenceUrlLabel}</span>
          <input
            type="url"
            value={evidenceUrl}
            onChange={(e) => onEvidenceUrlChange(e.target.value)}
            className="input"
            placeholder="https://..."
          />
        </label>
        <label className="grid gap-1.5 md:col-span-2">
          <span className="kicker">{evidenceDescriptionLabel}</span>
          <textarea
            value={evidenceDescription}
            onChange={(e) => onEvidenceDescriptionChange(e.target.value)}
            className="input min-h-[120px]"
          />
        </label>
        <div className="flex flex-wrap items-center gap-3 md:col-span-2">
          <Button type="button" variant="outline" onClick={onSubmit} disabled={isPending}>
            {isPending ? submittingLabel : submitLabel}
          </Button>
          {error ? (
            <p className="text-[0.86rem]" style={{ color: "var(--color-danger)" }}>
              {error}
            </p>
          ) : isSuccess ? (
            <p className="text-[0.86rem] text-fg-dim">{successLabel}</p>
          ) : null}
        </div>
      </div>
    </section>
  );
}
