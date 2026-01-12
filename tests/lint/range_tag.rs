use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_when_type_integer()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]="integer"
    #[range]="1..10"
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="5"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_success_when_type_float()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]="float"
    #[range]="1.0..10.0"
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="5.5"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_type()
{
  let expected_status = false;
  let expected_output = "info: Variable 'EXAMPLE_ENV' has a range but is not an 'integer' or 'float'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]="string"
    #[range]="1..10"
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="five"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_warn_when_empty_range()
{
  let expected_status = false;
  let expected_output =
    "warn: Variable 'EXAMPLE_ENV' has an empty range";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=integer
    #[range]="     "
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="5"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_range()
{
  let expected_status = false;
  let expected_output = "error: Variable 'EXAMPLE_ENV' has an invalid range '1..' at line 5";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]="integer"
    #[range]="1.."
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="5"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_integer_value_out_of_range()
{
  let expected_status = false;
  let expected_output = "error: value for key 'EXAMPLE_ENV' is not in the range '1900..2000'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]="integer"
    #[range]="1900..2000"
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="2025"
  "#;
  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_float_value_out_of_range()
{
  let expected_status = false;
  let expected_output = "error: value for key 'EXAMPLE_ENV' is not in the range '19.0..20.0'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]="float"
    #[range]="19.0..20.0"
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="20.25"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}