use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_when_value_exists()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[required]
    #[type]=string
    EXAMPLE_ENV="example"
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_warn_when_unknown_required_tag()
{
  let expected_status = true;
  let expected_output = "warn: Variable 'EXAMPLE_ENV' has an invalid required value 'unknown_value' at line 4. expected 'true' or 'false', consider removing the flag if it's not needed.";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[required]=unknown_value
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
fn should_error_when_variable_not_found()
{
  let expected_status = false;
  let expected_output = "error: missing value for key 'EXAMPLE_ENV' in file '.env'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[required]
    #[type]=string
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    NOT_EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_with_missing_value()
{
  let expected_status = false;
  let expected_output = "error: missing value for key 'EXAMPLE_ENV' in file '.env'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[required]
    #[type]=string
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV=
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_with_whitespace_value()
{
  let expected_status = false;
  let expected_output = "error: missing value for key 'EXAMPLE_ENV' in file '.env'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[required]
    #[type]=string
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="    "
  "#;
  
  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}