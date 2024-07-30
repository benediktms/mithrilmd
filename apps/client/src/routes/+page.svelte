<script lang="ts">
  import Form from '@shadcn-ui/form';
  import { createVaultSchema } from './schema';
  import Input from '@shadcn-ui/input';
  import { superForm, defaults } from 'sveltekit-superforms';
  import { zod, zodClient } from 'sveltekit-superforms/adapters';
  import { getAllVaults, setupNewVault, type Vault } from '../types/bindings';
  import { open } from '@tauri-apps/api/dialog';
  import VaultMenuRow from '$lib/Components/VaultMenu/VaultMenuRow.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import Button from '../../../../packages/shadcn-ui/src/lib/components/ui/button/button.svelte';

  const form = superForm(defaults({ name: '', location: '' }, zod(createVaultSchema)), {
    validators: zodClient(createVaultSchema),
    SPA: true,
    clearOnSubmit: 'none'
  });

  const { form: formData, enhance, validate, errors } = form;

  $: isDisabled = !!$errors._errors?.length || !$formData.location.length || !$formData.name.length;
  let vaults: Vault[] = [];

  async function createVault() {
    // TODO: this will fail if a vault exists in the database with the same name, but a different
    // path. This needs to be taken into account when implementing filewatcher logic
    let res = await setupNewVault({
      name: $formData.name,
      location: $formData.location
    });

    if (res.vault_id) {
      localStorage.setItem('active-vault', res.vault_id.toString());
      goto(`/vaults/${res.vault_id}/board`);
    } else if (res.error) {
      throw res.error;
    }
  }

  // TODO: add proper handler with notifications
  async function onSubmit(_: SubmitEvent) {
    await createVault();
  }

  async function locateExistingVault() {
    const selected = await open({
      directory: true,
      title: 'Select folder',
      defaultPath: '~/Documents'
    });

    if (!selected || Array.isArray(selected) || !formData) {
      return;
    }

    const pathSegemnts = selected?.split('/');
    const vaultName = pathSegemnts[pathSegemnts.length - 1];

    $formData.name = vaultName;
    $formData.location = selected;

    validate('location');
    validate('name');

    if ($errors._errors?.length) {
      return;
    }

    await createVault();
  }

  function loadVault(id: number) {
    localStorage.setItem('active-vault', id.toString());
    goto(`/vaults/${id}/board`);
  }

  onMount(async () => {
    // TODO: delay page render until mounting function is settled
    let activeVaultId = localStorage.getItem('active-vault');

    if (activeVaultId) {
      goto(`/vaults/${activeVaultId}/board`);
    }

    const vaultsRes = await getAllVaults();

    // TODO: add notifications here
    if (vaultsRes.vaults.length) {
      vaults = vaultsRes.vaults;
    }
  });
</script>

<div class="mx-auto flex h-full max-w-lg flex-col items-center md:max-w-2xl">
  <div class="mb-5 mt-10">
    <h1 class="text-center text-4xl">Vaults</h1>
  </div>
  <div class="w-full">
    <p class="text-muted-foreground my-8 w-full text-center text-xs">create a new vault</p>
    <form class="mt-8" use:enhance on:submit={onSubmit}>
      <Form.Field {form} name="name">
        <Form.Control let:attrs>
          <Form.Label>Vault name</Form.Label>
          <Input {...attrs} bind:value={$formData.name} on:input={() => validate('name')} />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
      <div class="mt-4">
        <Form.Field {form} name="location">
          <VaultMenuRow
            title="Create new vault"
            description="Select a folder to set up a new Mithril vault"
            buttonText="Select"
            overrideText={$formData.location}
            callback={async () => {
              const selected = await open({
                directory: true,
                title: 'Select folder',
                defaultPath: '~/Documents'
              });
              if (!selected || Array.isArray(selected)) {
                return;
              }

              if (formData) {
                $formData.location = selected;
                validate('location');
              }
            }}
          />
          <Form.FieldErrors />
          <Form.Button disabled={isDisabled} class="w-full">Submit</Form.Button>
        </Form.Field>
      </div>
    </form>
    <div class="mt-4">
      <p class="text-muted-foreground my-8 w-full text-center text-xs">select an existing vault</p>
      <ul class="my-8">
        {#each vaults as vault (vault.id)}
          <li>
            <Button variant="link" on:click={() => loadVault(vault.id)}>{vault.name}</Button>
          </li>
        {/each}
      </ul>
      <p class="text-muted-foreground my-8 w-full text-center text-xs">locate a vault</p>
      <VaultMenuRow
        title="Open vault"
        description="Select an existing folder to open as a vault"
        buttonText="Open"
        overrideText={undefined}
        callback={locateExistingVault}
      />
    </div>
  </div>
</div>
