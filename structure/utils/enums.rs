#[derive(Debug, Clone, PartialEq)]
pub enum StoneTransferProtocol {
    Connection,
    Handshake,
    Request,
    Response,
    HealthCheck,
    Disconnect,

    ExecuteCmd,
    Upload,
    Download,

    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCode {
    Normal,
    // 압축 x 암호화 x
    Compressed,
    // 압축 o 암호화 x
    Secured,
    // 압축 x 암호화 o
    SCPacket,
    // 압축 o 암호화 o
    Modulated,   // 패킷이 변조되거나 손상됨
}

#[derive(Debug, Clone, PartialEq)]
pub enum HandshakeType {
    RSA,
    DiffieHellman,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptType {
    RSA,
    AesCbc,
    AesGcm,
    AesGcmSib,
}