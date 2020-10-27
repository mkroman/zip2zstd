# zip2zstd

This is a quick and dirty tool for converting a zip file to a `zstd`-compressed
tar file without the intermediate i/o touching the disk

## Usage

To use the tool, run it with the following arguments

`zip2zstd <input zip> <output tar.zst>`

The tool doesn't print any information unless there was an error

## Example

```
% unzip -ql ESStatistikListeModtag-20201018-173254.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
83768793308  2020-10-18 17:11   ESStatistikListeModtag.xml
---------                     -------
83768793308                     1 file

% zip2zstd ESStatistikListeModtag-20201018-173254.zip ESStatistikListeModtag-20201018-173254.tar.zst

% file ESStatistikListeModtag-20201018-173254.tar.zst
ESStatistikListeModtag-20201018-173254.tar.zst: Zstandard compressed data (v0.8+), Dictionary ID: None

% tar tvf ESStatistikListeModtag-20201018-173254.tar.zst 
-rw-r--r-- 1000/1000 83768793308 2020-10-18 19:11 ESStatistikListeModtag.xml
```
