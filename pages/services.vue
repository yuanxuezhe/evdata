<template>
  <div>
    <ul class="vertical-list">
      <li v-for="service in services" :key="service.id" :class="[{ 'odd': service.id % 2 !== 0, 'even': service.id % 2 === 0 }, { 'hovered': hovered === service.id }]" @mouseover="hovered = service.id" @mouseleave="hovered = null" @contextmenu.prevent="showContextMenu($event, service)">{{ service.name }}</li>
    </ul>
    <div v-if="contextMenuVisible" :style="{ top: `${contextMenuY}px`, left: `${contextMenuX}px` }" class="context-menu">
      <ul>
        <li @click="startService">启动</li>
        <li @click="stopService">停止</li>
        <li @click="settingsService">设置</li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      services: [
        { id: 1, name: 'Service 1' },
        { id: 2, name: 'Service 2' },
        { id: 3, name: 'Service 3' }
      ],
      hovered: null,
      contextMenuVisible: false,
      contextMenuX: 0,
      contextMenuY: 0,
      selectedService: null
    };
  },
  methods: {
    showContextMenu(event, service) {
      this.contextMenuVisible = true;
      this.contextMenuX = event.clientX;
      this.contextMenuY = event.clientY;
      this.selectedService = service;
    },
    startService() {
      alert(`启动 ${this.selectedService.name}`);
      this.contextMenuVisible = false;
    },
    stopService() {
      alert(`停止 ${this.selectedService.name}`);
      this.contextMenuVisible = false;
    },
    settingsService() {
      alert(`设置 ${this.selectedService.name}`);
      this.contextMenuVisible = false;
    }
  },
  mounted() {
    window.addEventListener('click', () => {
      this.contextMenuVisible = false;
    });
  }
};
</script>

<style>
.vertical-list {
  list-style-type: none;
  padding: 0;
  margin: 0;
  width: 100%;
}

.vertical-list li {
  width: 100%;
  text-align: center;
  padding: 20px; /* 增加高度 */
  margin: 0; /* 间距为0 */
}

.vertical-list li.odd {
  background-color: #f0f8ff; /* 浅蓝色 */
}

.vertical-list li.even {
  background-color: #ffffff; /* 白色 */
}

.vertical-list li.hovered {
  border: 2px solid #000; /* 鼠标选中时加上边框 */
}

.context-menu {
  position: absolute;
  background-color: white;
  border: 1px solid #ccc;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  z-index: 1000;
}

.context-menu ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.context-menu li {
  padding: 8px 12px;
  cursor: pointer;
}

.context-menu li:hover {
  background-color: #eee;
}
</style>