<script lang="ts">
    import "@picocss/pico";

    import { once } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import { onDestroy, onMount } from "svelte";
    import {
        input_format,
        input_text,
        output_format,
        output_text,
        sync,
    } from "./state";
    import { onInterval } from "./utils";

    let formats = ["Json", "Toml", "Yaml", "Ron", "Xml", "Url"];

    let input_error = false;
    let output_error = false;

    $: $input_text, $input_format && $output_format && update();

    onInterval(async () => {
        await sync();
        console.log("Saving Data");
    }, 1000 * 60);
    onDestroy(async () => await sync());
    onMount(async () => {
        await once("close-requested", async (event) => {
            console.log("Saving data before close!");
            await sync();
            await appWindow.emit("close-safe", {});
        });
    });

    const copy = async () => await navigator.clipboard.writeText($output_text);

    const update = async () => {
        input_error = false;
        output_error = false;

        await invoke("convert", {
            input: $input_text
                .replace(/[\u2014]/g, "--")
                .replace(/[\u2022]/g, "*")
                .replace(/[\u2018\u2019]/g, "'")
                .replace(/[\u201C\u201D]/g, '"'),
            int: $input_format,
            out: $output_format,
        })
            .then((e: string) => {
                output_text.set(e);
            })
            .catch((e: boolean) => {
                e ? (input_error = true) : (output_error = true);
            });
    };
</script>

<div class="main">
    <div class="main-items">
        <div class="grid full">
            <div class="flex-container">
                <div class="flex-items text-parent">
                    <h3>Input</h3>
                </div>
                <div class="flex-items full">
                    <textarea
                        name="input"
                        class="full"
                        aria-invalid={input_error}
                        bind:value={$input_text}
                    />
                </div>
                <div class="flex-items text-parent">
                    <h3>Format</h3>
                </div>
                <div class="flex-items">
                    <select
                        bind:value={$input_format}
                        aria-invalid={input_error}
                    >
                        {#each formats as format}
                            <option value={format}>
                                {format.toUpperCase()}
                            </option>
                        {/each}
                    </select>
                </div>
            </div>

            <div class="flex-container">
                <div class="flex-items text-parent">
                    <h3>Output</h3>
                </div>
                <div class="flex-items full">
                    <textarea
                        name="output"
                        class="full noselect"
                        bind:value={$output_text}
                        readonly
                    />
                </div>
                <div class="flex-items text-parent">
                    <h3>Format</h3>
                </div>
                <div class="flex-items">
                    <select
                        bind:value={$output_format}
                        aria-invalid={output_error}
                    >
                        {#each formats as format}
                            <option value={format}>
                                {format.toUpperCase()}
                            </option>
                        {/each}
                    </select>
                </div>
            </div>
        </div>
    </div>
    <div class="main-items">
        <button on:click={copy}>Copy Output</button>
    </div>
</div>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Fira+Code&display=swap");

    button {
        widows: 100%;
    }

    h3 {
        margin: auto;
        line-height: normal;
    }

    textarea {
        resize: none;
        height: auto;
        font-size: 12px;
        font-family: "Fira Code", monospace;
    }

    .text-parent {
        padding: 10px;
    }

    .noselect {
        user-select: none;
    }

    .full {
        width: 100%;
        height: 100%;
        box-sizing: border-box;
    }

    .main {
        padding: 30px;
        display: flex;
        flex-direction: column;
        flex-wrap: nowrap;
        justify-content: flex-start;
        align-content: stretch;
        align-items: flex-start;
        height: 100vh;
        width: 100vw;
    }

    .main-items:nth-child(1) {
        order: 0;
        flex: 1 1 auto;
        align-self: auto;
        width: 100%;
    }

    .main-items:nth-child(2) {
        order: 0;
        flex: 0 1 auto;
        align-self: auto;
        width: 100%;
    }

    .flex-container {
        display: flex;
        flex-direction: column;
        flex-wrap: nowrap;
        justify-content: normal;
        align-items: normal;
        align-content: normal;
        height: 100%;
    }

    .flex-items:nth-child(1) {
        display: block;
        flex: 0 1 auto;
        align-self: auto;
        order: 0;
    }

    .flex-items:nth-child(2) {
        display: block;
        flex: 1 1 auto;
        align-self: auto;
        order: 0;
    }

    .flex-items:nth-child(3),
    .flex-items:nth-child(4) {
        display: block;
        flex: 0 1 auto;
        align-self: auto;
        order: 0;
    }
</style>
