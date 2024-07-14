<script>
  import Foo from '$lib/Foo.svelte';
  import { ComponentFooBar } from 'svelte-test-lib';
  import { res } from '../lib/add';
  // import { ComponentFooBar } from '@ui';
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
  <h1>home</h1>
  <Foo />
  <ComponentFooBar />
</section>
