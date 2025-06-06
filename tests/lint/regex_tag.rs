use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_when_type_string()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=string
    #[regex]=^[a-zA-Z0-9]+$
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example123"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_with_invalid_type()
{
  let expected_status = false;
  let expected_output = "warn: Variable 'EXAMPLE_ENV' has a regex but is not a string";
  
  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=integer
    #[regex]=^[0-9]+$
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="5"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_with_invalid_regex()
{
  let expected_status = false;
  let expected_output = "error: Variable 'EXAMPLE_ENV' has an invalid regex '[a-zA-Z+' at line 5";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=string
    #[regex]=[a-zA-Z+
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;
  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_success_with_valid_value_and_valid_regex()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=string
    #[regex]=^[a-zA-Z0-9]+$
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="alphanumeric123"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_with_invalid_value_and_valid_regex()
{
  let expected_status = false;
  let expected_output = "error: value for key 'EXAMPLE_ENV' does not match the regex '^[a-zA-Z0-9]+$'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=string
    #[regex]=^[a-zA-Z0-9]+$
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="alphanumeric123!"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

