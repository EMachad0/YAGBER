#[derive(Debug, Clone, Copy)]
pub struct WindowScanLine {
    last_y: Option<u8>,
    scan_line: u8,
}

impl WindowScanLine {
    pub fn new() -> Self {
        Self {
            last_y: None,
            scan_line: 0,
        }
    }

    pub fn reset(&mut self) {
        self.last_y = None;
        self.scan_line = 0;
    }

    pub fn get_and_update(&mut self, y: u8) -> u8 {
        if let Some(last_y) = self.last_y {
            if last_y != y {
                self.scan_line += 1;
            }
        }
        self.last_y = Some(y);
        self.scan_line
    }
}
