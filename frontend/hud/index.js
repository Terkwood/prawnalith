const rust = import('./hud');
rust
  .then(m => m.run())
  .catch(console.error);
