import { invoke } from '@tauri-apps/api/core';
import customPrompt from './script_en';
import customPromptSp from './script_es';
import customPromptBr from './script_br';

interface Props {
	inputText: string;
	choise: string;
	isTauriAvailable: boolean;
	isLoading: boolean;
	translatedText: string;
	language: 'es' | 'en' | 'pt';
}

async function handleTranslate({
	inputText,
	choise,
	isTauriAvailable,
	isLoading,
	language,
	translatedText,
}: Props): Promise<string> {
	if (!inputText.trim()) {
		alert('Please enter some text to translate');
		return '';
	}

	if (!isTauriAvailable) {
		alert("Tauri API not available. Make sure you're running with 'npm run tauri dev'");
		return '';
	}

	isLoading = true;
	translatedText = 'Translating...';

	try {
		let result: any;

		if (choise === 'simple') {
			console.log('=== CALLING SIMPLE TRANSLATION ===');

			// Teste se a função invoke está realmente disponível
			if (typeof invoke !== 'function') {
				throw new Error('Invoke function is not available');
			}

			result = await invoke('translate_simple', {
				text: inputText,
			});

			translatedText = result;
		} else if (choise === 'compose') {
			console.log('=== CALLING COMPOSE TRANSLATION ===');

			if (typeof invoke !== 'function') {
				throw new Error('Invoke function is not available');
			}

			result = await invoke('translate_with_custom_prompt', {
				text: inputText,
				customScript: language === 'pt' ? customPromptBr : language === 'es' ? customPromptSp : customPrompt,
				//customScript: language === 'es' ? customPromptSp : customPrompt,
			});

			// Parse do JSON retornado pela API do Groq
			let parseResult;
			if (typeof result === 'string') {
				try {
					parseResult = JSON.parse(result);
					console.log('Parsed result:', parseResult);
				} catch (parseError) {
					console.error('JSON parse error:', parseError);
					throw new Error('Failed to parse JSON response');
				}
			} else {
				parseResult = result;
			}

			// Extrai a resposta do formato da API do Groq
			if (parseResult.choices && parseResult.choices[0] && parseResult.choices[0].message) {
				translatedText = parseResult.choices[0].message.content;
			} else {
				console.error('Invalid response structure:', parseResult);
				throw new Error('Invalid response format from Groq API');
			}
		}

		console.log('=== FINAL RESULT ===');
		console.log('Translated text:', translatedText);
		return translatedText;
	} catch (error: any) {
		translatedText = 'Translation failed. Check console for details.';

		// Melhor tratamento de erro
		let errorMessage = 'Translation failed';
		if (typeof error === 'string') {
			errorMessage = error;
		} else if (error.message) {
			errorMessage = error.message;
		}

		return '';
		alert('Translation failed: ' + errorMessage);
	} finally {
		isLoading = false;
		console.log('=== TRANSLATION FINISHED ===');
	}
	return '';
}

export default handleTranslate;
