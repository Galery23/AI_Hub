<template>
  <div class="video-creation">
    <div class="header">
      <h1>视频创作</h1>
      <p>探索 AI 驱动的视频制作工具</p>
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
  name: 'VideoCreation',
  data() {
    return {
      models: [
        {
          name: 'OpenAI Sora',
          url: 'https://openai.com/sora',
          icon: '🎬',
          description: 'OpenAI视频生成模型，支持高质量场景合成，能够创建逼真的视频内容和特效',
          tags: ['视频', '场景', 'AI']
        },
        {
          name: 'Runway',
          url: 'https://runway.ml',
          icon: '🎥',
          description: 'AI视频创作平台，支持专业视频编辑和特效制作，提供丰富的创意工具和模板',
          tags: ['视频', '编辑', 'AI']
        },
        {
          name: 'HeyGen',
          url: 'https://www.heygen.com',
          icon: '🎪',
          description: 'AI数字人平台，支持多语言视频制作和教育培训内容生成，提供专业的虚拟人解决方案',
          tags: ['数人', '视频', 'AI']
        },
        {
          name: 'Synthesia',
          url: 'https://www.synthesia.io',
          icon: '📹',
          description: '专业AI视频制作平台，面向企业教育领域，提供定制化的数字人视频解决方案',
          tags: ['数人', '视频', 'AI']
        },
        {
          name: 'Descript',
          url: 'https://www.descript.com',
          icon: '🎙️',
          description: 'AI内容编辑平台，支持视频音频处理，提供专业的多媒体编辑工具和自动化功能',
          tags: ['编辑', '视频', 'AI']
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
        category: 'video-creation'
      })
    }
  }
}
</script>

<style scoped>
.video-creation {
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