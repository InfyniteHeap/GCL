import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

createApp(App).mount("#app");

const app = createApp(App);

// 禁用鼠标右键
document.addEventListener('contextmenu', (event) => {
    event.preventDefault();
});

// 禁用F1至F12功能键
document.addEventListener('keydown', (event) => {
    // 获取按下的键的键码
    const keyCode = event.keyCode;

    // 如果按下的键是F1至F12之间的键，则阻止默认行为
    if (keyCode >= 112 && keyCode <= 123) {
        event.preventDefault();
    }
});

app.mount('#app');