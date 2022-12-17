import { invoke } from "@tauri-apps/api/tauri";
import { ProductCard } from "./components/ProductCard";

let cardContainer: HTMLElement | null;
let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;


async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value, } );
  }
}

window.addEventListener("DOMContentLoaded", () => {
  cardContainer = document.querySelector("#card-container");
  if (cardContainer)
    cardContainer.innerHTML = ProductCard("Nike Air Force 1", "The Nike Air Force 1 has a design that celebrates the rebellious underground rave scene.", 130);

  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    ?.addEventListener("click", () => greet());
});
