import { invoke } from '@tauri-apps/api/core';

export function getSetting(key: string): Promise<string | null> {
	return invoke<string | null>('get_setting', { key });
}

export function setSetting(key: string, value: string): Promise<void> {
	return invoke('set_setting', { key, value });
}

export function getAllSettings(): Promise<[string, string][]> {
	return invoke<[string, string][]>('get_all_settings');
}

export function switchEmbeddingProvider(provider: string): Promise<number> {
	return invoke<number>('switch_embedding_provider', { provider });
}

export function switchEmbeddingModel(model: string): Promise<number> {
	return invoke<number>('switch_embedding_model', { model });
}

export function switchChatModel(model: string): Promise<void> {
	return invoke('switch_chat_model', { model });
}

export function reextractRelationships(): Promise<number> {
	return invoke<number>('reextract_relationships');
}

export function reembedAll(): Promise<number> {
	return invoke<number>('reembed_all');
}
