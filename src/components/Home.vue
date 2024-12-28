<template>
  <div class="home">
    <div class="dots-background"></div>
    <div class="header">
      <h1>navi</h1>
      <p>探索AI世界的桌面应用</p>
    </div>

    <div class="toolbar">
      <div class="search-box">
        <input 
          type="text" 
          v-model="searchQuery" 
          placeholder="搜索平台..." 
          @input="filterPlatforms"
        >
        <span class="search-icon">🔍</span>
      </div>
      
      <div class="filter-box">
        <select v-model="selectedCategory" @change="filterPlatforms">
          <option value="all">全部分类</option>
          <optgroup label="主要分类">
            <option value="foreignLLMs">国外模型</option>
            <option value="domesticLLMs">国内模型</option>
            <option value="multiModalModels">多功能模型</option>
          </optgroup>
          <optgroup label="功能分类">
            <option value="aiArt">AI绘画</option>
            <option value="videoCreation">视频创作</option>
            <option value="audioProcessing">音频处理</option>
            <option value="designTools">设计工具</option>
            <option value="devTools">开发工具</option>
          </optgroup>
        </select>
      </div>

      <div class="favorite-toggle">
        <button 
          :class="['toggle-btn', { active: showFavoritesOnly }]"
          @click="toggleFavoritesOnly"
        >
          <span class="star-icon">⭐</span>
          {{ showFavoritesOnly ? '收藏夹' : '全部' }}
        </button>
      </div>
    </div>

    <div class="sections">
      <div v-for="section in filteredSections" :key="section.title" class="section">
        <div class="section-header" :data-icon="section.icon">
          <h2>{{ section.title }}</h2>
          <p>{{ section.description }}</p>
        </div>
        <div class="card-container">
          <div 
            v-for="platform in filterPlatformsBySection(section.category)" 
            :key="platform.name"
            class="card"
            @click="openUrl(platform.url)"
          >
            <div class="card-content">
              <div class="card-icon">{{ platform.icon }}</div>
              <h3>{{ platform.name }}</h3>
              <p>{{ platform.description }}</p>
              <div class="card-tags">
                <span 
                  v-for="tag in platform.tags" 
                  :key="tag" 
                  class="tag"
                >
                  {{ tag }}
                </span>
              </div>
            </div>
            <button 
              class="favorite-btn" 
              @click.stop="toggleFavorite(platform)"
              :class="{ active: isFavorite(platform.name) }"
            >
              <span class="star-icon">⭐</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { listen } from '@tauri-apps/api/event'

export default {
  name: 'Home',
  setup() {
    const searchQuery = ref('')
    const selectedCategory = ref('all')
    const showFavoritesOnly = ref(false)
    const favorites = ref(new Set())
    const selectedTags = ref([])

    let unlistenTheme
    let unlistenFontSize
    let unlistenThemeColor

    // 加载主题设置
    const loadThemePreferences = () => {
      try {
        const savedPrefs = localStorage.getItem('preferences')
        if (savedPrefs) {
          const preferences = JSON.parse(savedPrefs)
          applyTheme(preferences)
          applyThemeColor(preferences.themeColor)
        }
      } catch (error) {
        console.error('加载主题设置失败:', error)
      }
    }

    // 应用主题色
    const applyThemeColor = (color) => {
      document.documentElement.style.setProperty('--theme-color', color)
      document.documentElement.style.setProperty('--theme-color-light', `${color}33`)
      
      // 更新相关的UI元素颜色
      document.documentElement.style.setProperty('--tag-bg', `${color}1A`)
      document.documentElement.style.setProperty('--tag-text', color)
      document.documentElement.style.setProperty('--toolbar-border', `${color}33`)
    }

    // 应用主题
    const applyTheme = (preferences) => {
      const theme = preferences.theme
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      const isDark = theme === 'dark' || (theme === 'system' && prefersDark)
      
      if (isDark) {
        // 卡片相关
        document.documentElement.style.setProperty('--card-bg', 'rgba(40, 40, 40, 0.7)')
        document.documentElement.style.setProperty('--card-text', '#ffffff')
        document.documentElement.style.setProperty('--description-text', '#cccccc')
        document.documentElement.style.setProperty('--tag-bg', 'rgba(155, 81, 224, 0.2)')
        document.documentElement.style.setProperty('--tag-text', '#ffffff')
        
        // 背景相关
        document.documentElement.style.setProperty('--bg-gradient-start', '#1a1a1a')
        document.documentElement.style.setProperty('--bg-gradient-mid', '#2d2d2d')
        document.documentElement.style.setProperty('--bg-gradient-end', '#333333')
        
        // 头部和工具栏
        document.documentElement.style.setProperty('--header-text', '#ffffff')
        document.documentElement.style.setProperty('--header-description', '#cccccc')
        document.documentElement.style.setProperty('--toolbar-bg', 'rgba(40, 40, 40, 0.7)')
        document.documentElement.style.setProperty('--toolbar-text', '#ffffff')
        document.documentElement.style.setProperty('--toolbar-border', 'rgba(155, 81, 224, 0.3)')
        
        // 搜索框和选择框
        document.documentElement.style.setProperty('--input-bg', 'rgba(40, 40, 40, 0.9)')
        document.documentElement.style.setProperty('--input-text', '#ffffff')
        document.documentElement.style.setProperty('--input-placeholder', '#999999')
        
        // 分类标题
        document.documentElement.style.setProperty('--section-bg', 'rgba(40, 40, 40, 0.7)')
        document.documentElement.style.setProperty('--section-text', '#ffffff')
        document.documentElement.style.setProperty('--section-description', '#cccccc')
      } else {
        // 卡片相关
        document.documentElement.style.setProperty('--card-bg', 'rgba(255, 255, 255, 0.7)')
        document.documentElement.style.setProperty('--card-text', '#2c3e50')
        document.documentElement.style.setProperty('--description-text', '#666666')
        document.documentElement.style.setProperty('--tag-bg', 'rgba(155, 81, 224, 0.1)')
        document.documentElement.style.setProperty('--tag-text', '#9B51E0')
        
        // 背景相关
        document.documentElement.style.setProperty('--bg-gradient-start', '#f6f8ff')
        document.documentElement.style.setProperty('--bg-gradient-mid', '#f0f4ff')
        document.documentElement.style.setProperty('--bg-gradient-end', '#e8f0ff')
        
        // 头部和工具栏
        document.documentElement.style.setProperty('--header-text', '#2c3e50')
        document.documentElement.style.setProperty('--header-description', '#666666')
        document.documentElement.style.setProperty('--toolbar-bg', 'rgba(255, 255, 255, 0.7)')
        document.documentElement.style.setProperty('--toolbar-text', '#2c3e50')
        document.documentElement.style.setProperty('--toolbar-border', 'rgba(155, 81, 224, 0.2)')
        
        // 搜索框和选择框
        document.documentElement.style.setProperty('--input-bg', 'rgba(255, 255, 255, 0.9)')
        document.documentElement.style.setProperty('--input-text', '#333333')
        document.documentElement.style.setProperty('--input-placeholder', '#999999')
        
        // 分类标题
        document.documentElement.style.setProperty('--section-bg', 'rgba(255, 255, 255, 0.7)')
        document.documentElement.style.setProperty('--section-text', '#2c3e50')
        document.documentElement.style.setProperty('--section-description', '#666666')
      }
    }

    onMounted(async () => {
      // 原有的 onMounted 代码
      const savedFavorites = localStorage.getItem('favorites')
      if (savedFavorites) {
        favorites.value = new Set(JSON.parse(savedFavorites))
      }

      // 添加主题相关的初始化
      loadThemePreferences()

      // 监听主题变更事件
      unlistenTheme = await listen('theme-changed', event => {
        applyTheme(event.payload)
      })

      // 监听字体大小变更事件
      unlistenFontSize = await listen('font-size-changed', event => {
        document.documentElement.style.setProperty('--base-font-size', event.payload)
      })

      // 监听主题色变更事件
      unlistenThemeColor = await listen('theme-color-changed', event => {
        applyThemeColor(event.payload)
      })

      // 监听布局设置变更事件
      const unlistenLayout = await listen('layout-changed', event => {
        const prefs = event.payload
        // 应用字体
        const fontFamilies = {
          system: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif",
          pingfang: "'PingFang SC', 'Microsoft YaHei', sans-serif",
          microsoft: "'Microsoft YaHei', 'PingFang SC', sans-serif",
          sourcehan: "'Source Han Sans CN', 'PingFang SC', 'Microsoft YaHei', sans-serif"
        }
        document.documentElement.style.setProperty('--font-family', fontFamilies[prefs.fontFamily])
        
        // 应用布局
        document.documentElement.setAttribute('data-layout', prefs.cardLayout)
        
        // 应用圆角
        const radiusValues = {
          small: {
            card: '10px',
            button: '5px',
            input: '5px'
          },
          medium: {
            card: '20px',
            button: '10px',
            input: '8px'
          },
          large: {
            card: '30px',
            button: '15px',
            input: '12px'
          }
        }
        const radiusVals = radiusValues[prefs.borderRadius]
        document.documentElement.style.setProperty('--card-radius', radiusVals.card)
        document.documentElement.style.setProperty('--button-radius', radiusVals.button)
        document.documentElement.style.setProperty('--input-radius', radiusVals.input)
        
        // 应用透明度
        const transparencyValues = {
          low: {
            card: '0.9',
            toolbar: '0.95'
          },
          medium: {
            card: '0.7',
            toolbar: '0.8'
          },
          high: {
            card: '0.5',
            toolbar: '0.6'
          }
        }
        const transparencyVals = transparencyValues[prefs.transparency]
        document.documentElement.style.setProperty('--card-opacity', transparencyVals.card)
        document.documentElement.style.setProperty('--toolbar-opacity', transparencyVals.toolbar)
        
        // 应用动画设置
        document.documentElement.setAttribute('data-animations', prefs.enableAnimations.toString())
      })

      // 监听系统主题变化
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      const handleChange = () => {
        loadThemePreferences()
      }
      mediaQuery.addEventListener('change', handleChange)

      onUnmounted(() => {
        mediaQuery.removeEventListener('change', handleChange)
        unlistenTheme?.()
        unlistenFontSize?.()
        unlistenThemeColor?.()
        unlistenLayout?.()
      })
    })

    // 监听收藏变化并保存到本地存储
    watch(favorites.value, () => {
      localStorage.setItem('favorites', JSON.stringify(Array.from(favorites.value)))
    })
    
    const sections = [
      {
        title: '国外大语言模型',
        category: 'foreignLLMs',
        description: '国际领先的AI语言模型平台',
        icon: '🌍'
      },
      {
        title: '国内大语言模型',
        category: 'domesticLLMs',
        description: '本土化优势的AI模型服务',
        icon: '🇨🇳'
      },
      {
        title: '多功能模型',
        category: 'multiModalModels',
        description: '集成多种AI能力的综合平台',
        icon: '🎯'
      }
    ]
    
    const platforms = [
      // 国外语言模型
      {
        name: 'ChatGPT',
        url: 'https://chat.openai.com',
        icon: '🤖',
        description: 'OpenAI旗舰模型，支持GPT-4超强对话和创作能力，可进行多轮对话和编程',
        category: 'foreignLLMs',
        tags: ['对话', '创作', 'AI']
      },
      {
        name: 'Claude',
        url: 'https://claude.ai',
        icon: '🎯',
        description: 'Anthropic开发的AI助手，支持100K超长文本处理，擅长学术分析和数据处理',
        category: 'foreignLLMs',
        tags: ['学术', '分析', '长文']
      },
      {
        name: 'Google Gemini',
        url: 'https://gemini.google.com',
        icon: '💎',
        description: 'Google最新AI模型，支持多模态理解和生成，具有强大的推理和实时信息处理能力',
        category: 'foreignLLMs',
        tags: ['多模', '推理', 'AI']
      },
      {
        name: 'Perplexity',
        url: 'https://www.perplexity.ai',
        icon: '🔍',
        description: 'AI智能搜索引擎，提供实时信息检索和事实核查，支持学术文献引用和深度分析',
        category: 'foreignLLMs',
        tags: ['搜索', '引用', 'AI']
      },
      {
        name: 'Microsoft Copilot',
        url: 'https://copilot.microsoft.com',
        icon: '🌟',
        description: '微软AI助手，深度集成Office全家桶，提供智能办公和数据处理的全方位支持',
        category: 'foreignLLMs',
        tags: ['办公', '文档', 'AI']
      },
      {
        name: 'Mistral AI',
        url: 'https://chat.mistral.ai/chat',
        icon: '🌪️',
        description: '开源大语言模型，支持多语言对话和API集成，性能优异且易于部署和定制',
        category: 'foreignLLMs',
        tags: ['开源', '多语', 'AI']
      },
      {
        name: 'Pi',
        url: 'https://pi.ai/talk',
        icon: '🥧',
        description: '个性化AI助手，注重情感交互和心理支持，提供温暖贴心的对话体验和建议',
        category: 'foreignLLMs',
        tags: ['情感', '对话', 'AI']
      },
      {
        name: 'Claude API',
        url: 'https://console.anthropic.com',
        icon: '🔌',
        description: 'Claude API服务，支持二次开发集成和系统定制，提供企业级API解决方案',
        category: 'foreignLLMs',
        tags: ['API', '开发', 'AI']
      },
      {
        name: 'Bard',
        url: 'https://bard.google.com',
        icon: '🎭',
        description: 'Google推出的AI助手，支持多语言对话和创意写作，具有强大的知识储备',
        category: 'foreignLLMs',
        tags: ['对话', '创意', 'AI']
      },
      {
        name: 'Poe',
        url: 'https://poe.com',
        icon: '📝',
        description: 'Quora开发的AI平台，集成多个���流模型，支持自定义机器人和知识问答',
        category: 'foreignLLMs',
        tags: ['问答', '集成', 'AI']
      },
      {
        name: 'Anthropic API',
        url: 'https://www.anthropic.com/api',
        icon: '🔧',
        description: 'Anthropic企业级API服务，提供Claude系列模型的深度定制和系统集成方案',
        category: 'foreignLLMs',
        tags: ['API', '企业', 'AI']
      },
      {
        name: 'OpenRouter',
        url: 'https://openrouter.ai/',
        icon: '🌐',
        description: '一站式AI模型调用平台，集成多家顶级模型，提供统一的API接口和计费服务',
        category: 'foreignLLMs',
        tags: ['API', '集成', 'AI']
      },

      // 国内大语言模型
      {
        name: '通义千问',
        url: 'https://tongyi.aliyun.com',
        icon: '💡',
        description: '阿里云大语言模型，深度理解中国文化语境，提供全面的中文AI服务和解决方案',
        category: 'domesticLLMs',
        tags: ['中文', '知识', 'AI']
      },
      {
        name: 'Deepseek',
        url: 'https://chat.deepseek.com',
        icon: '🔬',
        description: '专注科研和学术领域的AI助手，提供论文写作和代码生成支持，助力学术研究',
        category: 'domesticLLMs',
        tags: ['科研', '代码', 'AI']
      },
      {
        name: '豆包',
        url: 'https://www.doubao.com/chat',
        icon: '🌿',
        description: '字节跳动AI助手，提供多场景解决方案，支持创意写作和内容创作的智能辅助',
        category: 'domesticLLMs',
        tags: ['场景', '创作', 'AI']
      },
      {
        name: 'Kimi',
        url: 'https://kimi.moonshot.cn',
        icon: '🌙',
        description: '月之暗面AI，强化知识库问答能力，提供精准的专业解答和文档理解服务',
        category: 'domesticLLMs',
        tags: ['知识', '问答', 'AI']
      },
      {
        name: 'Coze',
        url: 'https://www.coze.cn/home',
        icon: '🎲',
        description: '火山引擎AI平台，支持机器人开发和应用构建，提供完整的API集成方案',
        category: 'domesticLLMs',
        tags: ['机器', '应用', 'AI']
      },
      {
        name: '文心一言',
        url: 'https://yiyan.baidu.com',
        icon: '💭',
        description: '百度AI模型，深度理解中文应用场景，提供全面的知识问答和行业解决方案',
        category: 'domesticLLMs',
        tags: ['中文', '知识', 'AI']
      },
      {
        name: '讯飞星火',
        url: 'https://xinghuo.xfyun.cn',
        icon: '⭐',
        description: '科大讯飞AI，具备优秀的语音交互能力和行业应用能力，支持多场景智能服务',
        category: 'domesticLLMs',
        tags: ['语音', '对话', 'AI']
      },
      {
        name: '智谱清言',
        url: 'https://chatglm.cn',
        icon: '📚',
        description: '注重知识准确性的学术AI助手，提供专业的学术研究支持和知识解答服务',
        category: 'domesticLLMs',
        tags: ['学术', '知识', 'AI']
      },
      {
        name: '360智脑',
        url: 'https://ai.360.cn',
        icon: '🧠',
        description: '360推出的AI助手，专注中文理解和商业应用，提供全面的安全解决方案和场景应用',
        category: 'domesticLLMs',
        tags: ['安全', '场景', 'AI']
      },
      {
        name: '天工AI',
        url: 'https://tiangong.kunlun.com',
        icon: '⚒️',
        description: '昆仑万维推出的AI助手，专注中文理解和商业应用，提供全方位智能服务',
        category: 'domesticLLMs',
        tags: ['商业', '中文', 'AI']
      },
      {
        name: '澜舟认知',
        url: 'https://lanzhou.zhipu.ai',
        icon: '🚤',
        description: '智谱AI推的认知大模型，专注金融和法律领域，提供专业解决方案',
        category: 'domesticLLMs',
        tags: ['金融', '法律', 'AI']
      },
      {
        name: '商汤日日新',
        url: 'https://chat.sensetime.com/',
        icon: '🌅',
        description: '商汤科技推出的AI助手，专注视觉理解和多模态交互，提供丰富的行业应用',
        category: 'domesticLLMs',
        tags: ['视觉', '多模', 'AI']
      },

      // 多功能模型
      {
        name: 'Midjourney',
        url: 'https://www.midjourney.com',
        icon: '🎨',
        description: '顶级AI绘画工具，通过文本生成高质量艺术作品，支持多种风格和商业创作需求',
        category: 'multiModalModels',
        tags: ['绘画', '艺术', 'AI']
      },
      {
        name: 'Stable Diffusion',
        url: 'https://stablediffusionweb.com',
        icon: '🖼️',
        description: '开源AI绘画平台，支持多种图像生成和编辑功能，提供丰富的模型和社区资源',
        category: 'multiModalModels',
        tags: ['绘画', '开源', 'AI']
      },
      {
        name: 'Leonardo.ai',
        url: 'https://leonardo.ai',
        icon: '🎭',
        description: '专业AI图像生成平台，针对游戏和设计场景优化，提供高质量的素材制作服务',
        category: 'multiModalModels',
        tags: ['绘画', '游戏', 'AI']
      },
      {
        name: 'Adobe Firefly',
        url: 'https://firefly.adobe.com',
        icon: '🔥',
        description: 'Adobe AI创意套件，支持专业图像创作和编辑，提供完整的设计工作流程解决方案',
        category: 'multiModalModels',
        tags: ['创意', '设计', 'AI']
      },
      {
        name: 'OpenAI Sora',
        url: 'https://openai.com/sora',
        icon: '🎬',
        description: 'OpenAI视频生成模型，支持高质量场景合成，能够创建逼真的视频内容和特效',
        category: 'multiModalModels',
        tags: ['视频', '场景', 'AI']
      },
      {
        name: 'Runway',
        url: 'https://runway.ml',
        icon: '🎥',
        description: 'AI视频创作平台，支持专业视频编辑和特效制作，提供丰富的创意工具和模板',
        category: 'multiModalModels',
        tags: ['视频', '编辑', 'AI']
      },
      {
        name: 'HeyGen',
        url: 'https://www.heygen.com',
        icon: '🎪',
        description: 'AI数字人平台，支持多语言视频制作和教育培训内容生成，提供专业的虚拟人解决方案',
        category: 'multiModalModels',
        tags: ['数人', '视频', 'AI']
      },
      {
        name: 'Synthesia',
        url: 'https://www.synthesia.io',
        icon: '📹',
        description: '专业AI视频制作平台，面向企业教育领域，提供定制化的数字人视频解决方案',
        category: 'multiModalModels',
        tags: ['数人', '视频', 'AI']
      },
      {
        name: 'Mubert',
        url: 'https://mubert.com',
        icon: '🎵',
        description: 'AI音乐生成平台，创作无版权背景音乐，支持多种风格和场景的音乐制作需求',
        category: 'multiModalModels',
        tags: ['音乐', '配乐', 'AI']
      },
      {
        name: 'Soundraw',
        url: 'https://soundraw.io',
        icon: '🎹',
        description: 'AI音乐创作平台，自动生成配乐和音效，提供专业的音频处理和情感音乐制作',
        category: 'multiModalModels',
        tags: ['音乐', '配乐', 'AI']
      },
      {
        name: 'ElevenLabs',
        url: 'https://elevenlabs.io',
        icon: '🗣️',
        description: 'AI语音克隆和合成平台，支持多语言配音和情感语音合成，提供专业的语音服务',
        category: 'multiModalModels',
        tags: ['语音', '配音', 'AI']
      },
      {
        name: 'Canva',
        url: 'https://www.canva.com',
        icon: '✨',
        description: 'AI设计平台，支持多种创意内容制作，提供丰富的模板和智能设计工具，适合各类创作需求',
        category: 'multiModalModels',
        tags: ['设计', '创意', 'AI']
      },
      {
        name: 'Descript',
        url: 'https://www.descript.com',
        icon: '🎙️',
        description: 'AI内容编辑平台，支持视频音频处理，提供专业的多媒体编辑工具和自动化功能',
        category: 'multiModalModels',
        tags: ['编辑', '视频', 'AI']
      },
      {
        name: 'Hugging Face',
        url: 'https://huggingface.co/spaces',
        icon: '🤗',
        description: 'AI模型展示和体验平台，提供开源工具和技术支持，全球AI模型和应用示例',
        category: 'multiModalModels',
        tags: ['开源', '模型', 'AI']
      },
      {
        name: 'Civitai',
        url: 'https://civitai.com',
        icon: '🎯',
        description: 'AI模型分享社区，提供海量Stable Diffusion模型和提示词，支持在线图像生成',
        category: 'multiModalModels',
        tags: ['社区', '模型', 'AI']
      },
      {
        name: 'Scenario',
        url: 'https://scenario.com',
        icon: '🎮',
        description: 'AI游戏资产生成平台，专注于游戏素材和场景制作，支持多种风格和类型',
        category: 'multiModalModels',
        tags: ['游戏', '素材', 'AI']
      },
      {
        title: '设计工具',
        category: 'designTools',
        description: 'AI辅助设计和创意工具',
        icon: '✨'
      }
    ]

    const filteredSections = computed(() => {
      if (showFavoritesOnly.value) {
        return [{
          title: '收藏夹',
          category: 'favorites',
          description: '我的收藏平台',
          icon: '⭐'
        }]
      }
      
      // 根据选择的分类返回对应的 section
      switch (selectedCategory.value) {
        case 'all':
          return sections
        case 'foreignLLMs':
          return [{
            title: '国外大语言模型',
            category: 'foreignLLMs',
            description: '国际领先的AI语言模型平台',
            icon: '🌍'
          }]
        case 'domesticLLMs':
          return [{
            title: '国内大语言模型',
            category: 'domesticLLMs',
            description: '本土化优势的AI语言服务',
            icon: '🇨🇳'
          }]
        case 'multiModalModels':
          return [{
            title: '多功能模型',
            category: 'multiModalModels',
            description: '集成多种AI能力的综合平台',
            icon: '🎯'
          }]
        case 'aiArt':
          return [{
            title: 'AI绘画',
            category: 'aiArt',
            description: 'AI驱动的像生成和编辑工具',
            icon: '🎨'
          }]
        case 'videoCreation':
          return [{
            title: '视频创作',
            category: 'videoCreation',
            description: 'AI视频生成和编辑平台',
            icon: '🎬'
          }]
        case 'audioProcessing':
          return [{
            title: '音频处理',
            category: 'audioProcessing',
            description: 'AI音频生成和处理工具',
            icon: '🎵'
          }]
        case 'designTools':
          return [{
            title: '设计工具',
            category: 'designTools',
            description: 'AI辅助设计和创意工具',
            icon: '✨'
          }]
        case 'devTools':
          return [{
            title: '开发工具',
            category: 'devTools',
            description: 'AI开发和部署平台',
            icon: '⚡'
          }]
        default:
          return sections
      }
    })

    const filterPlatformsBySection = (category) => {
      let filteredPlatforms = platforms
      
      // 如果是收藏夹视图，只显示收藏的平台
      if (category === 'favorites') {
        filteredPlatforms = platforms.filter(platform => favorites.value.has(platform.name))
      } else if (selectedCategory.value !== 'all') {
        // 如果选择了特定分类
        switch (selectedCategory.value) {
          case 'aiArt':
            filteredPlatforms = platforms.filter(platform => 
              platform.tags.some(tag => tag.includes('图像生成') || tag.includes('绘画'))
            )
            break
          case 'videoCreation':
            filteredPlatforms = platforms.filter(platform => 
              platform.tags.some(tag => tag.includes('视频'))
            )
            break
          case 'audioProcessing':
            filteredPlatforms = platforms.filter(platform => 
              platform.tags.some(tag => tag.includes('音频') || tag.includes('乐') || tag.includes('语音'))
            )
            break
          case 'designTools':
            filteredPlatforms = platforms.filter(platform => 
              platform.tags.some(tag => tag.includes('设计') || tag.includes('创意'))
            )
            break
          case 'devTools':
            filteredPlatforms = platforms.filter(platform => 
              platform.tags.some(tag => tag.includes('API') || tag.includes('开发') || tag.includes('部署'))
            )
            break
          default:
            filteredPlatforms = platforms.filter(platform => platform.category === selectedCategory.value)
        }

      }

      // 应用搜索过
      if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase()
        filteredPlatforms = filteredPlatforms.filter(platform => 
          platform.name.toLowerCase().includes(query) ||
          platform.description.toLowerCase().includes(query) ||
          platform.tags.some(tag => tag.toLowerCase().includes(query))
        )
      }

      // 如果不是收藏夹视图，且是主分类视图，根据分类过滤
      if (category !== 'favorites' && ['foreignLLMs', 'domesticLLMs', 'multiModalModels'].includes(category)) {
        filteredPlatforms = filteredPlatforms.filter(platform => platform.category === category)
      }

      return filteredPlatforms
    }

    const filterPlatforms = () => {
      // 触发重新渲染
      searchQuery.value = searchQuery.value
    }

    const openUrl = (url) => {
      window.open(url, '_blank')
    }

    const toggleFavorite = (platform) => {
      if (favorites.value.has(platform.name)) {
        favorites.value.delete(platform.name)
      } else {
        favorites.value.add(platform.name)
      }
    }

    const isFavorite = (platformName) => {
      return favorites.value.has(platformName)
    }

    const toggleFavoritesOnly = () => {
      showFavoritesOnly.value = !showFavoritesOnly.value
    }

    const availableTags = computed(() => {
      const tags = new Set()
      platforms.forEach(platform => {
        platform.tags.forEach(tag => tags.add(tag))
      })
      return Array.from(tags).sort()
    })

    const toggleTag = (tag) => {
      if (selectedTags.value.includes(tag)) {
        selectedTags.value = selectedTags.value.filter(t => t !== tag)
      } else {
        selectedTags.value.push(tag)
      }
    }

    return {
      searchQuery,
      selectedCategory,
      showFavoritesOnly,
      selectedTags,
      availableTags,
      sections: sections,
      filteredSections,
      platforms,
      filterPlatformsBySection,
      filterPlatforms,
      openUrl,
      toggleFavorite,
      isFavorite,
      toggleFavoritesOnly,
      toggleTag
    }
  }
}
</script>

<style scoped>
:root {
  --card-bg: rgba(255, 255, 255, 0.7);
  --card-text: #2c3e50;
  --description-text: #666666;
  --tag-bg: rgba(155, 81, 224, 0.1);
  --tag-text: #9B51E0;
  --bg-gradient-start: #f6f8ff;
  --bg-gradient-mid: #f0f4ff;
  --bg-gradient-end: #e8f0ff;
  --header-text: #2c3e50;
  --header-description: #666666;
  --toolbar-bg: rgba(255, 255, 255, 0.7);
  --toolbar-text: #2c3e50;
  --toolbar-border: rgba(155, 81, 224, 0.2);
  --input-bg: rgba(255, 255, 255, 0.9);
  --input-text: #333333;
  --input-placeholder: #999999;
  --section-bg: rgba(255, 255, 255, 0.7);
  --section-text: #2c3e50;
  --section-description: #666666;
}

.home {
  padding: 20px;
  min-height: 100vh;
  background: linear-gradient(135deg, 
    var(--bg-gradient-start) 0%,
    var(--bg-gradient-mid) 50%,
    var(--bg-gradient-end) 100%
  );
  position: relative;
  overflow: hidden;
}

.dots-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 0% 0%, rgba(155, 81, 224, 0.1) 0%, transparent 50%),
    radial-gradient(circle at 100% 100%, rgba(255, 107, 107, 0.1) 0%, transparent 50%);
  opacity: 0.8;
  pointer-events: none;
}

.header {
  text-align: center;
  margin-bottom: 40px;
  padding: 20px;
  position: relative;
}

.header h1 {
  font-size: 7rem;
  font-weight: 900;
  letter-spacing: 0.05em;
  margin-bottom: 15px;
  position: relative;
  display: inline-block;
  background: linear-gradient(
    45deg,
    #9B51E0 0%,
    #FF6B6B 50%,
    #9B51E0 100%
  );
  background-size: 200% auto;
  color: transparent;
  -webkit-background-clip: text;
  background-clip: text;
  animation: gradientMove 8s ease infinite;
  filter: drop-shadow(0 2px 4px rgba(155, 81, 224, 0.2));
}

@keyframes gradientMove {
  0% {
    background-position: 0% center;
  }
  50% {
    background-position: 100% center;
  }
  100% {
    background-position: 0% center;
  }
}

.header p {
  color: var(--header-description);
  font-size: 1.2em;
  font-weight: 500;
}

.toolbar {
  max-width: 1200px;
  margin: 0 auto 40px;
  padding: 0 20px;
  display: flex;
  gap: 20px;
  align-items: center;
  justify-content: center;
  flex-wrap: wrap;
}

.search-box {
  flex: 1;
  max-width: 500px;
  position: relative;
}

.search-box input {
  width: 100%;
  padding: 15px 25px 15px 50px;
  border: 2px solid var(--toolbar-border);
  border-radius: 30px;
  font-size: 1.1em;
  background: var(--input-bg);
  color: var(--input-text);
  transition: all 0.3s ease;
  box-shadow: 
    0 4px 12px rgba(155, 81, 224, 0.08),
    inset 0 0 0 1px rgba(255, 255, 255, 0.5);
}

.search-box input::placeholder {
  color: var(--input-placeholder);
}

.search-icon {
  position: absolute;
  left: 20px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 1.2em;
  color: #9B51E0;
  pointer-events: none;
}

.filter-box select {
  padding: 15px 25px;
  border: 2px solid var(--toolbar-border);
  border-radius: 30px;
  font-size: 1.1em;
  background: var(--input-bg);
  color: var(--input-text);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 
    0 4px 12px rgba(155, 81, 224, 0.08),
    inset 0 0 0 1px rgba(255, 255, 255, 0.5);
  appearance: none;
  -webkit-appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%239B51E0' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 15px center;
  background-size: 15px;
  padding-right: 45px;
}

.filter-box select:hover,
.filter-box select:focus {
  border-color: #9B51E0;
  box-shadow: 
    0 8px 20px rgba(155, 81, 224, 0.12),
    inset 0 0 0 1px rgba(255, 255, 255, 0.6);
  transform: translateY(-2px);
  background-color: white;
}

.filter-box select optgroup {
  background: white;
  color: #9B51E0;
  font-weight: 600;
  padding: 8px;
}

.filter-box select option {
  background: white;
  color: #333;
  padding: 12px;
  transition: all 0.3s ease;
}

.filter-box select option:hover,
.filter-box select option:focus {
  background: rgba(155, 81, 224, 0.1);
}

.toggle-btn {
  padding: 15px 25px;
  border: 2px solid var(--toolbar-border);
  border-radius: 30px;
  font-size: 1.1em;
  background: var(--input-bg);
  color: var(--input-text);
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
  box-shadow: 
    0 4px 12px rgba(155, 81, 224, 0.08),
    inset 0 0 0 1px rgba(255, 255, 255, 0.5);
}

.toggle-btn:hover {
  border-color: #9B51E0;
  box-shadow: 
    0 8px 20px rgba(155, 81, 224, 0.12),
    inset 0 0 0 1px rgba(255, 255, 255, 0.6);
  transform: translateY(-2px);
}

.toggle-btn.active {
  background: linear-gradient(135deg, #9B51E0, #FF6B6B);
  color: white;
  border: none;
}

.star-icon {
  font-size: 1.2em;
  line-height: 1;
}

.sections {
  max-width: 1200px;
  margin: 0 auto;
  position: relative;
}

.section {
  margin-bottom: 60px;
  position: relative;
  opacity: 0;
  transform: translateY(20px);
  animation: sectionFadeIn 0.6s ease forwards;
}

@keyframes sectionFadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.section:last-child {
  margin-bottom: 0;
}

.section-header {
  text-align: center;
  margin-bottom: 30px;
  padding: 25px;
  background: var(--section-bg);
  backdrop-filter: blur(10px);
  border-radius: 20px;
  box-shadow: 
    0 4px 15px rgba(155, 81, 224, 0.1),
    inset 0 0 0 1px rgba(255, 255, 255, 0.5);
  border: 1px solid var(--toolbar-border);
  position: relative;
  overflow: hidden;
}

.section-header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg,
    #FF6B6B,
    #9B51E0
  );
}

.section-header::before {
  content: attr(data-icon);
  position: absolute;
  right: 20px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 2.5em;
  opacity: 0.2;
}

.section-header h2 {
  font-size: 2.2em;
  color: var(--section-text);
  margin-bottom: 10px;
  font-weight: 700;
  background: linear-gradient(45deg, #9B51E0, #FF6B6B);
  -webkit-background-clip: text;
  color: transparent;
}

.section-header p {
  color: var(--section-description);
  font-size: 1.1em;
  max-width: 600px;
  margin: 0 auto;
  background: linear-gradient(
    90deg,
    #FF6B6B,
    #9B51E0,
    #FF6B6B
  );
  background-size: 200% auto;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: shine 8s linear infinite;
}

.section-header h2::after {
  content: '';
  position: absolute;
  bottom: -5px;
  left: 0;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, 
    transparent, 
    #FF6B6B,
    #9B51E0,
    transparent
  );
}

.card-container {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 25px;
  padding: 20px;
}

.card {
  position: relative;
  background: var(--card-bg);
  backdrop-filter: blur(10px);
  border-radius: 20px;
  padding: 25px;
  text-align: center;
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid var(--toolbar-border);
  overflow: hidden;
  box-shadow: 
    0 4px 15px rgba(155, 81, 224, 0.1),
    inset 0 0 0 1px rgba(255, 255, 255, 0.5);
  opacity: 0;
  transform: translateY(20px);
  animation: fadeInUp 0.5s ease forwards;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.card:hover {
  background: rgba(255, 255, 255, 0.9);
  transform: translateY(-5px) scale(1.02);
  border-color: var(--toolbar-border);
  box-shadow: 
    0 8px 25px rgba(155, 81, 224, 0.15),
    inset 0 0 0 1px rgba(255, 255, 255, 0.8);
}

.card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 3px;
  background: linear-gradient(90deg,
    #FF6B6B,
    #9B51E0
  );
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.3s ease;
}

.card:hover::before {
  transform: scaleX(1);
}

.card-content {
  position: relative;
  z-index: 1;
}

.card-icon {
  font-size: 3em;
  margin: 15px 0;
  transition: transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  filter: drop-shadow(0 2px 4px rgba(155, 81, 224, 0.2));
}

.card:hover .card-icon {
  transform: scale(1.1) rotate(8deg);
}

.card h3 {
  font-size: 1.4em;
  margin: 10px 0;
  color: var(--card-text);
  font-weight: 600;
  background: linear-gradient(45deg, #9B51E0, #FF6B6B);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.card p {
  font-size: 0.95em;
  color: var(--description-text);
  margin-bottom: 15px;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  height: 2.8em;
}

.card-tags {
  display: flex;
  flex-wrap: nowrap;
  gap: 8px;
  justify-content: center;
  margin-top: 15px;
  padding: 0 10px;
  overflow: hidden;
}

.tag {
  padding: 4px 12px;
  background: var(--tag-bg);
  border-radius: 15px;
  font-size: 0.85em;
  color: var(--tag-text);
  font-weight: 500;
  transition: all 0.3s ease;
  border: 1px solid var(--toolbar-border);
  position: relative;
  overflow: hidden;
  white-space: nowrap;
  flex: 1;
  text-align: center;
  backdrop-filter: blur(5px);
}

.tag::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(45deg,
    rgba(155, 81, 224, 0.2),
    rgba(255, 107, 107, 0.2)
  );
  opacity: 0;
  transition: opacity 0.3s ease;
}

.tag:hover {
  transform: translateY(-1px);
  background: rgba(255, 107, 107, 0.1);
  border-color: var(--toolbar-border);
  color: #FF6B6B;
}

.tag:hover::before {
  opacity: 1;
}

.favorite-btn {
  position: absolute;
  top: 15px;
  right: 15px;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(155, 81, 224, 0.3);
  font-size: 1.2em;
  color: rgba(255, 215, 0, 0.5);
  cursor: pointer;
  transition: all 0.3s ease;
  z-index: 2;
  padding: 5px;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.favorite-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
  border-color: rgba(155, 81, 224, 0.5);
  color: rgba(255, 215, 0, 0.8);
}

.favorite-btn.active {
  background: #9B51E0;
  color: #FFD700;
  border-color: #9B51E0;
  box-shadow: 0 2px 8px rgba(155, 81, 224, 0.3);
}

.dark-theme .favorite-btn {
  background: rgba(60, 60, 60, 0.95);
  border-color: rgba(155, 81, 224, 0.5);
  color: rgba(255, 215, 0, 0.5);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.25);
}

.dark-theme .favorite-btn:hover {
  border-color: rgba(155, 81, 224, 0.8);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
  color: rgba(255, 215, 0, 0.8);
}

.dark-theme .favorite-btn.active {
  background: #9B51E0;
  color: #FFD700;
  border-color: #9B51E0;
}

.favorite-btn .star-icon {
  font-size: 1.4em;
}

@keyframes bounce {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.3);
  }
}

[v-cloak] {
  display: none;
}

@media (max-width: 1400px) {
  .card-container {
    justify-content: center;
  }
  .card {
    min-width: 260px;
  }
}

@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
    gap: 15px;
  }

  .search-box,
  .filter-box select,
  .toggle-btn {
    width: 100%;
    max-width: none;
  }

  .search-box input,
  .filter-box select,
  .toggle-btn {
    padding: 12px 20px;
    font-size: 1em;
  }

  .card-container {
    grid-template-columns: 1fr;
    padding: 10px;
  }

  .header h1 {
    font-size: 4rem;
  }

  .header h1::before {
    filter: blur(10px);
  }
  
  .header h1::after {
    inset: -10px;
    filter: blur(20px);
  }

  .header p {
    font-size: 1em;
  }

  .section-header h2 {
    font-size: 1.8em;
  }

  .card {
    padding: 15px;
  }
}

/* 添加列表视图样式 */
[data-layout="list"] .card-container {
  grid-template-columns: 1fr !important;
  gap: 15px;
}

[data-layout="list"] .card {
  display: flex;
  padding: 20px;
  text-align: left;
}

[data-layout="list"] .card-content {
  display: flex;
  align-items: center;
  gap: 20px;
  width: 100%;
}

[data-layout="list"] .card-icon {
  margin: 0;
  font-size: 2em;
}

[data-layout="list"] .card h3 {
  margin: 0;
  flex: 1;
}

[data-layout="list"] .card p {
  flex: 2;
  margin: 0;
  height: auto;
}

[data-layout="list"] .card-tags {
  flex: 1;
  justify-content: flex-end;
  margin: 0;
}

/* 动画控制 */
[data-animations="false"] .card,
[data-animations="false"] .section,
[data-animations="false"] .card-icon,
[data-animations="false"] .tag,
[data-animations="false"] .favorite-btn,
[data-animations="false"] .toggle-btn,
[data-animations="false"] .search-box input,
[data-animations="false"] .filter-box select {
  transition: none !important;
  animation: none !important;
}

/* 字体设置 */
.home {
  font-family: var(--font-family, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif);
}

/* 圆角设置 */
.card {
  border-radius: var(--card-radius, 20px);
}

.toggle-btn,
.search-box input,
.filter-box select {
  border-radius: var(--button-radius, 30px);
}

/* 透明度设置 */
.card {
  background: rgba(var(--card-bg-rgb), var(--card-opacity, 0.7));
}

.toolbar {
  background: rgba(var(--toolbar-bg-rgb), var(--toolbar-opacity, 0.8));
}
</style>

