pub struct JoypRegister;

impl JoypRegister {
    pub fn joyp_transformer(_old_value: u8, new_value: u8) -> Option<u8> {
        let selected_bits = new_value & 0x30;
        Some(0xC0 | selected_bits | 0x0F)
    }
}

impl yagber_app::Component for JoypRegister {}
