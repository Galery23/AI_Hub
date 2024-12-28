<template>
  <div class="prompt-templates">
    <div class="toast-message" v-if="showMessage" :style="{ opacity: messageOpacity }">
      <span class="icon">✓</span>
      {{ message }}
    </div>

    <div class="header">
      <h1>Prompt 模板库</h1>
      <p>探索和管理你的 AI 对话提示词</p>
    </div>

    <div class="toolbar">
      <div class="search-box">
        <input 
          type="text" 
          v-model="searchQuery" 
          placeholder="搜索模板..." 
          @input="filterTemplates"
        >
        <span class="search-icon">🔍</span>
      </div>
      
      <div class="filter-box">
        <select v-model="selectedCategory" @change="filterTemplates">
          <option value="all">全部分类</option>
          <optgroup label="常用分类">
            <option value="academic">学术研究</option>
            <option value="programming">编程开发</option>
            <option value="creation">创意写作</option>
          </optgroup>
          <optgroup label="专业领域">
            <option value="translation">翻译优化</option>
            <option value="analysis">数据分析</option>
            <option value="business">商业写作</option>
          </optgroup>
        </select>
      </div>

      <button @click="showAddForm = true" 
              class="add-btn">
        <span class="icon">✨</span>
        添加模板
      </button>
    </div>

    <!-- 预设模板部分 -->
    <div class="template-section">
      <h2 class="section-title">
        <span class="icon">📚</span>
        预设模板
      </h2>
      <div class="template-grid">
        <div v-for="(template, index) in filteredPresetTemplates" 
             :key="'preset-'+index" 
             class="template-card">
          <div class="card-header">
            <div class="card-title">
              <h3>{{ template.name }}</h3>
              <div class="tags">
                <span class="tag" v-for="tag in template.tags" :key="tag">{{ tag }}</span>
              </div>
            </div>
            <button @click="copyPrompt(template.content)" 
                    class="copy-btn"
                    :class="{ 'copied': copiedIndex === 'preset-'+index }"
                    @mouseleave="copiedIndex = null">
              <span class="icon">{{ copiedIndex === 'preset-'+index ? '✓' : '📋' }}</span>
              <span class="text">{{ copiedIndex === 'preset-'+index ? '已复制' : '复制' }}</span>
            </button>
          </div>
          <p class="description">{{ template.description }}</p>
          <div class="content">
            <pre>{{ template.content }}</pre>
          </div>
        </div>
      </div>
    </div>

    <!-- 用户自定义模板部分 -->
    <div class="template-section">
      <h2 class="section-title">
        <span class="icon">🎯</span>
        我的模板
      </h2>
      <div class="template-grid">
        <div v-for="(template, index) in filteredUserTemplates" 
             :key="'user-'+index" 
             class="template-card">
          <div class="card-header">
            <div class="card-title">
              <h3>{{ template.name }}</h3>
              <div class="tags">
                <span class="tag" v-for="tag in template.tags" :key="tag">{{ tag }}</span>
              </div>
            </div>
            <div class="card-actions">
              <button @click="copyPrompt(template.content)" 
                      class="copy-btn"
                      :class="{ 'copied': copiedIndex === 'user-'+index }"
                      @mouseleave="copiedIndex = null">
                <span class="icon">{{ copiedIndex === 'user-'+index ? '✓' : '📋' }}</span>
                <span class="text">{{ copiedIndex === 'user-'+index ? '已复制' : '复制' }}</span>
              </button>
              <button @click="deleteTemplate(index)" class="delete-btn">
                <span class="icon">🗑️</span>
              </button>
            </div>
          </div>
          <p class="description">{{ template.description }}</p>
          <div class="content">
            <pre>{{ template.content }}</pre>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加模板表单 -->
    <div v-if="showAddForm" class="modal-overlay" @click.self="showAddForm = false">
      <div class="modal-content">
        <div class="modal-header">
          <h3>创建新模板</h3>
          <button @click="showAddForm = false" class="close-btn">×</button>
        </div>
        <form @submit.prevent="addTemplate" class="template-form">
          <div class="form-group">
            <label>模板名称</label>
            <input v-model="newTemplate.name" required
                   placeholder="给你的模板起个名字">
          </div>
          <div class="form-group">
            <label>描述</label>
            <input v-model="newTemplate.description" required
                   placeholder="简单描述模板的用途">
          </div>
          <div class="form-group">
            <label>分类标签</label>
            <div class="tags-input">
              <input v-model="tagInput"
                     @keydown.enter.prevent="addTag"
                     @keydown.tab.prevent="addTag"
                     placeholder="输入标签后按 Enter 添加">
              <div class="selected-tags">
                <span v-for="tag in newTemplate.tags" 
                      :key="tag" 
                      class="tag">
                  {{ tag }}
                  <button @click.prevent="removeTag(tag)" class="remove-tag">×</button>
                </span>
              </div>
            </div>
          </div>
          <div class="form-group">
            <label>Prompt 内容</label>
            <textarea v-model="newTemplate.content" required rows="6"
                      placeholder="在这里输入你的 Prompt 内容"></textarea>
          </div>
          <div class="form-actions">
            <button type="button" @click="showAddForm = false" class="cancel-btn">
              取消
            </button>
            <button type="submit" class="submit-btn">
              保存模板
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PromptTemplates',
  data() {
    return {
      showAddForm: false,
      searchQuery: '',
      selectedCategory: 'all',
      copiedIndex: null,
      tagInput: '',
      newTemplate: {
        name: '',
        description: '',
        content: '',
        tags: []
      },
      presetTemplates: [
        {
          name: '学术翻译',
          description: '将学术文献翻译成通顺的中文，保持专业性和准确性',
          content: '你是一位专业的学术翻译，请将下面的文本翻译成地道的中文，要求：\n1. 准确传达原文的学术含义\n2. 符合中文的表达习惯\n3. 保持专业术语的规范性\n\n以下是需要翻译的内容：',
          tags: ['翻译', '学术', '中文优化']
        },
        {
          name: '论文总结',
          description: '快速提取学术论文的关键信息和主要发现',
          content: '请帮我总结这篇论文的以下几个方面：\n1. 研究目的和背景\n2. 主要方法和创新点\n3. 关键发现和结论\n4. 研究局限性和未来展望\n\n请用简洁的中文表述。论文内容如下：',
          tags: ['学术', '总结', '研究']
        },
        {
          name: '代码解释',
          description: '详细分析代码的功能、结构和实现逻辑',
          content: '请详细解释以下代码的功能、实现逻辑和关键点，要求：\n1. 分析代码的主要功能\n2. 解释关键算法和数据结构\n3. 指出可能的优化空间\n4. 提供改进建议\n\n代码如下：',
          tags: ['编程', '代码分析', '优化']
        },
        {
          name: '创意写作',
          description: '生成富有创意和吸引力的文章内容',
          content: '作为一位专业的创意写手，请帮我创作一篇内容，要求：\n1. 风格独特，富有创意\n2. 语言生动，画面感强\n3. 结构完整，逻辑清晰\n4. 适合目标受众\n\n主题描述：',
          tags: ['写作', '创意', '内容创作']
        },
        {
          name: '论文评析',
          description: '对学术论文进行系统的分析和评价',
          content: '请对以下学术论文进行专业评析，重点关注：\n1. 研究价值和创新性\n2. 研究方法的合理性\n3. 结果的可靠性和意义\n4. 论文结构和写作规范\n5. 局限性和改进建议\n\n论文内容如下：',
          tags: ['学术', '分析', '研究']
        },
        {
          name: '故事创作',
          description: '生成结构完整、情感丰富的故事内容',
          content: '请创作一个引人入胜的故事，需要包含：\n1. 场景：时间、地点、氛围的简要描写\n2. 人物：性格特点和关系设定\n3. 情节：冲突、转折和结局\n4. 主题：故事想要传达的核心意义\n\n故事要素描述：',
          tags: ['创意', '写作', '故事']
        }
      ],
      userTemplates: [],
      showMessage: false,
      message: '',
      messageOpacity: 1
    }
  },
  computed: {
    filteredPresetTemplates() {
      return this.filterTemplatesByQuery(this.presetTemplates)
    },
    filteredUserTemplates() {
      return this.filterTemplatesByQuery(this.userTemplates)
    }
  },
  methods: {
    showToast(msg) {
      this.message = msg;
      this.showMessage = true;
      this.messageOpacity = 1;
      
      // 2秒后开始淡出
      setTimeout(() => {
        this.messageOpacity = 0;
        // 等待淡出动画完成后隐藏消息
        setTimeout(() => {
          this.showMessage = false;
        }, 300);
      }, 2000);
    },
    
    async copyPrompt(text) {
      try {
        await navigator.clipboard.writeText(text);
        // 显示复制成功消息
        this.showToast('复制成功');
        // 设置复制成功状态
        this.copiedIndex = true;
        setTimeout(() => {
          this.copiedIndex = null;
        }, 2000);
      } catch (err) {
        console.error('复制失败:', err);
        this.showToast('复制失败');
      }
    },
    addTemplate() {
      if (this.newTemplate.name && this.newTemplate.content) {
        this.userTemplates.push({...this.newTemplate});
        this.saveUserTemplates();
        this.showAddForm = false;
        this.newTemplate = {
          name: '',
          description: '',
          content: '',
          tags: []
        };
        this.tagInput = '';
      }
    },
    deleteTemplate(index) {
      this.userTemplates.splice(index, 1);
      this.saveUserTemplates();
    },
    saveUserTemplates() {
      localStorage.setItem('userPromptTemplates', JSON.stringify(this.userTemplates));
    },
    loadUserTemplates() {
      const saved = localStorage.getItem('userPromptTemplates');
      if (saved) {
        this.userTemplates = JSON.parse(saved);
      }
    },
    filterTemplatesByQuery(templates) {
      if (!this.searchQuery && this.selectedCategory === 'all') {
        return templates;
      }
      return templates.filter(template => {
        const matchesSearch = !this.searchQuery || 
          template.name.toLowerCase().includes(this.searchQuery.toLowerCase()) ||
          template.description.toLowerCase().includes(this.searchQuery.toLowerCase()) ||
          template.content.toLowerCase().includes(this.searchQuery.toLowerCase()) ||
          template.tags.some(tag => tag.toLowerCase().includes(this.searchQuery.toLowerCase()));
        
        const matchesCategory = this.selectedCategory === 'all' || 
          template.tags.includes(this.selectedCategory);
        
        return matchesSearch && matchesCategory;
      });
    },
    addTag() {
      if (this.tagInput && !this.newTemplate.tags.includes(this.tagInput)) {
        this.newTemplate.tags.push(this.tagInput);
        this.tagInput = '';
      }
    },
    removeTag(tag) {
      const index = this.newTemplate.tags.indexOf(tag);
      if (index > -1) {
        this.newTemplate.tags.splice(index, 1);
      }
    }
  },
  mounted() {
    this.loadUserTemplates();
  }
}
</script>

<style scoped>
.prompt-templates {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.header {
  text-align: center;
  margin-bottom: 2rem;
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

.toolbar {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  align-items: center;
  background: var(--bg-gradient);
  padding: 1rem;
  border-radius: 0.5rem;
}

.search-box {
  position: relative;
  flex: 1;
}

.search-box input {
  width: 100%;
  padding: 0.75rem 1rem 0.75rem 2.5rem;
  border: 1px solid var(--border-color);
  border-radius: 0.5rem;
  background: var(--bg-color);
  color: var(--text-color);
}

.search-icon {
  position: absolute;
  left: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  opacity: 0.5;
}

.filter-box select {
  padding: 0.75rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 0.5rem;
  background: var(--bg-color);
  color: var(--text-color);
  min-width: 150px;
}

.add-btn {
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

.add-btn:hover {
  background: var(--secondary-color);
}

.template-section {
  margin-bottom: 3rem;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 1.5rem;
  font-weight: bold;
  margin-bottom: 1.5rem;
  color: var(--text-color);
}

.template-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.template-card {
  background: var(--bg-color);
  border: 1px solid var(--border-color);
  border-radius: 0.5rem;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.template-card:hover {
  box-shadow: 0 0 20px var(--hover-shadow);
  transform: translateY(-2px);
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
  margin-bottom: 0.5rem;
  color: var(--text-color);
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
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
  margin-bottom: 1rem;
  font-size: 0.9rem;
}

.content {
  background: var(--bg-gradient);
  padding: 1rem;
  border-radius: 0.5rem;
  margin-bottom: 1rem;
}

.content pre {
  white-space: pre-wrap;
  word-wrap: break-word;
  color: var(--text-color);
  font-size: 0.9rem;
  line-height: 1.5;
}

.card-actions {
  display: flex;
  gap: 0.5rem;
}

.copy-btn, .delete-btn {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 0.25rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.copy-btn {
  background: var(--primary-color);
  color: white;
}

.copy-btn.copied {
  background: #4CAF50;
}

.delete-btn {
  background: var(--secondary-color);
  color: white;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-color);
  border-radius: 0.5rem;
  padding: 2rem;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.modal-header h3 {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--text-color);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: var(--text-color);
  cursor: pointer;
  padding: 0.5rem;
}

.template-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-weight: 500;
  color: var(--text-color);
}

.form-group input,
.form-group textarea {
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 0.25rem;
  background: var(--bg-color);
  color: var(--text-color);
}

.tags-input {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.selected-tags .tag {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.remove-tag {
  background: none;
  border: none;
  color: white;
  cursor: pointer;
  padding: 0 0.25rem;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1rem;
}

.cancel-btn,
.submit-btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 0.25rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.cancel-btn {
  background: var(--border-color);
  color: var(--text-color);
}

.submit-btn {
  background: var(--primary-color);
  color: white;
}

.submit-btn:hover {
  background: var(--secondary-color);
}

@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
  }
  
  .template-grid {
    grid-template-columns: 1fr;
  }
}

.toast-message {
  position: fixed;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  background: #4CAF50;
  color: white;
  padding: 12px 24px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  gap: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  z-index: 1000;
  transition: opacity 0.3s ease;
}

.toast-message .icon {
  font-size: 1.2em;
}
</style> 