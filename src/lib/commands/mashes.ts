import { invoke } from '@tauri-apps/api/core';
import type { Mash, MashType } from '$lib/types';

export function getMashesByStatus(status: string, query?: string): Promise<Mash[]> {
	return invoke<Mash[]>('get_mashes_by_status', { status, query });
}

export function addMash(mashType: MashType, summary: string, context: string, memo: string): Promise<Mash> {
	return invoke<Mash>('add_mash', { mashType, summary, context, memo });
}

export function deleteMash(id: string): Promise<void> {
	return invoke('delete_mash', { id });
}

export function updateMash(
	id: string,
	mashType?: MashType,
	summary?: string,
	context?: string,
	memo?: string
): Promise<Mash> {
	return invoke<Mash>('update_mash', { id, mashType, summary, context, memo });
}

export function setMashStatus(id: string, status: string): Promise<void> {
	return invoke('set_mash_status', { id, status });
}

export function searchMashes(query: string): Promise<Mash[]> {
	return invoke<Mash[]>('search_keyword', { query });
}

export function addMashWithAI(text: string): Promise<Mash> {
	return invoke<Mash>('add_mash_with_ai', { text });
}
