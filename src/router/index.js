import { createRouter, createWebHistory } from 'vue-router'
import ForeignLLM from '../pages/ForeignLLM.vue'
import DomesticLLM from '../pages/DomesticLLM.vue'
import MultimodalLLM from '../pages/MultimodalLLM.vue'
import Favorites from '../pages/Favorites.vue'
import PromptPage from '../pages/PromptPage.vue'
import Preferences from '../pages/Preferences.vue'
import About from '../pages/About.vue'
import AIDrawing from '../pages/AIDrawing.vue'
import VideoCreation from '../pages/VideoCreation.vue'
import AudioProcessing from '../pages/AudioProcessing.vue'
import DesignTools from '../pages/DesignTools.vue'
import DevTools from '../pages/DevTools.vue'

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
  history: createWebHistory(),
  routes
})

export default router 