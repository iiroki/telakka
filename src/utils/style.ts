import { CSSProperties } from 'vue'
import { toKebabCase } from './text'

export const createStyle = (...style: CSSProperties[]): CSSProperties => Object.assign({}, ...style)

export const createStyleString = (...style: CSSProperties[]): string =>
  Object.entries(createStyle(...style))
    .map(([k, v]) => `${toKebabCase(k)}: ${v};`)
    .join(' ')
