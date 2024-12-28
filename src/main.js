import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import { createStore } from 'vuex'

import App from './App.vue'
import About from './components/About.vue'
import Preferences from './components/Preferences.vue'
import PromptPage from './pages/PromptPage.vue'
import ForeignLLM from './pages/ForeignLLM.vue'
import DomesticLLM from './pages/DomesticLLM.vue'
import MultimodalLLM from './pages/MultimodalLLM.vue'
import Favorites from './pages/Favorites.vue'
import AIDrawing from './pages/AIDrawing.vue'
import VideoCreation from './pages/VideoCreation.vue'
import AudioProcessing from './pages/AudioProcessing.vue'
import DesignTools from './pages/DesignTools.vue'
import DevTools from './pages/DevTools.vue'

// 创建 Vuex store
const store = createStore({
  state() {
    return {
      favorites: JSON.parse(localStorage.getItem('favorites') || '[]')
    }
  },
  mutations: {
    toggleFavorite(state, model) {
      const index = state.favorites.findIndex(f => f.name === model.name)
      if (index === -1) {
        state.favorites.push(model)
      } else {
        state.favorites.splice(index, 1)
      }
      // 保存到本地存储
      localStorage.setItem('favorites', JSON.stringify(state.favorites))
    }
  },
  getters: {
    isFavorite: (state) => (model) => {
      return state.favorites.some(f => f.name === model.name)
    }
  }
})

const routes = [
  {
    path: '/',
    redirect: '/foreign-llm'
  },
  {
    path: '/foreign-llm',
    name: 'ForeignLLM',
    component: ForeignLLM
  },
  {
    path: '/domestic-llm',
    name: 'DomesticLLM',
    component: DomesticLLM
  },
  {
    path: '/multimodal-llm',
    name: 'MultimodalLLM',
    component: MultimodalLLM
  },
  {
    path: '/ai-drawing',
    name: 'AIDrawing',
    component: AIDrawing
  },
  {
    path: '/video-creation',
    name: 'VideoCreation',
    component: VideoCreation
  },
  {
    path: '/audio-processing',
    name: 'AudioProcessing',
    component: AudioProcessing
  },
  {
    path: '/design-tools',
    name: 'DesignTools',
    component: DesignTools
  },
  {
    path: '/dev-tools',
    name: 'DevTools',
    component: DevTools
  },
  {
    path: '/prompts',
    name: 'Prompts',
    component: PromptPage
  },
  {
    path: '/favorites',
    name: 'Favorites',
    component: Favorites
  },
  {
    path: '/preferences',
    name: 'Preferences',
    component: Preferences
  },
  {
    path: '/about',
    name: 'About',
    component: About
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

const app = createApp(App)
app.use(router)
app.use(store)
app.mount('#app')
