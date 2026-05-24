<script setup lang="ts">
defineProps<{ readonly message?: string }>()
</script>

<template>
  <div class="todo">
    <i class="pi pi-exclamation-triangle todo-icon" />
    <span class="todo-text">TODO — This feature is not implemented yet.</span>
    <span v-if="message" class="todo-text">({{ message }})</span>
  </div>
</template>

<style scoped>
.todo {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  color: var(--p-primary-400);
  opacity: 0.6;
  margin-top: 5rem;
}

.todo-icon {
  font-size: 4rem;
}

.todo-text {
  font-size: 1rem;
  font-style: italic;
}
</style>
