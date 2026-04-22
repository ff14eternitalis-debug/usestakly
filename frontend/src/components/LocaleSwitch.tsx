import { useLocaleStore, type Locale } from "../state/locale-store";

const OPTIONS: { value: Locale; label: string }[] = [
  { value: "en", label: "EN" },
  { value: "fr", label: "FR" }
];

export function LocaleSwitch() {
  const { locale, setLocale } = useLocaleStore();
  return (
    <div
      role="radiogroup"
      aria-label="Language"
      className="inline-flex items-center rounded-[5px] border border-line bg-surface p-[2px]"
    >
      {OPTIONS.map((o) => {
        const active = o.value === locale;
        return (
          <button
            key={o.value}
            type="button"
            role="radio"
            aria-checked={active}
            onClick={() => setLocale(o.value)}
            className={`px-2 py-[2px] text-[0.7rem] font-medium rounded-[4px] mono uppercase tracking-[0.12em] transition-colors ${
              active
                ? "bg-surface-elev text-fg shadow-[0_0_0_1px_var(--color-line-strong)]"
                : "text-fg-muted hover:text-fg"
            }`}
          >
            {o.label}
          </button>
        );
      })}
    </div>
  );
}
