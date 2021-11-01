use rustls::ClientConnection;
use std::io::prelude::*;
use std::io;
use std::net::TcpStream;

pub mod danger {
    use crate::errors::*;
    use rustls::{Certificate, ServerName};
    use rustls::client::ServerCertVerified;
    use sha2::{Sha256, Digest};
    use std::time::SystemTime;

    pub struct PinnedCertificateVerification {}

    fn verify_fingerprint(trusted: &ServerName, cert: &rustls::Certificate) -> Result<(), Error> {
        let trusted = if let ServerName::DnsName(name) = trusted {
            name.as_ref()
        } else {
            bail!("unsupported server name")
        };

        let idx = match trusted.find('-') {
            Some(idx) => idx,
            None => bail!("malformed fingerprint"),
        };

        let (algo, trusted_fp) = trusted.split_at(idx);

        let trusted_fp = base64::decode_config(&trusted_fp[1..], base64::URL_SAFE_NO_PAD).unwrap();

        let fingerprint = match algo {
            "SHA256" => {
                let mut h = Sha256::new();
                h.update(&cert.0);
                h.finalize().to_vec()
            },
            _ => bail!("unknown hash alog"),
        };

        if trusted_fp == fingerprint {
            Ok(())
        } else {
            bail!("untrusted fingerprint")
        }
    }

    impl rustls::client::ServerCertVerifier for PinnedCertificateVerification {
        fn verify_server_cert(
            &self,
            end_entity: &Certificate,
            _intermediates: &[Certificate],
            server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime
        ) -> Result<ServerCertVerified, rustls::Error> {
            if verify_fingerprint(server_name, &end_entity).is_ok() {
                Ok(ServerCertVerified::assertion())
            } else {
                Err(rustls::Error::General("Untrusted certificate".to_string()))
            }
        }
    }
}


#[derive(Debug)]
pub struct OwnedTlsStream {
    pub sess: rustls::ClientConnection,
    pub sock: TcpStream,
}

impl OwnedTlsStream {
    pub fn new(sess: ClientConnection, sock: TcpStream) -> OwnedTlsStream {
        OwnedTlsStream { sess, sock }
    }

    fn complete_prior_io(&mut self) -> io::Result<()> {
        if self.sess.is_handshaking() {
            self.sess.complete_io(&mut self.sock)?;
        }

        if self.sess.wants_write() {
            self.sess.complete_io(&mut self.sock)?;
        }

        Ok(())
    }
}

impl Read for OwnedTlsStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.complete_prior_io()?;

        if self.sess.wants_read() {
            self.sess.complete_io(&mut self.sock)?;
        }

        self.sess.reader().read(buf)
    }
}

impl Write for OwnedTlsStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.complete_prior_io()?;

        let len = self.sess.writer().write(buf)?;
        self.sess.complete_io(&mut self.sock)?;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.complete_prior_io()?;

        self.sess.writer().flush()?;
        if self.sess.wants_write() {
            self.sess.complete_io(&mut self.sock)?;
        }
        Ok(())
    }
}
