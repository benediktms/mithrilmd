<script lang="ts">
  import Board from '$lib/Components/Board/Board.svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let name = '';
  let greetMsg = '';

  async function greet() {
    greetMsg = await invoke('greet', { name });
  }
</script>

<div>
  <Board class="h-full pt-[38px]" />
  <div>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>Greet</button>
    <p>{greetMsg}</p>
  </div>
</div>
