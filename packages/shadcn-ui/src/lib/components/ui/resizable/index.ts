import { Pane } from 'paneforge';
import Handle from './resizable-handle.svelte';
import PaneGroup from './resizable-pane-group.svelte';

export {
  Pane,
  PaneGroup,
  Handle,
  //
  PaneGroup as ResizablePaneGroup,
  Pane as ResizablePane,
  Handle as ResizableHanlde
};

const Resizable = {
  Pane,
  PaneGroup,
  Handle
};

export default Resizable;
