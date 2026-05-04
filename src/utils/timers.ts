type SafeIntervalOptions = {
  /**
   * Run instantly on creation.
   */
  readonly instant?: boolean

  /** Backoff time in ms to wait before retrying if the handler fails. */
  readonly backoffMs?: number | ((errorCount: number) => number)
}

export const createSafeInterval = (
  handler: () => void | Promise<void>,
  timeoutMs: number,
  opt?: SafeIntervalOptions,
): number => {
  let errorCount = 0
  let busy = false
  const run = async () => {
    if (busy) {
      return
    }

    busy = true
    try {
      await handler()
      errorCount = 0
    } catch {
      // Log uncaught errors?
      ++errorCount
    } finally {
      busy = false

      // Backoff (if defined)
      if (errorCount > 0) {
        const backoffMs = typeof opt?.backoffMs === 'function' ? opt.backoffMs(errorCount) : (opt?.backoffMs ?? -1)

        if (backoffMs > 0) {
          await new Promise((r) => setTimeout(r, backoffMs))
        }
      }
    }
  }

  if (opt?.instant) {
    setTimeout(run)
  }

  return setInterval(run, timeoutMs)
}
