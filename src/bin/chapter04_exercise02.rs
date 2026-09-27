use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum MyError {
    Setup(String),
    StepOne(String),
    StepTwo(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Setup(msg) => write!(f, "setup failed: {msg}"),
            MyError::StepOne(msg) => write!(f, "step one failed: {msg}"),
            MyError::StepTwo(msg) => write!(f, "step two failed: {msg}"),
        }
    }
}

impl Error for MyError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Setup,
    StepOne,
    StepTwo,
}

struct Thing {
    should_fail_at: Option<Stage>,
    cleaned_up: bool,
}

impl Thing {
    fn new(should_fail_at: Option<Stage>) -> Self {
        Thing {
            should_fail_at,
            cleaned_up: false,
        }
    }

    fn setup(&mut self) -> Result<(), MyError> {
        if self.should_fail_at == Some(Stage::Setup) {
            return Err(MyError::Setup(
                "setup could not acquire resources".to_string(),
            ));
        }
        Ok(())
    }

    fn step_one(&mut self) -> Result<(), MyError> {
        if self.should_fail_at == Some(Stage::StepOne) {
            return Err(MyError::StepOne(
                "step one hit an unexpected state".to_string(),
            ));
        }
        Ok(())
    }

    fn step_two(&mut self) -> Result<(), MyError> {
        if self.should_fail_at == Some(Stage::StepTwo) {
            return Err(MyError::StepTwo(
                "step two hit an unexpected state".to_string(),
            ));
        }
        Ok(())
    }

    fn cleanup(&mut self) {
        self.cleaned_up = true;
    }
}

fn do_the_thing(thing: &mut Thing) -> Result<(), MyError> {
    thing.setup()?;

    // `?` inside this closure returns from the closure, not from
    // do_the_thing — so cleanup() below always still runs.
    let result = (|| -> Result<(), MyError> {
        thing.step_one()?;
        thing.step_two()?;
        Ok(())
    })();

    thing.cleanup();
    result
}

fn main() {
    let mut succeeds = Thing::new(None);
    match do_the_thing(&mut succeeds) {
        Ok(()) => println!("success; cleaned up: {}", succeeds.cleaned_up),
        Err(err) => eprintln!("unexpected error: {err}"),
    }

    let mut fails_at_step_two = Thing::new(Some(Stage::StepTwo));
    match do_the_thing(&mut fails_at_step_two) {
        Ok(()) => println!("unexpected success"),
        Err(err) => println!("Error: {err}; cleaned up: {}", fails_at_step_two.cleaned_up),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup_does_not_run_when_setup_fails() {
        let mut thing = Thing::new(Some(Stage::Setup));

        let result = do_the_thing(&mut thing);

        assert!(matches!(result, Err(MyError::Setup(_))));
        assert!(!thing.cleaned_up);
    }

    #[test]
    fn cleanup_runs_when_step_one_fails() {
        let mut thing = Thing::new(Some(Stage::StepOne));

        let result = do_the_thing(&mut thing);

        assert!(matches!(result, Err(MyError::StepOne(_))));
        assert!(thing.cleaned_up);
    }

    #[test]
    fn cleanup_runs_when_step_two_fails() {
        let mut thing = Thing::new(Some(Stage::StepTwo));

        let result = do_the_thing(&mut thing);

        assert!(matches!(result, Err(MyError::StepTwo(_))));
        assert!(thing.cleaned_up);
    }

    #[test]
    fn cleanup_runs_when_nothing_is_forced_to_fail() {
        let mut thing = Thing::new(None);

        let result = do_the_thing(&mut thing);

        assert!(result.is_ok());
        assert!(thing.cleaned_up);
    }
}
