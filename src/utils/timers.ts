export const createSafeInterval = (handler: () => void | Promise<void>, timeoutMs: number, instant = false): number => {
  let busy = false
  const run = async () => {
    if (busy) {
      return
    }

    busy = true
    try {
      await handler()
    } finally {
      // Log uncaught errors?
      busy = false
    }
  }

  if (instant) {
    setTimeout(run)
  }

  return setInterval(run, timeoutMs)
}
