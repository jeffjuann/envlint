use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_with_valid_title()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=string
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  
  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_warn_when_missing_title()
{
  let expected_status = true;
  let expected_output = "warn: Variable 'EXAMPLE_ENV' is missing a title";

  let template_env_content = r#"
    #[description]="This is a description"
    #[type]=string
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_warn_when_empty_title_value()
{
  let expected_status = true;
  let expected_output = "warn: Variable 'EXAMPLE_ENV' has an empty title at line 2";

  let template_env_content = r#"
    #[title]
    #[description]="This is a description"
    #[type]=string
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}