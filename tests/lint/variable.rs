use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_when_variable_valid()
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
fn should_warn_when_duplicate_variable()
{
  let expected_status = true;
  let expected_output = "warn: Variable 'EXAMPLE_ENV' is defined more than once at line 4";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=string
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
    
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_success_with_multiline_value()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[required]
    #[type]=string
    EXAMPLE_ENV=

    #[title]="next title"
    #[description]="next description"
    #[required]
    #[type]=integer
    EXAMPLE_INT=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="multiline value

    with escaped quotes \" and escaped backslashes \\"

    EXAMPLE_INT="123"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}