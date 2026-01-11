use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_when_valid_type()
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
fn should_error_when_missing_type()
{
  let expected_status = false;
  let expected_output = "warn: Variable 'EXAMPLE_ENV' is missing a type";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_type()
{
  let expected_status = false;
  let expected_output = "error: Variable 'EXAMPLE_ENV' has an unknown type 'invalid' at line 4";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=invalid
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_list_type()
{
  let expected_status = false;
  let expected_output = "error: Variable 'EXAMPLE_ENV' has an unknown type 'list<invalid>' at line 4";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=list<invalid>
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_list_type_is_nested()
{
  let expected_status = false;
  let expected_output = "error: Variable 'EXAMPLE_ENV' defines a nested list type, which is not allowed, at line 4";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=list<list<float>>
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_success_when_valid_string()
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
fn should_success_when_valid_boolean()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=boolean
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV=true
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_boolean()
{
  let expected_status = false;
  let expected_output = "error: value for key 'EXAMPLE_ENV' is not a valid boolean. expected '1', 't', 'T', 'TRUE', 'true', 'True', '0', 'f', 'F', 'FALSE', 'false', 'False'";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=boolean
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="not-a-boolean"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_success_when_valid_integer()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=integer
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV=123
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_integer()
{
  let expected_status = false;
  let expected_output = "error: value for key 'EXAMPLE_ENV' is not a valid integer";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=integer
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="123-four-five"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_success_when_valid_float()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=float
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV=123.45
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_float()
{
  let expected_status = false;
  let expected_output = "error: value for key 'EXAMPLE_ENV' is not a valid float";
  
  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=float
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="123.four-five"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_success_when_valid_list()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=list<float>
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV=123.45,789.01
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_list()
{
  let expected_status = false;
  let expected_output = "error: value for key 'EXAMPLE_ENV' is not a valid list of floats";
  
  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=list<float>
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="123.1,four.five,678.9,not_a_float"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}