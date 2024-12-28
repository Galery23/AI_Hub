<template>
  <div class="domestic-models">
    <h2 class="text-xl font-bold mb-4">国内模型</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <div v-for="model in models" :key="model.name" 
           class="model-card p-4 border rounded-lg hover:shadow-md transition-shadow">
        <div class="flex justify-between items-start">
          <div>
            <h3 class="font-medium text-lg">{{ model.name }}</h3>
            <p class="text-gray-600 text-sm mt-1">{{ model.description }}</p>
          </div>
          <div class="model-icon text-2xl">{{ model.icon }}</div>
        </div>
        <div class="mt-4 flex flex-wrap gap-2">
          <span v-for="tag in model.tags" :key="tag"
                class="px-2 py-1 bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-100 rounded text-xs">
            {{ tag }}
          </span>
        </div>
        <div class="mt-4 flex justify-between items-center">
          <button @click="openUrl(model.url)"
                  class="px-4 py-2 bg-primary text-white rounded hover:bg-primary-dark transition-colors">
            访问
          </button>
          <button @click="toggleFavorite(model)"
                  class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors">
            <span class="text-xl">{{ isFavorite(model) ? '⭐' : '☆' }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'DomesticModels',
  data() {
    return {
      models: [
        {
          name: '文心一言',
          description: '百度推出的知识增强大语言模型，擅长中文创作与对话',
          icon: '🔮',
          url: 'https://yiyan.baidu.com',
          tags: ['对话', '创作', '中文优化']
        },
        {
          name: '通义千问',
          description: '阿里云推出的大语言模型，具有强大的知识理解与推理能力',
          icon: '🤖',
          url: 'https://qianwen.aliyun.com',
          tags: ['知识问答', '多轮对话', 'API支持']
        },
        {
          name: '讯飞星火',
          description: '科大讯飞推出的认知大模型，专注于自然对话与行业应用',
          icon: '✨',
          url: 'https://xinghuo.xfyun.cn',
          tags: ['语音交互', '行业定制', '中文理解']
        }
      ],
      favorites: new Set()
    }
  },
  methods: {
    openUrl(url) {
      window.open(url, '_blank')
    },
    toggleFavorite(model) {
      const key = model.name
      if (this.favorites.has(key)) {
        this.favorites.delete(key)
      } else {
        this.favorites.add(key)
      }
      this.saveFavorites()
    },
    isFavorite(model) {
      return this.favorites.has(model.name)
    },
    saveFavorites() {
      localStorage.setItem('modelFavorites', JSON.stringify(Array.from(this.favorites)))
    },
    loadFavorites() {
      const saved = localStorage.getItem('modelFavorites')
      if (saved) {
        this.favorites = new Set(JSON.parse(saved))
      }
    }
  },
  mounted() {
    this.loadFavorites()
  }
}
</script>

<style scoped>
.domestic-models {
  max-width: 1200px;
  margin: 0 auto;
}

.model-card {
  background-color: var(--bg-color);
  border-color: var(--border-color);
}

.bg-primary {
  background-color: var(--primary-color);
}

.bg-primary-dark {
  background-color: var(--secondary-color);
}
</style> 