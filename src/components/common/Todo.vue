<script setup lang="ts">
// ...
</script>

<template>
  <div class="todo">
    <i class="pi pi-exclamation-triangle todo-icon" />
    <span class="todo-text">TODO — This feature is not implemented yet.</span>
  </div>
</template>

<style scoped>
.todo {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  height: 100%;
  color: var(--p-primary-400);
  opacity: 0.6;
}

.todo-icon {
  font-size: 4rem;
}

.todo-text {
  font-size: 1rem;
  font-style: italic;
}
</style>
