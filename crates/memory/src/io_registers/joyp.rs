pub struct JoypRegister;

impl JoypRegister {
    pub fn joyp_transformer(_old_value: u8, _new_value: u8) -> Option<u8> {
        Some(0xFF)
    }
}

impl yagber_app::Component for JoypRegister {}
