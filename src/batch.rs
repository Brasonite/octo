use std::{
    collections::HashMap,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::Instant,
};

use anyhow::Result;
use deunicode::deunicode;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use walkdir::WalkDir;

use crate::{
    question::Question,
    results::{FailureReason, TestResult, TestResults},
};

pub struct Batch {
    pub inputs: HashMap<String, String>,
    pub outputs: HashMap<String, String>,
}

impl Batch {
    fn collect_into(
        map: &mut HashMap<String, String>,
        root: &str,
        path: &Path,
        format: &str,
    ) -> Result<()> {
        for entry in WalkDir::new(path) {
            let entry = entry?;

            if !entry.file_type().is_file() {
                continue;
            }

            let extension = entry
                .path()
                .extension()
                .map(|x| x.to_str())
                .filter(|x| x.is_some())
                .map(|x| x.unwrap());

            if extension == Some(format) {
                let filename = match entry
                    .path()
                    .file_stem()
                    .map(|x| x.to_str())
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                {
                    Some(filename) => filename,
                    None => {
                        log::warn!("Skipped file {:?} (no or invalid filename)", entry.path());
                        continue;
                    }
                };

                let data =
                    deunicode(std::fs::read_to_string(entry.path())?.trim()).replace("\r\n", "\n");
                map.insert(format!("{root}-{filename}"), data);
            }
        }

        Ok(())
    }

    pub fn load(question: &Question) -> Result<Self> {
        let mut inputs = HashMap::new();
        let mut outputs = HashMap::new();

        for input_folder in &question.input_folders {
            Self::collect_into(
                &mut inputs,
                input_folder,
                &Path::new(question.root.as_ref().unwrap()).join(input_folder),
                &question.input_format,
            )?;
        }

        for output_folder in &question.output_folders {
            Self::collect_into(
                &mut outputs,
                output_folder,
                &Path::new(question.root.as_ref().unwrap()).join(output_folder),
                &question.output_format,
            )?;
        }

        Ok(Self { inputs, outputs })
    }

    pub fn process(&self, question: &Question) -> Result<TestResults> {
        let results: Vec<TestResult> = self
            .inputs
            .par_iter()
            .map(|(key, input)| {
                if !self.outputs.contains_key(key) {
                    log::warn!("Input {key:?} has no output of the same name. Skipping...");
                    return TestResult::Failure(FailureReason::MissingOutput);
                }

                let mut process = Command::new("python3")
                    .arg(format!(
                        "{}/{}",
                        question.root.as_ref().unwrap(),
                        question.solver
                    ))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to spawn child process");

                let mut stdin = process.stdin.take().expect("Failed to get process stdin");

                let start = Instant::now();

                stdin
                    .write_all(input.as_bytes())
                    .expect("Failed to write input to stdin");
                stdin.write(b"\n").expect("Failed to write input to stdin");

                let output = process
                    .wait_with_output()
                    .expect("Failed to execute process");

                let time = start.elapsed().as_secs_f32();

                if output.status.success() {
                    if time > question.time_limit {
                        TestResult::Failure(FailureReason::Timeout)
                    } else {
                        let results = deunicode(String::from_utf8_lossy(&output.stdout).trim())
                            .replace("\r\n", "\n");

                        if results.trim() == self.outputs[key] {
                            TestResult::Success
                        } else {
                            TestResult::Failure(FailureReason::WrongOutput)
                        }
                    }
                } else {
                    TestResult::Failure(FailureReason::RuntimeError)
                }
            })
            .collect();

        Ok(TestResults::new(results))
    }
}
