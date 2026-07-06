use maxminddb::Reader;
use rustls::{
    DigitallySignedStruct,
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    pki_types::{CertificateDer, ServerName, UnixTime},
};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::{net::TcpStream, time::timeout};
use tokio_rustls::TlsConnector;
use x509_parser::parse_x509_certificate;

use crate::models::ScanResult;

#[derive(Debug)]
pub struct DummyVerifier;

impl ServerCertVerifier for DummyVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA1,
            rustls::SignatureScheme::ECDSA_SHA1_Legacy,
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ED448,
        ]
    }
}

pub fn is_internal_ip(ip: &IpAddr) -> bool {
    if ip.is_loopback() || ip.is_multicast() || ip.is_unspecified() {
        return true;
    }
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            if octets[0] == 0 {
                return true;
            }
            if octets[0] == 10 {
                return true;
            }
            if octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 {
                return true;
            }
            if octets[0] == 192 && octets[1] == 168 {
                return true;
            }
            if octets[0] == 192 && octets[1] == 0 && octets[2] == 0 {
                return true;
            }
            if octets[0] == 192 && octets[1] == 0 && octets[2] == 2 {
                return true;
            }
            if octets[0] == 169 && octets[1] == 254 {
                return true;
            }
            if octets[0] == 100 && octets[1] >= 64 && octets[1] <= 127 {
                return true;
            }
            if octets[0] == 198 && (octets[1] == 18 || octets[1] == 19) {
                return true;
            }
            if octets[0] == 198 && octets[1] == 51 && octets[2] == 100 {
                return true;
            }
            if octets[0] == 203 && octets[1] == 0 && octets[2] == 113 {
                return true;
            }
            if octets[0] >= 240 {
                return true;
            }
            false
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            if segments[0] & 0xfe00 == 0xfc00 {
                return true;
            }
            if segments[0] & 0xffc0 == 0xfe80 {
                return true;
            }
            if segments[0] == 0x2001 && segments[1] == 0x0db8 {
                return true;
            }
            false
        }
    }
}

pub fn default_fail_result(ip: IpAddr, port: u16, origin: String, geo_code: String) -> ScanResult {
    ScanResult {
        ip: ip.to_string(),
        port,
        origin,
        tls_version: "".into(),
        alpn: "".into(),
        cert_length: "".into(),
        cert_signature: "".into(),
        cert_publickey: "".into(),
        cert_domain: "".into(),
        cert_issuer: "".into(),
        geo_code,
        feasible: false,
    }
}

pub async fn scan_tls(
    ip: IpAddr,
    origin: String,
    port: u16,
    timeout_secs: u64,
    tls_connector: TlsConnector,
    geo_reader: Option<Arc<Reader<Vec<u8>>>>,
) -> ScanResult {
    let mut geo_code = "N/A".to_string();
    if let Some(geo) = &geo_reader
        && let Ok(country) = geo.lookup::<maxminddb::geoip2::Country>(ip)
        && let Some(c) = country.country
        && let Some(iso) = c.iso_code
    {
        geo_code = iso.to_string();
    }

    let addr = SocketAddr::new(ip, port);
    let connect_future = TcpStream::connect(addr);
    let tcp_stream = match timeout(Duration::from_secs(timeout_secs), connect_future).await {
        Ok(Ok(s)) => s,
        _ => return default_fail_result(ip, port, origin, geo_code),
    };

    let server_name = ServerName::try_from(origin.clone())
        .unwrap_or_else(|_| ServerName::try_from("localhost").unwrap())
        .to_owned();

    let tls_future = tls_connector.connect(server_name, tcp_stream);
    let tls_stream = match timeout(Duration::from_secs(timeout_secs), tls_future).await {
        Ok(Ok(s)) => s,
        _ => return default_fail_result(ip, port, origin, geo_code),
    };

    let (_, connection) = tls_stream.into_inner();

    let alpn = connection
        .alpn_protocol()
        .map(|v| String::from_utf8_lossy(v).to_string())
        .unwrap_or_default();
    let tls_version = match connection.protocol_version() {
        Some(rustls::ProtocolVersion::TLSv1_3) => "TLS 1.3".to_string(),
        Some(rustls::ProtocolVersion::TLSv1_2) => "TLS 1.2".to_string(),
        _ => "Unknown".to_string(),
    };

    let certs = connection.peer_certificates().unwrap_or(&[]);
    let cert_count = certs.len();
    let cert_length: usize = certs.iter().map(|c| c.len()).sum();
    let cert_len_str = format!("{}(certs count: {})", cert_length, cert_count);

    let mut cert_domain = String::new();
    let mut cert_issuer = String::new();
    let mut cert_signature = String::new();
    let mut cert_publickey = String::new();

    if let Some(leaf) = certs.first()
        && let Ok((_, parsed_cert)) = parse_x509_certificate(leaf.as_ref())
    {
        cert_domain = parsed_cert.subject().to_string();
        cert_issuer = parsed_cert.issuer().to_string();
        cert_signature = format!("{:?}", parsed_cert.signature_algorithm.algorithm);
        cert_publickey = format!(
            "{:?}",
            parsed_cert.tbs_certificate.subject_pki.algorithm.algorithm
        );
    }

    let feasible = tls_version == "TLS 1.3"
        && alpn == "h2"
        && !cert_domain.is_empty()
        && !cert_issuer.is_empty();

    ScanResult {
        ip: ip.to_string(),
        port,
        origin,
        tls_version,
        alpn,
        cert_length: cert_len_str,
        cert_signature,
        cert_publickey,
        cert_domain,
        cert_issuer,
        geo_code,
        feasible,
    }
}
