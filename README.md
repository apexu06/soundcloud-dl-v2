# soundcloud-dl-v2
A cli tool written in rust to download songs from SoundCloud and apply metadata to them. Can be used by either entering
the input prompt or using cli options or both, whatever you prefer.


## features
- download any song from soundcloud (including those locked behind Go+) as mp3 files
- choose download location per download
- apply metadata to downloaded songs (customizable)
    - song title
    - artist name
    - genre
    - album name
    - album cover (will currently always apply the thumbnail from SoundCloud)
- interface made with [dialoguer](https://crates.io/crates/dialoguer) and [indicatif](https://crates.io/crates/indicatif)
- cli options to completely skip input prompt

## usage
### run with dialoguer interface
```
$ scdl
```
### specify url and artist
```
$ scdl --url https://www.soundcloud.com/user/song --artist shrek
```
This will prompt your for input, however it will skip the options to input a url and change the artist.

### use default metadata and url
```
$ scdl --url https://www.soundcloud.com/user/song --use-default-metadata
```
This will not prompt your for any input and will just print the location once finished. The default metadata applied will be
the data returned by spotify. *Note: As SoundCloud returns no data regarding an album name, this option will always be left empty if the
`--use-default-metadata` flag is used.*


