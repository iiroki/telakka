import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice'
import Tooltip from 'primevue/tooltip'
import { createApp } from 'vue'
import App from './App.vue'
import Theme, { DARK_MODE_CLASS } from './theme'

// Styles:
import 'primeicons/primeicons.css'
import '@fontsource-variable/inter/index.css'
import { pinia } from './stores/pinia'

const app = createApp(App)

app.use(pinia)
app.use(PrimeVue, {
  theme: {
    preset: Theme,
    options: { darkModeSelector: `.${DARK_MODE_CLASS}` },
  },
})

app.use(ToastService)
app.directive('tooltip', Tooltip)
app.mount('#app')
