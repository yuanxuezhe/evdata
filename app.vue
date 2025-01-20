<template>
  <div class="app-container">
    <!-- 左侧导航栏 -->
    <nav class="sidebar">
      <ul>
        <li>
          <NuxtLink to="/">Home</NuxtLink>
        </li>
        <li>
          <NuxtLink to="/process">Process</NuxtLink>
        </li>
        <li>
          <NuxtLink to="/services">Services</NuxtLink>
        </li>
        <li>
          <NuxtLink to="/contact">Contact</NuxtLink>
        </li>
      </ul>
    </nav>

    <!-- 右侧页面内容 -->
    <main class="content">
      <NuxtPage />
    </main>
  </div>
</template>

<script setup>
// 可以在这里添加一些全局逻辑
</script>

<style>
/* 全局样式 */
body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 0;
}

.app-container {
  display: flex;
  min-height: 100vh;
}

/* 左侧导航栏样式 */
.sidebar {
  width: 80px; /* 导航栏宽度 */
  background-color: #f0f0f0; /* 导航栏背景颜色 */
  padding: 1rem;
  box-shadow: 2px 0 5px rgba(0, 0, 0, 0.1);
  display: flex;
}

.sidebar ul {
  list-style: none;
  margin: 0;
  padding: 0;
  width: 100%; /* 让 ul 占满导航栏宽度 */
}

.sidebar li {
  margin-bottom: 1rem; /* 每个链接之间的间距 */
  display: flex;
  justify-content: center; /* 水平居中 */
}

.sidebar a {
  color: white;
  text-decoration: none;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 80px; /* 正方形宽度 */
  height: 80px; /* 正方形高度 */
  background-color: #6cb2eb; /* 浅蓝色背景 */
  border-radius: 8px;
  transition: background-color 0.3s;
}

.sidebar a:hover {
  background-color: #4a90e2; /* 悬停时更深的蓝色 */
  transform: scale(1.05); /* 悬停时稍微放大 */
}

/* 活动链接样式 */
.router-link-active {
  background-color: #666;
}

/* 右侧页面内容样式 */
.content {
  flex: 1; /* 占据剩余空间 */
  padding: 2rem;
  background-color: #f9f9f9;
}
</style>