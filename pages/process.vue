<template>
  <div>
    <button @click="selectFile">选择文件路径</button>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export default {
  data() {
    return {
      inputValue: '',
      greetValue: '',
      selectedFilePath: '' // 新增的状态变量
    };
  },
  methods: {
    sendMessage() {
      invoke('my_custom_command')
        .then(response => {
          this.inputValue = response; // 将返回的数据设置到输入框
        })
        .catch(error => {
          console.error('Error sending message:', error);
        });
    },
    getGreetMessage() {
      invoke('greet', { name: this.greetValue })
        .then(response => {
          this.greetValue = response; // 将返回的问候语设置到输入框
        })
        .catch(error => {
          console.error('Error getting greet message:', error);
        });
    },
    showAlert() {
      alert('This is an alert message!');
    },
    async selectFile() {
      try {
        const filePath = await open({
          multiple: false,
          directory: false,
          filters: [{
            name: 'All Files',
            extensions: ['*']
          }]
        });
        if (filePath) {
          this.selectedFilePath = filePath;
          alert(`选择的文件路径是: ${filePath}`);
        }
      } catch (error) {
        console.error('Error selecting file:', error);
      }
    }
  }
};
</script>