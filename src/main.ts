import { mount } from 'svelte';
<<<<<<< HEAD
import App from './App.svelte';

mount(App, {
  target: document.getElementById('app') ?? document.body,
});
=======
import './app.css';
import App from './App.svelte';

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
>>>>>>> 743870a (feat: initial FERRO launcher project structure)
