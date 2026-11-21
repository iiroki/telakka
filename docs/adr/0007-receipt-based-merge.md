# 0007 Receipt-based merge of polled lists

Each item in a polled list carries a Receipt computed by the backend over its identity-relevant fields only; volatile fields (timestamps, transient status detail) are left out, so identical states produce identical Receipts.
The frontend merges each poll into its current state and keeps the existing item object whenever the Receipt is unchanged.
When nothing changed, the merge returns the original list unchanged.

Replacing the lists on every poll would make the UI re-render everything every second and reset UI state attached to rows, such as open tooltips and popovers.
Keeping the same objects lets the UI skip work when nothing changed and keeps that UI state across polls.
Very small resources can be compared field by field without a Receipt.

**Backend invariant:** polled lists are returned sorted by their identifying field, ascending, so the merge can walk both lists in one pass.
