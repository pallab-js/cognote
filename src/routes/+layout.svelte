<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { appConfig, showToast } from '$lib/stores/app';
  import { getAppConfig, backupVault } from '$lib/commands';
  import Toast from '$lib/components/Toast.svelte';

  onMount(async () => {
    try {
      const cfg = await getAppConfig();
      appConfig.set(cfg);
      if (cfg.theme === 'light') document.body.classList.add('light');
    } catch {}

    // Daily Auto-Backup System (Phase 4)
    setTimeout(async () => {
      const BACKUP_INTERVAL = 24 * 60 * 60 * 1000; // 24 hours
      const lastBackupStr = localStorage.getItem('cognote_last_backup_time');
      const now = Date.now();
      
      if (!lastBackupStr || now - parseInt(lastBackupStr, 10) > BACKUP_INTERVAL) {
        try {
          console.log('[Auto-Backup] Starting daily background vault backup...');
          const backupPath = await backupVault();
          localStorage.setItem('cognote_last_backup_time', now.toString());
          console.log('[Auto-Backup] Successfully created auto-backup at:', backupPath);
        } catch (err) {
          console.error('[Auto-Backup] Failed to run automated backup:', err);
        }
      }
    }, 5000); // Delay execution by 5s after startup to prioritize load time

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
