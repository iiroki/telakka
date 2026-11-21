# Tables

Default behavior for every resource table (e.g. the [container list](./docker.md#container-list)).
Each table's doc lists only what is specific to it: its columns, search fields, options and row behavior.

## Columns

- Columns can be resized; widths are kept relative to the table width.
- A table can mark columns as **locked**: always visible, and fixed in place.
- Values that don't fit are truncated, with the full value in a tooltip.
- Tooltips and other open popups stay open while the table's data updates.

## Column selector

A button in the table header opens the column selector, which edits the table's View preferences:

- Show/hide columns and move them up/down. Locked columns can't be hidden or moved.
- Toggle the table's options (e.g. showing an actions column).
- **Reset** returns to the table's defaults; **Save** applies and stores the changes. Edits are a draft until saved.

Column order, visibility, relative widths and options are stored per table in [View preferences](./settings.md#view-preferences).

## Search

A search box in the table header filters rows by a case-insensitive substring match on the table's search fields.

`Mod` + `F` focuses the search box of the table in the active Tab (see [Keyboard shortcuts](./layout.md#keyboard-shortcuts)).

## Rows

Selecting a row opens that resource's detail Route in the current Tab.
