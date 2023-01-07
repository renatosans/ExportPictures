import { invoke } from "@tauri-apps/api/tauri";
import { ProductCard } from "./components/ProductCard";

let cardContainer: HTMLElement | null;
let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;


async function greet() {
//  let result = await invoke("insert_product");
//  if (result)
//      console.log("1 record stored");

  let productList = "";

  let produtos = JSON.parse(await invoke("get_products"));
  produtos.map((produto: any) => { productList += `${produto.nome} Price: ${produto.preco} <br/>`   })

  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.innerHTML = `Greetings ${greetInputEl.value} <br/><br/>` + productList;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  cardContainer = document.querySelector("#card-container");
  if (cardContainer)
    cardContainer.innerHTML = ProductCard("Nike Air Force 1", "The Nike Air Force 1 has a design that celebrates the rebellious underground rave scene.", 130);

  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-button")?.addEventListener("click", () => greet());
});
