# FEmacs website

The site uses `docs/index.html` as its entry and `docs/pages/` for its topic
pages, artwork, CSS and Eliscript. It follows the browser's color scheme and
keeps commands and documentation readable on narrow screens.

The published Eliscript **0.1.3** browser host compiles and runs `site.eli`
in the browser. Eliscript owns the language-summary switch, accessible command
tabs and copy controls, and documentation search. Headings, commands and links
remain readable when the compiler is unavailable. The language toggle switches
summary text only. The site artwork uses the unchanged `packaging/assets/fe-logo.png`
master and its published copy under `docs/pages/assets/`.

## Preview

Run this command from the repository root and open `http://127.0.0.1:8080/`.
Internet access to the pinned compiler is required for interactive features.

```sh
python3 -m http.server 8080 --bind 127.0.0.1 --directory docs
```

## Publication

The canonical domain is `femacs.com` and the Pages source is the `docs`
folder on `master`. The branch publication serves the checked-in website
without a site-specific build workflow. Source links open the original
engineering documentation and acceptance records in the repository.

The redesigned site was checked in Chrome at 320, 390 and 1440 CSS pixels,
using both light and dark appearances and both root and prefixed routes.
The checks covered live compilation, keyboard tabs, copying, search, the
summary switch, and readable build instructions with the compiler blocked.
The previous design's acceptance is retained in [acceptance.json](acceptance.json).
