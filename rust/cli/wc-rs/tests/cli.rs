mod wc_tests {
    use anyhow::Result;
    use assert_cmd::Command;
    use parameterized::parameterized;
    use predicates::prelude::predicate;

    #[test]
    fn empty_file() -> Result<()> {
        Command::cargo_bin("wc")?
            .args(["tests/inputs/empty.txt"])
            .assert()
            .success()
            .stdout("       0        0        0 tests/inputs/empty.txt\n");
        Ok(())
    }

    #[test]
    fn ascii_file() -> Result<()> {
        Command::cargo_bin("wc")?
            .args(["tests/inputs/fox.txt"])
            .assert()
            .success()
            .stdout("       1        9       48 tests/inputs/fox.txt\n");
        Ok(())
    }

    #[test]
    fn unicode_file() -> Result<()> {
        Command::cargo_bin("wc")?
            .args(["tests/inputs/atlamal.txt"])
            .assert()
            .success()
            .stdout("       4       29      177 tests/inputs/atlamal.txt\n");
        Ok(())
    }

    #[test]
    fn multiple_files() -> Result<()> {
        Command::cargo_bin("wc")?
            .args([
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
                "tests/inputs/atlamal.txt",
            ])
            .assert()
            .success()
            .stdout(predicate::eq(format!(
                "{}\n{}\n{}\n{}\n",
                "       0        0        0 tests/inputs/empty.txt",
                "       1        9       48 tests/inputs/fox.txt",
                "       4       29      177 tests/inputs/atlamal.txt",
                "       5       38      225 total"
            )));
        Ok(())
    }
}
