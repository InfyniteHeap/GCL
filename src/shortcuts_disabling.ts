export function disableShortcuts() {
  window.addEventListener("contextmenu", (event) => event.preventDefault());
  window.addEventListener("auxclick", (event) => event.preventDefault());
  window.addEventListener("keydown", function (event) {
    // Common shortcuts to disable
    const disabledShortcuts = [
      // Disable function keys
      "F1",
      "F2",
      "F3",
      "F4",
      "F5",
      "F6",
      "F7",
      "F8",
      "F9",
      "F10",
      "F11",
      "F12",
      // Disable Ctrl-based shortcuts
      "Ctrl+R",
      "Ctrl+N",
      "Ctrl+Shift+N",
      "Ctrl+T",
      "Ctrl+Shift+T",
      "Ctrl+U",
      "Ctrl+P",
      "Ctrl+S",
      "Ctrl+Shift+I",
      "Ctrl+Shift+J",
      "Ctrl+W",
      "Ctrl+Q",
      "Ctrl+Y",
      "Ctrl+K",
      "Ctrl+L",
      // Disable Alt-based shortcuts
      "Alt+F4",
      "Alt+Tab",
      "Alt+Home",
    ];

    // Check if the pressed key matches any disabled shortcuts
    if (
      disabledShortcuts.some((shortcut) => {
        const keys = shortcut.split("+");

        return keys.every((key) => {
          if (key === "Ctrl" || key === "Command")
            return event.ctrlKey || event.metaKey;
          if (key === "Shift") return event.shiftKey;
          if (key === "Alt") return event.altKey;
          return event.key === key; // Check the actual key
        });
      })
    ) {
      event.preventDefault(); // Prevent the default action
    }
  });
}
