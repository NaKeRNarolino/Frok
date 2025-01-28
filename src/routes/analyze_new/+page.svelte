<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {cardBgColor, color, colorItself} from "../../color";
    import type {AutosortTag} from "../../autosort_tags";
    import AddNewTag from "../../components/add_new_tag.svelte";

    let bgColor = color(32, 91);
    let textColor = color(78, 14);
    let buttonNotHoveredColor = color(100, 92);
    let dimColor = color(100, 22);

    let autosortTags: AutosortTag[] = $state([]);

    let isTagAddingOpened = $state(false)

    let folder = $state("");

    let isShown = () => isTagAddingOpened ? "shown" : "";

    function addNewTag() {
        isTagAddingOpened = true;
    }

    async function selectFolder() {
        folder = await invoke("select_folder");
    }

    async function autosort() {
        if (folder == "") return;

        await invoke("autosort_folders_and_gen_config", {
            root: folder,
            tags: autosortTags.map((x) => JSON.stringify(x))
        })
    }
</script>

{#await Promise.all([bgColor, cardBgColor, colorItself(), textColor, buttonNotHoveredColor, dimColor])}
    <div></div>
{:then color}
    <main style="--bg-color: {color[0]}; --accent-color: {color[2]}; --card-icon-color: {color[1]}; --text-color: {color[3]}; --button-not-hovered-color: {color[4]}; --dim-color: {color[5]}">
        <div class="container">
            <button id="select__folder" onclick={selectFolder}>{
                folder === "" ? "Select Folder" : folder
            }</button>
            <div class="secondary__container">
                <div id="autosort_title_and_add_new">
                    <h2>Autosort Tags</h2>
                    <button id="add__new__tag" onclick={addNewTag}>New Tag</button>
                </div>
                <ul id="tag__list">
                    {#each autosortTags as tag}
                    	<li><p>{tag.folder}: {tag.extensions.join(', ')}</p></li>
                    {/each}
                </ul>
            </div>
            <button id="select__folder" onclick={autosort}>
                Autosort!
            </button>
        </div>
        <div id="new__tag__place" class="{isShown()}">
            <AddNewTag onClose={() => {
                isTagAddingOpened = false
            }} onAdd={(tag: AutosortTag) => {
                isTagAddingOpened = false;
                autosortTags.push(tag)
            }}></AddNewTag>
        </div>
        <div id="dim" class="{isShown()}"></div>
    </main>
{/await}

<style>
    main {
        background-color: var(--bg-color);
        width: 100vw;
        height: 100vh;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .container {
        background-color: var(--card-icon-color);
        padding: 2rem;
        border-radius: 1rem;

        width: 50vw;
        height: 50vh;
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
    }

    p, h1, h2, h3, h4, h5, h6 {
        color: var(--text-color);
        font-family: "JetBrains Mono ExtraBold", monospace;
        font-style: italic;
    }

    #select__folder {
        background-color: var(--button-not-hovered-color);
        border-radius: 1rem;
        color: var(--text-color);
        font-family: "JetBrains Mono ExtraBold", monospace;
        font-style: italic;
        font-size: large;

        transition-duration: 0.2s;

        padding: 1rem 2rem;
        border: none;
        outline: none;
    }

    #select__folder:hover {
        background-color: var(--accent-color);
        color: var(--card-icon-color);
    }

    #add__new__tag {
        background-color: var(--card-icon-color);
        border-radius: 1rem;
        color: var(--text-color);
        font-family: "JetBrains Mono ExtraBold", monospace;
        font-style: italic;
        font-size: large;

        transition-duration: 0.2s;

        padding: 1rem 2rem;
        border: none;
        outline: none;
    }

    #add__new__tag:hover {
        background-color: var(--accent-color);
        color: var(--card-icon-color);
    }

    .secondary__container {
        border-radius: 1rem;
        background-color: var(--button-not-hovered-color);
        padding: 1rem;
    }

    #autosort_title_and_add_new {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    #new__tag__place {
        transform: scale(0%);
        position: absolute;
        left: auto;
        top: auto;
        transition-duration: 0.2s;
        z-index: 10;
    }

    #new__tag__place.shown {
        transform: scale(100%);
    }

    #dim {
        position: absolute;
        width: 100vw;
        height: 100vh;
        background-color: transparent;
        /*display: none;*/
        transition-duration: 0.2s;
        z-index: -1;
    }

    #dim.shown {
        z-index: 9;
        /*display: block;*/
        background-color: var(--dim-color);
        opacity: 0.5;
    }

    #tag__list {
        list-style: none;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: 0.3rem;
        padding-left: 0;
    }

    #tag__list > li {
        background-color: var(--card-icon-color);
        border-radius: 0.3rem;
        padding: 0 0.5rem;
    }
</style>
