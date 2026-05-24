import { MenuItem } from 'primevue/menuitem'
import { TabRoute } from '../stores/tabs'

export type RouteMenuItem = MenuItem & {
  readonly route: TabRoute
  readonly tooltip?: string
}

export const isRouteMenuItem = (item: MenuItem): item is RouteMenuItem => 'route' in item
