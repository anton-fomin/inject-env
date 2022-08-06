# inject-env

Simple tool to dump env variables to json, potentially filtering by prefix, stripping it and also with ability to put this json into placeholder in existing file (which is useful to pass settings from environment variables in staticfiles builds). 

## Usage examples

For example we define settings for our application in environment variables 

```
# env
APP_ENV_API_URL=https://example.com/api
APP_ENV_SENTRY_URL=https://sentry.com
SOME_UNSAFE_ENV_VAR=secret
```

We can inject these settings into our static application by replacing some placeholder.
First we put a placeholder into an html file.

```html
<script>/*APP_ENV*/</script>
```
Replace `/*APP_ENV*/` with `window.APP_ENV = {...environment variables values...}` in index.html
```bash
inject-env -o index.html -r '/*APP_ENV*/' --prefix APP_ENV_ --format 'window.APP_ENV = {}'
```

The result html should be
```html
<scritp>window.APP_ENV = {"API_URL":"https://example.com/api", "SENTRY_URL": "https://sentry.com"}</script>
```

Only environment variables with prefix end up in the result json.
For security reasons you should always use prefix to avoid secrets in your publicly accessible files.

See `inject-env -h` for more options
