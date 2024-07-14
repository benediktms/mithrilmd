<script>
  import Foo from '$lib/Foo.svelte';
  import { ComponentFooBar } from 'svelte-test-lib';
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
  <h1>home</h1>
  <p>from app</p>
  <Foo />
  <p>from lib</p>
  <ComponentFooBar />
</section>
