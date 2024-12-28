<template>
  <div class="multimodal-models">
    <h2 class="text-xl font-bold mb-4">多模态模型</h2>
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
  name: 'MultimodalModels',
  data() {
    return {
      models: [
        {
          name: 'Claude 3',
          description: 'Anthropic 推出的多模态大模型，支持图像理解与分析',
          icon: '🎨',
          url: 'https://claude.ai',
          tags: ['图像理解', '视觉分析', '多语言']
        },
        {
          name: 'GPT-4V',
          description: 'OpenAI 的视觉语言模型，可以理解和分析图像内容',
          icon: '👁️',
          url: 'https://chat.openai.com',
          tags: ['视觉理解', '图像分析', '多模态对话']
        },
        {
          name: 'Gemini',
          description: 'Google 推出的多模态 AI 模型，支持文本、图像、视频等多种输入',
          icon: '🌟',
          url: 'https://gemini.google.com',
          tags: ['多模态', '视频理解', 'API集成']
        },
        {
          name: 'DALL·E 3',
          description: 'OpenAI 的图像生成模型，可以根据文本描述生成高质量图像',
          icon: '🎨',
          url: 'https://labs.openai.com',
          tags: ['图像生成', '艺术创作', '设计辅助']
        },
        {
          name: 'Midjourney',
          description: '强大的 AI 图像生成工具，擅长艺术风格和创意表现',
          icon: '🎭',
          url: 'https://www.midjourney.com',
          tags: ['图像生成', '艺术创作', '设计']
        },
        {
          name: 'Stable Diffusion',
          description: '开源的图像生成模型，支持本地部署和自定义训练',
          icon: '🖼️',
          url: 'https://stability.ai',
          tags: ['开源', '本地部署', '自定义训练']
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
.multimodal-models {
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