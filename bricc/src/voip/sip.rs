use std::{
    net::IpAddr,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::prefs::kv_store::KvStore;
use rand::Rng;
use serde::{Deserialize, Serialize};

const OUTGOING_PORT: u16 = 50606;
const SIP_THREAD_STACK_SIZE_BYTES: usize = 16384usize;

pub fn start_thread<KvStoreImpl: KvStore>(
    kv_store: &mut KvStoreImpl,
    external_ip: IpAddr,
) -> Result<JoinHandle<()>, String> {
    let enabled: bool = match kv_store.get("sip.0.en".into())? {
        Some(val) => val,
        None => false,
    };

    if !enabled {
        return Err("SIP line disabled".into());
    }

    let user: String = {
        match kv_store.get("sip.0.us".into())? {
            Some(val) => val,
            None => return Err("SIP line enabled but no user".into()),
        }
    };
    let password: String = {
        match kv_store.get("sip.0.pw".into())? {
            Some(val) => val,
            None => return Err("SIP line enabled but no pw".into()),
        }
    };
    let domain: String = {
        match kv_store.get("sip.0.dm".into())? {
            Some(val) => val,
            None => return Err("SIP line enabled but no domain".into()),
        }
    };
    let builder = thread::Builder::new().stack_size(SIP_THREAD_STACK_SIZE_BYTES);
    Ok(builder
        .spawn(|| loop {
            thread::sleep(Duration::from_secs(1));
        })
        .unwrap())
}

fn register(
    user: String,
    password: String,
    domain: String,
    external_ip: IpAddr,
) -> Result<rsip::SipMessage, String> {
    let display_name: String = "KYP".into();

    let tag_bytes: [u8; 4] = rand::thread_rng().gen();
    let tag_string = hex::encode(tag_bytes);
    let branch_bytes: [u8; 4] = rand::thread_rng().gen();
    let branch_string = hex::encode(tag_bytes);

    let mut headers: rsip::Headers = Default::default();

    let base_uri = rsip::Uri {
        scheme: Some(rsip::Scheme::Sips),
        auth: Some((user, Some(password)).into()),
        host_with_port: rsip::Domain::from(domain.clone()).into(),
        ..Default::default()
    };

    println!("IP: {}", external_ip.to_string());

    headers.push(
        rsip::typed::Via {
            version: rsip::Version::V2,
            transport: rsip::Transport::Tls,
            uri: rsip::Uri {
                host_with_port: (rsip::Domain::from(external_ip.to_string()), OUTGOING_PORT).into(),
                ..Default::default()
            },
            params: vec![rsip::Param::Branch(rsip::param::Branch::new(branch_string))],
        }
        .into(),
    );
    headers.push(rsip::headers::MaxForwards::default().into());
    headers.push(
        rsip::typed::From {
            display_name: Some(display_name.clone()),
            uri: base_uri.clone(),
            params: vec![rsip::Param::Tag(rsip::param::Tag::new(tag_string))],
        }
        .into(),
    );
    headers.push(
        rsip::typed::To {
            display_name: Some(display_name.clone()),
            uri: base_uri.clone(),
            params: Default::default(),
        }
        .into(),
    );
    headers.push(rsip::headers::CallId::default().into());
    headers.push(
        rsip::typed::CSeq {
            seq: 1,
            method: rsip::Method::Register,
        }
        .into(),
    );
    headers.push(
        rsip::typed::Contact {
            display_name: None,
            uri: base_uri,
            params: Default::default(),
        }
        .into(),
    );
    headers.push(rsip::headers::ContentLength::default().into());

    Ok(rsip::Request {
        method: rsip::Method::Register,
        uri: rsip::Uri {
            scheme: Some(rsip::Scheme::Sips),
            host_with_port: rsip::Domain::from(domain).into(),
            ..Default::default()
        },
        headers: headers,
        version: rsip::Version::V2,
        body: Default::default(),
    }
    .into())
}
