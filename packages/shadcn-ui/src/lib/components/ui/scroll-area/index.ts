import Scrollbar from './scroll-area-scrollbar.svelte';
import Root from './scroll-area.svelte';

export {
  Root,
  Scrollbar,
  //
  Root as ScrollArea,
  Scrollbar as ScrollAreaScrollBar
};

const ScrollArea = {
  Root,
  Scrollbar
};

export default ScrollArea;
