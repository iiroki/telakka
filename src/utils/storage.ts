import z from 'zod/v4'
import { commands } from '../tauri/bindings.gen'

export interface StorageService {
  readonly readRaw: (key: string) => Promise<string | null>
  readonly read: <T>(key: string, parser: z.ZodType<T>) => Promise<T | null>
  readonly write: (key: string, value: object) => Promise<void>
}

const readRaw: StorageService['readRaw'] = async (key) => {
  const result = await commands.storageReadJson(key)
  return result.status === 'ok' ? result.data : null
}

export const storage: StorageService = {
  readRaw,
  read: async <T>(key: string, parser: z.ZodType<T>): Promise<T | null> => {
    const result = await readRaw(key)
    return result ? parser.parse(JSON.parse(result)) : null
  },
  write: async (key: string, value: object): Promise<void> => {
    const result = await commands.storageWriteJson(key, JSON.stringify(value, null, 2))
    if (result.status === 'error') throw new Error(result.error)
  },
}
