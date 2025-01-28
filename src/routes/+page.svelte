<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {cardBgColor, color, colorItself} from "../color";
  import {goto} from "$app/navigation";
  import Card from "../components/card.svelte";

  let bgColor = color(32, 91);
  let textColor = color(78, 14);

  function onAnalyzeNew() {
    goto("/analyze_new")
  }
</script>

{#await Promise.all([bgColor, cardBgColor, colorItself(), textColor])}
  <div></div>
{:then color}
  <main class="container" style="--bg-color: {color[0]}; --accent-color: {color[2]}; --card-icon-color: {color[1]}; --text-color: {color[3]}">
    <div id="startup__container">
      <div id="cards__container">
        <div class="startup__card" style="--card-height: 40vh">
          <Card text="Open a Project">
            <svg id="open__icon" xmlns="http://www.w3.org/2000/svg" height="8rem" viewBox="0 -960 960 960"><path d="M720-330q0 104-73 177T470-80q-104 0-177-73t-73-177v-370q0-75 52.5-127.5T400-880q75 0 127.5 52.5T580-700v350q0 46-32 78t-78 32q-46 0-78-32t-32-78v-370h80v370q0 13 8.5 21.5T470-320q13 0 21.5-8.5T500-350v-350q-1-42-29.5-71T400-800q-42 0-71 29t-29 71v370q-1 71 49 120.5T470-160q70 0 119-49.5T640-330v-390h80v390Z"/></svg>
          </Card>
        </div>
        <div class="startup__card" style="--card-height: 40vh">
          <Card text="Analyze New" onPressed={onAnalyzeNew}>
            <svg id="new__icon" xmlns="http://www.w3.org/2000/svg" height="8rem" viewBox="0 -960 960 960"><path d="M440-200h80v-167l64 64 56-57-160-160-160 160 57 56 63-63v167ZM240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z"/></svg>
          </Card>
        </div>
      </div>
    </div>
  </main>
{/await}

<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap');

  main {
    background-color: var(--bg-color);
    width: 100vw;
    height: 100vh;
  }

  .startup__card {
    width: 13rem;
  }

  #open__icon {
    width: 8rem;
    transform: rotate(45deg);
    fill: var(--card-bg-color);
  }

  #new__icon {
    width: 8rem;
    fill: var(--card-bg-color);
  }

  #startup__container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  #cards__container {
    display: flex;
    gap: 4rem;
  }
</style>
