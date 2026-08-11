<script lang="ts">
  import { invoke, Channel } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  interface JournalPayload {
    markdownContent: string;
    moodScore: number;
    energyLevel: string;
    wins: string[];
    frictions: string;
    intentions: string;
    timestamp: string;
  }

  let markdownContent: string = "# Daily Reflection\n\nStarted working on Sottum today. Feeling focused...";
  let moodScore: number = 7;
  let energyLevel: string = "High";
  let wins: string[] = ["Completed initial setup", ""];
  let frictions: string = "";
  let intentions: string = "";

  let saveStatus: string = "";
  let historicalEntries: JournalPayload[] = [];
  let showHistory: boolean = false;
  let isPreviewMode: boolean = false;

  let aiInsight: string = "";
  let isAnalyzing: boolean = false;
  let aiError: string = "";

  onMount(async () => {
    await fetchHistory();
  });

  async function fetchHistory() {
    try {
      historicalEntries = await invoke<JournalPayload[]>('get_journal_entries');
    } catch (err) {
      console.error("Failed to load entries:", err);
    }
  }

  function loadEntryIntoEditor(entry: JournalPayload) {
    markdownContent = entry.markdownContent;
    moodScore = entry.moodScore;
    energyLevel = entry.energyLevel;
    wins = entry.wins.length > 0 ? [...entry.wins] : [""];
    frictions = entry.frictions;
    intentions = entry.intentions;
    aiInsight = "";
    aiError = "";
    showHistory = false;
  }

  function addWin() {
    wins = [...wins, ""];
  }

  function removeWin(index: number) {
    if (wins.length > 1) {
      wins = wins.filter((_, i) => i !== index);
    }
  }

  async function handleAnalyze() {
    if (!markdownContent.trim() && !frictions.trim()) return;

    isAnalyzing = true;
    aiError = "";
    aiInsight = "";

    const promptPayload = `
Journal Entry:
${markdownContent}

Key Wins:
${wins.filter(w => w.trim() !== "").join("\n- ") || "None listed"}

Frictions / Drains:
${frictions || "None listed"}
    `.trim();

    const onToken = new Channel<string>();
    onToken.onmessage = (token: string) => {
      aiInsight += token;
    };

    try {
      await invoke('analyze_journal_entry_stream', {
        content: promptPayload,
        onToken
      });
    } catch (err) {
      console.error("SLM Stream Error:", err);
      aiError = `Inference failed: ${err}`;
    } finally {
      isAnalyzing = false;
    }
  }

  async function handleSave() {
    const payload: JournalPayload = {
      markdownContent,
      moodScore,
      energyLevel,
      wins: wins.filter(w => w.trim() !== ""),
      frictions,
      intentions,
      timestamp: new Date().toISOString()
    };

    try {
      const response = await invoke<string>('save_journal_entry', { payload });
      saveStatus = response;
      await fetchHistory();
      setTimeout(() => { saveStatus = ""; }, 4000);
    } catch (err) {
      console.error("Save error:", err);
      saveStatus = `Error saving file: ${err}`;
    }
  }

  $: wordCount = markdownContent.trim().split(/\s+/).filter(Boolean).length;
</script>

<div class="top-bar">
  <button class="nav-btn" on:click={() => showHistory = !showHistory}>
    {showHistory ? 'Close History' : `History (${historicalEntries.length})`}
  </button>
</div>

{#if showHistory}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="drawer-backdrop" on:click={() => showHistory = false}></div>
  <div class="history-drawer">
    <h3>Past Entries</h3>
    {#if historicalEntries.length === 0}
      <p class="empty-text">No saved entries found.</p>
    {:else}
      <div class="history-list">
        {#each historicalEntries as item, index (item.timestamp || index)}
          <button class="history-card" on:click={() => loadEntryIntoEditor(item)}>
            <div class="card-header">
              <span class="card-date">{new Date(item.timestamp).toLocaleString()}</span>
              <span class="card-mood">Mood: {item.moodScore}/10</span>
            </div>
            <p class="card-snippet">
              {item.markdownContent.replace(/^#+\s*/, '').slice(0, 80)}...
            </p>
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<main class="app-container">
  <section class="panel editor-panel">
    <div class="panel-header">
      <h2>Brain Dump (Markdown)</h2>
      <div class="header-actions">
        <button 
          class="toggle-btn" 
          on:click={() => isPreviewMode = !isPreviewMode}
        >
          {isPreviewMode ? 'Edit' : 'Preview'}
        </button>
        <span class="word-count">{wordCount} words</span>
      </div>
    </div>

    {#if isPreviewMode}
      <div class="markdown-preview">
        <pre class="preview-text">{markdownContent}</pre>
      </div>
    {:else}
      <textarea 
        class="markdown-input" 
        bind:value={markdownContent} 
        placeholder="Write freely..."
      ></textarea>
    {/if}
  </section>

  <section class="panel dashboard-panel">
    <div class="panel-header">
      <h2>Daily Dashboard</h2>
      <span class="date-badge">{new Date().toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}</span>
    </div>

    <div class="metrics-grid">
      <div class="field-group">
        <label for="mood">Mood Rating: <strong>{moodScore}/10</strong></label>
        <input id="mood" type="range" min="1" max="10" bind:value={moodScore} class="slider" />
      </div>

      <div class="field-group">
        <label for="energy">Energy Level</label>
        <select id="energy" bind:value={energyLevel} class="select-input">
          <option value="Low">Low</option>
          <option value="Medium">Medium</option>
          <option value="High">High</option>
          <option value="Peak">Peak</option>
        </select>
      </div>
    </div>

    <div class="field-group">
      <div class="label-row">
        <label for="win-0">Wins & Progress</label>
        <button type="button" class="text-btn" on:click={addWin}>+ Add Win</button>
      </div>
      {#each wins as win, index (index)}
        <div class="input-row">
          <input 
            id={`win-${index}`}
            type="text" 
            bind:value={wins[index]} 
            placeholder={`Win #${index + 1}`} 
            class="text-input" 
          />
          {#if wins.length > 1}
            <button type="button" class="icon-btn" on:click={() => removeWin(index)}>✕</button>
          {/if}
        </div>
      {/each}
    </div>

    <div class="field-group">
      <label for="friction">Friction & Roadblocks</label>
      <textarea 
        id="friction"
        bind:value={frictions} 
        placeholder="What drained your energy today?" 
        class="textarea-small"
      ></textarea>
    </div>

    <div class="field-group">
      <label for="intentions">Tomorrow's Focus</label>
      <input 
        id="intentions"
        type="text" 
        bind:value={intentions} 
        placeholder="Key priority..." 
        class="text-input" 
      />
    </div>

    <div class="ai-section">
      <button 
        class="ai-btn" 
        on:click={handleAnalyze} 
        disabled={isAnalyzing || (!markdownContent.trim() && !frictions.trim())}
      >
        {isAnalyzing ? "🧠 Running Native Inference..." : "✨ Analyze with Local SLM"}
      </button>

      {#if aiError}
        <div class="ai-box ai-error">
          <p>{aiError}</p>
        </div>
      {/if}

      {#if aiInsight || isAnalyzing}
        <div class="ai-box ai-insight">
          <div class="ai-box-header">
            <span>🧠 Local Brain Reflection</span>
          </div>
          <p class="ai-text">
            {aiInsight}<span class:blinking={isAnalyzing}>▌</span>
          </p>
        </div>
      {/if}
    </div>

    {#if saveStatus}
      <p class="status-msg">{saveStatus}</p>
    {/if}

    <button class="save-btn" on:click={handleSave}>
      Save Entry
    </button>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background-color: #121214;
    color: #e4e4e7;
    height: 100vh;
    overflow: hidden;
  }

  .top-bar {
    padding: 8px 16px;
    background: #18181b;
    border-bottom: 1px solid #27272a;
  }

  .nav-btn {
    background: #27272a;
    color: #e4e4e7;
    border: none;
    padding: 6px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .drawer-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.4);
    z-index: 90;
  }

  .history-drawer {
    position: absolute;
    top: 40px;
    left: 16px;
    z-index: 100;
    width: 320px;
    max-height: 80vh;
    background: #18181b;
    border: 1px solid #27272a;
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 10px 25px rgba(0,0,0,0.5);
    overflow-y: auto;
  }

  .history-drawer h3 {
    margin-top: 0;
    font-size: 1rem;
    color: #f4f4f5;
  }

  .history-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .history-card {
    background: #09090b;
    border: 1px solid #27272a;
    border-radius: 6px;
    padding: 10px;
    text-align: left;
    cursor: pointer;
    color: #e4e4e7;
  }

  .history-card:hover {
    border-color: #6366f1;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: #a1a1aa;
    margin-bottom: 6px;
  }

  .card-snippet {
    margin: 0;
    font-size: 0.85rem;
    color: #d4d4d8;
  }

  .empty-text {
    font-size: 0.85rem;
    color: #71717a;
  }

  .app-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    height: calc(100vh - 40px);
    padding: 16px;
    box-sizing: border-box;
  }

  .panel {
    background: #18181b;
    border: 1px solid #27272a;
    border-radius: 12px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #27272a;
    padding-bottom: 12px;
  }

  .panel-header h2 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toggle-btn {
    background: #27272a;
    color: #a1a1aa;
    border: none;
    padding: 4px 8px;
    border-radius: 6px;
    font-size: 0.8rem;
    cursor: pointer;
  }

  .word-count, .date-badge {
    font-size: 0.8rem;
    color: #a1a1aa;
    background: #27272a;
    padding: 4px 8px;
    border-radius: 6px;
  }

  .editor-panel {
    height: 100%;
  }

  .markdown-input, .markdown-preview {
    flex: 1;
    background: #09090b;
    border: 1px solid #27272a;
    border-radius: 8px;
    padding: 14px;
    color: #e4e4e7;
    font-family: monospace;
    font-size: 0.95rem;
    line-height: 1.6;
    resize: none;
    outline: none;
    overflow-y: auto;
  }

  .preview-text {
    margin: 0;
    white-space: pre-wrap;
    font-family: inherit;
  }

  .dashboard-panel {
    overflow-y: auto;
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  label {
    font-size: 0.85rem;
    font-weight: 500;
    color: #a1a1aa;
  }

  .label-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .text-input, .select-input, .textarea-small {
    background: #09090b;
    border: 1px solid #27272a;
    border-radius: 6px;
    padding: 10px;
    color: #f4f4f5;
    font-size: 0.9rem;
    outline: none;
  }

  .textarea-small {
    height: 60px;
    resize: none;
  }

  .input-row {
    display: flex;
    gap: 8px;
  }

  .input-row .text-input {
    flex: 1;
  }

  .slider {
    accent-color: #6366f1;
  }

  .text-btn {
    background: none;
    border: none;
    color: #818cf8;
    font-size: 0.8rem;
    cursor: pointer;
  }

  .icon-btn {
    background: #27272a;
    border: none;
    color: #a1a1aa;
    border-radius: 6px;
    padding: 0 10px;
    cursor: pointer;
  }

  .ai-section {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-top: 4px;
  }

  .ai-btn {
    background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
    color: white;
    border: none;
    padding: 10px;
    border-radius: 8px;
    font-weight: 600;
    font-size: 0.88rem;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .ai-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .ai-box {
    border-radius: 8px;
    padding: 12px 14px;
    font-size: 0.85rem;
    line-height: 1.5;
  }

  .ai-insight {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-left: 3px solid #818cf8;
  }

  .ai-box-header {
    font-weight: 600;
    color: #a5b4fc;
    margin-bottom: 6px;
    font-size: 0.8rem;
  }

  .ai-text {
    margin: 0;
    color: #cbd5e1;
    white-space: pre-wrap;
  }

  .blinking {
    display: inline-block;
    margin-left: 2px;
    color: #818cf8;
    animation: blink 1s steps(2, start) infinite;
  }

  @keyframes blink {
    to {
      visibility: hidden;
    }
  }

  .ai-error {
    background: #2a1215;
    border: 1px solid #451a1d;
    color: #f87171;
  }

  .status-msg {
    font-size: 0.8rem;
    color: #34d399;
    margin: 0;
    text-align: center;
  }

  .save-btn {
    margin-top: auto;
    background: #27272a;
    color: #e4e4e7;
    border: 1px solid #3f3f46;
    padding: 12px;
    border-radius: 8px;
    font-weight: 600;
    font-size: 0.95rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .save-btn:hover {
    background: #3f3f46;
  }
</style>