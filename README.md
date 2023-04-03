# Emoji parser

The range of emojis is not continguous, and additionally there are varying forms
that a given emoji might take (16 bits, 32 bits, or more).

The source of truth for these characters is unicode.org, specifically [this
page](https://unicode.org/emoji/charts/full-emoji-list.html) which in turn links
to some underlying datasheets.

This program parses all of the emoji out of one of those datasheets and emits
them to stdout as a JSON-formatted array of string values.
