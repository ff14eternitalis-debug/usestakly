import { useLocaleStore, type Locale } from "../state/locale-store";

const OPTIONS: { value: Locale; label: string }[] = [
  { value: "en", label: "EN" },
  { value: "fr", label: "FR" }
];

export function LocaleSwitch() {
  const { locale, setLocale } = useLocaleStore();
  const activeIndex = OPTIONS.findIndex((o) => o.value === locale);
  return (
    <div
      role="radiogroup"
      aria-label="Language"
      className="relative inline-flex items-center rounded-[5px] border border-line bg-surface p-[2px] transition-colors duration-200 hover:border-[var(--color-accent)]"
    >
      <span
        aria-hidden="true"
        className="pointer-events-none absolute top-[2px] bottom-[2px] left-[2px] w-[calc(50%-2px)] rounded-[4px] bg-surface-elev shadow-[0_0_0_1px_var(--color-accent)] transition-transform duration-300 ease-[cubic-bezier(0.4,0,0.2,1)]"
        style={{ transform: `translateX(${activeIndex * 100}%)` }}
      />
      {OPTIONS.map((o) => {
        const active = o.value === locale;
        return (
          <button
            key={o.value}
            type="button"
            role="radio"
            aria-checked={active}
            onClick={() => setLocale(o.value)}
            className={`relative z-10 px-2 py-[2px] text-[0.7rem] font-medium rounded-[4px] mono uppercase tracking-[0.12em] transition-colors ${
              active ? "text-fg" : "text-fg-muted hover:text-fg"
            }`}
          >
            {o.label}
          </button>
        );
      })}
    </div>
  );
}
