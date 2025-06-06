# Usage Guide

run the following command to lint the environment variables in the .env file.
```bash
  envlint lint [options]
```

### Options
| Option | Description | Default |
| --- | --- | --- |
| `-f, --file <file>` | specify the .env file to lint | `.env` in current working directory |
| `-t, --template <file>` | specify the template file to use | `.env.template` in current working directory |

## File Standards and Metadata Tags

### #[title]

```
  #[title]=<title>
```

optional. title of the environment. can be wrapped in double quotes.

### #[description]

```
  #[description]=<description>
```

optional. description of the environment variable. can be wrapped in double quotes.

### #[required]

```
  #[required]
```

optional. required environment variables. if the required tag is found, the current environment variable value is required, otherwise it is not required.

### #[type]

```
  #[type]=<env-type>
```

optional. type of the environment variable. can be one of the following:
- string
- integer
- float
- boolean

### #[regex]

```
  #[regex]=<regex>
```

optional. only for string type. regex is a regular expression. (e.g. `^[a-zA-Z0-9_]*$`)

### #[range]

```
  #[range]=<range>
```

optional. only for integer and float types. range can be one of the following:
- `1..12`
- `1,2,3,5,7.13`
- `1,3,5,6..10,11,13,15,16..20`
- `1.1..12.12`
- `1.1,2.2,3.3,4.4,5.5`
- `1.1,3.3,5.5,6.6..10.10,11.11,13.13,15.15,16.16..20.20`