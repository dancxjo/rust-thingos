# Feature: vfs_test_provider — synthetic VFS provider for ServiceLoop stress and fault testing

> Last run: 2026-04-25 13:10:40

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| vfs_test_provider mounts at /dev/test/provider | 0/0 | ✅ | [View Details](vfs-test-provider-mounts-at-dev-test-provider/README.md) |
| fast endpoint responds immediately | 0/0 | ✅ | [View Details](fast-endpoint-responds-immediately/README.md) |
| slow endpoint responds after a delay | 0/0 | ✅ | [View Details](slow-endpoint-responds-after-a-delay/README.md) |
| large endpoint delivers a large payload | 0/0 | ✅ | [View Details](large-endpoint-delivers-a-large-payload/README.md) |
| burst endpoint is readable repeatedly without provider crash | 0/0 | ✅ | [View Details](burst-endpoint-is-readable-repeatedly-without-provider-crash/README.md) |
| provider directory listing includes all endpoints | 0/0 | ✅ | [View Details](provider-directory-listing-includes-all-endpoints/README.md) |
| system remains stable after concurrent access to fast endpoint | 0/0 | ✅ | [View Details](system-remains-stable-after-concurrent-access-to-fast-endpoint/README.md) |
| hang endpoint causes client to wait but does not crash provider | 0/0 | ✅ | [View Details](hang-endpoint-causes-client-to-wait-but-does-not-crash-provider/README.md) |
| provider shutdown is clean via inbox message | 0/0 | ✅ | [View Details](provider-shutdown-is-clean-via-inbox-message/README.md) |
