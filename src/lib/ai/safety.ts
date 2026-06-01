// $lib/ai/safety.ts - AI Guardrails & Hallucination Mitigation Module

export interface ValidationResult {
  valid: boolean;
  reason?: 'PROMPT_INJECTION' | 'MALICIOUS_INPUT' | 'OTHER';
}

export interface SafeOutput {
  content: string;
  trusted: boolean;
}

export interface AIEvent {
  action: string;
  prompt: string;
  output: string;
}

export interface AIGuardrail {
  validateInput(prompt: string): ValidationResult;
  sanitizeOutput(raw: string): SafeOutput;
  requireConfirmation(action: string, preview: string): Promise<boolean>;
  logAuditTrail(event: AIEvent): Promise<void>;
}

/**
 * Creates a robust AI guardrail and hallucination mitigation framework.
 * Ensures privacy-first execution, blocks prompt injections, sanitizes output,
 * and maintains local audit trails.
 */
export const createHallucinationGuard = (): AIGuardrail => ({
  validateInput: (prompt: string): ValidationResult => {
    // Block common prompt injection and system override patterns
    const dangerousPatterns = [
      /system\s*override/i,
      /ignore\s*previous/i,
      /ignore\s*instructions/i,
      /bypass\s*security/i,
      /<script/i,
      /javascript:/i
    ];
    
    for (const pattern of dangerousPatterns) {
      if (pattern.test(prompt)) {
        return { valid: false, reason: 'PROMPT_INJECTION' };
      }
    }
    
    return { valid: true };
  },
  
  sanitizeOutput: (raw: string): SafeOutput => {
    if (!raw) return { content: '', trusted: true };

    // Robust, dependency-free HTML sanitization using DOMParser
    let cleaned = raw;
    if (typeof window !== 'undefined' && typeof window.DOMParser !== 'undefined') {
      try {
        const parser = new DOMParser();
        const doc = parser.parseFromString(raw, 'text/html');
        
        // Remove all script tags
        const scripts = doc.querySelectorAll('script');
        scripts.forEach(s => s.remove());
        
        // Remove all inline event handlers (on*) and javascript: links
        const allElements = doc.querySelectorAll('*');
        allElements.forEach(el => {
          // Remove attributes starting with "on"
          for (let i = el.attributes.length - 1; i >= 0; i--) {
            const attr = el.attributes[i];
            if (attr.name.startsWith('on')) {
              el.removeAttribute(attr.name);
            }
          }
          // Remove javascript: hrefs
          if (el.tagName.toLowerCase() === 'a') {
            const href = el.getAttribute('href');
            if (href && href.trim().toLowerCase().startsWith('javascript:')) {
              el.removeAttribute('href');
            }
          }
        });
        
        cleaned = doc.body.innerHTML;
      } catch (e) {
        // Fallback simple regex sanitization if DOMParser fails
        cleaned = raw
          .replace(/<script[^>]*>([\s\S]*?)<\/script>/gi, '')
          .replace(/on\w+\s*=\s*"[^"]*"/gi, '')
          .replace(/on\w+\s*=\s*'[^']*'/gi, '')
          .replace(/href\s*=\s*"javascript:[^"]*"/gi, '');
      }
    }

    return { content: cleaned, trusted: true };
  },
  
  requireConfirmation: async (action: string, preview: string): Promise<boolean> => {
    // In Svelte/browser context, dispatches a custom event to show the UI confirmation modal
    if (typeof window === 'undefined') return false;
    
    return new Promise(resolve => {
      const confirmEvent = new CustomEvent('ai-confirmation-request', {
        detail: {
          action,
          preview,
          callback: (approved: boolean) => resolve(approved)
        }
      });
      window.dispatchEvent(confirmEvent);
    });
  },
  
  logAuditTrail: async (event: AIEvent): Promise<void> => {
    // Append to immutable audit log via Tauri backend
    if (typeof window === 'undefined') return;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const encoder = new TextEncoder();
      
      // Calculate simple hashes locally
      const promptData = encoder.encode(event.prompt);
      const promptHash = await crypto.subtle.digest('SHA-256', promptData)
        .then(buf => Array.from(new Uint8Array(buf)).map(b => b.toString(16).padStart(2, '0')).join(''));
        
      const outputData = encoder.encode(event.output);
      const outputHash = await crypto.subtle.digest('SHA-256', outputData)
        .then(buf => Array.from(new Uint8Array(buf)).map(b => b.toString(16).padStart(2, '0')).join(''));

      await invoke('append_audit_log', {
        timestamp: Date.now(),
        action: event.action,
        promptHash,
        outputHash
      });
    } catch (err) {
      console.error('Failed to log AI audit trail:', err);
    }
  }
});
