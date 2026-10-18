# Game string archive IDs

`TextBankIds::for_game` is the source of archive IDs for game string tables.
The primary key is `(GameFamily, GameLanguage)`. The archive ID is the member
index in the message NARC:

- DP: `msgdata/msg.narc`
- Platinum: `msgdata/pl_msg.narc`
- HGSS: `a/0/2/7`

Do not use the text archive encryption key to select a category. Different
categories can have the same key. A text editor can also change that key.

## Checked layouts

Western means English, French, German, Italian, or Spanish. English USA and
Europe use the same IDs. The games in each pair use the same IDs. No ID
difference was found between the checked revisions.

| Family | Language | Species | Items | Moves | Abilities | Types |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| DP | Japanese | 356 | 341 | 575 | 544 | 555 |
| DP | Korean | 357 | 342 | 577 | 546 | 557 |
| DP | Western | 362 | 344 | 588 | 552 | 565 |
| Platinum | Japanese | 408 | 390 | 636 | 604 | 616 |
| Platinum | Korean | 408 | 390 | 637 | 605 | 617 |
| Platinum | Western | 412 | 392 | 647 | 610 | 624 |
| HGSS | Japanese | 232 | 219 | 739 | 711 | 724 |
| HGSS | Korean | 233 | 220 | 743 | 715 | 728 |
| HGSS | Western | 237 | 222 | 750 | 720 | 735 |

| Family | Language | Locations | Trainers | Trainer classes | Sounds |
| --- | --- | ---: | ---: | ---: | ---: |
| DP | Japanese | 374 | 550 | 551 | 482 |
| DP | Korean | 376 | 552 | 553 | 484 |
| DP | Western | 382 | 559 | 560 | 490 |
| Platinum | Japanese | 427 | 611 | 612 | 539 |
| Platinum | Korean | 428 | 612 | 613 | 540 |
| Platinum | Western | 433 | 618 | 619 | 545 |
| HGSS | Japanese | 272 | 719 | 720 | 430 |
| HGSS | Korean | 274 | 723 | 724 | 432 |
| HGSS | Western | 279 | 729 | 730 | 437 |

These are the plain name tables, not descriptions, names with articles,
uppercase alternatives, or foreign Pokédex name tables.

The DP sound archive has 829 rows. The checked Western DP archives contain
placeholders. The Japanese and Korean archives contain sound names, but
`load_dspre_sound_archive_constants` does not support their sound ID mapping.
Consumers must keep their existing fallback for DP sound constants.

## ROM survey

The survey read all available DS Pokemon Games: 46 distinct header identities, and 23 distinct message NARCs.

The survey used ndspy to read the ROM file system, Uxie to read each NARC,
and Chatot to decode its text archives. The selected tables were checked
by their contents and row counts, not by their encryption keys. No ROM
files or project files were changed.

The version values below are the bytes in the ROM headers. For example,
the English Europe releases have version 10 or 13; these are not the
revision labels in their filenames.

| Game codes | Header versions checked |
| --- | --- |
| ADAJ | 0, 1, 5, 6 |
| APAJ | 0, 5, 6 |
| ADAK, APAK | 0 |
| ADAD, ADAF, ADAI, ADAS, APAD, APAF, APAI, APAS | 5 |
| ADAE, APAE | 5, 13 |
| CPUJ, CPUK, CPUD, CPUF, CPUI, CPUS | 0 |
| CPUE | 0, 1, 10 |
| IPKJ, IPGJ, IPKK, IPGK | 0 |
| IPKD, IPKF, IPKI, IPKS, IPGD, IPGF, IPGI, IPGS | 0 |
| IPKE, IPGE | 0, 10 |

`ADA` is Diamond, `APA` is Pearl, `CPU` is Platinum, `IPK` is HeartGold,
and `IPG` is SoulSilver. The last code letter identifies the language:
`J` Japanese, `K` Korean, `E` English, `D` German, `F` French,
`I` Italian, and `S` Spanish.
