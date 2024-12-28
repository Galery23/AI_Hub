<template>
  <div class="app-container">
    <Sidebar @sidebar-toggle="handleSidebarToggle" />
    <div class="main-content" :class="{ 'sidebar-collapsed': isSidebarCollapsed }">
      <router-view></router-view>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import Sidebar from './components/Sidebar.vue'

export default {
  name: 'App',
  components: {
    Sidebar
  },
  setup() {
    const isSidebarCollapsed = ref(false)
    let unlistenTheme
    let unlistenFontSize

    const handleSidebarToggle = (collapsed) => {
      isSidebarCollapsed.value = collapsed
    }

    const loadPreferences = () => {
      try {
        const savedPrefs = localStorage.getItem('preferences')
        if (savedPrefs) {
          const preferences = JSON.parse(savedPrefs)
          const theme = preferences.theme
          const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
          const isDark = theme === 'dark' || (theme === 'system' && prefersDark)
          
          document.documentElement.classList.toggle('dark-theme', isDark)
          if (isDark) {
            document.documentElement.style.setProperty('--bg-color', '#1a1a1a')
            document.documentElement.style.setProperty('--text-color', '#ffffff')
            document.documentElement.style.setProperty('--bg-gradient', '#2d2d2d')
          } else {
            document.documentElement.style.setProperty('--bg-color', '#ffffff')
            document.documentElement.style.setProperty('--text-color', '#000000')
            document.documentElement.style.setProperty('--bg-gradient', '#f5f5f5')
          }
          applyFontSize(preferences.fontSize)
        }
      } catch (error) {
        console.error('加载设置失败:', error)
      }
    }

    const applyTheme = (preferences) => {
      const theme = preferences.theme
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      const isDark = theme === 'dark' || (theme === 'system' && prefersDark)
      
      document.documentElement.classList.toggle('dark-theme', isDark)
      if (isDark) {
        document.documentElement.style.setProperty('--bg-color', '#1a1a1a')
        document.documentElement.style.setProperty('--text-color', '#ffffff')
        document.documentElement.style.setProperty('--bg-gradient', '#2d2d2d')
      } else {
        document.documentElement.style.setProperty('--bg-color', '#ffffff')
        document.documentElement.style.setProperty('--text-color', '#000000')
        document.documentElement.style.setProperty('--bg-gradient', '#f5f5f5')
      }
    }

    const applyFontSize = (fontSize) => {
      const fontSizes = {
        small: '14px',
        medium: '16px',
        large: '18px'
      }
      const size = fontSizes[fontSize] || fontSizes.medium
      document.documentElement.style.setProperty('--base-font-size', size)
    }

    // 监听系统主题变化
    const watchSystemTheme = () => {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      const handleChange = () => {
        const savedPrefs = localStorage.getItem('preferences')
        if (savedPrefs) {
          const preferences = JSON.parse(savedPrefs)
          applyTheme(preferences)
        }
      }
      mediaQuery.addEventListener('change', handleChange)
      return () => mediaQuery.removeEventListener('change', handleChange)
    }

    onMounted(async () => {
      loadPreferences()
      const cleanup = watchSystemTheme()

      // 监听来自偏好设置窗口的主题变更事件
      unlistenTheme = await listen('theme-changed', event => {
        localStorage.setItem('preferences', JSON.stringify(event.payload))
        applyTheme(event.payload)
      })

      // 监听来自偏好设置窗口的字体大小变更事件
      unlistenFontSize = await listen('font-size-changed', event => {
        const savedPrefs = localStorage.getItem('preferences')
        if (savedPrefs) {
          const preferences = JSON.parse(savedPrefs)
          preferences.fontSize = event.payload
          localStorage.setItem('preferences', JSON.stringify(preferences))
        }
        applyFontSize(event.payload)
      })
    })

    onUnmounted(() => {
      unlistenTheme?.()
      unlistenFontSize?.()
    })

    return {
      isSidebarCollapsed,
      handleSidebarToggle
    }
  }
}
</script>

<style>
:root {
  --bg-color: #ffffff;
  --bg-gradient: #f5f5f5;
  --text-color: #000000;
  --base-font-size: 16px;
  --primary-color: var(--theme-color, #9B51E0);
  --secondary-color: var(--theme-color-light, #9B51E033);
  --border-color: rgba(128, 128, 128, 0.2);
  --shadow-color: rgba(0, 0, 0, 0.1);
  --hover-shadow: rgba(var(--theme-color-rgb, 155, 81, 224), 0.2);
  --sidebar-width: 260px;
  --sidebar-collapsed-width: 80px;

  /* 主题色相关 */
  --theme-color: #9B51E0;
  --theme-color-light: #9B51E033;
  --theme-color-rgb: 155, 81, 224;

  /* 按钮和交互元素 */
  --button-bg: var(--theme-color);
  --button-hover-bg: var(--theme-color-light);
  --link-color: var(--theme-color);
  --active-color: var(--theme-color);
  --focus-ring: var(--theme-color-light);
}

.dark-theme {
  --bg-color: #1a1a1a;
  --bg-gradient: #2d2d2d;
  --text-color: #ffffff;
  --border-color: rgba(255, 255, 255, 0.1);
  --shadow-color: rgba(0, 0, 0, 0.3);
  --hover-shadow: rgba(var(--theme-color-rgb), 0.4);
}

body {
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background-color: var(--bg-color);
  color: var(--text-color);
  font-size: var(--base-font-size);
  line-height: 1.6;
}

.app-container {
  min-height: 100vh;
  background-color: var(--bg-color);
  color: var(--text-color);
}

.main-content {
  margin-left: var(--sidebar-width);
  padding: 2rem;
  transition: all 0.3s ease;
  min-height: 100vh;
  background-color: var(--bg-color);
}

.main-content.sidebar-collapsed {
  margin-left: var(--sidebar-collapsed-width);
}

@media (max-width: 768px) {
  .main-content {
    margin-left: 0;
    padding: 1rem;
  }
  
  .main-content.sidebar-collapsed {
    margin-left: 0;
  }
}

* {
  box-sizing: border-box;
  transition: background-color 0.3s ease, color 0.3s ease;
}

input, button {
  font-family: inherit;
}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-color);
}

::-webkit-scrollbar-thumb {
  background: var(--primary-color);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--secondary-color);
}

/* 全局按钮样式 */
button {
  background-color: var(--button-bg);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s ease;
}

button:hover {
  background-color: var(--button-hover-bg);
  transform: translateY(-1px);
}

/* 链接样式 */
a {
  color: var(--link-color);
  text-decoration: none;
  transition: color 0.3s ease;
}

a:hover {
  opacity: 0.8;
}

/* 选中和焦点样式 */
:focus {
  outline: 2px solid var(--focus-ring);
  outline-offset: 2px;
}

::selection {
  background-color: var(--theme-color-light);
  color: var(--text-color);
}
</style>