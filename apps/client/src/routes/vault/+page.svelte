<script lang="ts">
  import Board from '$lib/Components/Board/Board.svelte';
  import { onMount } from 'svelte';
  import { findVault } from '../../types/bindings';
  import { goto } from '$app/navigation';

  onMount(async () => {
    const vaultId = localStorage.getItem('active-vault');

    if (!vaultId) {
      await goto('/');
    } else {
      const res = await findVault(parseInt(vaultId));

      if (res.vault) {
        console.log(res.vault);
      }
    }
  });
</script>

<Board />
