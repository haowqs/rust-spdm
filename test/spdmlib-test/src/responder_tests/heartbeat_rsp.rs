// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use crate::common::device_io::{FakeSpdmDeviceIoReceve, SharedBuffer};
use crate::common::secret_callback::*;
use crate::common::transport::PciDoeTransportEncap;
use crate::common::util::create_info;
use codec::{Codec, Writer};
use spdmlib::common::session::{SpdmSession, SpdmSessionState};
use spdmlib::message::*;
use spdmlib::protocol::*;
use spdmlib::{responder, secret};

#[test]
fn test_case0_handle_spdm_heartbeat() {
    let (config_info, provision_info) = create_info();
    let pcidoe_transport_encap = &mut PciDoeTransportEncap {};
    let shared_buffer = SharedBuffer::new();
    let mut socket_io_transport = FakeSpdmDeviceIoReceve::new(&shared_buffer);
    secret::asym_sign::register(SECRET_ASYM_IMPL_INSTANCE.clone());
    let mut context = responder::ResponderContext::new(
        &mut socket_io_transport,
        pcidoe_transport_encap,
        config_info,
        provision_info,
    );

    let rsp_session_id = 0xffu16;
    let session_id = (0xffu32 << 16) + rsp_session_id as u32;
    context.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
    context.common.session = gen_array_clone(SpdmSession::new(), 4);
    context.common.session[0].setup(session_id).unwrap();
    context.common.session[0].set_crypto_param(
        SpdmBaseHashAlgo::TPM_ALG_SHA_384,
        SpdmDheAlgo::SECP_384_R1,
        SpdmAeadAlgo::AES_256_GCM,
        SpdmKeyScheduleAlgo::SPDM_KEY_SCHEDULE,
    );
    context.common.session[0].set_session_state(SpdmSessionState::SpdmSessionHandshaking);

    let bytes = &mut [0u8; 1024];
    let mut writer = Writer::init(bytes);
    let value = SpdmMessageHeader {
        version: SpdmVersion::SpdmVersion10,
        request_response_code: SpdmRequestResponseCode::SpdmRequestChallenge,
    };
    assert!(value.encode(&mut writer).is_ok());

    context.handle_spdm_heartbeat(session_id, bytes);
}
