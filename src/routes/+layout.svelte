<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { appConfig, showToast } from '$lib/stores/app';
  import { getAppConfig } from '$lib/commands';
  import Toast from '$lib/components/Toast.svelte';

  onMount(async () => {
    try {
      const cfg = await getAppConfig();
      appConfig.set(cfg);
      if (cfg.theme === 'light') document.body.classList.add('light');
    } catch {}

    window.onerror = (msg) => {
      showToast(String(msg), 'error');
      return false;
    };
    window.onunhandledrejection = (e) => {
      showToast('An error occurred', 'error');
    };
  });
</script>

<Toast />
<slot />
