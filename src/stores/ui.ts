import { defineStore } from 'pinia'
import { useNotificationStore } from './notification'
import { storage } from '../utils/storage'
import z from 'zod/v4'

const UI_KEY_PREFIX = 'ui__'

export const useUiStore = defineStore('ui', () => {
  const { notify } = useNotificationStore()

  const get = async <T = unknown>(key: string, schema?: z.ZodType<T>): Promise<T | undefined> => {
    try {
      return (await storage.read(UI_KEY_PREFIX + key, schema ?? (z.unknown() as z.ZodType<T>))) ?? undefined
    } catch (err) {
      notify({ level: 'warn', title: `Failed to load UI setting: ${key}`, err })
    }
  }

  const set = async (key: string, value: object): Promise<void> => {
    try {
      await storage.write(UI_KEY_PREFIX + key, value)
    } catch (err) {
      notify({ level: 'warn', title: `Failed to save UI setting: ${key}`, err })
    }
  }

  return { get, set }
})
