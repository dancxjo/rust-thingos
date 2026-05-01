# Bloom Rendering Acceleration – Implementation Audit

> **Date:** 2026-05-01  
> **Scope:** [`abi/src/display/`](../../abi/src/display/), [`bloom/src/`](../../bloom/src/), [`pistil/src/`](../../pistil/src/), [`drivers/display_virtio_gpu/src/`](../../drivers/display_virtio_gpu/src/), [`drivers/display_bootfb/src/`](../../drivers/display_bootfb/src/)  
> **Issue:** "Plan progressive Bloom rendering acceleration"

Legend: ✅ done · ⚠️ partial / in progress · 🔲 not started

---

## Stage 1 – Smarter CPU composition

| Task | Status | Notes |
|------|--------|-------|
| Damage clipping to output bounds | ✅ | `bloom/src/damage.rs::clip_rect` + `clip_all` |
| Rect merging (overlap + cheap-merge heuristic) | ✅ | `should_merge` uses area-ratio threshold (`MERGE_RATIO_NUM/DEN = 5/4`); `coalesce_in_place` is O(n²) over the bounded working set |
| Full-output fallback when rect count > threshold | ✅ | `MAX_DAMAGE_RECTS = 32`; `take()` collapses to single rect |
| Keep cursor damage separate (no cursor inflation) | ✅ | `DamageTracker` maintains a separate `cursor_regions` pool; `mark_cursor_rect` / `take_cursor` |
| Opaque-region culling | ✅ | `bloom/src/compositor.rs::cull_composition` walks top-to-bottom, maintains `covered` rect list |
| Skip fully-occluded lower surfaces | ✅ | `hidden_regions_skipped` counter; `is_fully_covered` check |
| Fast-path opaque α=255 planes (row copy instead of blend) | ✅ | `plane_can_use_gpu_path()` in `display_virtio_gpu`; opaque planes use `copy_nonoverlapping` into the frame-pool buffer |
| Detect fullscreen-opaque + direct-present | ✅ | `fullscreen_direct_present` counter in `CompositorCounters`; `cull_composition` detects topmost fully-opaque surface at (0,0) that fills the output |

**Acceptance check:**

- ✅ Small damage stays small (cursor motion ≠ full-screen blit).
- ✅ Opaque windows reduce blending work.
- ✅ Fullscreen opaque window avoids unnecessary composition (flagged; compositor strips lower planes).
- ✅ QEMU compositor demo still works.

---

## Stage 2 – Hardware cursor / cursor-only commits

| Task | Status | Notes |
|------|--------|-------|
| `DisplayCaps::HARDWARE_CURSOR` flag | ✅ | `abi/src/display/types.rs` bit 0 |
| `DISPLAY_OP_SET_CURSOR` (upload image + hotspot) | ✅ | ABI ioctl defined; `bloom/src/display.rs::set_cursor_image` |
| `DISPLAY_OP_MOVE_CURSOR` (reposition without recompose) | ✅ | ABI ioctl defined; `bloom/src/display.rs::move_cursor` |
| Virtio GPU cursor queue implementation | ✅ | `display_virtio_gpu` uses cursor DMA buffer; advertises `HARDWARE_CURSOR` when cursor queue + DMA buffer are available |
| Software cursor fallback | ✅ | `world.rs::try_present` falls back to software cursor plane when `supports_hw_cursor()` is false or `set_cursor_image` fails |
| Cursor-only frame path (no window recomposite) | ✅ | `damage.has_only_cursor_damage()` → `display.move_cursor()` → return early without full scene recomposition |

**Acceptance check:**

- ✅ Cursor movement does not dirty window content when hardware cursor is available.
- ✅ Software cursor fallback remains correct.

---

## Stage 3 – Display capability negotiation

| Capability | Flag bit | Defined | Reported by virtio GPU | Bloom queries | Notes |
|------------|----------|---------|------------------------|---------------|-------|
| `HW_CURSOR` | 0 | ✅ | ✅ (conditional) | ✅ | Used to gate hw-cursor path |
| `OVERLAYS` | 1 | ✅ | 🔲 | 🔲 | Not used yet |
| `ATOMIC` | 2 | ✅ | ✅ | 🔲 | Reported, not branched on |
| `VBLANK` | 3 | ✅ | 🔲 | ✅ | `supports_vblank()` exists; virtio GPU does not yet advertise it |
| `DMABUF_IMPORT` | 4 | ✅ | ✅ | 🔲 | Reported, not branched on |
| `GPU_BLIT` | 5 | ✅ | ✅ | ✅ | `supports_gpu_blit()` exists; Bloom does not yet take a separate branch |
| `GPU_ALPHA_BLEND` | 6 | ✅ | 🔲 | ✅ | Defined; virtio GPU not advertising yet (CPU blend today) |
| `GPU_SCALE` | 7 | ✅ | 🔲 | ✅ | Defined; not advertised or used |
| `GPU_ROUNDED_CLIP` | 8 | ✅ | 🔲 | ✅ | Defined; not advertised or used |
| `DIRECT_SCANOUT` | 9 | ✅ | ✅ | 🔲 | Reported, not branched on by Bloom |
| `PARTIAL_FLUSH` | 10 | ✅ | ✅ | 🔲 | Reported; backend respects damage rects in `transfer_to_host` |
| `FENCES` | 11 | ✅ | 🔲 | 🔲 | Defined; not implemented |
| `RESOURCE_CACHE` | 12 | ✅ | ✅ | 🔲 | Reported; cache lives in Bloom today |
| `ACCEL2D_CLEAR` | 13 | ✅ | 🔲 | 🔲 | Defined (see Stage 5) |
| `ACCEL2D_COPY` | 14 | ✅ | 🔲 | 🔲 | Defined (see Stage 5) |
| `ACCEL2D_STRETCH` | 15 | ✅ | 🔲 | 🔲 | Defined (see Stage 5) |
| `ACCEL2D_ALPHA_BLIT` | 16 | ✅ | 🔲 | 🔲 | Defined (see Stage 5) |
| `ACCEL2D_MASKED_BLIT` | 17 | ✅ | 🔲 | 🔲 | Defined (see Stage 5) |
| `ACCEL2D_ROUNDED_CLIP_BLIT` | 18 | ✅ | 🔲 | 🔲 | Defined (see Stage 5) |
| `ACCEL2D_FLUSH_DAMAGE` | 19 | ✅ | 🔲 | 🔲 | Defined (see Stage 5) |

**Overall:** The capability vocabulary is complete in the ABI. Virtio GPU reports a reasonable baseline subset. Bloom has `supports_*()` query methods for all the GPU-acceleration caps but currently only acts on `HARDWARE_CURSOR`. The remaining capability bits are present as forward-compatible scaffolding.

---

## Stage 4 – Resource caching

| Task | Status | Notes |
|------|--------|-------|
| Client buffer import cache | ✅ | `bloom/src/cache.rs::ResourceCache::import_client_buffer`; keyed on `(handle, format, w, h, stride, modifier)` |
| Cache hit/miss/invalidation counters | ✅ | `CacheCounters`; logged every 300 frames |
| Buffer eviction on client release | ✅ | `release_client_buffer` removes key from map and calls `display.release_buffer` |
| Full invalidation on backend reset | ✅ | `invalidate_all` releases every cached buffer |
| Wallpaper path cache | ✅ | `wallpaper_path` slot in `ResourceCache`; prevents re-importing unchanged wallpaper |
| Shadow-atlas buffer cache slot | ✅ | `shadow_buffer_id` slot; `set_shadow_buffer_id` / `invalidate_shadow` |
| Window chrome buffer cache slot | ⚠️ | Reserved as a comment (`chrome_buffer_id: Option<u32>`); not yet populated |
| Rounded-mask buffer cache slot | ⚠️ | Reserved as a comment; not yet populated |
| Glyph-atlas buffer cache slot | 🔲 | Not reserved; depends on Stage 7 glyph atlas |
| Cache keyed on buffer generation (dirty flag) | ⚠️ | Currently keyed on handle+metadata; no explicit generation counter. A handle reuse with new pixels would not re-import (the handle is always released first, so this is usually safe, but there is no monotonic generation field in the ABI yet) |

---

## Stage 5 – Small GPU 2D command model

| Item | Status | Notes |
|------|--------|-------|
| `ACCEL2D_CMD_CLEAR_RECT` ABI type | ✅ | `abi/src/display/accel2d.rs` |
| `ACCEL2D_CMD_COPY_RECT` ABI type | ✅ | Same |
| `ACCEL2D_CMD_STRETCH_BLIT` ABI type | ✅ | Same |
| `ACCEL2D_CMD_ALPHA_BLIT` ABI type | ✅ | Same |
| `ACCEL2D_CMD_MASKED_BLIT` ABI type | ✅ | Same |
| `ACCEL2D_CMD_ROUNDED_CLIP_BLIT` ABI type | ✅ | Same |
| `ACCEL2D_CMD_FLUSH_DAMAGE` ABI type | ✅ | Same |
| `DISPLAY_OP_ACCEL2D` ioctl constant | ✅ | `abi/src/display/ioctl.rs` |
| `Accel2dBatch` header + `Accel2dCommand` union wire types | ✅ | Complete |
| Boot-fb CPU fallback for all 7 commands | ✅ | `display_bootfb/src/driver.rs::execute_accel2d` + `execute_accel2d_cmd` |
| Virtio GPU implementation of `DISPLAY_OP_ACCEL2D` | 🔲 | Not handled in `vfs_device_call`; the switch statement does not include this opcode |
| Bloom issuing Accel2d commands | 🔲 | Bloom still uses only the plane-commit (`DISPLAY_OP_COMMIT`) path |

**Summary:** The ABI is fully specified and boot-fb has CPU fallbacks for every command. The integration work – virtio GPU handler and Bloom call sites – is the remaining gap.

---

## Stage 6 – GPU compositor path in display backend

| Task | Status | Notes |
|------|--------|-------|
| CPU fallback composition path preserved | ✅ | Per-pixel `alpha_over_argb` loop in `display_virtio_gpu` |
| Opaque-plane fast path (skip blend) | ✅ | `plane_can_use_gpu_path()` — `alpha==255 && !has_alpha && no_rounded_clip` → `copy_nonoverlapping` into frame-pool buffer |
| Rounded-clip coverage helper | ✅ | `rounded_clip_coverage()` in `display_virtio_gpu`; produces per-pixel coverage byte |
| Per-plane path counter (`gpu_path_planes` / `cpu_fallback_planes`) | ✅ | In `VirtioGpuDriver` struct; logged on commit |
| True GPU alpha-blend (not CPU per-pixel) | 🔲 | All non-opaque planes still use CPU `alpha_over_argb`; the GPU_ALPHA_BLEND capability is defined but not advertised or used |
| GPU copy for opaque planes (no CPU memcpy into framebuffer) | ⚠️ | Currently "GPU path" means CPU `copy_nonoverlapping` into the frame-pool DMA buffer, then `TRANSFER_TO_HOST_2D + RESOURCE_FLUSH`. It avoids alpha arithmetic but is not zero-copy; the CPU still touches every pixel for opaque planes |
| GPU scaling for destination-rect resize | 🔲 | Not implemented |
| Damage-respecting transfer (partial flush) | ✅ | `transfer_to_host_with_stride(res_id, dmg, stride)` uses the damage rect, not the full frame |
| Plane-list → GPU blit command conversion | 🔲 | Planes are still rasterised into the CPU-side frame-pool buffer rather than issued as GPU operations |

---

## Stage 7 – Pistil and Blossom acceleration

| Task | Status | Notes |
|------|--------|-------|
| Nine-slice shadow atlas | ✅ | `pistil/src/shadow.rs`; static cached active/inactive patches, full nine-slice blit (corners + stretched edges, center skipped) |
| Cached gradient strips | ✅ | `pistil/src/gradient.rs::GradientCache`; LRU-ish cache capped at `GRADIENT_CACHE_CAPACITY = 8` entries; `get_or_create` + `draw` |
| Rounded-mask cache | ✅ | `pistil/src/mask.rs::MaskCache`; masks keyed on `(w, h, radius)` |
| Glyph atlas stub | ✅ | `pistil/src/glyph.rs::GlyphAtlas`; API scaffolding only — no actual glyph rasterisation yet |
| Bloom calling pistil for chrome / shadows | ✅ | `bloom/src/render.rs` uses pistil primitives for compositor-owned drawing |
| Shadow-atlas GPU import (as display buffer) | ⚠️ | `ResourceCache::shadow_buffer_id` slot exists; not yet wired to upload and reference the pistil patch as a display buffer |
| Nine-slice blit via Accel2d `COPY_RECT` / `ALPHA_BLIT` | 🔲 | Pistil blits into the Bloom-owned chrome overlay buffer on the CPU; could be replaced by Accel2d commands once Stage 5 is complete |
| Glyph atlas full implementation | 🔲 | Depends on font/text rendering pipeline |

---

## First-milestone summary

The four items called out as "cheap and testable" first wins:

| Milestone item | Status |
|----------------|--------|
| 1. Damage coalescing | ✅ Complete |
| 2. Opaque-region culling | ✅ Complete |
| 3. Cursor-only / hardware-cursor capability | ✅ Complete |
| 4. Fullscreen opaque direct-present fast path | ✅ Complete (detected and flagged; lower planes are stripped) |

All first-milestone items are shipped and passing their unit tests.

---

## What's left (priority order)

1. **Stage 5 – Virtio GPU `DISPLAY_OP_ACCEL2D` handler.**  
   Boot-fb CPU fallbacks exist; wiring the opcode into `display_virtio_gpu::vfs_device_call` and advertising the matching `ACCEL2D_*` capability bits is the immediate next step.

2. **Stage 6 – True GPU alpha blend in virtio GPU.**  
   `GPU_ALPHA_BLEND` is defined and the query helper exists, but the driver still does per-pixel CPU blending. This requires either a Virgl 3D context for GPU-side blending or a software-accelerated path that avoids touching every pixel on the CPU.

3. **Stage 4 – Buffer generation counter.**  
   Add a monotonic generation field to the buffer import ABI so that a re-attached buffer with new pixel content (same handle) can be detected without relying solely on release-then-reattach ordering.

4. **Stage 4 – Chrome and rounded-mask cache slots.**  
   The `ResourceCache` has comment placeholders; populate them once the compositor-owned overlay and mask buffers stabilise.

5. **Stage 7 – Shadow-atlas GPU import.**  
   Upload the pistil nine-slice patch as a display buffer once and blit it per frame via `ACCEL2D_ALPHA_BLIT` instead of CPU-blending it into the chrome overlay buffer every frame.

6. **Stage 7 – Glyph atlas full implementation.**  
   The API stub is in place; needs a font rasteriser integration.

7. **Stage 3 – Bloom branching on `GPU_BLIT` / `DIRECT_SCANOUT`.**  
   The capability bits are advertised by the driver but Bloom does not yet take separate code paths based on them (other than `HARDWARE_CURSOR`). Direct-scanout for a fullscreen opaque client (already detected) would be the first concrete use of `DIRECT_SCANOUT`.
