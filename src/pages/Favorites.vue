<template>
  <div class="favorites">
    <div class="header">
      <h1>我的收藏</h1>
      <p>管理你收藏的 AI 模型</p>
    </div>

    <div class="model-grid" v-if="favorites.length > 0">
      <div v-for="model in favorites" :key="model.name" 
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
          <button @click="removeFromFavorites(model)" 
                  class="favorite-btn">
            <span class="icon">⭐</span>
          </button>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <div class="empty-icon">⭐</div>
      <h3>暂无收藏</h3>
      <p>你可以在浏览模型时点击星标添加收藏</p>
      <router-link to="/foreign-llm" class="browse-btn">
        浏览模型
      </router-link>
    </div>
  </div>
</template>

<script>
import { mapState, mapMutations } from 'vuex'

export default {
  name: 'Favorites',
  computed: {
    ...mapState(['favorites'])
  },
  methods: {
    ...mapMutations(['toggleFavorite']),
    openUrl(url) {
      window.open(url, '_blank')
    },
    removeFromFavorites(model) {
      this.toggleFavorite(model)
    }
  }
}
</script>

<style scoped>
.favorites {
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
  padding: 0.75rem;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.25rem;
  color: var(--text-color);
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  color: var(--text-color);
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-state h3 {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
}

.empty-state p {
  color: var(--text-color);
  opacity: 0.8;
  margin-bottom: 2rem;
}

.browse-btn {
  display: inline-flex;
  align-items: center;
  padding: 0.75rem 1.5rem;
  background: var(--primary-color);
  color: white;
  text-decoration: none;
  border-radius: 0.5rem;
  transition: all 0.3s ease;
}

.browse-btn:hover {
  background: var(--secondary-color);
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