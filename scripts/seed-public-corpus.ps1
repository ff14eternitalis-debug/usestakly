param(
  [string]$Api = "https://mcp.usestakly.com"
)

$repos = @(
  # React tables and grids
  "TanStack/table",
  "ag-grid/ag-grid",
  "handsontable/handsontable",

  # UI kits and component systems
  "shadcn-ui/ui",
  "mui/material-ui",
  "chakra-ui/chakra-ui",
  "radix-ui/primitives",
  "tailwindlabs/headlessui",
  "ant-design/ant-design",
  "mantinedev/mantine",

  # Video / training content
  "remotion-dev/remotion",
  "obsproject/obs-studio",
  "FFmpeg/FFmpeg",
  "ManimCommunity/manim",
  "mifi/lossless-cut",
  "mltframework/shotcut",

  # TypeScript ORM / database
  "prisma/prisma",
  "drizzle-team/drizzle-orm",
  "typeorm/typeorm",
  "sequelize/sequelize",
  "knex/knex",

  # Auth
  "nextauthjs/next-auth",
  "better-auth/better-auth",
  "auth0/nextjs-auth0",

  # Validation
  "colinhacks/zod",
  "fabian-hiller/valibot",
  "jquense/yup",
  "ajv-validator/ajv",

  # HTTP clients
  "axios/axios",
  "sindresorhus/ky",
  "sindresorhus/got",

  # Testing
  "vitest-dev/vitest",
  "microsoft/playwright",
  "jestjs/jest",

  # Python web/API
  "fastapi/fastapi",
  "django/django",
  "pallets/flask",

  # Rust async/web
  "tokio-rs/tokio",
  "serde-rs/serde",
  "tokio-rs/axum"
)

foreach ($repo in $repos) {
  Write-Host "Seeding $repo"
  try {
    Invoke-RestMethod `
      -Method Post `
      -Uri "$Api/api/repos/add" `
      -ContentType "application/json" `
      -Body (@{ repo = $repo } | ConvertTo-Json) | Out-Null
  } catch {
    Write-Warning "Failed to seed ${repo}: $($_.Exception.Message)"
  }
}
