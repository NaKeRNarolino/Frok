<script lang="ts">
    import type {ProjectConfig} from "../../../project";
    import {invoke} from "@tauri-apps/api/core";
    import { page } from "$app/state";
    import FileView from "../../../components/file_view.svelte";
    import FolderView from "../../../components/folder_view.svelte";

    import type { PageProps } from "./$types"
    import type {FolderData} from "../../../file_data";
    import {cardBgColor, color, colorItself} from "../../../color";
    import type {AutosortTag} from "../../../autosort_tags";
    import AddNewTag from "../../../components/add_new_tag.svelte";
    import {goto} from "$app/navigation";
    import {info} from "@tauri-apps/plugin-log";

    let split = $derived(page.params.folder.split("CONFIG2FOLDER"));
    let config: Promise<ProjectConfig> = $derived(invoke("get_config", { path: split[0] }));
    let folderData: Promise<FolderData> = $derived(invoke("get_folder_data", { path: split[1] }));

    let bgColor = color(32, 91);
    let textColor = color(78, 14);
    let buttonNotHoveredColor = color(100, 92);
    let dimColor = color(100, 22);

    async function goUp() {
        let conf = (await config).project_root.replaceAll('\\', '/');
        let pat = (await folderData).path.replaceAll('\\', '/').replace(/\/[^/]*$/, '');
        info(`/file_manager/${conf}/project.frok.yamlCONFIG2FOLDER${pat}`)
        await goto(`/file_manager/${conf}/project.frok.yamlCONFIG2FOLDER${pat}`)
    }
</script>

{#await Promise.all([config, folderData, Promise.all([bgColor, cardBgColor, colorItself(), textColor, buttonNotHoveredColor, dimColor])])}
	<div>Loading</div>
{:then [config, folderData, color]}
    <main style="--bg-color: {color[0]}; --accent-color: {color[2]}; --card-icon-color: {color[1]}; --text-color: {color[3]}; --button-not-hovered-color: {color[4]}; --dim-color: {color[5]}">
        <nav>
            {#if folderData.path !== config.project_root}
                <button id="back__button" class="container" onclick={goUp}>
                    <div>
                        <p>Up</p>
                    </div>
                </button>
            {/if}
            <div class="container">
                <p>
                    {folderData.path}
                </p>
            </div>
            <div class="container" style="flex: 1">
                <p><i>Quick props are currently unavailable</i></p>
            </div>
        </nav>
        <div class="container items__grid">
            {#each folderData.folders as folder}
                <div class="container">
                    <FolderView data={folder} config={config}></FolderView>
                </div>
            {/each}
            {#each folderData.files as file}
                <div class="container">
                    <FileView data={file}></FileView>
                </div>
            {/each}
        </div>
    </main>
{/await}

<style>
    main {
      width: 100vw;
      height: 100vh;
      background-color: var(--bg-color);
    }

    .container {
        background-color: var(--card-icon-color);
        border-radius: 1rem;
        padding: 1rem;
        margin: 1rem;
    }

    nav > .container {
        margin: 0;
    }

    p {
        color: var(--text-color);
        font-family: "JetBrains Mono ExtraBold", monospace;
        padding: 0;
        margin: 0 auto;
    }

    nav {
        padding: 1rem;
        display: flex;
        gap: 0.5rem;
        flex-direction: row;
    }

    .items__grid {
        display: grid;
        grid-template-columns: auto auto auto auto;

        height: 80vh;
        overflow: scroll;
        overflow-x: hidden;

        scrollbar-width: none;
    }

    .container > .container {
        background-color: var(--button-not-hovered-color);
    }

    .items__grid > .container {
        height: 20rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    #back__button {
        border: none;
        outline: none;
        transition-duration: 0.2s;
    }

    #back__button:hover, #back__button:focus {
        transform: scale(1.2) rotate(12deg);
        background-color: var(--accent-color);
        box-shadow: var(--dim-color) 0 0 3rem;

        div {
            p {
                color: var(--card-icon-color);
            }
        }
    }
</style>