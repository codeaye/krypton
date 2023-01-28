<script lang="ts">
  import "@picocss/pico";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let input_text = "";
  let output_text = "";
  let input_format = "Json";
  let output_format = "Toml";

  let input_error = false;
  let output_error = false;

  $: input_text, input_format && output_format && update();

  const update = async () => {
    input_error = false;
    output_error = false;

    await invoke("convert", {
      input: input_text
        .replace(/[\u2014]/g, "--")
        .replace(/[\u2022]/g, "*")
        .replace(/[\u2018\u2019]/g, "'")
        .replace(/[\u201C\u201D]/g, '"'),
      int: input_format,
      out: output_format,
    })
      .then((e: string) => {
        output_text = e;
      })
      .catch((e: boolean) => {
        e ? (input_error = true) : (output_error = true);
      });
  };

  onMount(async () => {
    await update();
  });
</script>

<div class="grid">
  <div class="container full">
    <div class="flex-container">
      <div class="flex-items">
        <div class="text-parent"><h3>Input</h3></div>
      </div>
      <div class="flex-items">
        <textarea
          class="full"
          placeholder="Input"
          bind:value={input_text}
          aria-invalid={input_error}
          required
        />
      </div>
      <div class="flex-items">
        <div class="text-parent"><h3>Format</h3></div>
      </div>
      <div class="flex-items">
        <select required bind:value={input_format} aria-invalid={input_error}>
          <option value="Json" selected>JSON</option>
          <option value="Toml" selected>TOML</option>
          <option value="Yaml" selected>YAML</option>
          <option value="Ron" selected>RON</option>
          <option value="Xml" selected>XML</option>
          <option value="Url" selected>URL</option>
        </select>
      </div>
    </div>
  </div>
  <div class="container full">
    <div class="flex-container">
      <div class="flex-items">
        <div class="text-parent"><h3>Output</h3></div>
      </div>
      <div class="flex-items">
        <textarea
          class="full"
          placeholder="Output"
          bind:value={output_text}
          readonly
        />
      </div>
      <div class="flex-items">
        <div class="text-parent"><h3>Format</h3></div>
      </div>
      <div class="flex-items">
        <select required bind:value={output_format} aria-invalid={output_error}>
          <option value="Json" selected>JSON</option>
          <option value="Toml" selected>TOML</option>
          <option value="Yaml" selected>YAML</option>
          <option value="Ron" selected>RON</option>
          <option value="Xml" selected>XML</option>
          <option value="Url" selected>URL</option>
        </select>
      </div>
    </div>
  </div>
</div>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Fira+Code&display=swap");

  .grid {
    height: 100vh;
    padding: 40px;
  }

  textarea {
    font-size: 12px;
    font-family: "Fira Code", monospace;
  }

  .full {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
  }

  .text-parent {
    padding: 10px;
  }

  h3 {
    margin: auto;
    line-height: normal;
  }

  .flex-container {
    height: 100%;
    text-align: center;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: normal;
    align-items: normal;
    align-content: normal;
  }

  .flex-items:nth-child(1) {
    display: block;
    flex-grow: 0;
    flex-shrink: 1;
    flex-basis: auto;
    align-self: auto;
    order: 0;
  }

  .flex-items:nth-child(2) {
    display: block;
    flex-grow: 1;
    flex-shrink: 1;
    flex-basis: auto;
    align-self: auto;
    order: 0;
  }

  .flex-items:nth-child(3),
  .flex-items:nth-child(4) {
    display: block;
    flex-grow: 0;
    flex-shrink: 1;
    flex-basis: auto;
    align-self: auto;
    order: 0;
  }
</style>
