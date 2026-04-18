import { useEffect, useState } from "react";

import { apiGet, apiPost, authUrl } from "../../lib/api-client";

type CurrentUser = {
  id: string;
  email: string;
  username: string;
  displayName: string | null;
  avatarUrl: string | null;
};

export function AppShell() {
  const [user, setUser] = useState<CurrentUser | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;

    async function loadUser() {
      try {
        const response = await fetch(authUrl("/api/me"), {
          credentials: "include"
        });

        if (!response.ok) {
          if (!cancelled) {
            setUser(null);
          }
          return;
        }

        const data = (await response.json()) as CurrentUser;
        if (!cancelled) {
          setUser(data);
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadUser();

    return () => {
      cancelled = true;
    };
  }, []);

  async function handleLogout() {
    await apiPost("/api/auth/logout");
    setUser(null);
  }

  return (
    <main style={{ maxWidth: 1120, margin: "0 auto", padding: "48px 24px" }}>
      <section
        style={{
          border: "1px solid rgba(31, 26, 23, 0.1)",
          background: "rgba(255, 253, 248, 0.9)",
          borderRadius: 24,
          padding: 32,
          boxShadow: "0 12px 40px rgba(31, 26, 23, 0.08)"
        }}
      >
        <p style={{ margin: 0, color: "var(--muted)", fontSize: 14 }}>UseStakly MVP</p>
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            gap: 16,
            alignItems: "center",
            flexWrap: "wrap"
          }}
        >
          <div>
            <h1 style={{ marginTop: 12, marginBottom: 12, fontSize: 44, lineHeight: 1.05 }}>
              Build apps by resolving libraries before generating code.
            </h1>
            <p style={{ maxWidth: 760, fontSize: 18, lineHeight: 1.6, marginBottom: 24 }}>
              UseStakly is now scaffolded with a backend, a frontend, Docker for PostgreSQL, and the
              documentation blueprint that drives the implementation.
            </p>
          </div>
          <div style={{ display: "flex", gap: 12, alignItems: "center" }}>
            {loading ? (
              <span style={{ color: "var(--muted)" }}>Checking session…</span>
            ) : user ? (
              <>
                <div style={{ textAlign: "right" }}>
                  <strong style={{ display: "block" }}>
                    {user.displayName ?? user.username}
                  </strong>
                  <span style={{ color: "var(--muted)", fontSize: 14 }}>{user.email}</span>
                </div>
                <button
                  type="button"
                  onClick={() => {
                    void handleLogout();
                  }}
                  style={buttonStyle("secondary")}
                >
                  Logout
                </button>
              </>
            ) : (
              <a href={authUrl("/api/auth/github/start")} style={buttonStyle("primary")}>
                Login with GitHub
              </a>
            )}
          </div>
        </div>
        <ul style={{ paddingLeft: 18, margin: 0, lineHeight: 1.8 }}>
          <li>Backend target: Rust + Axum + SQLx</li>
          <li>Frontend target: React + Vite + Tailwind v4</li>
          <li>Core primitive: addressable libraries and snippets</li>
          <li>Auth target: GitHub OAuth (direct MVP flow)</li>
        </ul>
      </section>
    </main>
  );
}

function buttonStyle(variant: "primary" | "secondary") {
  const isPrimary = variant === "primary";

  return {
    appearance: "none",
    textDecoration: "none",
    border: isPrimary ? "1px solid #1f1a17" : "1px solid rgba(31, 26, 23, 0.16)",
    background: isPrimary ? "#1f1a17" : "transparent",
    color: isPrimary ? "#fffdf8" : "#1f1a17",
    borderRadius: 999,
    padding: "12px 18px",
    fontSize: 14,
    fontWeight: 600,
    cursor: "pointer"
  } as const;
}
