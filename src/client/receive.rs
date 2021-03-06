use {
    crate::{
        client::unsafe_::UnsafeClient,
        error::{Error, Result},
    },
    std::{sync::Arc, time::Duration},
};

#[derive(Debug)]
pub struct ReceiveClient {
    pub(crate) inner: Arc<UnsafeClient>,
}

/// SAFE: the receive method can be called by any thread.
unsafe impl Send for ReceiveClient {}

impl ReceiveClient {
    pub fn receive(&mut self, timeout: Duration) -> Result<Option<&str>> {
        // SAFE: we are taking self by mutable reference.
        unsafe { self.inner.receive(timeout) }
    }

    #[cfg(feature = "types")]
    pub fn receive_typed(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<tdlib_types::types::Response>> {
        match self.receive(timeout) {
            Ok(ok) => match ok {
                Some(ok) => Ok(serde_json::from_str(ok).map_err(Error::InvalidRequestData))?,
                None => Ok(None),
            },
            Err(e) => Err(e),
        }
    }
}
