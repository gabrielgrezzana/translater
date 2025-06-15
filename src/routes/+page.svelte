<script lang="ts">	
  
  import { invoke } from '@tauri-apps/api/core';
  import handleTranslate from './handle.translate';
  import type { TranslateOptions } from './types';

  
  let choise = "simple";
  let inputText = "";
  let translatedText = "Translated text will appear here";
  let isLoading = false;
  let language: TranslateOptions = {language: "en"};
  
  let isTauriAvailable = false;
  
  // Check if Tauri is available
  try {
    isTauriAvailable = typeof invoke === 'function';
  } catch {
    isTauriAvailable = false;
  }


  // Function to switch translation mode
  function setTranslationMode(mode: string) {
    choise = mode;
    console.log(`Mode changed to: ${mode}`);
  }

  // MOCK function for browser testing
  async function mockTranslation(text: string, mode: string): Promise<string> {
    // Simulates API delay
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    if (mode === "simple") {
      return `[MOCK] Simple translation of: "${text}"`;
    } else {
      return `[MOCK] Compose translation of: "${text}" - As an English teacher, I would say this translates to: [translated version]`;
    }
  }

  // Main translation function
  

  async function translate() {
  if (!inputText.trim()) return;
  
  isLoading = true;
  try {
    translatedText = await handleTranslate({
      inputText, 
      choise, 
      isTauriAvailable,
      isLoading,
      language: language.language,
      translatedText: ""
    });
  } catch (error) {
    translatedText = "Translation error occurred";
  } finally {
    isLoading = false;
  }
}





// Function to handle Enter in textarea
async function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    await translate();
  }
}

</script>

<main class="container">  
  <div class="container-title">
    <h2 class="translate-title">TRANSLATE</h2>
  </div>
  
  <div class="container-buttons">
    <button 
      class:active={choise === 'simple'}
      on:click={() => setTranslationMode('simple')}
      disabled={isLoading}
    >
      Simple
    </button>
    <button 
      class:active={choise === 'compose'}
      on:click={() => setTranslationMode('compose')}
      disabled={isLoading}
    >
      Compose
    </button>
    <select class="language-select" bind:value={language.language}>
      <option value="en">English</option>
      <option value="es">Spanish</option>
    </select>
  </div>
  
  <div class="container-input">
    <textarea 
      class="input-text" 
      placeholder="Enter text to translate" 
      bind:value={inputText}
      on:keydown={handleKeydown}
      disabled={isLoading}
    ></textarea>
  </div>
  
  <div class="container-translate-button">
    <button 
      class="translate-button"
      on:click={translate}
      disabled={isLoading || !inputText.trim()}
    >
      {isLoading ? 'Translating...' : 'Translate'}
    </button>
  </div>
  
  <div class="container-output">
    <p class:loading={isLoading}>{translatedText}</p>
  </div>
</main>

<style>
:global(*) {
  margin: 0 !important;
  padding: 0 !important;
  box-sizing: border-box !important;
  border: none !important;
  outline: none !important;
}

:global(html) {
  margin: 0 !important;
  padding: 0 !important;
  width: 100% !important;
  height: 100% !important;
}

:global(body) {
  margin: 0 !important;
  padding: 0 !important;
  width: 100vw !important;
  height: 100vh !important;
  background: #000 !important;
}

.container {
  flex: 1;
  display: flex;  
  background: #323131;
  width: 100%;
  height: 100%;    
  flex-direction: column;
  
}

.container-title {
  width: 100%;
  height: 10%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.translate-title {
  color: #fff;
  font-size: 24px;
  margin: 0;  
  width: 40%;  
  border-radius: 10px;
  height: 3rem;  
  display: flex;
  align-items: center;
  justify-content: center;
}

.container-buttons {
  width: 100%;
  height: 8%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;  
}

.container-buttons button {
  width: 7rem;
  height: 2rem;
  border-radius: 10px;
  background-color: #15b6d6e2;
  font-size: 1rem;
  font-weight: bold;
  box-shadow: 0 0 20px 0 #15b6d6e2;
  transition: all 0.3s ease;
}

.container-buttons button:hover:not(:disabled) {
  cursor: pointer;
  color: #fff;
  box-shadow: 0 0 20px 0 #fff;
}

.container-buttons button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.container-buttons button.active {
  background-color: #fff;
  color: #323131;
  box-shadow: 0 0 25px 0 #fff;
}

.container-input {
  width: 100%;
  height: 35%;
  display: flex;
  align-items: center;
  justify-content: center;  
}

.container-input textarea {
  width: 90%;
  height: 80%;
  background: #0d313ef6;
  border-radius: 10px;
  border: 2px solid #fff;
  font-size: 1rem;
  font-weight: bold;
  color: #fff;
  border-color: #fff;
  box-shadow: 0 0 15px 0 #15b6d6e2;
  resize: none;
  min-width: 200px;
  max-width: 90%;
  min-height: 100px;
  max-height: 250px;
  text-align: center;
  align-items: center;
  justify-content: center;
}

.container-input textarea:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.container-input textarea::placeholder {
  color: #bbb;
  text-align: center;
}

.container-translate-button {
  width: 100%;
  height: 8%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.translate-button {
  width: 10rem;
  height: 2.5rem;
  border-radius: 10px;
  background-color: #14d7a7e2;
  font-size: 1.1rem;
  font-weight: bold;
  color: #fff;
  box-shadow: 0 0 20px 0 #14d7a7e2;
  transition: all 0.3s ease;
}

.translate-button:hover:not(:disabled) {
  cursor: pointer;
  box-shadow: 0 0 25px 0 #14d7a7;
  transform: translateY(-2px);
}

.translate-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

.container-output {
  align-self: center;
  width: 95%;
  height: 30%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1f1f1f;
  border-radius: 10px;
  box-shadow: 0 0 15px 0 #14d7a7e2;
  color: #fff;
  font-size: 1.2rem;
  font-weight: bold;
  text-align: center;
  padding: 20px;
  min-width: 200px;
  max-width: 90%;
  overflow-y: auto;
}

.container-output p {
  word-wrap: break-word;
  line-height: 1.4;
}

.container-output p.loading {
  opacity: 0.7;
  animation: pulse 2s infinite;
}

.language-select {
  width: 10rem;
  height: 2rem;
  border-radius: 10px;
  background-color: #14d7a7e2;
  font-size: 1.1rem;
  font-weight: bold;
  color: #fff;
}

@keyframes pulse {
  0%, 100% { opacity: 0.7; }
  50% { opacity: 1; }
}
</style>