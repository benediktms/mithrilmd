<script lang="ts">
  import Button from '@shadcn-ui/button';
  import Seperator from '@shadcn-ui/separator';

  export let title: string;
  export let description: string;
  export let buttonText: string;
  export let callback: () => Promise<void>;
</script>

<div class="flex w-full items-center justify-between pb-2">
  <div>
    <h4 class="text-sm font-semibold tracking-tight">{title}</h4>
    <p class="text-xs">{description}</p>
  </div>
  <Button variant="outline" size="sm" class="min-w-32" on:click={callback}>{buttonText}</Button>
</div>
<Seperator />
