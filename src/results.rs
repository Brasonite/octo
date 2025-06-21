use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FailureReason {
    MissingOutput,
    RuntimeError,
    Timeout,
    WrongOutput,
}

#[derive(Debug, Clone)]
pub enum TestResult {
    Success,
    Failure(FailureReason),
}

impl TestResult {
    pub fn is_success(&self) -> bool {
        match &self {
            Self::Success => true,
            _ => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        match &self {
            Self::Failure(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestResults {
    pub successful: u32,
    pub failed: u32,
    pub failure_reasons: HashMap<FailureReason, u32>,
}

impl TestResults {
    pub fn new(results: Vec<TestResult>) -> Self {
        let successful = results.iter().filter(|x| x.is_success()).count() as u32;
        let failed = results.iter().filter(|x| x.is_failure()).count() as u32;

        let mut failure_reasons = HashMap::new();
        results
            .iter()
            .filter(|x| x.is_failure())
            .for_each(|x| match x {
                TestResult::Failure(reason) => {
                    if failure_reasons.contains_key(reason) {
                        *failure_reasons.get_mut(reason).unwrap() += 1;
                    } else {
                        failure_reasons.insert(reason.clone(), 1);
                    }
                }
                _ => {}
            });

        Self {
            successful,
            failed,
            failure_reasons,
        }
    }
}
