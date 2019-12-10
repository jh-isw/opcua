use std::fmt::Debug;
use std::cmp::PartialEq;
use std::io::Cursor;

use tempdir::TempDir;

use opcua_types::*;
use opcua_types::status_code::StatusCode;

use opcua_crypto::{
    pkey::PrivateKey, x509::{X509, X509Data}, certificate_store::*, security_policy::SecurityPolicy,
};

use crate::comms::secure_channel::SecureChannel;

pub fn serialize_test_and_return<T>(value: T) -> T
    where T: BinaryEncoder<T> + Debug + PartialEq
{
    // Ask the struct for its byte length
    let byte_len = value.byte_len();
    let mut stream = Cursor::new(vec![0u8; byte_len]);

    // Encode to stream
    let start_pos = stream.position();
    let result = value.encode(&mut stream);
    let end_pos = stream.position();
    assert!(result.is_ok());

    // This ensures the size reported is the same as the byte length impl
    assert_eq!(result.unwrap(), byte_len);

    // Test that the position matches the byte_len
    assert_eq!((end_pos - start_pos) as usize, byte_len);

    let actual = stream.into_inner();
    println!("value = {:?}", value);
    println!("encoded bytes = {:?}", actual);
    let mut stream = Cursor::new(actual);

    let decoding_limits = DecodingLimits::default();
    let new_value: T = T::decode(&mut stream, &decoding_limits).unwrap();
    println!("new value = {:?}", new_value);
    assert_eq!(value, new_value);
    new_value
}

pub fn serialize_test<T>(value: T)
    where T: BinaryEncoder<T> + Debug + PartialEq
{
    let _ = serialize_test_and_return(value);
}

/// Makes a secure channel
fn make_secure_channel(security_mode: MessageSecurityMode, security_policy: SecurityPolicy, local_nonce: Vec<u8>, remote_nonce: Vec<u8>) -> SecureChannel {
    let mut secure_channel = SecureChannel::new_no_certificate_store();
    secure_channel.set_security_mode(security_mode);
    secure_channel.set_security_policy(security_policy);
    secure_channel.set_local_nonce(&local_nonce);
    secure_channel.set_remote_nonce(&remote_nonce);
    secure_channel.derive_keys();
    secure_channel
}

/// Makes a pair of secure channels representing local and remote side to test crypto
fn make_secure_channels(security_mode: MessageSecurityMode, security_policy: SecurityPolicy) -> (SecureChannel, SecureChannel) {
    let local_nonce = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let remote_nonce = vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    let secure_channel1 = make_secure_channel(security_mode, security_policy, local_nonce.clone(), remote_nonce.clone());
    let secure_channel2 = make_secure_channel(security_mode, security_policy, remote_nonce.clone(), local_nonce.clone());
    (secure_channel1, secure_channel2)
}

fn make_open_secure_channel_response() -> OpenSecureChannelResponse {
    OpenSecureChannelResponse {
        response_header: ResponseHeader {
            timestamp: DateTime::now(),
            request_handle: 444,
            service_result: StatusCode::BadProtocolVersionUnsupported,
            service_diagnostics: DiagnosticInfo::default(),
            string_table: None,
            additional_header: ExtensionObject::null(),
        },
        server_protocol_version: 0,
        security_token: ChannelSecurityToken {
            channel_id: 1,
            token_id: 2,
            created_at: DateTime::now(),
            revised_lifetime: 777,
        },
        server_nonce: ByteString::null(),
    }
}

fn make_sample_message() -> SupportedMessage {
    GetEndpointsRequest {
        request_header: RequestHeader {
            authentication_token: NodeId::new(0, 99),
            timestamp: DateTime::now(),
            request_handle: 1,
            return_diagnostics: DiagnosticBits::empty(),
            audit_entry_id: UAString::null(),
            timeout_hint: 123456,
            additional_header: ExtensionObject::null(),
        },
        endpoint_url: UAString::null(),
        locale_ids: None,
        profile_uris: None,
    }.into()
}

struct Test;

impl Test {
    pub fn setup() -> Test {
        Test {}
    }
}

mod chunk;
mod services;
mod comms;
mod secure_channel;