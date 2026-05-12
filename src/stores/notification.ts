import { defineStore } from 'pinia'
import { ToastMessageOptions, useToast } from 'primevue'
import { log, LogLevel } from '../utils/log'

export type NotificationMessage = {
  readonly level?: ToastMessageOptions['severity']
  readonly title?: string
  readonly content?: string
  readonly toastMs?: number

  // Some non-toast properties shown in the notificatin center?

  readonly styleClass?: string
  readonly contentStyleClass?: string

  // Internal
  /**
   * Whether to also log the notification with the logger (default: `true`).
   */
  readonly log?: boolean
  // Error?
}

export const useNotificationStore = defineStore('notification', () => {
  const toast = useToast()

  /**
   * Invokes a notification with the given message to various channels, such as toast, notification center, logger, etc.
   */
  const notify = (message: NotificationMessage) => {
    const toastMs = message.toastMs ?? (message.level == 'error' || message.level == 'warn' ? 10_000 : 3000)
    toast.add({
      severity: message.level,
      summary: message.title,
      detail: message.content,
      life: toastMs,
      styleClass: message.styleClass,
      contentStyleClass: message.contentStyleClass,
    })

    if (message.log !== false) {
      log.write(getLogLevel(message.level), [message.title, message.content].filter(Boolean).join(' — '))
    }
  }

  return { notify }
})

const getLogLevel = (level: NotificationMessage['level']): LogLevel => {
  switch (level) {
    case 'debug':
      return 'debug'
    case 'warn':
      return 'warn'
    case 'error':
      return 'error'
    default:
      return 'info'
  }
}
