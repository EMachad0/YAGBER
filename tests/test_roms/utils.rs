pub const MAX_CYCLES: u32 = 1 << 30; // 1 billion cycles

#[derive(Debug, PartialEq, Eq)]
pub enum TestResult {
    Passed,
    Failed,
    TimedOut,
}

impl TestResult {
    pub fn is_success(&self) -> bool {
        self == &TestResult::Passed
    }
}
