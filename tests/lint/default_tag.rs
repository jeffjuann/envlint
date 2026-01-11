use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_with_valid_default_value()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=float
    #[default]=123.45
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="678.90"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_warn_when_has_required_tag()
{
  let expected_status = true;
  let expected_output = "warn: Variable 'EXAMPLE_ENV' is required but has a default value at line 5";

  let template_env_content = r#"
    #[title]="this is a title"
    #[required]
    #[type]=string
    #[default]=default_value
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;
  
  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_type_default_value()
{
  let expected_status = false;
  let expected_output = "error: default value for key 'EXAMPLE_ENV' is not an integer";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=integer
    #[default]=123-four-five
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="123"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}