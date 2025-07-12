pub struct DivRegister;

impl DivRegister {
    pub(crate) fn div_transformer(_old_value: u8, _new_value: u8) -> Option<u8> {
        Some(0)
    }
}
