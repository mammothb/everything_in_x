mod echo_tests {
    use anyhow::Result;
    use assert_cmd::Command;
    use parameterized::parameterized;
    use predicates::prelude::predicate;

    #[test]
    fn no_args_fails() -> Result<()> {
        Command::cargo_bin("echo")?
            .assert()
            .failure()
            .stderr(predicate::str::contains("Usage:"));
        Ok(())
    }

    #[parameterized(args = {vec!["-n"], vec!["-e"], vec!["-E"]})]
    fn no_text_fails(args: Vec<&str>) -> Result<()> {
        Command::cargo_bin("echo")?
            .args(args)
            .assert()
            .failure()
            .stderr(predicate::str::contains("Usage:"));
        Ok(())
    }

    #[test]
    fn invalid_options_fails() -> Result<()> {
        Command::cargo_bin("echo")?
            .args(["-e", "-E", "asdf"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "Conflicting options: provided both '-e' and '-E'.",
            ));
        Ok(())
    }

    #[parameterized(
        args = {vec!["qwer   asdf"], vec!["qwer", "asdf", "zxcv"]},
        expected = {"qwer   asdf\n", "qwer asdf zxcv\n"}
    )]
    fn prints_text(args: Vec<&str>, expected: &str) -> Result<()> {
        Command::cargo_bin("echo")?
            .args(args)
            .assert()
            .success()
            .stdout(predicate::eq(expected));
        Ok(())
    }

    #[parameterized(
        args = {vec!["-n", "qwer   asdf"], vec!["-n", "qwer", "asdf", "zxcv"]},
        expected = {"qwer   asdf", "qwer asdf zxcv"}
    )]
    fn no_newline(args: Vec<&str>, expected: &str) -> Result<()> {
        Command::cargo_bin("echo")?
            .args(args)
            .assert()
            .success()
            .stdout(predicate::eq(expected));
        Ok(())
    }
}
