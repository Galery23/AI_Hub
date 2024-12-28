<template>
  <div class="sidebar" :class="{ 'collapsed': isCollapsed }">
    <div class="sidebar-header">
      <h2 class="logo">AI Hub</h2>
      <button class="toggle-btn" @click="toggleSidebar">
        {{ isCollapsed ? '→' : '←' }}
      </button>
    </div>

    <nav class="nav-menu">
      <div class="nav-section">
        <h3 class="section-title">AI 模型</h3>
        <router-link to="/foreign-llm" class="nav-item" active-class="active">
          <span class="icon">🌍</span>
          <span class="text">国外大语言模型</span>
        </router-link>
        <router-link to="/domestic-llm" class="nav-item" active-class="active">
          <span class="icon">🇨🇳</span>
          <span class="text">国内大语言模型</span>
        </router-link>
        <router-link to="/multimodal-llm" class="nav-item" active-class="active">
          <span class="icon">🎯</span>
          <span class="text">多功能模型</span>
        </router-link>
      </div>

      <div class="nav-section">
        <h3 class="section-title">创作工具</h3>
        <router-link to="/ai-drawing" class="nav-item" active-class="active">
          <span class="icon">🎨</span>
          <span class="text">AI 绘画</span>
        </router-link>
        <router-link to="/video-creation" class="nav-item" active-class="active">
          <span class="icon">🎬</span>
          <span class="text">视频创作</span>
        </router-link>
        <router-link to="/audio-processing" class="nav-item" active-class="active">
          <span class="icon">🎵</span>
          <span class="text">音频处理</span>
        </router-link>
        <router-link to="/design-tools" class="nav-item" active-class="active">
          <span class="icon">✨</span>
          <span class="text">设计工具</span>
        </router-link>
        <router-link to="/dev-tools" class="nav-item" active-class="active">
          <span class="icon">🛠️</span>
          <span class="text">开发工具</span>
        </router-link>
      </div>

      <div class="nav-section">
        <h3 class="section-title">工具</h3>
        <router-link to="/prompts" class="nav-item" active-class="active">
          <span class="icon">📝</span>
          <span class="text">提示词模板</span>
        </router-link>
        <router-link to="/favorites" class="nav-item" active-class="active">
          <span class="icon">⭐</span>
          <span class="text">收藏夹</span>
        </router-link>
      </div>

      <div class="nav-section">
        <h3 class="section-title">系统</h3>
        <router-link to="/preferences" class="nav-item" active-class="active">
          <span class="icon">⚙️</span>
          <span class="text">偏好设置</span>
        </router-link>
        <router-link to="/about" class="nav-item" active-class="active">
          <span class="icon">ℹ️</span>
          <span class="text">关于</span>
        </router-link>
      </div>
    </nav>
  </div>
</template>

<script>
export default {
  name: 'Sidebar',
  data() {
    return {
      isCollapsed: false
    }
  },
  methods: {
    toggleSidebar() {
      this.isCollapsed = !this.isCollapsed
    }
  }
}
</script>

<style scoped>
.sidebar {
  width: 260px;
  height: 100vh;
  background: var(--bg-color);
  border-right: 1px solid var(--border-color);
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
  position: fixed;
  left: 0;
  top: 0;
}

.sidebar.collapsed {
  width: 80px;
}

.sidebar-header {
  padding: 1.5rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--border-color);
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--primary-color);
  margin: 0;
}

.toggle-btn {
  background: none;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  color: var(--text-color);
  padding: 0.5rem;
  border-radius: 0.5rem;
  transition: all 0.3s ease;
}

.toggle-btn:hover {
  background: var(--hover-shadow);
}

.nav-menu {
  padding: 1rem;
  flex: 1;
  overflow-y: auto;
}

.nav-section {
  margin-bottom: 2rem;
}

.section-title {
  font-size: 0.875rem;
  text-transform: uppercase;
  color: var(--text-color);
  opacity: 0.6;
  margin: 0 0 0.75rem 0.5rem;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 0.75rem 1rem;
  color: var(--text-color);
  text-decoration: none;
  border-radius: 0.5rem;
  margin-bottom: 0.25rem;
  transition: all 0.3s ease;
}

.nav-item:hover {
  background: var(--hover-shadow);
}

.nav-item.active {
  background: var(--primary-color);
  color: white;
}

.icon {
  font-size: 1.25rem;
  margin-right: 1rem;
}

.collapsed .text,
.collapsed .section-title {
  display: none;
}

.collapsed .nav-item {
  justify-content: center;
  padding: 0.75rem;
}

.collapsed .icon {
  margin: 0;
}

@media (max-width: 768px) {
  .sidebar {
    width: 100%;
    height: auto;
    position: relative;
  }
  
  .sidebar.collapsed {
    width: 100%;
  }
  
  .nav-menu {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }
  
  .nav-section {
    flex: 1;
    min-width: 200px;
  }
  
  .text {
    display: inline;
  }
}
</style> 