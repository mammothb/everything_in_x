mod false_tests {
    use assert_cmd::Command;

    #[test]
    fn fails() {
        let mut cmd = Command::cargo_bin("false").unwrap();
        cmd.assert().failure();
    }
}
