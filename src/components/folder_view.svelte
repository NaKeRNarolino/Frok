<script lang="ts">
    import type {FileData} from "../file_data";
    import {convertFileSrc} from "@tauri-apps/api/core";
    import {goto} from "$app/navigation";
    import {info} from "@tauri-apps/plugin-log";

    let {data, config} = $props();

    async function onOpenFolder() {
        let conf = config.project_root.replaceAll('\\', '/');
        let pat = data.path.replaceAll('\\', '/');
        await goto(`/file_manager/${conf}/project.frok.yamlCONFIG2FOLDER${pat}`)
        info(`Hello, world??? /file_manager/${conf}/project.frok.yamlCONFIG2FOLDER${pat}`)
    }
</script>

<div id="holder">
    <div>
        <button id="preview" onclick={onOpenFolder} aria-label="Navigate to folder">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h240l80 80h320q33 0 56.5 23.5T880-640v400q0 33-23.5 56.5T800-160H160Zm0-80h640v-400H447l-80-80H160v480Zm0 0v-480 480Z"/></svg>
        </button>
        <p>{data.name}</p>
    </div>
</div>

<style>
    #holder {
        display: flex;
        flex-direction: column;
        justify-content: center;
        width: 100%;
        height: 100%;
        align-items: center;
        border: none;
        background-color: transparent;
        outline: none;
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

    #preview {
        background-color: var(--button-not-hovered-color);
        width: 15rem;
        height: 15rem;
        transition-duration: 0.2s;
        border: none;
        outline: none;
    }

    #holder:hover, #holder:focus {
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
