use abi::{display_driver_protocol as drvproto, display_protocol};

fn encode_payload_message(msg_type: u16, payload: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0; drvproto::HEADER_SIZE + payload.len()];
    let len = drvproto::encode_message(&mut bytes, msg_type, payload).expect("encode message");
    bytes.truncate(len);
    bytes
}

#[test]
fn compositor_and_driver_negotiate_common_capabilities() {
    let hello = drvproto::HelloPayload {
        proto_major: drvproto::PROTO_MAJOR,
        proto_minor: drvproto::PROTO_MINOR,
        want_caps: drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME | drvproto::CAP_FENCE,
    };
    let mut hello_payload = [0; drvproto::HELLO_PAYLOAD_WIRE_SIZE];
    drvproto::encode_hello_payload_le(&hello, &mut hello_payload).expect("encode hello");

    let client_msg = encode_payload_message(drvproto::MSG_HELLO, &hello_payload);
    let (header, payload) = drvproto::parse_message(&client_msg).expect("driver parses hello");
    assert_eq!(header.msg_type, drvproto::MSG_HELLO);

    let requested = drvproto::decode_hello_payload_le(payload).expect("decode hello");
    let driver_supported = drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME;
    let welcome = drvproto::WelcomePayload {
        proto_major: requested.proto_major,
        proto_minor: requested.proto_minor,
        have_caps: requested.want_caps & driver_supported,
        max_rects: 32,
        reserved: 0,
    };
    let mut welcome_payload = [0; drvproto::WELCOME_PAYLOAD_WIRE_SIZE];
    drvproto::encode_welcome_payload_le(&welcome, &mut welcome_payload).expect("encode welcome");

    let driver_msg = encode_payload_message(drvproto::MSG_WELCOME, &welcome_payload);
    let (header, payload) = drvproto::parse_message(&driver_msg).expect("client parses welcome");
    assert_eq!(header.msg_type, drvproto::MSG_WELCOME);

    let negotiated = drvproto::decode_welcome_payload_le(payload).expect("decode welcome");
    assert_eq!(negotiated.have_caps, drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME);
    assert_eq!(negotiated.have_caps & drvproto::CAP_FENCE, 0);
    assert_eq!(negotiated.max_rects, 32);
}

#[test]
fn framebuffer_offer_accept_and_present_round_trip() {
    let offer = drvproto::OfferFramebufferPayload {
        handle: 7,
        _pad: 0,
        width: 800,
        height: 600,
        stride: 800 * 4,
        format: display_protocol::FORMAT_XRGB8888,
    };
    let mut offer_payload = [0; drvproto::OFFER_FRAMEBUFFER_PAYLOAD_WIRE_SIZE];
    drvproto::encode_offer_framebuffer_payload_le(&offer, &mut offer_payload)
        .expect("encode framebuffer offer");

    let offer_msg = encode_payload_message(drvproto::MSG_OFFER_FRAMEBUFFER, &offer_payload);
    let (header, payload) = drvproto::parse_message(&offer_msg).expect("client parses offer");
    assert_eq!(header.msg_type, drvproto::MSG_OFFER_FRAMEBUFFER);

    let decoded_offer =
        drvproto::decode_offer_framebuffer_payload_le(payload).expect("decode offer");
    assert_eq!(decoded_offer.handle, 7);
    assert_eq!(decoded_offer.stride, decoded_offer.width * 4);
    assert_eq!(decoded_offer.format, display_protocol::FORMAT_XRGB8888);

    let accept = drvproto::AcceptFramebufferPayload { accepted: 1, _pad: 0 };
    let mut accept_payload = [0; drvproto::ACCEPT_FRAMEBUFFER_PAYLOAD_WIRE_SIZE];
    drvproto::encode_accept_framebuffer_payload_le(&accept, &mut accept_payload)
        .expect("encode framebuffer accept");

    let accept_msg = encode_payload_message(drvproto::MSG_ACCEPT_FRAMEBUFFER, &accept_payload);
    let (header, payload) = drvproto::parse_message(&accept_msg).expect("driver parses accept");
    assert_eq!(header.msg_type, drvproto::MSG_ACCEPT_FRAMEBUFFER);
    assert_eq!(drvproto::decode_accept_framebuffer_payload_le(payload).unwrap().accepted, 1);

    let rect = drvproto::Rect { x: 0, y: 0, w: decoded_offer.width, h: decoded_offer.height };
    let mut present_payload = [0; drvproto::PRESENT_HEADER_WIRE_SIZE + drvproto::RECT_WIRE_SIZE];
    let len = drvproto::encode_present_payload_with_flags_le(
        1,
        drvproto::PRESENT_FLAG_FULLFRAME,
        [rect],
        &mut present_payload,
    )
    .expect("encode present");

    let present_msg = encode_payload_message(drvproto::MSG_PRESENT, &present_payload[..len]);
    let (header, payload) = drvproto::parse_message(&present_msg).expect("driver parses present");
    assert_eq!(header.msg_type, drvproto::MSG_PRESENT);

    let present = drvproto::decode_present_header_le(payload).expect("decode present header");
    assert_eq!(present.rect_count, 1);
    assert_eq!(present._pad, drvproto::PRESENT_FLAG_FULLFRAME);
    assert_eq!(
        drvproto::decode_rect_le(&payload[drvproto::PRESENT_HEADER_WIRE_SIZE..]).unwrap().w,
        800
    );
}
