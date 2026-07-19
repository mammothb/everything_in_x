mod true_tests {
    use assert_cmd::Command;

    #[test]
    fn succeeds() {
        let mut cmd = Command::cargo_bin("true").unwrap();
        cmd.assert().success();
    }
}
