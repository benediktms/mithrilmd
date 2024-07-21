<script lang="ts">
  import Button, { type ButtonEvents, type ButtonProps } from '../button/index.js';

  type $$Props = ButtonProps;
  type $$Events = ButtonEvents;
</script>

<Button type="submit" on:click on:keydown {...$$restProps}>
  <slot />
</Button>
