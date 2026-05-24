<script setup lang="ts">
import { Breadcrumb } from 'primevue'
import { ref } from 'vue'
import { useBreadcrumb } from '../../composables/breadcrumb'
import { RouteMenuItem } from '../types'
import { MaybeHasTabRoute } from '../../stores/tabs'

const { route } = defineProps<MaybeHasTabRoute>()
const breadcrumb = useBreadcrumb(() => route ?? null)
const nop = ref<RouteMenuItem>({ route: ['_unknown'] })
</script>

<template>
  <Breadcrumb :home="nop" :model="[...breadcrumb]" />
</template>
