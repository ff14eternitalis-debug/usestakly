$projectUuid = "k941ef9qz0aqh6nvcxbr2cgb"
$serverUuid = "pkwsw0ggw4so44s44wgscs88"
$environmentName = "production"
$repoUrl = "https://github.com/REPLACE_ME/usestakly.git"
$branch = "main"

# Backend application
coolify app create public `
  --server-uuid $serverUuid `
  --project-uuid $projectUuid `
  --environment-name $environmentName `
  --name "usestakly-backend" `
  --git-repository $repoUrl `
  --git-branch $branch `
  --build-pack dockerfile `
  --base-directory "backend" `
  --ports-exposes "4000" `
  --health-check-enabled `
  --health-check-path "/health" `
  --domains "api.usestakly.com"

# Frontend application
coolify app create public `
  --server-uuid $serverUuid `
  --project-uuid $projectUuid `
  --environment-name $environmentName `
  --name "usestakly-frontend" `
  --git-repository $repoUrl `
  --git-branch $branch `
  --build-pack dockerfile `
  --base-directory "frontend" `
  --ports-exposes "8080" `
  --health-check-enabled `
  --health-check-path "/health" `
  --domains "usestakly.com"
