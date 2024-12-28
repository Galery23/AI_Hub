<template>
  <div class="foreign-llm">
    <div class="header">
      <h1>国外大语言模型</h1>
      <p>探索全球领先的人工智能语言模型</p>
    </div>

    <div class="model-grid">
      <div v-for="model in models" :key="model.name" 
           class="model-card">
        <div class="card-header">
          <div class="card-title">
            <h3>{{ model.name }}</h3>
            <div class="tags">
              <span v-for="tag in model.tags" :key="tag" class="tag">{{ tag }}</span>
            </div>
          </div>
          <div class="model-icon">{{ model.icon }}</div>
        </div>
        <p class="description">{{ model.description }}</p>
        <div class="card-actions">
          <button @click="openUrl(model.url)" class="visit-btn">
            <span class="icon">🔗</span>
            访问
          </button>
          <button @click="handleFavorite(model)" 
                  class="favorite-btn"
                  :class="{ 'active': isFavorite(model) }">
            <span class="icon">{{ isFavorite(model) ? '⭐' : '☆' }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapMutations, mapGetters } from 'vuex'

export default {
  name: 'ForeignLLM',
  data() {
    return {
      models: [
        {
          name: 'ChatGPT',
          url: 'https://chat.openai.com',
          icon: '🤖',
          description: 'OpenAI旗舰模型，支持GPT超强对话和创作能力',
          tags: ['对话', '创作', 'AI']
        },
        {
          name: 'Claude',
          url: 'https://claude.ai',
          icon: '🎯',
          description: 'Anthropic开发的AI助手，支持100K超长文本处理',
          tags: ['学术', '分析', '长文']
        },
        {
          name: 'Google Gemini',
          url: 'https://gemini.google.com',
          icon: '💎',
          description: 'Google最新AI模型，支持多模态理解和生成',
          tags: ['多模', '推理', 'AI']
        },
        {
          name: 'Perplexity',
          url: 'https://www.perplexity.ai',
          icon: '🔍',
          description: 'AI智能搜索引擎，提供实时信息检索',
          tags: ['搜索', '引用', 'AI']
        },
        {
          name: 'Microsoft Copilot',
          url: 'https://copilot.microsoft.com',
          icon: '🌟',
          description: '微软AI助手，深度集成Office全家桶',
          tags: ['办公', '文档', 'AI']
        },
        {
          name: 'Mistral AI',
          url: 'https://chat.mistral.ai/chat',
          icon: '🌪️',
          description: '开源大语言模型，支持多语言对话',
          tags: ['开源', '多语', 'AI']
        },
        {
          name: 'Pi',
          url: 'https://pi.ai/talk',
          icon: '🥧',
          description: '个性化AI助手，注重情感交互',
          tags: ['情感', '对话', 'AI']
        },
        {
          name: 'Claude API',
          url: 'https://console.anthropic.com',
          icon: '🔌',
          description: 'Claude API服务，支持二次开发集成',
          tags: ['API', '开发', 'AI']
        },
        {
          name: 'Bard',
          url: 'https://bard.google.com',
          icon: '🎭',
          description: 'Google推出的AI助手，支持多语言对话',
          tags: ['对话', '创意', 'AI']
        },
        {
          name: 'Poe',
          url: 'https://poe.com',
          icon: '📝',
          description: 'Quora开发的AI平台，集成多个主流模型',
          tags: ['问答', '集成', 'AI']
        },
        {
          name: 'Anthropic API',
          url: 'https://www.anthropic.com/api',
          icon: '🔧',
          description: 'Anthropic企业级API服务',
          tags: ['API', '企业', 'AI']
        },
        {
          name: 'OpenRouter',
          url: 'https://openrouter.ai/',
          icon: '🌐',
          description: '一站式AI模型调用平台',
          tags: ['API', '集成', 'AI']
        }
      ]
    }
  },
  computed: {
    ...mapGetters(['isFavorite'])
  },
  methods: {
    ...mapMutations(['toggleFavorite']),
    openUrl(url) {
      window.open(url, '_blank')
    },
    handleFavorite(model) {
      this.toggleFavorite({
        ...model,
        category: 'foreign-llm'
      })
    }
  }
}
</script>

<style scoped>
.foreign-llm {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.header {
  text-align: center;
  margin-bottom: 3rem;
}

.header h1 {
  font-size: 2.5rem;
  font-weight: bold;
  color: var(--primary-color);
  margin-bottom: 0.5rem;
}

.header p {
  font-size: 1.1rem;
  color: var(--text-color);
  opacity: 0.8;
}

.model-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 2rem;
}

.model-card {
  background: var(--bg-color);
  border: 1px solid var(--border-color);
  border-radius: 1rem;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.model-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px var(--shadow-color);
  background: var(--theme-color-light);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.card-title h3 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-color);
  margin-bottom: 0.5rem;
}

.model-icon {
  font-size: 2rem;
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag {
  padding: 0.25rem 0.75rem;
  background: var(--primary-color);
  color: white;
  border-radius: 1rem;
  font-size: 0.75rem;
}

.description {
  color: var(--text-color);
  opacity: 0.8;
  margin-bottom: 1.5rem;
  line-height: 1.6;
}

.card-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.visit-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.visit-btn:hover {
  background: var(--secondary-color);
}

.favorite-btn {
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid var(--theme-color);
  font-size: 1.2em;
  color: var(--theme-color);
  opacity: 0.5;
  cursor: pointer;
  transition: all 0.3s ease;
  padding: 5px;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  flex-shrink: 0;
}

.favorite-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
  border-color: var(--theme-color);
  opacity: 0.8;
}

.favorite-btn.active {
  background: var(--theme-color);
  color: white;
  border-color: var(--theme-color);
  opacity: 1;
  box-shadow: 0 2px 8px var(--theme-color-light);
}

@media (max-width: 768px) {
  .model-grid {
    grid-template-columns: 1fr;
  }
  
  .header h1 {
    font-size: 2rem;
  }
}
</style> 