# Feature: netd concurrent TCP and DNS/RPC stability

> Last run: 2026-04-24 11:14:27

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| DNS lookup completes while TCP fetch is in-progress | 0/0 | ✅ | [View Details](dns-lookup-completes-while-tcp-fetch-is-in-progress/README.md) |
| Multiple sequential DNS lookups do not degrade TCP connectivity | 0/0 | ✅ | [View Details](multiple-sequential-dns-lookups-do-not-degrade-tcp-connectivity/README.md) |
| TCP connect via hostname resolves through the deferred-connect path | 0/0 | ✅ | [View Details](tcp-connect-via-hostname-resolves-through-the-deferred-connect-path/README.md) |
