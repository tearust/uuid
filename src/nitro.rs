use crate::prelude::*;
use nsm_io::ErrorCode;

impl Uuid {
    /// Creates a random UUID from a nitro enclave instance.
    ///
    pub fn new() -> Uuid {
        let fd = nsm::nsm_lib_init();

        let mut bytes = [0u8; 16];
        let mut len = bytes.len();
        let err_code =
            unsafe { nsm::nsm_get_random(fd, bytes.as_mut_ptr(), &mut len) };

        nsm::nsm_lib_exit(fd);

        match err_code {
            ErrorCode::Success => crate::Builder::from_bytes(bytes)
                .set_variant(Variant::RFC4122)
                .set_version(Version::Random)
                .build(),
            _ => panic!(
                "failed to generate random from nsm, error code: {:?}",
                err_code
            ),
        }
    }
}
