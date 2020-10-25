# zip2zstd

This is a quick and dirty tool for extracting a compressed zip file that contains _only a single_
file and compressing it with `zstd` without the intermediate file touching the disk

## Example

```
% unzip -ql ESStatistikListeModtag-20201018-173254.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
83768793308  2020-10-18 17:11   ESStatistikListeModtag.xml
---------                     -------
83768793308                     1 file

% zip2zstd ESStatistikListeModtag-20201018-173254.zip ESStatistikListeModtag-20201018-173254.xml.zst

% file ESStatistikListeModtag-20201018-173254.xml.zst
ESStatistikListeModtag-20201018-173254.xml.zst: Zstandard compressed data (v0.8+), Dictionary ID: None

% ls -l ESStatistikListeModtag-20201018-173254.xml.zst 
-rw-r--r-- 1 … … 3136236637 Oct 25 05:51 ESStatistikListeModtag-20201018-173254.xml.zst
```
