# Manual

Command line interface client for PersonalMediaVault

<ins>**Usage:**</ins>

```
pmv-cli [OPTIONS] <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| `login` | Logins into an active vault, printing a session URL into the standard output |
| `logout` | Closes the active session, given a session URL |
| `account` | Manages accounts |
| `random` | Retrieves random media assets from the vault |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-u, --vault-url <VAULT_URL>` | HTTP connection URL to the active vault |
| `-v, --verbose` | Turn verbose messages on |
| `-y, --yes` | Auto confirm actions |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

## Command: login

Logins into an active vault, printing a session URL into the standard output

<ins>**Usage:**</ins>

```
pmv-cli login [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-U, --username <USERNAME>` | Vault username. You can also specify the credentials in the URL |
| `-h, --help` | Print help |

## Command: logout

Closes the active session, given a session URL

<ins>**Usage:**</ins>

```
pmv-cli logout
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

## Command: account

Manages accounts

<ins>**Usage:**</ins>

```
pmv-cli account <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| `context` | Prints account context to the standard output |
| `change-username` | Changes username (only for root account) |
| `change-password` | Changes account password |
| `list` | List accounts |
| `create` | Creates new account |
| `delete` | Deletes an existing account |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: account context

Prints account context to the standard output

<ins>**Usage:**</ins>

```
pmv-cli account context
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: account change-username

Changes username (only for root account)

<ins>**Usage:**</ins>

```
pmv-cli account change-username <USERNAME>
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: account change-password

Changes account password

<ins>**Usage:**</ins>

```
pmv-cli account change-password
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: account list

List accounts

<ins>**Usage:**</ins>

```
pmv-cli account list [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-c, --csv` | CSV format |
| `-h, --help` | Print help |

### Command: account create

Creates new account

<ins>**Usage:**</ins>

```
pmv-cli account create [OPTIONS] <USERNAME>
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-a, --allow-write` | Allows the new account to modify the vault |
| `-h, --help` | Print help |

### Command: account delete

Deletes an existing account

<ins>**Usage:**</ins>

```
pmv-cli account delete <USERNAME>
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

## Command: random

Retrieves random media assets from the vault

<ins>**Usage:**</ins>

```
pmv-cli random [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-s, --seed <SEED>` | PRNG seed |
| `-p, --page-size <PAGE_SIZE>` | Page size, 10 by default |
| `-t, --tag <TAG>` | Filter by a tag |
| `-e, --extended` | Extended version of the results table |
| `-c, --csv` | CSV format |
| `-h, --help` | Print help |
