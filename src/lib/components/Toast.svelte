<script lang="ts">
  import { toasts } from '$lib/stores/app';
  import { X } from 'lucide-svelte';
</script>

<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <div class="toast toast-{toast.type}">
      <span>{toast.message}</span>
      <button class="toast-close" onclick={() => toasts.update(t => t.filter(x => x.id !== toast.id))}>
        <X size={14} />
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 60px;
    right: 16px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .toast {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 13px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    animation: slideIn 0.2s ease;
  }
  @keyframes slideIn {
    from { transform: translateX(100%); opacity: 0; }
    to { transform: translateX(0); opacity: 1; }
  }
  .toast-error {
    background: #4a2020;
    border: 1px solid #ff6b6b;
    color: #ffcccc;
  }
  .toast-success {
    background: #1a4a2e;
    border: 1px solid #3ecf8e;
    color: #ccffeb;
  }
  .toast-info {
    background: var(--surface-glass, #2a2a2a);
    border: 1px solid var(--border-standard, #444);
    color: var(--text-primary, #fff);
  }
  .toast-close {
    background: none;
    border: none;
    cursor: pointer;
    opacity: 0.6;
    display: flex;
    padding: 2px;
  }
  .toast-close:hover { opacity: 1; }
</style>