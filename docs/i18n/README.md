# Documentation languages

Translated documents live under `docs/i18n/<locale>/`, where `<locale>` is a BCP 47 language tag
(`zh-CN`, `ja`, `de`, ...). A translation mirrors the path of its English original, so any
document can be found in every language at the same relative path.

| Locale | Language | Homepage |
| --- | --- | --- |
| `en` | English | [README.md](../../README.md) (repository default) |
| `zh-CN` | 简体中文 | [README.md](zh-CN/README.md) |

## Conventions

- The root `README.md` is the English homepage and the repository default. Every other language
  mirrors it at `docs/i18n/<locale>/README.md`.
- The English source of any further document lives at `docs/<name>.md`; its translations live at
  `docs/i18n/<locale>/<name>.md`.
- Adding a language means adding one directory and one row in the table above.
- This index is structural and is maintained in English only.
