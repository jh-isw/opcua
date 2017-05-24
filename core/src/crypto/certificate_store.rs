//! Certificate manager for OPC UA for Rust.
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, Read};

use openssl::x509;
use openssl::x509::extension::*;
use openssl::pkey;
use openssl::rsa::*;
use openssl::asn1::*;
use openssl::hash::*;

use prelude::*;

/// The name that the server/client's certificate is expected to be
const OWN_CERTIFICATE_NAME: &'static str = "cert.der";
/// The name that the server/client's private key is expected to be
const OWN_PRIVATE_KEY_NAME: &'static str = "private.pem";

/// The directory holding the server/client's own cert
const OWN_CERTIFICATE_DIR: &'static str = "own";
/// The directory holding the server/client's own private key
const OWN_PRIVATE_KEY_DIR: &'static str = "private";
/// The directory holding trusted certificates
const TRUSTED_CERTS_DIR: &'static str = "trusted";
/// The directory holding rejected certificates
const REJECTED_CERTS_DIR: &'static str = "rejected";

pub struct CertificateStore {
    pub pki_path: PathBuf,
    pub check_issue_time: bool,
    pub check_expiration_time: bool,
}

impl CertificateStore {
    pub fn new(pki_path: &Path) -> CertificateStore {
        CertificateStore {
            pki_path: pki_path.to_path_buf(),
            check_issue_time: true,
            check_expiration_time: true
        }
    }

    /// Creates an X509 certificate from the creation args
    pub fn create_cert_and_pkey(args: &X509Data) -> Result<(X509, PKey), String> {
        // Create a keypair
        let pkey = {
            let rsa = Rsa::generate(args.key_size).unwrap();
            pkey::PKey::from_rsa(rsa).unwrap()
        };

        // Create an X509 cert (the public part) as a .pem
        let cert = {
            let mut builder: x509::X509Builder = x509::X509Builder::new().unwrap();
            let _ = builder.set_version(3);
            let subject_name = {
                let mut name = x509::X509NameBuilder::new().unwrap();
                name.append_entry_by_text("CN", &args.common_name).unwrap();
                name.build()
            };
            let _ = builder.set_subject_name(&subject_name);
            let issuer_name = {
                let mut name = x509::X509NameBuilder::new().unwrap();
                // Common name
                name.append_entry_by_text("CN", &args.common_name).unwrap();
                // Organization
                name.append_entry_by_text("O", &args.organization).unwrap();
                // Organizational Unit
                name.append_entry_by_text("OU", &args.organizational_unit).unwrap();
                // Country
                name.append_entry_by_text("C", &args.country).unwrap();
                // State
                name.append_entry_by_text("ST", &args.state).unwrap();
                name.build()
            };
            let _ = builder.set_issuer_name(&issuer_name);
            builder.set_not_before(&Asn1Time::days_from_now(0).unwrap()).unwrap();
            builder.set_not_after(&Asn1Time::days_from_now(args.certificate_duration_days).unwrap()).unwrap();
            builder.set_pubkey(&pkey).unwrap();

            // Alt hostnames
            if !args.alt_host_names.is_empty() {
                let subject_alternative_name = {
                    let mut subject_alternative_name = SubjectAlternativeName::new();
                    for alt_host_name in args.alt_host_names.iter() {
                        subject_alternative_name.dns(alt_host_name);
                    }
                    subject_alternative_name.build(&builder.x509v3_context(None, None)).unwrap()
                };
                builder.append_extension(subject_alternative_name).unwrap();
            }

            // Self-sign
            let _ = builder.sign(&pkey, MessageDigest::sha1());

            builder.build()
        };

        Ok((X509::wrap(cert), PKey::wrap(pkey)))
    }

    /// This function will use the supplied arguments to create a public/private key pair and from
    /// those create a private key file and self-signed public cert file.
    pub fn create_and_store_cert(args: &X509Data, overwrite: bool, pki_path: &Path) -> Result<(), String> {
        // Create the cert and corresponding private key
        let (cert, pkey) = CertificateStore::create_cert_and_pkey(args)?;

        // Public cert goes under own/
        let public_cert_path = CertificateStore::make_and_ensure_file_path(&CertificateStore::own_cert_dir(pki_path), OWN_CERTIFICATE_NAME)?;
        // Private key goes under private/
        let private_key_path = CertificateStore::make_and_ensure_file_path(&CertificateStore::private_key_dir(pki_path), OWN_PRIVATE_KEY_NAME)?;

        // Write the public cert
        CertificateStore::write_cert(&cert, &public_cert_path, overwrite)?;

        // Write the private key
        let pem = pkey.value.private_key_to_pem().unwrap();
        info!("Writing private key to {}", private_key_path.display());
        CertificateStore::write_to_file(&pem, &private_key_path, overwrite)?;

        Ok(())
    }

    /// OPC UA Part 6 MessageChunk structure
    ///
    /// The thumbprint is the SHA1 digest of the DER form of the certificate. The hash is 160 bits
    /// (20 bytes) in length and is sent in some secure conversation headers.
    ///
    /// The thumbprint might be used by the server / client for look-up purposes.
    pub fn thumbprint(cert: &X509) -> Vec<u8> {
        use openssl::hash::{MessageDigest, hash};
        let der = cert.value.to_der().unwrap();
        hash(MessageDigest::sha1(), &der).unwrap()
    }

    /// Validates the cert and if its unknown, writes the value to the rejected folder so it can
    /// be moved to trusted by user
    /// # Errors
    ///
    /// A non `GOOD` status code indicates a failure in the cert or in some action required in
    /// order to validate it.
    ///
    pub fn validate_or_reject_cert(&self, cert: &X509) -> StatusCode {
        let result = self.validate_cert(cert);
        if result == BAD_CERTIFICATE_UNTRUSTED {
            // Store result in rejected folder
            let _ = self.write_rejected_cert(cert);
        }
        result
    }

    /// This function is to stop collision errors / tampering where someone renames a cert on disk
    /// to match another cert and somehow bypasses or subverts a check. The disk cert must match
    /// the memory cert or the test is assumed to fail.
    fn ensure_cert_and_file_are_the_same(cert: &X509, cert_path: &Path) -> bool {
        if !cert_path.exists() {
            false
        } else {
            let cert2 = CertificateStore::read_cert(cert_path);
            if cert2.is_err() {
                // No cert2 to compare to
                false
            } else {
                // Compare the buffers
                let der = cert.value.to_der().unwrap();
                let der2 = cert2.unwrap().value.to_der().unwrap();
                der == der2
            }
        }
    }

    /// Validates the certificate according to the strictness set in the CertificateStore itself.
    /// Validation might include checking the issue time, expiration time, revocation, trust chain
    /// etc. In the first instance this function will only check if the cert is recognized
    /// and is already contained in the trusted or rejected folder.
    ///
    /// # Errors
    ///
    /// A non `GOOD` status code indicates a failure in the cert or in some action required in
    /// order to validate it.
    ///
    pub fn validate_cert(&self, cert: &X509) -> StatusCode {
        let cert_file_name = CertificateStore::cert_file_name(&cert);
        debug!("Validating cert with name on disk {}", cert_file_name);

        // Look for the cert in the rejected folder. If it's rejected there is no purpose going
        // any further
        {
            let mut cert_path = CertificateStore::rejected_certs_dir(&self.pki_path);
            if !cert_path.exists() {
                error!("Path for rejected certificates {} does not exist", cert_path.display());
                return BAD_UNEXPECTED_ERROR;
            }
            cert_path.push(&cert_file_name);
            if cert_path.exists() {
                warn!("Certificate {} is untrusted because it resides in the rejected directory", cert_file_name);
                return BAD_CERTIFICATE_UNTRUSTED;
            }
        }

        // Check the trusted folder. These checks are more strict to ensure the cert is genuinely
        // trusted
        {
            // Check the trusted folder
            let mut cert_path = CertificateStore::trusted_certs_dir(&self.pki_path);
            if !cert_path.exists() {
                error!("Path for rejected certificates {} does not exist", cert_path.display());
                return BAD_UNEXPECTED_ERROR;
            }
            cert_path.push(&cert_file_name);

            // Check if cert is in the trusted folder
            if !cert_path.exists() {
                // ... trust checks based on ca could be added here to add cert straight to trust folder
                warn!("Certificate {} is unknown and untrusted so it will be stored in rejected directory", cert_file_name);
                let _ = self.write_rejected_cert(cert);
                return BAD_CERTIFICATE_UNTRUSTED;
            }

            // Read the cert from the trusted folder to make sure it matches the one supplied
            if !CertificateStore::ensure_cert_and_file_are_the_same(cert, &cert_path) {
                error!("Certificate in memory does not match the one on disk {} so cert will automatically be treated as untrusted", cert_path.display());
                return BAD_UNEXPECTED_ERROR;
            }

            // Now inspect the cert not before / after values to ensure its validity
            {
                use chrono::UTC;
                let now = UTC::now();
                // Issuer time
                if self.check_issue_time {
                    let not_before = cert.not_before();
                    if let Ok(not_before) = not_before {
                        if now.lt(&not_before) {
                            return BAD_CERTIFICATE_UNTRUSTED;
                        }
                    } else {
                        return BAD_CERTIFICATE_UNTRUSTED;
                    }
                }

                // Expiration time
                if self.check_expiration_time {
                    let not_after = cert.not_after();
                    if let Ok(not_after) = not_after {
                        if now.gt(&not_after) {
                            return BAD_CERTIFICATE_UNTRUSTED;
                        }
                    } else {
                        return BAD_CERTIFICATE_UNTRUSTED;
                    }
                }
            }

            // Other tests that we might do
            // ... issuer
            // ... trust (self-signed, ca etc.)
            // ... revocation
        }

        GOOD
    }

    /// Returns a certificate file name from the cert's issuer and thumbprint fields
    fn cert_file_name(cert: &X509) -> String {
        let thumbprint = CertificateStore::thumbprint(cert);
        // Hex name = 20 bytes = 40 chars in hex + 4 for .der ext
        let mut file_name = String::with_capacity(20 * 2 + 4);
        for b in thumbprint.iter() {
            file_name.push_str(&format!("{:02x}", b))
        }
        file_name.push_str(".der");
        file_name
    }

    /// Creates the PKI directory structure
    ///
    /// # Errors
    ///
    /// A string description of any failure
    ///
    pub fn ensure_pki_path(&self) -> Result<(), String> {
        let mut path = self.pki_path.clone();
        let subdirs = [OWN_CERTIFICATE_DIR, OWN_PRIVATE_KEY_DIR, TRUSTED_CERTS_DIR, REJECTED_CERTS_DIR];
        for subdir in subdirs.iter() {
            path.push(subdir);
            CertificateStore::ensure_dir(&path)?;
            path.pop();
        }
        Ok(())
    }

    /// Ensure the directory exists, creating it if necessary
    ///
    /// # Errors
    ///
    /// A string description of any failure
    ///
    fn ensure_dir(path: &Path) -> Result<(), String> {
        use std;
        if path.exists() {
            if !path.is_dir() {
                return Err(format!("{} is not a directory ", path.display()));
            }
        } else {
            let result = std::fs::create_dir_all(path);
            if result.is_err() {
                return Err(format!("Cannot make directories for {}", path.display()));
            }
        }
        Ok(())
    }

    /// Returns the path to the private key dir
    pub fn private_key_dir(pki_path: &Path) -> PathBuf {
        let mut path = PathBuf::from(&pki_path);
        path.push(OWN_PRIVATE_KEY_DIR);
        path
    }

    /// Returns the path to the own certificate dir
    pub fn own_cert_dir(pki_path: &Path) -> PathBuf {
        let mut path = PathBuf::from(&pki_path);
        path.push(OWN_CERTIFICATE_DIR);
        path
    }

    /// Returns the path to the rejected certs dir
    pub fn rejected_certs_dir(pki_path: &Path) -> PathBuf {
        let mut path = PathBuf::from(&pki_path);
        path.push(REJECTED_CERTS_DIR);
        path
    }

    /// Returns the path to the trusted certs dir
    pub fn trusted_certs_dir(pki_path: &Path) -> PathBuf {
        let mut path = PathBuf::from(&pki_path);
        path.push(TRUSTED_CERTS_DIR);
        path
    }

    /// Write a cert to the rejected directory. If the write succeeds, the function
    /// returns a path to the written file.
    ///
    /// # Errors
    ///
    /// A string description of any failure
    ///
    pub fn write_rejected_cert(&self, cert: &X509) -> Result<PathBuf, String> {
        // Store the cert in the rejected folder where untrusted certs go
        let cert_file_name = CertificateStore::cert_file_name(&cert);
        let mut cert_path = CertificateStore::rejected_certs_dir(&self.pki_path);
        cert_path.push(&cert_file_name);
        CertificateStore::write_cert(cert, &cert_path, true)?;
        Ok(cert_path)
    }

    /// Writes a cert to the specified directory
    ///
    /// # Errors
    ///
    /// A string description of any failure
    ///
    fn write_cert(cert: &X509, path: &Path, overwrite: bool) -> Result<(), String> {
        let der = cert.value.to_der().unwrap();
        info!("Writing X509 cert to {}", path.display());
        CertificateStore::write_to_file(&der, &path, overwrite)
    }

    /// Reads an X509 certificate from disk
    ///
    /// # Errors
    ///
    /// A string description of any failure
    ///
    fn read_cert(path: &Path) -> Result<X509, String> {
        let file = File::open(path);
        if file.is_err() {
            return Err(format!("Could not open cert file {}", path.display()));
        }

        let mut file: File = file.unwrap();
        let mut cert = Vec::new();
        let bytes_read = file.read_to_end(&mut cert);
        if bytes_read.is_err() {
            return Err(format!("Could not read bytes from cert file {}", path.display()));
        }

        let cert = x509::X509::from_pem(&cert);
        if cert.is_err() {
            return Err(format!("Could not read cert from cert file {}", path.display()));
        }

        Ok(X509::wrap(cert.unwrap()))
    }

    /// Makes a path
    ///
    /// # Errors
    ///
    /// A string description of any failure
    ///

    fn make_and_ensure_file_path(path: &Path, file_name: &str) -> Result<PathBuf, String> {
        let mut path = PathBuf::from(&path);
        CertificateStore::ensure_dir(&path)?;
        path.push(file_name);
        Ok(path)
    }

    /// Writes to file or prints an error for the reason why it can't.
    ///
    /// # Errors
    ///
    /// A string description of any failure
    ///
    fn write_to_file(bytes: &[u8], file_path: &Path, overwrite: bool) -> Result<(), String> {
        if !overwrite && file_path.exists() {
            return Err(format!("File {} already exists and will not be overwritten. Use --overwrite to disable this safeguard.", file_path.display()))
        }
        let file = File::create(file_path);
        if file.is_err() {
            return Err(format!("Could not create file {}", file_path.display()));
        }
        let mut file = file.unwrap();

        let written = file.write(bytes);
        if written.is_err() {
            return Err(format!("Could not write bytes to file {}", file_path.display()));
        }
        Ok(())
    }
}