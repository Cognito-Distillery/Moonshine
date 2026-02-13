import { invoke } from '@tauri-apps/api/core';
import type { Mash } from '$lib/types';
import type { GraphData } from '$lib/commands/graph';

export interface RecentSearch {
	id: number;
	query: string;
	resultCount: number;
	createdAt: number;
}

export function searchKeyword(query: string): Promise<Mash[]> {
	return invoke<Mash[]>('search_keyword', { query });
}

export function searchSemantic(query: string): Promise<GraphData> {
	return invoke<GraphData>('search_semantic', { query });
}

export function getRecentSearches(): Promise<RecentSearch[]> {
	return invoke<RecentSearch[]>('get_recent_searches');
}

export function replayCachedSearch(cacheId: number): Promise<GraphData> {
	return invoke<GraphData>('replay_cached_search', { cacheId });
}

export function deleteCachedSearch(cacheId: number): Promise<void> {
	return invoke('delete_cached_search', { cacheId });
}
