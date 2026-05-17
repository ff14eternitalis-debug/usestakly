import { useT } from "../../i18n";

import { LandingKpi } from "./landing-kpi";

export function LandingMetrics() {
  const t = useT();
  return (
    <div className="mt-6 grid grid-cols-3 gap-5 border-t border-line pt-6 rise-in rise-in-d2">
      <LandingKpi k="4" label={t.landing.kpi1} />
      <LandingKpi k="v2" label={t.landing.kpi2} />
      <LandingKpi k="0%" label={t.landing.kpi3} />
    </div>
  );
}
