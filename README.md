### Create .env

```shell
touch .env
```

Within the .env file created, paste the following:

```plaintext
OPEN_AI_ORG=YOUR_OPEN_AI_ORG_ID
OPEN_AI_KEY=YOUR_OPEN_AI_KEY
```

### Update Paths

Update constants in the src/helpers/general path.

These should link to a code template which you want your web server to use and the main.rs file where it will attempt to execute new code it writes.

### Build Project

```shell
cargo build
```

### Run Project

```shell
cargo run
```
