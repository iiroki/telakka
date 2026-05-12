export const isRecord = (v: unknown): v is Record<string, unknown> =>
  v !== null && typeof v === 'object' && !Array.isArray(v)

/**
 * Flattens the given object: `{ nested: { value: true } }` -> `{ 'nested.value': true }`
 */
export const flattenObject = (obj: Record<string, unknown>, prefix: string | null = null): Record<string, unknown> =>
  Object.entries(obj).reduce<Record<string, unknown>>((acc, [k, v]) => {
    const key = prefix ? `${prefix}.${k}` : k
    if (isRecord(v)) {
      Object.assign(acc, flattenObject(v, key))
    } else {
      acc[key] = v
    }

    return acc
  }, {})

/**
 * Expands the given object: `{ 'nested.value': true }` -> `{ nested: { value: true } }`
 */
export const expandObject = (obj: Record<string, unknown>): Record<string, unknown> =>
  Object.entries(obj).reduce<Record<string, unknown>>((acc, [k, v]) => {
    const keys = k.split('.')
    let current = acc
    for (let i = 0; i < keys.length - 1; i++) {
      const key = keys[i]
      if (!isRecord(current[key])) {
        current[key] = {}
      }

      const next = current[key]
      if (isRecord(next)) {
        current = next
      }
    }

    current[keys[keys.length - 1]] = v
    return acc
  }, {})
