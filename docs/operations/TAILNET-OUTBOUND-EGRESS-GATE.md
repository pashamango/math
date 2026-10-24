# NET-EGRESS-001 — Controlled Exit from the Tailnet

**Status:** PLAN ONLY. Purpose: support public-source GitHub reads, future public portfolio publication and *optional* Grafana Cloud sanitized metrics, without weakening the sovereign Tailnet perimeter.

## Trust boundaries

- **Inside sovereign perimeter:** Darwin E1 hash-ledger, DP-01B shadow, Muse source JSON + full journal, Wolfram actual Token Governor source, SSH credentials and Tailscale policy.
- **Allowed public outputs only after review:** public math release source/certificates, privacy-sanitized demo data, aggregate telemetry metrics with stable allowlisted names and very low cardinality.
- **External services:** GitHub (already authorized via connector but host-local GitHub auth may differ) and potentially Grafana Cloud Free. External data is not an instruction or authority source. Public data display is distinct from agent write/execute privileges.

Tailnet membership does NOT imply the machine needs a Tailscale exit node to access ordinary internet HTTPS. An exit node routes external traffic through a selected node and requires separate activation/approval. **Do not configure exit nodes, routes, DNS, SSH, serve/funnel or inbound listeners for this test.**

## Gate E0 — read-only connection test from Wolfram, not this ChatGPT sandbox

1. Verify correct host and ask the operator to run under their existing user.
2. Use **TLS-verified HTTPS only** (no `curl -k`, insecure certificates or credentials).
3. Public endpoint HEAD checks may be done with a bounded timeout; sanitize the output to success/error code and timestamp only, never emit local/Tailnet IPs, cookies, env variables, public SSH keys or verbose traces:

```sh
for url in https://github.com/ https://grafana.com/; do
  curl --proto '=https' --proto-redir '=https' \
    --max-time 8 --silent --show-error --location --head \
    --output /dev/null --write-out 'http_status=%{http_code}\n' \
    "$url" || echo 'HTTPS_HEAD_BLOCKED'
done
```

4. This confirms only public HTTPS/TLS reachability, **not** private GitHub repository authorization or safety of future data uploads. Authorized private Git operations must use the existing audited GitHub credentials/plugin and pin exact commit SHAs. Private `raw.githubusercontent.com` HTTP 404 does not prove the repo/ref is absent.
5. Receipt `NET_EGRESS_READONLY_RECEIPT/v1`: host attribution, endpoint classes (GitHub, Grafana), TLS and response outcome, git native `SOURCE_READ` separately from web, no network configuration changes.

## Gate E1 — no-secret synthetic cloud canary (separate explicit approval)

- Choose Grafana Cloud Free **only after** a human signs up/authorizes a stack and reviews provider terms and locale/data retention.
- Do NOT expose Darwin/Wolfram's local metrics endpoint to the internet; Grafana Cloud's public Metrics Endpoint scrape option would require an externally reachable target and is inappropriate for this topology.
- Grafana Cloud accepts Prometheus `remote_write` via an outbound, authenticated HTTPS client such as a separately vetted lightweight collector (e.g., Alloy). Grafana Cloud does **not** accept bare Prometheus Pushgateway protocol as remote_write. A `.prom` file alone is not an HTTPS remote_write payload.
- First test **synthetic series only** `simsim_canary_egress_value{source="synthetic"} 1`, with no real host names/IPs, telemetry, unique identifiers, cardinality explosion or production private content. An approved credential/token is to be held in the OS secret store or a private 0600 config; never commit to Git, Linear, receipts, dashboards, logs, chat or public URLs.
- Grant **minimum metrics write scope**, separate from read scope. Require explicit allowlist of destination remote_write HTTPS hostname and certificate checks; do not turn off VPN restrictions as workaround. Use TLS and authenticate endpoint. Set cost/cardinality ceilings and daily transmission cap.
- On the vendor side, verify actual returned query for synthetic series, then delete/expire test data by standard retention. Only afterward seek **separate approval** for deliberately redacted Muse aggregates and other host telemetry.
- Never claim that a single 200/204 response proves telemetry arrived correctly. Need remote query and source/receiver count consistency; classify remote metrics as EXTERNAL_AGENT_TESTIMONY if Muse-sourced.
- Preapprove a simple **rollback**: stop ONLY the new egress collector, revoke its narrow cloud write token, leave sovereign ledgers untouched and verify no new scheduled network task remains.

## Safety acceptance

`NET_EGRESS_SOURCE_READ_PASS` = outbound public HTTPS TLS test, without touching network controls.  
`GRAFANA_CANARY_PASS` = separate credentialed synthetic write **and** remote readback (not yet run).  
`TELEMETRY_EXPORT_APPROVED` = owner explicitly consents to each field/label exported after a privacy threat model (NOT implied by approval of the cloud concept).

Do not turn metrics absence into a zero; don't assert Work quota from Muse quota; don't publish source files or test vectors containing personal or tailnet metadata.

External references: [Grafana Cloud remote_write](https://grafana.com/docs/grafana-cloud/send-data/metrics/metrics-prometheus/) and [Tailscale exit nodes](https://tailscale.com/kb/1103/exit-nodes/).
