<template>
  <div>
    <h1>Contact Page</h1>
    <p>Get in touch with us.</p>
    <input type="text" v-model="inputValue" placeholder="Enter your message" readonly/>
    <button @click="sendMessage">Click</button>
    <br />
    <input type="text" v-model="greetValue" placeholder="Greet message" />
    <button @click="getGreetMessage">Greet</button>
    <br />
    <button @click="showAlert">Show Alert</button>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  data() {
    return {
      inputValue: '',
      greetValue: ''
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
    }
  }
};
</script>