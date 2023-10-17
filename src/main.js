const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  // Disable right-click menu
  document.oncontextmenu = function (event) {
    event.preventDefault();
  };

  // Disable F1 to F12 function keys
  document.addEventListener('keydown', function (event) {
    if (event.key.startsWith('F') && event.key.length === 2) {
      event.preventDefault();
    }
  });
});
