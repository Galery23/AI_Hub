<template>
  <nav class="navigation">
    <router-link to="/" class="nav-item">
      <span class="icon">🏠</span>
      <span class="text">首页</span>
    </router-link>
    <router-link to="/prompts" class="nav-item">
      <span class="icon">📝</span>
      <span class="text">Prompt 模板</span>
    </router-link>
    <router-link to="/preferences" class="nav-item">
      <span class="icon">⚙️</span>
      <span class="text">设置</span>
    </router-link>
    <router-link to="/about" class="nav-item">
      <span class="icon">ℹ️</span>
      <span class="text">关于</span>
    </router-link>
  </nav>
</template>

<script>
export default {
  name: 'Navigation'
}
</script>

<style scoped>
.navigation {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-around;
  align-items: center;
  background-color: var(--bg-color);
  border-top: 1px solid var(--border-color);
  padding: 0.5rem;
  z-index: 100;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-decoration: none;
  color: var(--text-color);
  padding: 0.5rem;
  border-radius: 0.5rem;
  transition: all 0.3s ease;
}

.nav-item:hover {
  background-color: var(--hover-shadow);
}

.nav-item.router-link-active {
  color: var(--primary-color);
}

.icon {
  font-size: 1.5rem;
  margin-bottom: 0.25rem;
}

.text {
  font-size: 0.75rem;
}
</style> 