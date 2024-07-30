<script lang="ts">
  import Board from '$lib/Components/Board/Board.svelte';
  import { onMount } from 'svelte';
  import type { PageData } from './$types';
  import { findVault } from '../../../../types/bindings';

  export let data: PageData;
  const { vaultId } = data;

  onMount(async () => {
    const res = await findVault(vaultId);
    if (res.vault) {
      console.log(res.vault);
    }
  });
</script>

<Board />
