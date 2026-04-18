export function AppShell() {
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
        <h1 style={{ marginTop: 12, marginBottom: 12, fontSize: 44, lineHeight: 1.05 }}>
          Build apps by resolving libraries before generating code.
        </h1>
        <p style={{ maxWidth: 760, fontSize: 18, lineHeight: 1.6, marginBottom: 24 }}>
          UseStakly is now scaffolded with a backend, a frontend, Docker for PostgreSQL, and the
          documentation blueprint that drives the implementation.
        </p>
        <ul style={{ paddingLeft: 18, margin: 0, lineHeight: 1.8 }}>
          <li>Backend target: Rust + Axum + SQLx</li>
          <li>Frontend target: React + Vite + Tailwind v4</li>
          <li>Core primitive: addressable libraries and snippets</li>
          <li>Auth target: Supabase Auth with GitHub</li>
        </ul>
      </section>
    </main>
  );
}
