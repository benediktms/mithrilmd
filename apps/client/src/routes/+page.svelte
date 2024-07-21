<script lang="ts">
  import Form from '@shadcn-ui/form';
  import { createVault, type CreateVaultSchema } from './schema';
  import Input from '@shadcn-ui/input';
  import { superForm, defaults } from 'sveltekit-superforms';
  import { zod, zodClient } from 'sveltekit-superforms/adapters';

  const form = superForm(defaults({ name: '', location: '' }, zod(createVault)), {
    validators: zodClient(createVault),
    SPA: true
  });

  const { form: formData, enhance, validate } = form;

  function onSubmit(_: SubmitEvent) {
    console.log($formData);
  }
</script>

<div class="m-3 max-w-lg">
  <h1 class="text-lg">create vault page</h1>

  <form use:enhance on:submit={onSubmit}>
    <Form.Field {form} name="name">
      <Form.Control let:attrs>
        <Form.Label>Vault name</Form.Label>
        <Input {...attrs} bind:value={$formData.name} on:input={() => validate('name')} />
      </Form.Control>
      <Form.Description>Vault name.</Form.Description>
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="location">
      <Form.Control let:attrs>
        <Form.Label>Vault location</Form.Label>
        <Input {...attrs} bind:value={$formData.location} on:input={() => validate('location')} />
      </Form.Control>
      <Form.Description>Vault setup information.</Form.Description>
      <Form.FieldErrors />
    </Form.Field>
    <Form.Button>Submit</Form.Button>
  </form>
</div>
