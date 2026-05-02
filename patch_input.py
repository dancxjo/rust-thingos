import re

with open("thingos/bloom/src/input.rs", "r") as f:
    content = f.read()

content = content.replace(
    "if bytes.len() < BristleEventHeader::SIZE {",
    "if bytes.len() < abi::hid::WaylandIpcHeader::SIZE {"
)

content = content.replace(
    """        let mut hdr_bytes = [0u8; BristleEventHeader::SIZE];
        hdr_bytes.copy_from_slice(&bytes[..BristleEventHeader::SIZE]);
        let Ok(header) = BristleEventHeader::from_bytes(&hdr_bytes) else {
            return false;
        };
        let event_type = header.event_type;
        let timestamp_ns = header.timestamp_ns;
        let event_no = BLOOM_HANDLE_EVENT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        let _trace = BloomInputTrace::new(event_no, event_type);
        let payload = &bytes[BristleEventHeader::SIZE..];""",
    """        let mut hdr_bytes = [0u8; abi::hid::WaylandIpcHeader::SIZE];
        hdr_bytes.copy_from_slice(&bytes[..abi::hid::WaylandIpcHeader::SIZE]);
        let header = abi::hid::WaylandIpcHeader::from_bytes(&hdr_bytes);
        
        let event_type = header.opcode();
        let timestamp_ns = stem::monotonic_ns();
        let event_no = BLOOM_HANDLE_EVENT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        let _trace = BloomInputTrace::new(event_no, event_type);
        let payload = &bytes[abi::hid::WaylandIpcHeader::SIZE..];"""
)

content = content.replace(
    "Ok(EventType::PointerMove) if payload.len() >= PointerMovePayload::SIZE => {",
    "Ok(EventType::PointerMove) if payload.len() >= abi::hid::WaylandPointerMotion::SIZE => {"
)

content = content.replace(
    """                let mut p = [0u8; PointerMovePayload::SIZE];
                p.copy_from_slice(&payload[..PointerMovePayload::SIZE]);
                let move_ev = PointerMovePayload::from_bytes(&p);
                let dx = move_ev.dx;
                let dy = move_ev.dy;
                if defer_cursor_motion {
                    self.defer_cursor_motion(dx, dy, timestamp_ns);
                    return false;
                }
                let old_x = self.pointer_x;
                let old_y = self.pointer_y;
                self.pointer_x = self
                    .pointer_x
                    .saturating_add(dx as i32)
                    .clamp(0, self.output_w.saturating_sub(1));
                self.pointer_y = self
                    .pointer_y
                    .saturating_add(dy as i32)
                    .clamp(0, self.output_h.saturating_sub(1));""",
    """                let mut p = [0u8; abi::hid::WaylandPointerMotion::SIZE];
                p.copy_from_slice(&payload[..abi::hid::WaylandPointerMotion::SIZE]);
                let move_ev = abi::hid::WaylandPointerMotion::from_bytes(&p);
                let dx = (move_ev.x - self.pointer_x) as i16;
                let dy = (move_ev.y - self.pointer_y) as i16;
                if defer_cursor_motion {
                    self.defer_cursor_motion(dx, dy, timestamp_ns);
                    return false;
                }
                let old_x = self.pointer_x;
                let old_y = self.pointer_y;
                self.pointer_x = move_ev.x;
                self.pointer_y = move_ev.y;"""
)

content = content.replace(
    "Ok(EventType::PointerButtonDown) if payload.len() >= PointerButtonPayload::SIZE => {",
    "Ok(EventType::PointerButtonDown) if payload.len() >= abi::hid::WaylandPointerButton::SIZE => {"
)

content = content.replace(
    """                let mut p = [0u8; PointerButtonPayload::SIZE];
                p.copy_from_slice(&payload[..PointerButtonPayload::SIZE]);
                let btn = PointerButtonPayload::from_bytes(&p);""",
    """                let mut p = [0u8; abi::hid::WaylandPointerButton::SIZE];
                p.copy_from_slice(&payload[..abi::hid::WaylandPointerButton::SIZE]);
                let btn = abi::hid::WaylandPointerButton::from_bytes(&p);"""
)

content = content.replace(
    "Ok(EventType::PointerButtonUp) if payload.len() >= PointerButtonPayload::SIZE => {",
    "Ok(EventType::PointerButtonUp) if payload.len() >= abi::hid::WaylandPointerButton::SIZE => {"
)

content = content.replace(
    """                let mut p = [0u8; PointerButtonPayload::SIZE];
                p.copy_from_slice(&payload[..PointerButtonPayload::SIZE]);
                let btn = PointerButtonPayload::from_bytes(&p);""",
    """                let mut p = [0u8; abi::hid::WaylandPointerButton::SIZE];
                p.copy_from_slice(&payload[..abi::hid::WaylandPointerButton::SIZE]);
                let btn = abi::hid::WaylandPointerButton::from_bytes(&p);"""
)

with open("thingos/bloom/src/input.rs", "w") as f:
    f.write(content)

