import { MenuItem } from 'primevue/menuitem'
import { TabRouteKey } from '../stores/tab'

export type RouteMenuItem = MenuItem & {
  readonly route?: TabRouteKey
}
