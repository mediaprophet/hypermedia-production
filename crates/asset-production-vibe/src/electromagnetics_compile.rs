//! Canonical EMF field production; implementation and derived consumers share one adapter.
use crate::*;
impl AssetHost<'_> {
    pub(crate) fn compile_emf(&mut self, args: &Args) -> Result<Value, String> {
        self.compile_emf_current(args)
    }
}
