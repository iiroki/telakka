import { onUnmounted } from 'vue'

export type Shortcut = {
  readonly key: string
  /** **Cmd** on macOS, **Ctrl** elsewhere. */
  readonly mod?: boolean
  readonly shift?: boolean
  readonly alt?: boolean
}

export type ShortcutOptions = {
  /** Shortcut is ignored when this false. */
  readonly enabled?: boolean | (() => boolean)
}

const isShortcutEvent = (e: KeyboardEvent, { key, mod, shift, alt }: Shortcut): boolean =>
  e.key.toLowerCase() === key.toLowerCase() &&
  (e.metaKey || e.ctrlKey) === !!mod &&
  e.shiftKey === !!shift &&
  e.altKey === !!alt

/**
 * Registers a keyboard shortcut for the lifetime of the calling component.
 */
export const useShortcut = (shortcut: Shortcut, handler: (e: KeyboardEvent) => void, opt?: ShortcutOptions) => {
  const onKeyDown = (e: KeyboardEvent) => {
    const isEnabled = typeof opt?.enabled === 'function' ? opt.enabled() : opt?.enabled
    if (!isShortcutEvent(e, shortcut) || isEnabled === false) {
      return
    }

    e.preventDefault()
    handler(e)
  }

  document.addEventListener('keydown', onKeyDown)
  onUnmounted(() => document.removeEventListener('keydown', onKeyDown))
}
