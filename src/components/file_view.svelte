<script lang="ts">
    import type {FileData} from "../file_data";
    import {convertFileSrc, invoke} from "@tauri-apps/api/core";
    import {info} from "@tauri-apps/plugin-log";

    let {data} = $props();

    const isImage = data.name.endsWith(".png") || data.name.endsWith(".jpeg") || data.name.endsWith(".svg");
    const isUndetermined = !isImage;

    async function openFile() {
        info(`${data.path.replaceAll("/", "\\")}`)
        await invoke("open_file_with_selector", { path: data.path.replaceAll("/", "\\") })
    }

</script>

<div id="holder">
    <button id="preview" onclick={openFile}>
        {#if isImage}
            <img src={convertFileSrc(data.path)} alt="Cannot be determined">
        {:else if isUndetermined}
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M200-200h560v-367L567-760H200v560Zm0 80q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h400l240 240v400q0 33-23.5 56.5T760-120H200Zm80-160h400v-80H280v80Zm0-160h400v-80H280v80Zm0-160h280v-80H280v80Zm-80 400v-560 560Z"/></svg>
        {/if}
    </button>
    <p>{data.name}</p>
</div>

<style>
    #holder {
        display: flex;
        flex-direction: column;
        justify-content: center;
        width: 100%;
        height: 100%;
        align-items: center;
    }

    p {
        font-family: "JetBrains Mono ExtraBold", monospace;
        width: 15rem;
        overflow-x: clip;
        text-overflow: ellipsis;
        max-lines: 1;
        text-wrap: nowrap;
        text-align: center;
        color: var(--text-color)
    }

    img {
        border-radius: 1rem;
        width: 15rem;
    }

    #preview {
        background-color: var(--button-not-hovered-color);
        width: 15rem;
        height: 15rem;
        transition-duration: 0.2s;
        border: none;
        outline: none;
    }

    #holder:hover {
        #preview {
            border-radius: 1rem;
            padding: 0;
            margin: 0;
            transform: scale(1.2) rotate(12deg);

            box-shadow: var(--dim-color) 0 0 10rem;
        }
    }

    svg {
        fill: var(--text-color);
    }
</style>
