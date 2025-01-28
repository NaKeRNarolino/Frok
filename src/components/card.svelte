<script lang="ts">
    import {cardBgColor, color} from "../color";

    let { children, text, onPressed=() => {}, ...rest } = $props();

    let shadowColor = color(51, 27)
</script>

{#await Promise.all([cardBgColor, shadowColor])}
	<div></div>
{:then color}
    <button class="card__button" style="--card-bg-color: {color[0]}; --shadow-color: {color[1]};" {...rest} onclick={() => {
        onPressed()
    }}>
        <div class="card">
            <div class="card__content">
                <div class="card__icon">
                    {@render children?.()}
                </div>
                <div class="__text">
                    <p>{ text }</p>
                </div>
            </div>
        </div>
    </button>
{/await}

<style>
    .card {
        height: var(--card-height);
    }

    .card__button {
        background-color: transparent;
        border: none;
        outline: none;
        transition-duration: 0.2s;
        padding: 0;
        border-radius: 1rem;
    }

    .card__content {
        background-color: var(--card-bg-color);
        padding: 0;
        border-radius: 1rem;
        font-family: "JetBrains Mono ExtraBold", monospace;
        font-style: italic;
        height: var(--card-height);

        .__text {
            height: max-content;

            p {
                margin-top: 1.2rem;
                text-align: center;
            }
        }
    }

    .card__icon {
        background-color: var(--accent-color);
        padding: 3rem;
        border-radius: 1rem;
        height: calc(var(--card-height) - 10rem);

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .card__button:hover, .card__button:focus {
        transform: rotate(3deg) scale(120%);
        box-shadow: var(--shadow-color) 0 0 10rem;
    }

    p {
        color: var(--text-color);
    }
</style>