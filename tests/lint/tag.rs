use crate::common::test_environment::TestEnvironment;

#[test]
fn should_success_when_all_tag_valid()
{
  let expected_status = true;
  let expected_output = "";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[required]
    #[type]=string
    #[regex]="^example$"
    EXAMPLE_ENV=

    #[title]="this is second title"
    #[description]="This is second description"
    #[required]
    #[type]=integer
    #[range]=1..10
    EXAMPLE_ENV2=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"

    EXAMPLE_ENV2="5"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}

#[test]
fn should_error_when_invalid_tag()
{
  let expected_status = false;
  let expected_output = "error: Invalid tag 'invalid_tag' at line 5";

  let template_env_content = r#"
    #[title]="this is a title"
    #[description]="This is a description"
    #[type]=string
    #[invalid_tag]=invalid
    EXAMPLE_ENV=
  "#;

  let env_content = r#"
    EXAMPLE_ENV="example"
  "#;

  let mut test_environment = TestEnvironment::new();
  test_environment.test_lint(env_content, template_env_content, &[], expected_status, expected_output);
}