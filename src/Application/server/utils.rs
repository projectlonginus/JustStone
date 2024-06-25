use crate::utility::secure::utils::{AesGcmSivCrypto, RsaCrypto};
use std::collections::{HashSet, HashMap};
use std::net::Ipv4Addr;
use std::net::TcpStream;
use uuid::Uuid;
pub struct Server {
    pub(crate) max_sessions: usize,
    pub(crate) max_threads: usize,
    pub(crate) default_session_setup: SessionsSetup,
    pub(crate) session_list: HashSet<Uuid>,
    pub(crate) sessions : HashMap<Uuid, SessionInfo>,
    pub(crate) ipv4addr: Ipv4Addr,
}

pub struct SessionsSetup{
    pub(crate) use_encryption: bool,
    pub(crate) use_compression: bool,
    pub(crate) aes_cipher: AesGcmSivCrypto,
    pub(crate) rsa_cipher: RsaCrypto,
}

pub struct SessionInfo {
    pub(crate) session_id: Uuid,
    pub(crate) ipv4addr: Ipv4Addr,
    pub(crate) socket: TcpStream,
    pub(crate) setup: SessionsSetup,
}