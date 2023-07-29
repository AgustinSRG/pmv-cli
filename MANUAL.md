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
| `media` | Manages media assets |
| `random` | Retrieves random media assets from the vault |
| `search` | Searches for media assets in the vault (Basic) |
| `advanced-search` | Searches for media assets in the vault (Advanced) |
| `tag` | Manages tags |
| `album` | Manages albums |
| `config` | Manages vault configuration |
| `task` | Retrieves tasks information |
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

## Command: media

Manages media assets

<ins>**Usage:**</ins>

```
pmv-cli media <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| `get` | Gets media asset metadata and download links |
| `stats` | Gets media asset size stats |
| `download` | Downloads a media asset |
| `export` | Exports a media asset, downloading everything (metadata + assets) into a folder |
| `upload` | Uploads a new media asset, waits for encryption and adds tags if specified |
| `set-title` | Changes the title of a media asset |
| `set-description` | Changes the description of a media asset |
| `set-extended-description` | Changes the extended description of a media asset |
| `set-force-start-beginning` | Changes the description of a media asset |
| `set-thumbnail` | Sets the thumbnail of a media asset |
| `get-time-slices` | Prints the time slices of a media asset |
| `set-time-slices` | Sets the time slices of a media asset |
| `set-image-notes` | Sets the image notes of a media asset |
| `add-resolution` | Adds new resolution to the media asset |
| `remove-resolution` | Removes a resolution from the media asset |
| `add-subtitle` | Adds subtitle file to a media asset |
| `remove-subtitle` | Removes subtitle file from a media asset |
| `add-audio` | Adds audio track file to a media asset |
| `remove-audio` | Removes audio track file from a media asset |
| `re-encode` | Re-Encodes a media asset |
| `delete` | Deletes a media asset |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media get

Gets media asset metadata and download links

<ins>**Usage:**</ins>

```
pmv-cli media get <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media stats

Gets media asset size stats

<ins>**Usage:**</ins>

```
pmv-cli media stats <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media download

Downloads a media asset

<ins>**Usage:**</ins>

```
pmv-cli media download [OPTIONS] <MEDIA> [ASSET]
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `[ASSET]` | Asset to download. Examples: original, thumbnail, resolution:1280x720:30, sub:ID, audio:ID, notes, preview:Index, ext_desc |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-o, --output <OUTPUT>` | Path to the file to download the asset into |
| `-p, --print-link` | Prints the download link, instead of downloading to a file |
| `-h, --help` | Print help |

### Command: media export

Exports a media asset, downloading everything (metadata + assets) into a folder

<ins>**Usage:**</ins>

```
pmv-cli media export [OPTIONS] <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-o, --output <OUTPUT>` | Path to the folder to download the files into |
| `-h, --help` | Print help |

### Command: media upload

Uploads a new media asset, waits for encryption and adds tags if specified

<ins>**Usage:**</ins>

```
pmv-cli media upload [OPTIONS] <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<PATH>` | Path to the file to upload |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-t, --title <TITLE>` | A title for the media asset |
| `-a, --album <ALBUM>` | Album to upload the media asset into |
| `-T, --tags <TAGS>` | Tags to add to the media asset, separated by spaces |
| `-s, --skip-encryption` | Do not wait for encryption |
| `-h, --help` | Print help |

### Command: media set-title

Changes the title of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-title <MEDIA> <TITLE>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<TITLE>` | Title |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media set-description

Changes the description of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-description <MEDIA> <DESCRIPTION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<DESCRIPTION>` | Description |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media set-extended-description

Changes the extended description of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-extended-description <MEDIA> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<PATH>` | Path to the text file containing the extended description |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media set-force-start-beginning

Changes the description of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-force-start-beginning <MEDIA> <FORCE_START_BEGINNING>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<FORCE_START_BEGINNING>` | Set to 'true' if you want to tell the clients not to store the time, so they always start from the beginning |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media set-thumbnail

Sets the thumbnail of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-thumbnail <MEDIA> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<PATH>` | Path to the thumbnail file |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media get-time-slices

Prints the time slices of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media get-time-slices <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media set-time-slices

Sets the time slices of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-time-slices <MEDIA> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<PATH>` | Path to the file containing the time slices |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media set-image-notes

Sets the image notes of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-image-notes <MEDIA> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<PATH>` | Path to the image notes file |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media add-resolution

Adds new resolution to the media asset

<ins>**Usage:**</ins>

```
pmv-cli media add-resolution <MEDIA> <RESOLUTION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<RESOLUTION>` | Resolution. Example: 1280x720:30 |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media remove-resolution

Removes a resolution from the media asset

<ins>**Usage:**</ins>

```
pmv-cli media remove-resolution <MEDIA> <RESOLUTION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<RESOLUTION>` | Resolution. Example: 1280x720:30 |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media add-subtitle

Adds subtitle file to a media asset

<ins>**Usage:**</ins>

```
pmv-cli media add-subtitle [OPTIONS] <MEDIA> <SUB_ID> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<SUB_ID>` | Subtitle file identifier. Example: EN |
| `<PATH>` | Path to the subtitles file |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `--name <NAME>` | Subtitle file display name. If not specified, the identifier is used |
| `-h, --help` | Print help |

### Command: media remove-subtitle

Removes subtitle file from a media asset

<ins>**Usage:**</ins>

```
pmv-cli media remove-subtitle <MEDIA> <SUB_ID>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<SUB_ID>` | Subtitle file identifier. Example: EN |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media add-audio

Adds audio track file to a media asset

<ins>**Usage:**</ins>

```
pmv-cli media add-audio [OPTIONS] <MEDIA> <TRACK_ID> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<TRACK_ID>` | Audio track file identifier. Example: EN |
| `<PATH>` | Path to the audio track file |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `--name <NAME>` | Audio track file display name. If not specified, the identifier is used |
| `-h, --help` | Print help |

### Command: media remove-audio

Removes audio track file from a media asset

<ins>**Usage:**</ins>

```
pmv-cli media remove-audio <MEDIA> <TRACK_ID>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<TRACK_ID>` | Audio track file identifier. Example: EN |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media re-encode

Re-Encodes a media asset

<ins>**Usage:**</ins>

```
pmv-cli media re-encode <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media delete

Deletes a media asset

<ins>**Usage:**</ins>

```
pmv-cli media delete <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |

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

## Command: album

Manages albums

<ins>**Usage:**</ins>

```
pmv-cli album <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| `list` | List albums |
| `get` | Get album and prints it |
| `create` | Creates a new album |
| `rename` | Renames an album |
| `delete` | Deletes album |
| `add` | Adds a media asset to an album |
| `remove` | Removes a media asset from an album |
| `set-position` | Changes the position of a media asset inside al album |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: album list

List albums

<ins>**Usage:**</ins>

```
pmv-cli album list [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-m, --media <MEDIA>` | Filter by media |
| `-c, --csv` | CSV format |
| `-a, --alphabetically` | Sort alphabetically by name |
| `-i, --id-sorted` | Sort by ID |
| `-h, --help` | Print help |

### Command: album get

Get album and prints it

<ins>**Usage:**</ins>

```
pmv-cli album get [OPTIONS] <ALBUM>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-e, --extended` | Extended version of the results table |
| `-c, --csv` | CSV format |
| `-h, --help` | Print help |

### Command: album create

Creates a new album

<ins>**Usage:**</ins>

```
pmv-cli album create <NAME>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<NAME>` | Album name |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: album rename

Renames an album

<ins>**Usage:**</ins>

```
pmv-cli album rename <ALBUM> <NAME>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |
| `<NAME>` | Album name |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: album delete

Deletes album

<ins>**Usage:**</ins>

```
pmv-cli album delete <ALBUM>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: album add

Adds a media asset to an album

<ins>**Usage:**</ins>

```
pmv-cli album add <ALBUM> <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: album remove

Removes a media asset from an album

<ins>**Usage:**</ins>

```
pmv-cli album remove <ALBUM> <MEDIA>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |
| `<MEDIA>` | Media asset ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: album set-position

Changes the position of a media asset inside al album

<ins>**Usage:**</ins>

```
pmv-cli album set-position <ALBUM> <MEDIA> <POSITION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |
| `<MEDIA>` | Media asset ID |
| `<POSITION>` | New position for the media asset, starting at 1 |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

## Command: config

Manages vault configuration

<ins>**Usage:**</ins>

```
pmv-cli config <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| `get` | Gets vault configuration |
| `get-css` | Gets custom CSS code configured for the vault |
| `set-title` | Sets vault title |
| `set-max-tasks` | Sets max tasks in parallel |
| `set-encoding-threads` | Sets number of encoding threads to use |
| `set-video-previews-interval` | Sets the video previews interval in seconds |
| `set-css` | Sets custom CSS for the vault |
| `clear-css` |  |
| `add-video-resolution` | Adds video resolution |
| `remove-video-resolution` | Removes video resolution |
| `add-image-resolution` | Adds image resolution |
| `remove-image-resolution` | Removes image resolution |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config get

Gets vault configuration

<ins>**Usage:**</ins>

```
pmv-cli config get
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config get-css

Gets custom CSS code configured for the vault

<ins>**Usage:**</ins>

```
pmv-cli config get-css
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config set-title

Sets vault title

<ins>**Usage:**</ins>

```
pmv-cli config set-title <TITLE>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<TITLE>` | Vault title |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config set-max-tasks

Sets max tasks in parallel

<ins>**Usage:**</ins>

```
pmv-cli config set-max-tasks <MAX_TASKS>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MAX_TASKS>` | Max tasks in parallel |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config set-encoding-threads

Sets number of encoding threads to use

<ins>**Usage:**</ins>

```
pmv-cli config set-encoding-threads <ENCODING_THREADS>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ENCODING_THREADS>` | Number of encoding threads to use |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config set-video-previews-interval

Sets the video previews interval in seconds

<ins>**Usage:**</ins>

```
pmv-cli config set-video-previews-interval <INTERVAL_SECONDS>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<INTERVAL_SECONDS>` | Interval in seconds |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config set-css

Sets custom CSS for the vault

<ins>**Usage:**</ins>

```
pmv-cli config set-css <FILE_PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<FILE_PATH>` | Path to the css file to use |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config clear-css

Usage: pmv-cli config clear-css

<ins>**Usage:**</ins>

```

```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config add-video-resolution

Adds video resolution

<ins>**Usage:**</ins>

```
pmv-cli config add-video-resolution <RESOLUTION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<RESOLUTION>` | Video resolution. Example: 1280x720:30 |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config remove-video-resolution

Removes video resolution

<ins>**Usage:**</ins>

```
pmv-cli config remove-video-resolution <RESOLUTION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<RESOLUTION>` | Video resolution. Example: 1280x720:30 |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config add-image-resolution

Adds image resolution

<ins>**Usage:**</ins>

```
pmv-cli config add-image-resolution <RESOLUTION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<RESOLUTION>` | Image resolution. Example: 1280x720 |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: config remove-image-resolution

Removes image resolution

<ins>**Usage:**</ins>

```
pmv-cli config remove-image-resolution <RESOLUTION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<RESOLUTION>` | Image resolution. Example: 1280x720 |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

## Command: task

Retrieves tasks information

<ins>**Usage:**</ins>

```
pmv-cli task <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| `list` | Lists current existing tasks |
| `monitor` | Monitors tasks |
| `get` | Get task status |
| `wait` | Waits for a task to finish, monitoring its status |
| `help` | Print this message or the help of the given subcommand(s) |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: task list

Lists current existing tasks

<ins>**Usage:**</ins>

```
pmv-cli task list [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-c, --csv` | CSV format |
| `-h, --help` | Print help |

### Command: task monitor

Monitors tasks

<ins>**Usage:**</ins>

```
pmv-cli task monitor
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: task get

Get task status

<ins>**Usage:**</ins>

```
pmv-cli task get <TASK>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<TASK>` | Task identifier |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: task wait

Waits for a task to finish, monitoring its status

<ins>**Usage:**</ins>

```
pmv-cli task wait <TASK>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<TASK>` | Task identifier |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |
