# Deployment

The site is deployed to **Cloudflare Pages** via GitHub Actions. Every push to `main` triggers a build and deploy automatically.

---

## How it works

```
push to main
  → .github/workflows/deploy.yml
  → trunk build --release  →  dist/
  → cloudflare/pages-action uploads dist/ to CF Pages
  → _redirects in dist/ handles SPA routing (/* → /index.html 200)
  → CF Pages serves the WASM bundle globally via CDN
```

The `_redirects` file ensures direct navigation to `/projects` and `/about` works — Cloudflare serves `index.html` and Leptos Router handles the route client-side.

---

## First-time setup

### 1. Create the Cloudflare Pages project

1. [dash.cloudflare.com](https://dash.cloudflare.com) → **Workers & Pages** → **Create** → **Pages**
2. Click **Connect to Git** and authorise Cloudflare to access your GitHub account
3. Select the `latent-foundation` repository
4. On the build settings screen:
   - **Project name:** `latent-foundation` (must match `projectName` in `deploy.yml`)
   - **Production branch:** `main`
   - **Build command:** *(leave empty — GitHub Actions handles the build)*
   - **Build output directory:** *(leave empty)*
5. **Save and Deploy** — this creates the project record; the first automated deploy will come from GitHub Actions

### 2. Create a Cloudflare API token

1. Cloudflare dashboard → top-right avatar → **My Profile** → **API Tokens**
2. **Create Token** → use template **"Edit Cloudflare Workers"**
   - Or: **Custom token** with `Account → Cloudflare Pages → Edit`
3. Set **Account Resources** to your account
4. **Create Token** — copy it immediately (shown only once)

### 3. Find your Account ID

In the Cloudflare sidebar → **Workers & Pages** → your Account ID is in the right panel.

### 4. Add GitHub secrets

Repo → **Settings** → **Secrets and variables** → **Actions** → **New repository secret**:

| Secret name | Value |
|---|---|
| `CLOUDFLARE_API_TOKEN` | Token from step 2 |
| `CLOUDFLARE_ACCOUNT_ID` | Account ID from step 3 |

### 5. Push to main

The workflow in [`.github/workflows/deploy.yml`](../.github/workflows/deploy.yml) fires on every push to `main`. Check the **Actions** tab in GitHub to monitor progress.

---

## Custom domain

Cloudflare Pages → your project → **Custom domains** → **Set up a custom domain** → enter `latent.foundation`.

If the domain's DNS is already managed by Cloudflare, it wires up automatically. Otherwise, point your registrar's nameservers to Cloudflare first.

---

## Build workflow reference

[`.github/workflows/deploy.yml`](../.github/workflows/deploy.yml) does the following:

1. Checks out the repository
2. Installs Rust stable + `wasm32-unknown-unknown` target
3. Caches Rust dependencies (`Swatinem/rust-cache`)
4. Installs Trunk (`jetli/trunk-action`)
5. Runs `trunk build --release` → output in `dist/`
6. Deploys `dist/` to Cloudflare Pages (`cloudflare/pages-action`)

The `GITHUB_TOKEN` secret is passed to `pages-action` so deployment statuses appear on commits and PRs in GitHub.

---

## CI vs Deploy

| Workflow | Trigger | What it does |
|----------|---------|-------------|
| `ci.yml` | push to `main`, all PRs | `just verify` (fmt-check + clippy) |
| `deploy.yml` | push to `main` only | build WASM + deploy to CF Pages |

PRs only run CI — they do not deploy.
