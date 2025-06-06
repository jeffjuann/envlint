<h1 align="center">envlint</h1>
<p align="center">
  <strong>.env file template & linter</strong>
</p>

envlint is a lightweight `.env` file linter that ensures environment variables conform to a structured template. It helps developers maintain consistency and avoid misconfigurations by validating format, types, and constraints across `.env` files.

## Installation

All commands use `curl` and `tar` to download and extract the archive.

### Windows

* **x64:**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-win32-x64.tar.gz -sSfL | tar -xzf -
```

* **ARM64:**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-win32-arm64.tar.gz -sSfL | tar -xzf -
```

### Linux

* **x64 (musl):**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-linux-x64-musl.tar.gz -sSfL | tar -xzf -
```

* **x64 (glibc):**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-linux-x64-gnu.tar.gz -sSfL | tar -xzf -
```

* **ARM64 (musl):**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-linux-arm64-musl.tar.gz -sSfL | tar -xzf -
```

* **ARM64 (glibc):**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-linux-arm64-gnu.tar.gz -sSfL | tar -xzf -
```

### macOS

* **x64:**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-darwin-x64.tar.gz -sSfL | tar -xzf -
```

* **ARM64:**

```bash
curl https://github.com/jeffjuann/envlint/releases/download/v1.0.0/envlint-v1.0.0-darwin-arm64.tar.gz -sSfL | tar -xzf -
```

> **Note:** add to your `PATH` after extraction to use `envlint` from any location.

## Usage
```bash
  envlint lint [options]
```

### Options
| Option | Description | Default |
| --- | --- | --- |
| `-f, --file <file>` | specify the .env file to lint | `.env` in current working directory |
| `-t, --template <file>` | specify the template file to use | `.env.template` in current working directory |

## Usage Guide & Example

see [USAGE.md](docs/USAGE.md) for detailed usage guide.

```bash
#[title]=Example environment Variable
#[description]=This is a required environment variable variable in alphanumeric format
#[required]
#[type]=string
#[regex]=^[a-zA-Z0-9_]*$
ENV="example"
```

More examples can be found in [EXAMPLES](examples).

## Contribution
Feel free to submit any issues or open discussions if you found any bugs or have any suggestions.

## License
[MIT License](LICENSE)
