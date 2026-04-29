import { debug, error, info, LogOptions, warn } from '@tauri-apps/plugin-log'

// TODO: Create logger with frontend prefixes.

export type LogLevel = 'debug' | 'info' | 'warn' | 'error'

/**
 * Tauri logger.
 */
export const log = {
  debug,
  info,
  warn,
  error,
  write: (level: LogLevel, message: string, opt?: LogOptions) => {
    switch (level) {
      case 'debug':
        debug(message, opt)
        break
      case 'info':
        info(message, opt)
        break
      case 'warn':
        warn(message, opt)
        break
      case 'error':
        error(message, opt)
        break
    }
  },
}
