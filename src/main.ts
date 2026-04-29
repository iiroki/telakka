import Aura from '@primeuix/themes/aura'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice'
import Tooltip from 'primevue/tooltip'
import { createApp } from 'vue'
import App from './App.vue'

// Styles:
import 'primeicons/primeicons.css'
import '@fontsource-variable/inter/index.css'

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      // darkModeSelector: false, // Uncomment to test the light mode
    },
  },
})

app.use(ToastService)
app.directive('tooltip', Tooltip)
app.mount('#app')
