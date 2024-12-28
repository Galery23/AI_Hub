<template>
  <div class="preferences-container">
    <h1>偏好设置</h1>
    <div class="section">
      <h2>外观</h2>
      <div class="option">
        <label for="theme">主题</label>
        <select v-model="preferences.theme" id="theme" @change="applyTheme">
          <option value="light">浅色</option>
          <option value="dark">深色</option>
          <option value="system">跟随系统</option>
        </select>
      </div>
      <div class="option">
        <label for="themeColor">主题色</label>
        <div class="theme-color-container">
          <div 
            v-for="color in presetColors" 
            :key="color.value"
            class="color-option"
            :class="{ active: preferences.themeColor === color.value }"
            :style="{ backgroundColor: color.value }"
            @click="selectThemeColor(color.value)"
          ></div>
          <input 
            type="color" 
            id="customColor"
            v-model="preferences.themeColor"
            @input="selectThemeColor($event.target.value)"
            class="custom-color-picker"
          >
        </div>
      </div>
      <div class="option">
        <label for="fontSize">字体大小</label>
        <select v-model="preferences.fontSize" id="fontSize" @change="applyFontSize">
          <option value="small">小</option>
          <option value="medium">中</option>
          <option value="large">大</option>
        </select>
      </div>
    </div>

    <div class="button-container">
      <button @click="savePreferences" class="save-button">保存设置</button>
    </div>

    <div v-if="showMessage" class="message-toast" :class="{ 'message-show': showMessage }">
      {{ message }}
    </div>
  </div>
</template>

<script>
import { ref, onMounted, onUnmounted } from "vue"
import { emit, listen } from '@tauri-apps/api/event'

export default {
  name: "Preferences",
  setup() {
    const preferences = ref({
      theme: "system",
      fontSize: "medium",
      themeColor: "#9B51E0",
    })

    const showMessage = ref(false)
    const message = ref('')

    const presetColors = [
      { name: "紫色", value: "#9B51E0" },
      { name: "蓝色", value: "#4A90E2" },
      { name: "绿色", value: "#50E3C2" },
      { name: "红色", value: "#FF6B6B" },
      { name: "橙色", value: "#F5A623" },
      { name: "粉色", value: "#FF7597" }
    ]

    let unlistenTheme
    let unlistenFontSize

    const loadPreferences = () => {
      try {
        const savedPrefs = localStorage.getItem("preferences")
        if (savedPrefs) {
          preferences.value = JSON.parse(savedPrefs)
          applyTheme()
          applyFontSize()
          applyThemeColor()
        }
      } catch (error) {
        console.error("加载设置失败:", error)
      }
    }

    const savePreferences = () => {
      // 保存到本地存储
      localStorage.setItem("preferences", JSON.stringify(preferences.value))
      // 直接显示成功消息
      showToast("保存设置成功")
    }

    const selectThemeColor = async (color) => {
      try {
        preferences.value.themeColor = color
        applyThemeColor()
        await emit('theme-color-changed', color)
        localStorage.setItem("preferences", JSON.stringify(preferences.value))
      } catch (error) {
        console.error("应用主题色失败:", error)
      }
    }

    const applyThemeColor = () => {
      const color = preferences.value.themeColor
      document.documentElement.style.setProperty('--theme-color', color)
      document.documentElement.style.setProperty('--theme-color-light', `${color}33`)
      
      const rgb = color.match(/^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i)
      if (rgb) {
        const rgbValue = `${parseInt(rgb[1], 16)}, ${parseInt(rgb[2], 16)}, ${parseInt(rgb[3], 16)}`
        document.documentElement.style.setProperty('--theme-color-rgb', rgbValue)
      }
    }

    const applyTheme = () => {
      const theme = preferences.value.theme
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches
      const isDark = theme === "dark" || (theme === "system" && prefersDark)
      
      document.documentElement.classList.toggle("dark-theme", isDark)
      if (isDark) {
        document.documentElement.style.setProperty("--bg-color", "#1a1a1a")
        document.documentElement.style.setProperty("--text-color", "#ffffff")
        document.documentElement.style.setProperty("--bg-gradient", "#2d2d2d")
      } else {
        document.documentElement.style.setProperty("--bg-color", "#ffffff")
        document.documentElement.style.setProperty("--text-color", "#000000")
        document.documentElement.style.setProperty("--bg-gradient", "#f5f5f5")
      }
      applyThemeColor()
    }

    const applyFontSize = () => {
      const fontSizes = {
        small: "14px",
        medium: "16px",
        large: "18px"
      }
      const size = fontSizes[preferences.value.fontSize] || "16px"
      document.documentElement.style.setProperty("--base-font-size", size)
    }

    const watchSystemTheme = () => {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      mediaQuery.addEventListener('change', applyTheme)
      return () => mediaQuery.removeEventListener('change', applyTheme)
    }

    const showToast = (msg, duration = 2000) => {
      message.value = msg
      showMessage.value = true
      setTimeout(() => {
        showMessage.value = false
      }, duration)
    }

    onMounted(async () => {
      loadPreferences()
      const cleanup = watchSystemTheme()
      
      unlistenTheme = await listen('theme-changed', event => {
        preferences.value = event.payload
        applyTheme()
      })

      unlistenFontSize = await listen('font-size-changed', event => {
        preferences.value.fontSize = event.payload
        applyFontSize()
      })

      onUnmounted(() => {
        cleanup()
        unlistenTheme?.()
        unlistenFontSize?.()
      })
    })

    return {
      preferences,
      presetColors,
      savePreferences,
      applyTheme,
      applyFontSize,
      selectThemeColor,
      applyThemeColor,
      showMessage,
      message
    }
  }
}
</script>

<style>
:root {
  --bg-color: #ffffff;
  --text-color: #000000;
  --base-font-size: 16px;
  --theme-color: #9B51E0;
  --theme-color-light: #9B51E033;
}

.dark-theme {
  --bg-color: #1a1a1a;
  --text-color: #ffffff;
}
</style>

<style scoped>
.preferences-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  background: var(--bg-color);
  color: var(--text-color);
  min-height: 100vh;
  font-size: var(--base-font-size);
}

h1 {
  color: var(--text-color);
  text-align: center;
  margin-bottom: 30px;
  font-size: calc(var(--base-font-size) * 1.5);
}

.section {
  background: var(--bg-color);
  padding: 20px;
  border-radius: 10px;
  margin-bottom: 20px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(128, 128, 128, 0.2);
}

h2 {
  color: var(--text-color);
  margin-top: 0;
  font-size: calc(var(--base-font-size) * 1.2);
}

.option {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
  color: var(--text-color);
}

select, input[type="text"] {
  width: 100%;
  padding: 8px;
  border: 1px solid rgba(128, 128, 128, 0.2);
  border-radius: 4px;
  font-size: var(--base-font-size);
  background: var(--bg-color);
  color: var(--text-color);
}

.button-container {
  text-align: center;
  margin-top: 30px;
}

.save-button {
  background: var(--theme-color);
  color: white;
  border: none;
  padding: 12px 40px;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  font-size: calc(var(--base-font-size) * 1.1);
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.save-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  opacity: 0.9;
}

.save-button:active {
  transform: translateY(0);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  opacity: 1;
}

@media (max-width: 600px) {
  .preferences-container {
    padding: 10px;
  }
  
  h1 {
    font-size: calc(var(--base-font-size) * 1.2);
  }
  
  .section {
    padding: 15px;
  }
}

.theme-color-container {
  display: flex;
  gap: 10px;
  align-items: center;
  flex-wrap: wrap;
  margin-top: 5px;
}

.color-option {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.3s ease;
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.active {
  border-color: var(--theme-color);
  box-shadow: 0 0 0 2px var(--bg-color), 0 0 0 4px var(--theme-color);
}

.custom-color-picker {
  width: 30px;
  height: 30px;
  padding: 0;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  background: none;
  transition: all 0.3s ease;
}

.custom-color-picker:hover {
  transform: scale(1.1);
}

.custom-color-picker::-webkit-color-swatch {
  border: 2px solid transparent;
  border-radius: 50%;
  padding: 0;
}

.custom-color-picker::-webkit-color-swatch-wrapper {
  border: none;
  border-radius: 50%;
  padding: 0;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  margin: 0;
}

select {
  width: 100%;
  padding: 8px;
  border: 1px solid rgba(128, 128, 128, 0.2);
  border-radius: var(--input-radius, 4px);
  font-size: var(--base-font-size);
  background: var(--bg-color);
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.3s ease;
}

select:hover {
  border-color: var(--border-hover-color);
}

.option {
  margin-bottom: 20px;
}

.section {
  margin-bottom: 30px;
  background: var(--card-bg);
  padding: 25px;
  border-radius: var(--card-radius, 10px);
  box-shadow: var(--card-shadow);
  border: 1px solid var(--card-border);
}

.section:hover {
  box-shadow: var(--card-hover-shadow);
}

.section h2 {
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 2px solid var(--theme-color-light);
  color: var(--text-color);
}

.message-toast {
  position: fixed;
  bottom: 30px;
  left: 50%;
  transform: translateX(-50%) translateY(100px);
  background: #4CAF50;
  color: white;
  padding: 12px 24px;
  border-radius: 6px;
  font-size: var(--base-font-size);
  opacity: 0;
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 1000;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  display: flex;
  align-items: center;
  gap: 8px;
}

.message-toast::before {
  content: "✓";
  font-weight: bold;
}

.message-toast.message-show {
  transform: translateX(-50%) translateY(0);
  opacity: 1;
}

.message-toast.message-hide {
  opacity: 0;
  transform: translateX(-50%) translateY(20px);
}
</style>
