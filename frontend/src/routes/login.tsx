import { Link } from "@tanstack/react-router";

import { Wordmark } from "../components/Wordmark";
import { useT } from "../i18n";
import { authUrl } from "../lib/api-client";
import { authPath, loginReturnTo } from "../lib/return-to";
import { useAuthStore } from "../state/auth-store";

const githubIcon = (
  <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden>
    <path d="M12 .5a12 12 0 0 0-3.79 23.4c.6.11.82-.26.82-.58v-2c-3.34.73-4.04-1.61-4.04-1.61-.54-1.38-1.33-1.75-1.33-1.75-1.09-.74.08-.73.08-.73 1.2.08 1.84 1.24 1.84 1.24 1.07 1.84 2.81 1.31 3.5 1 .11-.78.42-1.31.76-1.61-2.66-.3-5.47-1.33-5.47-5.91 0-1.3.47-2.37 1.24-3.2-.12-.3-.54-1.52.12-3.16 0 0 1-.32 3.3 1.22a11.47 11.47 0 0 1 6 0c2.3-1.54 3.3-1.22 3.3-1.22.66 1.64.24 2.86.12 3.16.77.83 1.24 1.9 1.24 3.2 0 4.6-2.81 5.61-5.49 5.9.43.37.81 1.1.81 2.22v3.3c0 .32.21.7.82.58A12 12 0 0 0 12 .5Z" />
  </svg>
);

const discordIcon = (
  <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" aria-hidden>
    <path d="M20.317 4.37a19.79 19.79 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.38-.444.87-.608 1.26a18.27 18.27 0 0 0-5.487 0 12.93 12.93 0 0 0-.617-1.26.077.077 0 0 0-.079-.036 19.74 19.74 0 0 0-4.885 1.516.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.058a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.1 13.1 0 0 1-1.872-.892.077.077 0 0 1-.008-.128c.126-.094.252-.192.372-.291a.074.074 0 0 1 .077-.01c3.927 1.793 8.18 1.793 12.061 0a.074.074 0 0 1 .078.009c.12.099.246.198.373.292a.077.077 0 0 1-.006.127c-.598.349-1.22.645-1.873.891a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.84 19.84 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.673-3.548-13.66a.061.061 0 0 0-.031-.03ZM8.02 15.331c-1.182 0-2.157-1.085-2.157-2.419s.955-2.419 2.157-2.419c1.21 0 2.177 1.094 2.158 2.42 0 1.333-.956 2.418-2.158 2.418Zm7.974 0c-1.183 0-2.157-1.085-2.157-2.419s.955-2.419 2.157-2.419c1.21 0 2.177 1.094 2.158 2.42 0 1.333-.948 2.418-2.158 2.418Z" />
  </svg>
);

export function LoginPage() {
  const t = useT();
  const status = useAuthStore((s) => s.status);
  const returnTo = loginReturnTo();

  return (
    <section className="shell grid min-h-[78vh] place-items-center py-12">
      <div className="grid w-full max-w-[840px] gap-10 md:grid-cols-[1fr_1fr] md:items-center">
        <div className="grid gap-5 rise-in">
          <span className="kicker">{t.login.eyebrow}</span>
          <h1 className="display-lg">
            {t.login.h1Part1} <span className="accent">{t.login.h1Accent}</span>
          </h1>
          <p className="max-w-[44ch] text-[0.98rem] leading-relaxed text-fg-dim">
            {t.login.body}
          </p>
          <Link
            to="/discover"
            className="link-underline w-fit text-[0.92rem] text-fg-dim hover:text-accent"
          >
            {t.login.browseWithoutSignIn} <span className="arrow">→</span>
          </Link>
        </div>

        <div className="surface relative overflow-hidden rise-in rise-in-d1">
          <div className="flex items-center justify-between border-b border-line px-5 py-3">
            <Wordmark scale="sm" />
            <span className="inline-flex items-center gap-1.5 mono text-[0.7rem] text-fg-muted">
              <span className="dot text-accent" />
              {status === "loading"
                ? t.common.checkingStatus
                : t.common.readyStatus}
            </span>
          </div>
          <div className="grid gap-3 px-6 py-7">
            <a
              href={authUrl(authPath("/api/auth/github/start", returnTo))}
              className="inline-flex items-center justify-center gap-3 rounded-[6px] border border-line-strong bg-surface px-5 py-3.5 text-[0.92rem] font-medium text-fg hover:border-accent hover:text-accent transition-colors"
            >
              {githubIcon}
              {t.login.continueGithub}
            </a>
            <a
              href={authUrl(authPath("/api/auth/discord/start", returnTo))}
              className="inline-flex items-center justify-center gap-3 rounded-[6px] border border-line-strong bg-surface px-5 py-3.5 text-[0.92rem] font-medium text-fg hover:border-accent hover:text-accent transition-colors"
            >
              {discordIcon}
              {t.login.continueDiscord}
            </a>
            <p className="pt-3 text-[0.8rem] leading-relaxed text-fg-muted">
              {t.login.privacy}
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}
