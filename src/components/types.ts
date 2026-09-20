import { ColumnProps } from 'primevue'
import { MenuItem } from 'primevue/menuitem'
import { TabRoute } from '../stores/tabs'

export type RouteMenuItem = MenuItem & {
  readonly route: TabRoute
  readonly tooltip?: string
}

export type EnhancedColumnsProps = Omit<ColumnProps, 'columnKey'> & {
  index?: number
}

/**
 * Table column identity, the part needed to select and order columns.
 *
 * `K` = column key union.
 */
export type TableColumnBase<K extends string = string> = {
  readonly key: K
  readonly header?: string
  /** Locked columns are always visible and keep their place in the column order. */
  readonly locked?: boolean
}

/**
 * Table column definition.
 *
 * `T` = row data, `K` = column key union.
 */
export type TableColumn<T, K extends string = string> = EnhancedColumnsProps &
  TableColumnBase<K> & {
    readonly monospace?: boolean
    readonly value?: (data: T) => string | null
  }

export const isRouteMenuItem = (item: MenuItem): item is RouteMenuItem => 'route' in item
