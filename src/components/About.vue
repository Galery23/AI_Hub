<template>
  <div class="about-container">
    <h1>关于</h1>
    <div class="content">
      <p>本项目是一个基于 Rust 开发的 Windows 桌面应用程序，旨在提供一个统一的大语言模型桌面应用平台。项目使用 Tauri 框架作为核心，结合 Rust 后端和 Vue.js 前端，实现了一个现代化的桌面应用程序。</p>
      <p>版本：0.1.0</p>
      <p>作者：缪秉辰 布文东</p>
    </div>
  </div>
</template>

<script>
export default {
  name: 'About'
}
</script>

<style scoped>
.about-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  background: var(--bg-color);
  color: var(--text-color);
  min-height: 100vh;
}

h1 {
  color: var(--text-color);
  text-align: center;
  margin-bottom: 30px;
  font-size: calc(var(--base-font-size) * 1.5);
}

.content {
  background: var(--bg-color);
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(128, 128, 128, 0.2);
}

p {
  margin-bottom: 15px;
  line-height: 1.6;
}
</style>
