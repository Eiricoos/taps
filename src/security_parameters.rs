// This is a sekelton of security parameters: I am clueless here so this
// might be deleted, who knows
// Which means: NOTHING OF NOTE HERE, you can stop reading
// Why are you still reading, there is nothing here

use std::collections::HashSet;

// Placeholder for security protocols
enum SecurityProtocol {
    Tls1_2,
    Tls1_3,
}

// Placeholders for certificates
struct Certificate {
    member_1: i32,
    member_2: u32,
}

// Placeholder for pinned server certificate
struct CertificateChain {
    blabla_1: u32,
    blabla_2: i32,
}

// Another placeholder
enum SupportedGroup {
    Secp256r1,
}

// Yet another placeholder
enum Ciphersuite {
    TlsAes128GcmSha256,
}

// You guessed it, a placeholder
enum SignatureAlgorithm {
    Bliipbloop,
}

// Do I need to say this every time
enum SecurityParameterValue {
    ProtocolSet(HashSet<SecurityProtocol>),
    CertificateArray(Vec<Certificate>),
}

// Probably not
struct PreSharedKey {
    key: String,
    identity: String,
}

// Oh well
struct CallBack {
    something: String,
}

// Why are you still here
impl CallBack {
    pub fn new() -> Self {
        Self {
            something: String::from("BAH"),
        }
    }
}

struct SecurityParameters {
    allowed_security_protocols: HashSet<SecurityProtocol>,
    server_certificate: Vec<Certificate>,
    client_certificate: Vec<Certificate>,
    pinned_server_certificate: Vec<CertificateChain>,
    alpn: Vec<String>,
    supported_group: Vec<SupportedGroup>,
    ciphersuite: Vec<Ciphersuite>,
    signature_algorithm: Vec<SignatureAlgorithm>,
    max_cached_sessions: i32,
    cached_session_lifetime_seconds: i32,
    pre_shared_key: PreSharedKey,
}

impl SecurityParameters {
    // Constructor for the security parameters with default values
    // as given in RFC 9622
    pub fn new() {
        //
    }

    pub fn new_disabled() {
        //
    }

    pub fn new_opportunistic() {
        //
    }

    pub fn set() {
        // Set members
    }

    // Placeholder
    pub fn set_trust_verification_callback(trust: CallBack) {
        trust.something;
    }

    // Also, you guessed it, a placeholder
    pub fn set_identity_challenge_callback(challenge: CallBack) {
        challenge.something;
    }
}
