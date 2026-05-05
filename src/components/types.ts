import { MenuItem } from 'primevue/menuitem'

export type RouteMenuItem = MenuItem & {
  readonly route?: string
}
