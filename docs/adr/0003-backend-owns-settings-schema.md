# 0003 Backend owns the settings schema

The backend defines the Settings schema and its defaults; the frontend uses the types generated from it.
Settings are split into top-level sections, and each section is validated and written independently.

- **Reading:** an invalid section is replaced with its defaults in the returned Settings only. The file on disk is left untouched, and the backend reports which sections failed so the frontend can warn the user.
- **Writing:** saving writes only the sections being saved into the file. Other sections, including invalid ones, are left as they are.

A single file holds both settings and View preferences, so one broken entry must not reset everything else, and the app must never destroy settings it couldn't read.
This replaces earlier frontend-side validation, where any invalid value reset the whole configuration to defaults.
