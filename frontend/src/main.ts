import "./app.css";
import App from "./App.svelte";
import { mount } from "svelte";

function mountApp() {
  mount(App, { target: document.getElementById("app")! });
}

if (window.pywebview?.api) {
  mountApp();
} else {
  let mounted = false;
  window.addEventListener("pywebviewready", () => {
    if (!mounted) { mounted = true; mountApp(); }
  });
  setTimeout(() => {
    if (!mounted) { mounted = true; mountApp(); }
  }, 300);
}
