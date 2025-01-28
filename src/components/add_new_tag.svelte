<script lang="ts">
    import type {AutosortTag} from "../autosort_tags";

    let { onClose=() => {}, onAdd } = $props();

    let extensions = $state("");
    let folderName = $state("");
</script>

<div class="container">
    <div id="top__bar">
        <p>Add a Tag</p>
        <button id="close__button" aria-label="Close" onclick={() => onClose()}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
        </button>
    </div>
    <h5>
        File extensions (separate with ';')
    </h5>
    <input type="text" bind:value={extensions} name="extensions">
    <h5>
        Folder name
    </h5>
    <input type="text" bind:value={folderName} name="folder_name">
    <button id="add__tag" onclick={() => {
        onAdd({
            folder: folderName,
            extensions: extensions.split(';')
        })
    }}>Add</button>
</div>

<style>
    .container {
        background-color: var(--card-icon-color);
        padding: 1rem;
        width: 25vw;
        border-radius: 1rem;
    }

    #top__bar {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    #close__button {
        border: none;
        outline: none;
        background: none;
    }

    #close__button > svg {
        width: 2rem;
        height: 2rem;
        fill: var(--text-color);
        transition-duration: 0.2s;
    }

    #close__button:hover > svg {
        width: 2.3rem;
        height: 2.3rem;
        fill: var(--text-color);
    }

    p, h1, h2, h3, h4, h5, h6 {
        font-family: "JetBrains Mono ExtraBold", monospace;
        font-style: italic;
        color: var(--text-color);
    }

    h5 {
        margin-bottom: 0.5rem;
    }

    input[type="text"] {
        background: white;
        padding: 0.5rem;
        width: 95%;
        border: none;
        border-radius: 1rem;
        color: var(--text-color);
        outline: none;
    }

    #add__tag {
        margin-top: 1rem;
        background-color: var(--button-not-hovered-color);
        border-radius: 1rem;
        color: var(--text-color);
        font-family: "JetBrains Mono ExtraBold", monospace;
        font-style: italic;
        font-size: large;
        width: 100%;

        transition-duration: 0.2s;

        padding: 1rem 2rem;
        border: none;
        outline: none;
    }

    #add__tag:hover {
        background-color: var(--accent-color);
        color: var(--card-icon-color);
    }
</style>