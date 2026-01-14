(function() {
  'use strict';

  const DeepResearch = {
    state: {
      isRunning: false,
      todoId: null,
      startedAt: null,
      observer: null
    },

    selectors: {
      inputBox: '#prompt-textarea, [data-testid="text-input"], textarea[placeholder*="Message"]',
      sendButton: 'button[data-testid="send-button"], button[aria-label*="Send"]',
      deepResearchButton: '[data-testid="deep-research-button"], button:has-text("Deep Research"), [aria-label*="Deep Research"]',
      responseContainer: '[data-message-author-role="assistant"]',
      thinkingIndicator: '[data-testid="thinking"], .result-thinking'
    },

    async start(todoId, prompt) {
      if (this.state.isRunning) {
        console.log('[DeepResearch] Already running');
        return { success: false, error: 'Research already in progress' };
      }

      this.state.isRunning = true;
      this.state.todoId = todoId;
      this.state.startedAt = new Date().toISOString();

      try {
        if (!this.isLoggedIn()) {
          throw new Error('Not logged in to ChatGPT');
        }

        await this.injectPrompt(prompt);
        await this.triggerDeepResearch();
        this.startObserving();

        return { success: true, startedAt: this.state.startedAt };
      } catch (error) {
        this.reset();
        return { success: false, error: error.message };
      }
    },

    cancel() {
      this.stopObserving();
      this.reset();
      return { success: true };
    },

    getStatus() {
      return {
        isRunning: this.state.isRunning,
        todoId: this.state.todoId,
        startedAt: this.state.startedAt
      };
    },

    isLoggedIn() {
      const loginButton = document.querySelector('[data-testid="login-button"]');
      const userMenu = document.querySelector('[data-testid="user-menu"], [aria-label*="Open menu"]');
      return !loginButton && !!userMenu;
    },

    async injectPrompt(prompt) {
      const input = await this.waitForElement(this.selectors.inputBox, 5000);
      if (!input) {
        throw new Error('Could not find input box');
      }

      input.focus();
      input.value = prompt;
      
      const inputEvent = new Event('input', { bubbles: true, cancelable: true });
      input.dispatchEvent(inputEvent);
      
      const changeEvent = new Event('change', { bubbles: true });
      input.dispatchEvent(changeEvent);

      await this.sleep(200);
    },

    async triggerDeepResearch() {
      let deepResearchBtn = document.querySelector(this.selectors.deepResearchButton);
      
      if (!deepResearchBtn) {
        const buttons = document.querySelectorAll('button');
        for (const btn of buttons) {
          const text = btn.textContent?.toLowerCase() || '';
          const ariaLabel = btn.getAttribute('aria-label')?.toLowerCase() || '';
          if (text.includes('deep research') || text.includes('research') || 
              ariaLabel.includes('deep research') || ariaLabel.includes('research')) {
            deepResearchBtn = btn;
            break;
          }
        }
      }

      if (deepResearchBtn) {
        deepResearchBtn.click();
        await this.sleep(500);
      }

      const sendButton = await this.waitForElement(this.selectors.sendButton, 3000);
      if (sendButton && !sendButton.disabled) {
        sendButton.click();
      } else {
        const enterEvent = new KeyboardEvent('keydown', {
          key: 'Enter',
          code: 'Enter',
          keyCode: 13,
          which: 13,
          bubbles: true
        });
        document.querySelector(this.selectors.inputBox)?.dispatchEvent(enterEvent);
      }
    },

    startObserving() {
      this.stopObserving();

      const targetNode = document.body;
      const config = { childList: true, subtree: true, characterData: true };

      let lastContent = '';
      let stableCount = 0;
      const STABILITY_THRESHOLD = 5;

      this.state.observer = new MutationObserver((mutations) => {
        if (!this.state.isRunning) return;

        const responseElements = document.querySelectorAll(this.selectors.responseContainer);
        const lastResponse = responseElements[responseElements.length - 1];
        
        if (!lastResponse) return;

        const thinking = document.querySelector(this.selectors.thinkingIndicator);
        if (thinking) {
          stableCount = 0;
          return;
        }

        const currentContent = lastResponse.textContent || '';
        
        if (currentContent === lastContent) {
          stableCount++;
          if (stableCount >= STABILITY_THRESHOLD && currentContent.length > 100) {
            this.handleCompletion(lastResponse);
          }
        } else {
          stableCount = 0;
          lastContent = currentContent;
        }
      });

      this.state.observer.observe(targetNode, config);
    },

    stopObserving() {
      if (this.state.observer) {
        this.state.observer.disconnect();
        this.state.observer = null;
      }
    },

    handleCompletion(responseElement) {
      this.stopObserving();

      const content = this.extractMarkdown(responseElement);
      const citations = this.extractCitations(responseElement);

      if (window.__TAURI__) {
        window.__TAURI__.event.emit('research_complete', {
          todoId: this.state.todoId,
          content: content,
          citations: citations,
          source: 'ChatGPT Deep Research',
          startedAt: this.state.startedAt
        });
      }

      this.reset();
    },

    extractMarkdown(element) {
      let markdown = '';
      
      const walk = (node) => {
        if (node.nodeType === Node.TEXT_NODE) {
          markdown += node.textContent;
          return;
        }

        if (node.nodeType !== Node.ELEMENT_NODE) return;

        const tag = node.tagName.toLowerCase();

        switch (tag) {
          case 'h1': markdown += '# '; break;
          case 'h2': markdown += '## '; break;
          case 'h3': markdown += '### '; break;
          case 'h4': markdown += '#### '; break;
          case 'p': markdown += '\n\n'; break;
          case 'br': markdown += '\n'; break;
          case 'strong':
          case 'b': markdown += '**'; break;
          case 'em':
          case 'i': markdown += '*'; break;
          case 'code':
            if (node.parentElement?.tagName.toLowerCase() === 'pre') {
              const lang = node.className?.match(/language-(\w+)/)?.[1] || '';
              markdown += '\n```' + lang + '\n';
            } else {
              markdown += '`';
            }
            break;
          case 'li': 
            const isOrdered = node.parentElement?.tagName.toLowerCase() === 'ol';
            markdown += isOrdered ? '1. ' : '- ';
            break;
          case 'a':
            markdown += '[';
            break;
        }

        for (const child of node.childNodes) {
          walk(child);
        }

        switch (tag) {
          case 'h1':
          case 'h2':
          case 'h3':
          case 'h4':
          case 'p': markdown += '\n'; break;
          case 'strong':
          case 'b': markdown += '**'; break;
          case 'em':
          case 'i': markdown += '*'; break;
          case 'code':
            if (node.parentElement?.tagName.toLowerCase() === 'pre') {
              markdown += '\n```\n';
            } else {
              markdown += '`';
            }
            break;
          case 'li': markdown += '\n'; break;
          case 'a':
            const href = node.getAttribute('href');
            markdown += `](${href})`;
            break;
        }
      };

      walk(element);
      return markdown.trim();
    },

    extractCitations(element) {
      const citations = [];
      const links = element.querySelectorAll('a[href]');
      
      for (const link of links) {
        const href = link.getAttribute('href');
        const text = link.textContent;
        if (href && !href.startsWith('#') && !href.startsWith('javascript:')) {
          citations.push({ url: href, title: text });
        }
      }
      
      return citations;
    },

    reset() {
      this.state.isRunning = false;
      this.state.todoId = null;
      this.state.startedAt = null;
    },

    async waitForElement(selector, timeout = 10000) {
      const startTime = Date.now();
      while (Date.now() - startTime < timeout) {
        const element = document.querySelector(selector);
        if (element) return element;
        await this.sleep(100);
      }
      return null;
    },

    sleep(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }
  };

  window.DeepResearch = DeepResearch;
  console.log('[DeepResearch] Automation script loaded');
})();
