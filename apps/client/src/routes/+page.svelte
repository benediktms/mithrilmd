<script lang="ts">
  import Form from '@shadcn-ui/form';
  import { createVault } from './schema';
  import Input from '@shadcn-ui/input';
  import { superForm, defaults } from 'sveltekit-superforms';
  import { zod, zodClient } from 'sveltekit-superforms/adapters';
  import { setupNewVault } from '../types/bindings';
  import { open } from '@tauri-apps/api/dialog';
  import VaultMenuRow from '$lib/Components/VaultMenu/VaultMenuRow.svelte';

  const form = superForm(defaults({ name: '', location: '' }, zod(createVault)), {
    validators: zodClient(createVault),
    SPA: true,
    clearOnSubmit: 'none'
  });

  const { form: formData, enhance, validate, errors } = form;

  $: isDisabled = !!$errors._errors?.length || !$formData.location.length || !$formData.name.length;

  // TODO: add proper handler with notifications
  async function onSubmit(_: SubmitEvent) {
    let res = await setupNewVault({
      name: $formData.name,
      location: $formData.location
    });
    console.log(res);
  }
</script>

<div class="container">
  <div class="max-w-lg">
    <h1 class="text-lg">create vault page</h1>

    <form use:enhance on:submit={onSubmit}>
      <Form.Field {form} name="name">
        <Form.Control let:attrs>
          <Form.Label>Vault name</Form.Label>
          <Input {...attrs} bind:value={$formData.name} on:input={() => validate('name')} />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
    </form>
    <div class="mt-4">
      <Form.Field {form} name="location">
        <VaultMenuRow
          title="Create new vault"
          description="Create a new folder to set up a Mithril vault"
          buttonText="Create"
          callback={async () => {
            const selected = await open({
              directory: true,
              title: 'Select folder',
              defaultPath: '~/Documents'
            });

            console.log(selected);
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
      <p class="text-muted-foreground my-8 w-full text-center text-xs">
        or select an existing folder
      </p>
      <VaultMenuRow
        title="Open vault"
        description="Open an existing folder as a vault"
        buttonText="Open"
        callback={async () => {
          const selected = await open({
            directory: true,
            title: 'Select folder',
            defaultPath: '~/Documents'
          });

          console.log(selected);

          if (!selected || Array.isArray(selected)) {
            return;
          }

          const pathSegemnts = selected?.split('/');
          const vaultName = pathSegemnts[pathSegemnts.length - 1];
          console.log(vaultName);

          if (formData) {
            $formData.name = vaultName;
            $formData.location = '';
            validate('location');
            validate('name');
          }
        }}
      />
    </div>
  </div>
</div>
