<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { appConfig } from '$lib/stores/app';
  import { getAppConfig } from '$lib/commands';

  onMount(async () => {
    try {
      const cfg = await getAppConfig();
      appConfig.set(cfg);
      if (cfg.theme === 'light') document.body.classList.add('light');
    } catch {}
  });
</script>

<slot />
