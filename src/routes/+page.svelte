<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import handleTranslate from "./handle.translate";
  import type { TranslateOptions } from "./types";

  let choise = "simple";
  let inputText = "";
  let translatedText = "Translated text will appear here";
  let isLoading = false;
  let language: TranslateOptions = { language: "en" };

  let isTauriAvailable = false;

  // Check if Tauri is available
  try {
    isTauriAvailable = typeof invoke === "function";
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
    await new Promise((resolve) => setTimeout(resolve, 1500));

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
        translatedText: "",
      });
    } catch (error) {
      translatedText = "Translation error occurred";
    } finally {
      isLoading = false;
    }
  }

  // Function to handle Enter in textarea
  async function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.shiftKey) {
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
    <button class:active={choise === "simple"} on:click={() => setTranslationMode("simple")} disabled={isLoading}>
      <text>Simple</text>
    </button>
    <button class:active={choise === "compose"} on:click={() => setTranslationMode("compose")} disabled={isLoading}>
      <text>Compose</text>
    </button>
    <select class="language-select" bind:value={language.language}>
      <option value="en">English</option>
      <option value="es">Spanish</option>
      <option value="pt">Portuguese</option>
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
    <button class="translate-button" on:click={translate} disabled={isLoading || !inputText.trim()}>
      {isLoading ? "Translating..." : "Translate"}
    </button>
  </div>

  <div class="container-output">
    <p class:loading={isLoading}>{translatedText}</p>
  </div>
</main>

<style>
  /* Global Reset - More targeted approach */
  :global(*) {
    box-sizing: border-box;
  }

  :global(html, body) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background: linear-gradient(135deg, #1a1a1a 0%, #2d2d2d 100%);
    color: #ffffff;
    overflow: hidden;
  }

  /* Main Container */
  .container {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
    padding: 1.5rem;
    gap: 1.5rem;
    background: linear-gradient(135deg, #1e1e1e 0%, #2a2a2a 100%);
  }

  /* Header Section */
  .container-title {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem 0;
  }

  .translate-title {
    font-size: 2rem;
    font-weight: 700;
    color: #ffffff;
    text-align: center;
    margin: 0;
    letter-spacing: 0.5px;
    background: linear-gradient(135deg, #64b5f6, #42a5f5);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  /* Controls Section */
  .container-buttons {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 0.5rem 0;
  }

  .container-buttons button {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.1);
    color: #ffffff;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    min-width: 100px;
  }

  .container-buttons button:hover:not(:disabled) {
    background: rgba(100, 181, 246, 0.2);
    border-color: rgba(100, 181, 246, 0.4);
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(100, 181, 246, 0.3);
  }

  .container-buttons button.active {
    background: linear-gradient(135deg, #64b5f6, #42a5f5);
    color: #1a1a1a;
    border-color: transparent;
    box-shadow: 0 4px 20px rgba(100, 181, 246, 0.4);
  }

  .container-buttons button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }

  .container-buttons button text {
    font-size: inherit;
    font-weight: inherit;
    color: inherit;
  }

  .language-select {
    padding: 0.75rem 1rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.1);
    color: #ffffff;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
    min-width: 120px;
  }

  .language-select:hover {
    border-color: rgba(100, 181, 246, 0.4);
    background: rgba(100, 181, 246, 0.1);
  }

  .language-select option {
    background: #2a2a2a;
    color: #ffffff;
    padding: 0.5rem;
  }

  /* Input Section */
  .container-input {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem 0;
  }

  .input-text {
    width: 100%;
    max-width: 800px;
    height: 200px;
    padding: 1.5rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(10px);
    color: #ffffff;
    font-size: 1rem;
    font-family: inherit;
    resize: vertical;
    transition: all 0.3s ease;
    line-height: 1.6;
  }

  .input-text:focus {
    border-color: #64b5f6;
    background: rgba(100, 181, 246, 0.1);
    box-shadow: 0 0 0 3px rgba(100, 181, 246, 0.2);
  }

  .input-text:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .input-text::placeholder {
    color: rgba(255, 255, 255, 0.5);
  }

  /* Translate Button */
  .container-translate-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem 0;
  }

  .translate-button {
    padding: 1rem 2.5rem;
    border: none;
    border-radius: 16px;
    background: linear-gradient(135deg, #4caf50, #45a049);
    color: #ffffff;
    font-size: 1.1rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4px 20px rgba(76, 175, 80, 0.3);
    min-width: 160px;
  }

  .translate-button:hover:not(:disabled) {
    transform: translateY(-3px);
    box-shadow: 0 8px 30px rgba(76, 175, 80, 0.4);
    background: linear-gradient(135deg, #5cbf60, #4caf50);
  }

  .translate-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  /* Output Section */
  .container-output {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    max-width: 800px;
    margin: 0 auto;
    width: 100%;
    padding: 2rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.03);
    backdrop-filter: blur(20px);
    overflow-y: auto;
    min-height: 120px;
  }

  .container-output p {
    margin: 0;
    font-size: 1.1rem;
    line-height: 1.7;
    text-align: center;
    word-wrap: break-word;
    color: rgba(255, 255, 255, 0.9);
    padding: 1rem;
    margin-top: 1rem;
  }

  .container-output p.loading {
    color: rgba(100, 181, 246, 0.8);
    animation: pulse 2s ease-in-out infinite;
  }

  /* Animations */
  @keyframes pulse {
    0%,
    100% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .container {
      padding: 1rem;
      gap: 1rem;
    }

    .translate-title {
      font-size: 1.5rem;
    }

    .container-buttons {
      flex-wrap: wrap;
      gap: 0.75rem;
    }

    .container-buttons button,
    .language-select {
      padding: 0.6rem 1.2rem;
      font-size: 0.9rem;
    }

    .input-text {
      height: 140px;
      padding: 1.2rem;
    }

    .translate-button {
      padding: 0.9rem 2rem;
      font-size: 1rem;
    }

    .container-output {
      padding: 1.5rem;
    }
  }

  @media (max-width: 480px) {
    .container {
      padding: 0.75rem;
    }

    .translate-title {
      font-size: 1.3rem;
    }

    .input-text {
      height: 120px;
      padding: 1rem;
    }

    .container-output p {
      font-size: 1rem;
    }
  }

  /* Scrollbar Styling */
  .container-output::-webkit-scrollbar {
    width: 6px;
  }

  .container-output::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
  }

  .container-output::-webkit-scrollbar-thumb {
    background: rgba(100, 181, 246, 0.5);
    border-radius: 3px;
  }

  .container-output::-webkit-scrollbar-thumb:hover {
    background: rgba(100, 181, 246, 0.7);
  }
</style>
