import { defineStore } from 'pinia'
import { useNotificationStore } from './notification'
import { storage } from '../utils/storage'
import { log } from '../utils/log'
import z from 'zod/v4'

const UI_KEY_PREFIX = 'ui__'

export const zTableOptions = z.object({
  columns: z
    .object({
      widths: z.number().array().optional(),
    })
    .optional(),
})

export type TableOptions = z.infer<typeof zTableOptions>

export const useUiStore = defineStore('ui', () => {
  const { notify } = useNotificationStore()

  const get = async <T = unknown>(key: string, schema?: z.ZodType<T>): Promise<T | undefined> => {
    try {
      const value = (await storage.read(UI_KEY_PREFIX + key, schema ?? (z.unknown() as z.ZodType<T>))) ?? undefined
      log.debug(`Loaded UI setting: ${key} — ${JSON.stringify(value)}`)
      return value
    } catch (err) {
      notify({ level: 'warn', title: `Failed to load UI setting: ${key}`, err })
    }
  }

  const set = async (key: string, value: object): Promise<void> => {
    try {
      await storage.write(UI_KEY_PREFIX + key, value)
      log.debug(`Saved UI setting: ${key} — ${JSON.stringify(value)}`)
    } catch (err) {
      notify({ level: 'warn', title: `Failed to save UI setting: ${key}`, err })
    }
  }

  return { get, set }
})
