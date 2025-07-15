pub struct Vbk;

impl Vbk {
    pub(crate) fn vbk_transformer(_old_value: u8, new_value: u8) -> Option<u8> {
        Some(0xFE | new_value)
    }
}
