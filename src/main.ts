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

const PRIMEUI_LICENSE = 'eyJpZCI6IjRmMmM2NmI2LTMyODYtNDE1Yy1hNjdiLWQyYjAxZmVmMWFiOCIsInByb2R1Y3QiOiJwcmltZXVpIiwidGllciI6ImNvbW11bml0eSIsInR5cGUiOiJkZXYiLCJpYXQiOjE3ODk5NzMyNjMsImV4cCI6MTgyMTUwOTI2M30.V3VnTFmNOAWFSDA3y-xb4HP1bIG7rdH_pbGM0udacMegWofHblTyAxLDmSCpn1rpC66H_th_P5Uh7rhT7cZJCQ'

const app = createApp(App)

app.use(pinia)
app.use(PrimeVue, {
  theme: {
    preset: Theme,
    options: { darkModeSelector: `.${DARK_MODE_CLASS}` },
  },
  license: PRIMEUI_LICENSE,
})

app.use(ToastService)
app.directive('tooltip', Tooltip)
app.mount('#app')
