<script lang="ts">	
	
  import { invoke } from "@tauri-apps/api/core";
  
  let choise = "simple";
  let inputText = "";
  let translatedText = "Translated text will appear here";
  let isLoading = false;
  let callfn = '';
  const customPrompt = `
Imagine that you are an English teacher, with many years of experience, and you love what you do, which is teaching English to the rest of the world. Your student, Gabriel, whenever he is in trouble or has problems, calls you, asking for the translation of a word, or speaking words in Portuguese, so that you can organize the exact sequence of a native English speaker and you simply respond to Gabriel with the translation of what he asked for, without any fuss, without any more random words, just send the correct sequence and the words he asked for.
Gabriel sent the phrase or words:`;

  $: if(choise === "simple") {
    console.log("simple");
    callfn = "translate_simple";
  } else if(choise === "compose") {
    console.log("compose");
    callfn = "translate_with_custom_prompt";
  }

  async function handleTranslate() {
  if (!inputText.trim()) {
    alert("Please enter some text to translate");
    return;
  }

  isLoading = true;

  try {
    let result : any;
    
    if (choise === "simple") {
      // Chama função simples (se existir)
      result = await invoke("translate_simple", {
        text: inputText
      });
    } else {
      // Chama função com prompt customizado  
      result = await invoke("translate_with_custom_prompt", {
        text: inputText,
        customScript: customPrompt  // <- Nome correto do parâmetro
      });
    }
    
    const parseResult = JSON.parse(result);

    translatedText = parseResult.choices[0].message.content;
  } catch (error: any) {
    console.log(error);
    alert("Translation failed: " + error);
  } finally {
    isLoading = false;
  }
}
</script>

<main class="container">  
  <div class="translate-container">
    <h2 class="translate-title">TRANSLATE</h2>    
    <div class="choise-buttons-translate">
      <button 
        class="simple-btn" 
        class:active={choise === "simple"} 
        on:click={() => choise = "simple"}
      >
        Simple translate
      </button>
      <button 
        class="compose-btn" 
        class:active={choise === "compose"} 
        on:click={() => choise = "compose"}
      >
        Compose translate
      </button>
    </div>
    
    <textarea 
      class="input-text" 
      placeholder="Enter text to translate"
      bind:value={inputText}
    ></textarea>     
    
    <button 
      on:click={handleTranslate}
      disabled={isLoading}
    >
      {isLoading ? "Translating..." : "Translate"}
    </button>
    
    <p>{translatedText}</p>
  </div>
</main>

<style>
/* Reset TOTAL - zera absolutamente tudo */
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

.choise-buttons-translate button {
  width: 150px;
  height: 30px;  
  color: #fff;
  background: #555;
  border: 2px solid #666;
  border-radius: 5px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.simple-btn.active {
  border-color: #007bff;
  background: #007bff20;
  color: #007bff;
  box-shadow: 0 0 10px #007bff50;
}

.compose-btn.active {
  border-color: #28a745;
  background: #28a74520;
  color: #28a745;
  box-shadow: 0 0 10px #28a74550;
}

.choise-buttons-translate {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.choise-buttons-translate button {
  width: 150px;
  height: 30px;  
  color: #000;
  border: 2px solid #fff;
}

.choise-buttons-translate button.active {
  border-color: #007bff;
}

.container {
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #000;
}

.translate-container {
  background: #333;
  padding: 40px;
  border-radius: 10px;
  color: white;
  display: flex;
  flex-direction: column;
  gap: 20px;
  min-width: 400px;
  min-height: 500px;
  align-items: center;
  justify-content: space-around;
}

.input-text {
  min-width: 250px;
  min-height: 100px;
  background-color: #555;
  color: #fff;
  border-radius: 10px;
  border: 2px solid #fff;  
  font-size: 16px;
  font-family: inherit;
  padding: 20px;
  resize: vertical;
  overflow-y: auto;
  text-align: center;  
  align-items: center;
  justify-content: center;
  display: flex;
}

.translate-container h2 {
  color: white;
  font-size: 24px;
  margin: 0;
  box-shadow: 0 0 10px 0 #fff;
  border-radius: 10px;
  width: 200px;
  height: 50px;
  text-align: center;
  justify-content: center;
  align-items: center;
  display: flex;
}




.translate-container button {
  background: #007bff;
  color: white;
  border: none;
  border-radius: 5px;
  font-size: 16px;
  cursor: pointer;
  min-width: 100px;
  max-width: 150px;
  min-height: 30px;
  max-height: 50px;

}

.translate-container button:hover {
  background: #0056b3;
}

.translate-container p {
  background: #222;
  padding: 15px;
  border-radius: 5px;
  color: #ccc;
  min-height: 50px;
  text-align: center;
  justify-content: center;
  align-items: center;
  display: flex;
  width: 90%;
  min-height: 50px;
  max-height: 100px;
}
</style>