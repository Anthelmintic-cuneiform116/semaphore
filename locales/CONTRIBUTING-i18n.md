# Contributing translations

Semaphore ships with **English** (`en`) and **Portuguese** (`pt-BR`).

To add a new language:

1. Copy `locales/en.json` to `locales/<locale>.json` (e.g. `es.json`, `de.json`)
2. Translate all string values; keep keys unchanged
3. Add the locale to the `<select id="locale-select">` in `index.html`
4. Register the locale in `src/i18n.ts`
5. Open a pull request

Use BCP 47 locale codes (`fr`, `ja`, `zh-CN`, …).

We do not ship machine-translated locales in the main repo. Community contributions are welcome.
