<template>
  <div class="prompt-page">
    <PromptTemplates />
  </div>
</template>

<script>
import PromptTemplates from '../components/PromptTemplates.vue'

export default {
  name: 'PromptPage',
  components: {
    PromptTemplates
  }
}
</script>

<style scoped>
.prompt-page {
  padding: 2rem;
}
</style> 