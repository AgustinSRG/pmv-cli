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
| `search` | Searches for media assets in the vault (Basic) |
| `advanced-search` | Searches for media assets in the vault (Advanced) |
| `tag` | Manages tags |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-u, --vault-url <VAULT_URL>` | HTTP connection URL to the active vault |
| `-d, --debug` | Turn debug messages on |
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

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<USERNAME>` | Username to change into |

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

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<USERNAME>` | Username for the new account |

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

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<USERNAME>` | Username of the account to delete |

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

## Command: search

Searches for media assets in the vault (Basic)

<ins>**Usage:**</ins>

```
pmv-cli search [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-p, --page <PAGE>` | Selects the results page. The fist page is the page 1 |
| `-s, --page-size <PAGE_SIZE>` | Page size, 10 by default |
| `-t, --tag <TAG>` | Filter by a tag |
| `-r, --reverse` | Reverses results sorting. By default newest results are first. With this option, oldest results are first |
| `-e, --extended` | Extended version of the results table |
| `-c, --csv` | CSV format |
| `-h, --help` | Print help |

## Command: advanced-search

Searches for media assets in the vault (Advanced)

<ins>**Usage:**</ins>

```
pmv-cli advanced-search [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-q, --title <TITLE>` | Filter by title |
| `-d, --description <DESCRIPTION>` | Filter by description |
| `-k, --media-type <MEDIA_TYPE>` | Filter by media type. Can be: video, audio or image |
| `-t, --tags <TAGS>` | Filter by tags. Expected a list of tag names, separated by spaces |
| `-m, --tags-mode <TAGS_MODE>` | Tag filtering mode. Can be: all, any, none or untagged |
| `-a, --album <ALBUM>` | Filter by album. Expected an album ID, like: #1 |
| `-l, --limit <LIMIT>` | Limit on the number of results to get. 25 by default |
| `-s, --skip <SKIP>` | Number of results to skip. 0 by default |
| `-r, --reverse` | Reverses results sorting. By default newest results are first. With this option, oldest results are first |
| `-e, --extended` | Extended version of the results table |
| `-c, --csv` | CSV format |
| `-h, --help` | Print help |

## Command: tag

Manages tags

<ins>**Usage:**</ins>

```
pmv-cli tag <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| `list` | List tags |
| `add` | Adds a tag to a media asset |
| `remove` | Removes a tag from a media asset |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: tag list

List tags

<ins>**Usage:**</ins>

```
pmv-cli tag list [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-c, --csv` | CSV format |
| `-a, --alphabetically` | Sort alphabetically by name |
| `-h, --help` | Print help |

### Command: tag add

Adds a tag to a media asset

<ins>**Usage:**</ins>

```
pmv-cli tag add <TAG> <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<TAG>` | Tag name or identifier |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: tag remove

Removes a tag from a media asset

<ins>**Usage:**</ins>

```
pmv-cli tag remove <TAG> <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<TAG>` | Tag name or identifier |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |
