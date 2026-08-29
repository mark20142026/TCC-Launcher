-- TCC backend schema for Supabase.
-- Apply via the Supabase dashboard SQL editor, or `psql $DATABASE_URL -f schema.sql`.
--
-- RLS is enabled with no public policies on purpose: the Vercel functions read
-- these tables with the server-side secret key, so the anon/publishable keys
-- must NOT be able to read or write anything.

create table if not exists public.changelog (
    id         bigint generated always as identity primary key,
    version    text,
    body_md    text not null,
    created_at timestamptz not null default now()
);

create table if not exists public.terms (
    id         bigint generated always as identity primary key,
    document   jsonb not null,
    created_at timestamptz not null default now()
);

create table if not exists public.releases (
    id         bigint generated always as identity primary key,
    version    text not null,
    platform   text not null,          -- e.g. windows-x86_64, darwin-aarch64, linux-x86_64
    url        text not null,
    signature  text,                   -- minisign signature, when publishing our own builds
    pub_date   timestamptz not null default now(),
    notes      text,
    created_at timestamptz not null default now(),
    unique (version, platform)
);

alter table public.changelog enable row level security;
alter table public.terms    enable row level security;
alter table public.releases enable row level security;
