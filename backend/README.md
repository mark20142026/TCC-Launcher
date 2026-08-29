# TCC Backend (Vercel + Supabase)

Hosted backend for the TCC launcher. Deployed on Vercel, data in Supabase.
After deployment it lives at **https://api.theazizi.space** (covered by the
existing `*.theazizi.space` wildcard on Cloudflare — no DNS changes needed).

## Endpoints the launcher calls

The launcher's data base URL (`meta_url_base`) now points here. Paths:

| Path                        | What                                                | Backed by |
|-----------------------------|-----------------------------------------------------|-----------|
| `/oneclient/CHANGE_LOG.md`  | Changelog shown in settings                         | Supabase `changelog`, fallback: upstream |
| `/oneclient/tos.json`       | Terms document shown during onboarding              | Supabase `terms`, fallback: upstream |
| `/oneclient/latest.json`    | Autoupdater feed (minisign-signed)                  | Supabase `releases`, fallback: upstream |
| `/oneclient/versions/*`     | Version metadata manifest                           | Proxy to upstream (migration target) |
| `/oneclient/*` (everything) | Bundle archives, art, other launcher data           | Proxy to upstream (migration target) |
| `/health`                   | Health check                                        | Static |

The fallback/proxy strategy means the launcher works immediately on day one,
and each surface moves into Supabase as we populate the tables.

## Supabase tables

`changelog`, `terms`, `releases` — see `schema.sql`. RLS is on with no public
policies; only the Vercel functions (secret key) can read them.

## Deploying

```sh
cd backend
vercel login          # one time
vercel link           # create/select the project
vercel env add SUPABASE_URL          # paste from .env
vercel env add SUPABASE_SECRET_KEY
vercel --prod        # deploys, then run the vercel.json domain check
```

Then in the Vercel dashboard, add the domain `api.theazizi.space` to the
project (it validates instantly against the existing wildcard DNS).

## Applying the schema

Either paste `schema.sql` into the Supabase dashboard SQL editor, or hand a
database password / personal access token to the agent and it will apply it.
