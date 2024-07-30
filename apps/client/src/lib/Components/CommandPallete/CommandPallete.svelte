<script lang="ts">
  import Check from 'lucide-svelte/icons/check';
  import { onDestroy, onMount, tick } from 'svelte';
  import { Search } from 'lucide-svelte';
  import { commandPallete } from './commandPalleteStore.js';
  import { allTasks, currentlyFocusedTaskId } from '../Board/taskStores.js';
  import { cn } from '../../utils';
  import { Button } from '@shadcn-ui/button';
  import Popover from '@shadcn-ui/popover';
  import Command from '@shadcn-ui/command';
  import ScrollArea from '@shadcn-ui/scroll-area';

  let open = false;
  let selectedTask: string | undefined = undefined;

  const componentWidth = 'min-w-[360px]';

  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger(triggerId: string) {
    open = false;
    selectedTask = undefined;
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  }

  const unsubscribe = commandPallete.subscribe(state => {
    open = state.open;
    selectedTask = state.value;
  });

  function handleKeydown(e: KeyboardEvent) {
    // TODO: add ctrl key option for windows/linux
    if (e.key === 'p' && e.metaKey) {
      e.preventDefault();
      open = !open;
    }
  }

  onMount(() => {
    document.addEventListener('keydown', handleKeydown);
  });

  onDestroy(() => {
    unsubscribe();
    document.removeEventListener('keydown', handleKeydown);
  });
</script>

<Popover.Root bind:open let:ids>
  <Popover.Trigger asChild let:builder>
    <Button
      builders={[builder]}
      variant="outline"
      role="combobox"
      aria-expanded={open}
      class={cn('hover:border-primary h-[23px] justify-around', componentWidth)}
    >
      <span class="flex items-center">
        <Search class="text-primary mr-2 h-4 w-4" />
        <p>Tasks</p>
      </span>
    </Button>
  </Popover.Trigger>
  <Popover.Content class={cn('mt-2 p-0', componentWidth)}>
    <Command.Root>
      <Command.Input placeholder="Search tasks..." />
      <Command.Empty>No framework found.</Command.Empty>
      <ScrollArea.Root class="h-80">
        <Command.Group>
          {#each $allTasks as task}
            <Command.Item
              value={task.id}
              onSelect={taskId => {
                selectedTask = taskId;
                $currentlyFocusedTaskId = selectedTask;
                closeAndFocusTrigger(ids.trigger);
              }}
            >
              <Check
                class={cn(
                  'mr-2 h-4 w-4',
                  $currentlyFocusedTaskId !== task.id && 'text-transparent'
                )}
              />
              {task.title}
            </Command.Item>
          {/each}
        </Command.Group>
      </ScrollArea.Root>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
