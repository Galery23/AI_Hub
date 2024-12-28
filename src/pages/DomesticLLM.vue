<template>
  <div class="domestic-llm">
    <div class="header">
      <h1>国内大语言模型</h1>
      <p>探索中国领先的人工智能语言模型</p>
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
  name: 'DomesticLLM',
  data() {
    return {
      models: [
        {
          name: '通义千问',
          url: 'https://tongyi.aliyun.com',
          icon: '💡',
          description: '阿里云大语言模型，深度理解中国文化语境，提供全面的中文AI服务和解决方案',
          tags: ['中文', '知识', 'AI']
        },
        {
          name: 'Deepseek',
          url: 'https://chat.deepseek.com',
          icon: '🔬',
          description: '专注科研和学术领域的AI助手，提供论文写作和代码生成支持，助力学术研究',
          tags: ['科研', '代码', 'AI']
        },
        {
          name: '豆包',
          url: 'https://www.doubao.com/chat',
          icon: '🌿',
          description: '字节跳动AI助手，提供多场景解决方案，支持创意写作和内容创作的智能辅助',
          tags: ['场景', '创作', 'AI']
        },
        {
          name: 'Kimi',
          url: 'https://kimi.moonshot.cn',
          icon: '🌙',
          description: '月之暗面AI，强化知识库问答能力，提供精准的专业解答和文档理解服务',
          tags: ['知识', '问答', 'AI']
        },
        {
          name: 'Coze',
          url: 'https://www.coze.cn/home',
          icon: '🎲',
          description: '火山引擎AI平台，支持机器人开发和应用构建，提供完整的API集成方案',
          tags: ['机器', '应用', 'AI']
        },
        {
          name: '文心一言',
          url: 'https://yiyan.baidu.com',
          icon: '💭',
          description: '百度AI模型，深度理解中文应用场景，提供全面的知识问答和行业解决方案',
          tags: ['中文', '知识', 'AI']
        },
        {
          name: '讯飞星火',
          url: 'https://xinghuo.xfyun.cn',
          icon: '⭐',
          description: '科大讯飞AI，具备优秀的语音交互能力和行业应用能力，支持多场景智能服务',
          tags: ['语音', '对话', 'AI']
        },
        {
          name: '智谱清言',
          url: 'https://chatglm.cn',
          icon: '📚',
          description: '注重知识准确性的学术AI助手，提供专业的学术研究支持和知识解答服务',
          tags: ['学术', '知识', 'AI']
        },
        {
          name: '360智脑',
          url: 'https://ai.360.cn',
          icon: '🧠',
          description: '360推出的AI助手，专注中文理解和商业应用，提供全面的安全解决方案和场景应用',
          tags: ['安全', '场景', 'AI']
        },
        {
          name: '天工AI',
          url: 'https://tiangong.kunlun.com',
          icon: '⚒️',
          description: '昆仑万维推出的AI助手，专注中文理解和商业应用，提供全方位智能服务',
          tags: ['商业', '中文', 'AI']
        },
        {
          name: '澜舟认知',
          url: 'https://lanzhou.zhipu.ai',
          icon: '🚤',
          description: '智谱AI推的认知大模型，专注金融和法律领域，提供专业解决方案',
          tags: ['金融', '法律', 'AI']
        },
        {
          name: '商汤日日新',
          url: 'https://chat.sensetime.com/',
          icon: '🌅',
          description: '商汤科技推出的AI助手，专注视觉理解和多模态交互，提供丰富的行业应用',
          tags: ['视觉', '多模', 'AI']
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
        category: 'domestic-llm'
      })
    }
  }
}
</script>

<style scoped>
.domestic-llm {
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