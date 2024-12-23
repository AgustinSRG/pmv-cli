# Manual

Command line interface client for PersonalMediaVault

<ins>**Usage:**</ins>

```
pmv-cli [OPTIONS] <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| [login](#command-login) | Logins into an active vault, printing a session URL into the standard output |
| [logout](#command-logout) | Closes the active session, given a session URL |
| [account](#command-account) | Manages accounts |
| [media](#command-media) | Manages media assets |
| [random](#command-random) | Retrieves random media assets from the vault |
| [search](#command-search) | Searches for media assets in the vault (Basic) |
| [advanced-search](#command-advanced-search) | Searches for media assets in the vault (Advanced) |
| [tag](#command-tag) | Manages tags |
| [album](#command-album) | Manages albums |
| [config](#command-config) | Manages vault configuration |
| [task](#command-task) | Retrieves tasks information |
| [invites](#command-invites) | Manages invites |
| [batch](#command-batch) | Applies a batch operation to a list of media assets |
| [get-server-information](#command-get-server-information) | Gets server information, like the version it is using |

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
| `-D, --duration <DURATION>` | Session duration. Can be: day, week, month or year |
| `-I, --invite-code <INVITE_CODE>` | Invite code. Setting this option will ignore the credentials and use the code |
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
| [context](#command-account-context) | Prints account context to the standard output |
| [change-username](#command-account-change-username) | Changes username (only for root account) |
| [change-password](#command-account-change-password) | Changes account password |
| [list](#command-account-list) | List accounts |
| [create](#command-account-create) | Creates new account |
| [delete](#command-account-delete) | Deletes an existing account |

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
| [get](#command-media-get) | Gets media asset metadata and download links |
| [stats](#command-media-stats) | Gets media asset size stats |
| [download](#command-media-download) | Downloads a media asset |
| [export](#command-media-export) | Exports a media asset, downloading everything (metadata + assets) into a folder |
| [upload](#command-media-upload) | Uploads a new media asset, waits for encryption and adds tags if specified |
| [import](#command-media-import) | Imports a media asset, expecting a folder with the same format the export command uses |
| [set-title](#command-media-set-title) | Changes the title of a media asset |
| [set-description](#command-media-set-description) | Changes the description of a media asset |
| [set-extended-description](#command-media-set-extended-description) | Changes the extended description of a media asset |
| [set-force-start-beginning](#command-media-set-force-start-beginning) | Changes the forced start from beginning parameter of a media asset |
| [set-is-animation](#command-media-set-is-animation) | Changes the is-animation parameter of a media asset |
| [set-thumbnail](#command-media-set-thumbnail) | Sets the thumbnail of a media asset |
| [get-time-slices](#command-media-get-time-slices) | Prints the time slices of a media asset |
| [set-time-slices](#command-media-set-time-slices) | Sets the time slices of a media asset |
| [set-image-notes](#command-media-set-image-notes) | Sets the image notes of a media asset |
| [add-resolution](#command-media-add-resolution) | Adds new resolution to the media asset |
| [remove-resolution](#command-media-remove-resolution) | Removes a resolution from the media asset |
| [add-subtitle](#command-media-add-subtitle) | Adds subtitle file to a media asset |
| [rename-subtitle](#command-media-rename-subtitle) | Renames a subtitles file |
| [remove-subtitle](#command-media-remove-subtitle) | Removes subtitle file from a media asset |
| [add-audio](#command-media-add-audio) | Adds audio track file to a media asset |
| [rename-audio](#command-media-rename-audio) | Renames an audio track file |
| [remove-audio](#command-media-remove-audio) | Removes audio track file from a media asset |
| [add-attachment](#command-media-add-attachment) | Adds attachment file |
| [rename-attachment](#command-media-rename-attachment) | Renames attachment file |
| [remove-attachment](#command-media-remove-attachment) | Removes attachment file |
| [re-encode](#command-media-re-encode) | Re-Encodes a media asset |
| [replace](#command-media-replace) | Replaces the media asset with another file |
| [delete](#command-media-delete) | Deletes a media asset |

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
| `[ASSET]` | Asset to download. Examples: original, thumbnail, resolution:1280x720:30, sub:ID, audio:ID, attachment:ID, notes, preview:Index, ext_desc |

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

### Command: media import

Imports a media asset, expecting a folder with the same format the export command uses

<ins>**Usage:**</ins>

```
pmv-cli media import [OPTIONS] <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<PATH>` | Path to the folder to import |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-a, --album <ALBUM>` | Album to upload the media asset into |
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

Changes the forced start from beginning parameter of a media asset

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

### Command: media set-is-animation

Changes the is-animation parameter of a media asset

<ins>**Usage:**</ins>

```
pmv-cli media set-is-animation <MEDIA> <IS_ANIMATION>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<IS_ANIMATION>` | Set to 'true' if you want to tell the clients to treat the media as an animation, so they force the loop and disable time skipping |

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

### Command: media rename-subtitle

Renames a subtitles file

<ins>**Usage:**</ins>

```
pmv-cli media rename-subtitle [OPTIONS] <MEDIA> <SUB_ID>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<SUB_ID>` | Subtitle file identifier. Example: EN |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `--new-id <NEW_ID>` | New ID for the subtitles file |
| `--new-name <NEW_NAME>` | New name for the subtitles file |
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

### Command: media rename-audio

Renames an audio track file

<ins>**Usage:**</ins>

```
pmv-cli media rename-audio [OPTIONS] <MEDIA> <TRACK_ID>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<TRACK_ID>` | Audio track file identifier. Example: EN |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `--new-id <NEW_ID>` | New ID for the audio track file |
| `--new-name <NEW_NAME>` | New name for the audio track file |
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

### Command: media add-attachment

Adds attachment file

<ins>**Usage:**</ins>

```
pmv-cli media add-attachment <MEDIA> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<PATH>` | Path to the attachment file |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media rename-attachment

Renames attachment file

<ins>**Usage:**</ins>

```
pmv-cli media rename-attachment <MEDIA> <ATTACHMENT_ID> <NAME>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<ATTACHMENT_ID>` | Attachment ID |
| `<NAME>` | New name for the attachment file |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: media remove-attachment

Removes attachment file

<ins>**Usage:**</ins>

```
pmv-cli media remove-attachment <MEDIA> <ATTACHMENT_ID>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<ATTACHMENT_ID>` | Attachment ID |

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

### Command: media replace

Replaces the media asset with another file

<ins>**Usage:**</ins>

```
pmv-cli media replace <MEDIA> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<MEDIA>` | Media asset ID |
| `<PATH>` | Path to the media file to upload |

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
| `-s, --start-from <START_FROM>` | Media id to use as a stating point for the scanning process |
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
| [list](#command-tag-list) | List tags |
| [add](#command-tag-add) | Adds a tag to a media asset |
| [remove](#command-tag-remove) | Removes a tag from a media asset |

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
| [list](#command-album-list) | List albums |
| [get](#command-album-get) | Get album and prints it |
| [download-thumbnail](#command-album-download-thumbnail) | Downloads the thumbnail of an album |
| [create](#command-album-create) | Creates a new album |
| [rename](#command-album-rename) | Renames an album |
| [change-thumbnail](#command-album-change-thumbnail) | Changes the thumbnail of an album |
| [delete](#command-album-delete) | Deletes album |
| [add](#command-album-add) | Adds a media asset to an album |
| [remove](#command-album-remove) | Removes a media asset from an album |
| [set-position](#command-album-set-position) | Changes the position of a media asset inside al album |
| [export](#command-album-export) | Exports an album, downloading everything (metadata + assets) into a folder |
| [import](#command-album-import) | Imports an album, expecting a folder with the same format the export command uses |
| [optimize-thumbnails](#command-album-optimize-thumbnails) | Optimizes thumbnails of albums, making the loading process faster |

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

### Command: album download-thumbnail

Downloads the thumbnail of an album

<ins>**Usage:**</ins>

```
pmv-cli album download-thumbnail [OPTIONS] <ALBUM>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-o, --output <OUTPUT>` | Path to the file to download the asset into |
| `-p, --print-link` | Prints the download link, instead of downloading to a file |
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

### Command: album change-thumbnail

Changes the thumbnail of an album

<ins>**Usage:**</ins>

```
pmv-cli album change-thumbnail <ALBUM> <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |
| `<PATH>` | Path to the thumbnail file |

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

### Command: album export

Exports an album, downloading everything (metadata + assets) into a folder

<ins>**Usage:**</ins>

```
pmv-cli album export [OPTIONS] <ALBUM>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-o, --output <OUTPUT>` | Path to the folder to download the files into |
| `-h, --help` | Print help |

### Command: album import

Imports an album, expecting a folder with the same format the export command uses

<ins>**Usage:**</ins>

```
pmv-cli album import <PATH>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<PATH>` | Path to the folder to import |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: album optimize-thumbnails

Optimizes thumbnails of albums, making the loading process faster

<ins>**Usage:**</ins>

```
pmv-cli album optimize-thumbnails
```

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
| [get](#command-config-get) | Gets vault configuration |
| [get-css](#command-config-get-css) | Gets custom CSS code configured for the vault |
| [set-title](#command-config-set-title) | Sets vault title |
| [set-max-tasks](#command-config-set-max-tasks) | Sets max tasks in parallel |
| [set-encoding-threads](#command-config-set-encoding-threads) | Sets number of encoding threads to use |
| [set-video-previews-interval](#command-config-set-video-previews-interval) | Sets the video previews interval in seconds |
| [set-css](#command-config-set-css) | Sets custom CSS for the vault |
| [clear-css](#command-config-clear-css) | Clears custom CSS for the vault |
| [add-video-resolution](#command-config-add-video-resolution) | Adds video resolution |
| [remove-video-resolution](#command-config-remove-video-resolution) | Removes video resolution |
| [add-image-resolution](#command-config-add-image-resolution) | Adds image resolution |
| [remove-image-resolution](#command-config-remove-image-resolution) | Removes image resolution |

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

Clears custom CSS for the vault

<ins>**Usage:**</ins>

```
pmv-cli config clear-css
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
| [list](#command-task-list) | Lists current existing tasks |
| [monitor](#command-task-monitor) | Monitors tasks |
| [get](#command-task-get) | Get task status |
| [wait](#command-task-wait) | Waits for a task to finish, monitoring its status |

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

## Command: invites

Manages invites

<ins>**Usage:**</ins>

```
pmv-cli invites <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| [check](#command-invites-check) | Prints the current invite code, if any |
| [generate](#command-invites-generate) | Generates a new invite code |
| [clear](#command-invites-clear) | Clears the current invite code |
| [list-sessions](#command-invites-list-sessions) | List active invited sessions |
| [close-session](#command-invites-close-session) | Closes an invited session |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: invites check

Prints the current invite code, if any

<ins>**Usage:**</ins>

```
pmv-cli invites check
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: invites generate

Generates a new invite code

<ins>**Usage:**</ins>

```
pmv-cli invites generate [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-D, --duration <DURATION>` | Session duration. Can be: day, week, month or year |
| `-h, --help` | Print help |

### Command: invites clear

Clears the current invite code

<ins>**Usage:**</ins>

```
pmv-cli invites clear
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: invites list-sessions

List active invited sessions

<ins>**Usage:**</ins>

```
pmv-cli invites list-sessions [OPTIONS]
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-c, --csv` | CSV format |
| `-h, --help` | Print help |

### Command: invites close-session

Closes an invited session

<ins>**Usage:**</ins>

```
pmv-cli invites close-session <INDEX>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<INDEX>` | Session index |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

## Command: batch

Applies a batch operation to a list of media assets

<ins>**Usage:**</ins>

```
pmv-cli batch [OPTIONS] <COMMAND>
```

<ins>**Commands:**</ins>

| Command | Description |
| --- | --- |
| [add-tags](#command-batch-add-tags) | Adds tags to the media assets |
| [remove-tags](#command-batch-remove-tags) | Removes tags from the media assets |
| [add-to-album](#command-batch-add-to-album) | Adds media assets into an album |
| [remove-from-album](#command-batch-remove-from-album) | Removes media assets from an album, if they were in it |
| [delete](#command-batch-delete) | Delete media assets |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-q, --title <TITLE>` | Filter by title |
| `-d, --description <DESCRIPTION>` | Filter by description |
| `-k, --media-type <MEDIA_TYPE>` | Filter by media type. Can be: video, audio or image |
| `-t, --tags <TAGS>` | Filter by tags. Expected a list of tag names, separated by spaces |
| `-m, --tags-mode <TAGS_MODE>` | Tag filtering mode. Can be: all, any, none or untagged |
| `-a, --album <ALBUM>` | Filter by album. Expected an album ID, like: #1 |
| `-e, --everything` | Do not filter. Apply to the entire vault instead |
| `-h, --help` | Print help |

### Command: batch add-tags

Adds tags to the media assets

<ins>**Usage:**</ins>

```
pmv-cli batch add-tags <TAGS>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<TAGS>` | List of tag names, separated by spaces |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: batch remove-tags

Removes tags from the media assets

<ins>**Usage:**</ins>

```
pmv-cli batch remove-tags <TAGS>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<TAGS>` | List of tag names, separated by spaces |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: batch add-to-album

Adds media assets into an album

<ins>**Usage:**</ins>

```
pmv-cli batch add-to-album <ALBUM>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: batch remove-from-album

Removes media assets from an album, if they were in it

<ins>**Usage:**</ins>

```
pmv-cli batch remove-from-album <ALBUM>
```

<ins>**Arguments:**</ins>

| Argument | Description |
| --- | --- |
| `<ALBUM>` | Album ID |

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

### Command: batch delete

Delete media assets

<ins>**Usage:**</ins>

```
pmv-cli batch delete
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |

## Command: get-server-information

Gets server information, like the version it is using

<ins>**Usage:**</ins>

```
pmv-cli get-server-information
```

<ins>**Options:**</ins>

| Option | Description |
| --- | --- |
| `-h, --help` | Print help |
