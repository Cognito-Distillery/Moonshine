import type { GraphNode } from '$lib/types/graph';

export type SearchMode = 'local' | 'keyword' | 'natural';

function createSearchStore() {
	let query = $state('');
	let results = $state<GraphNode[]>([]);
	let isSearching = $state(false);
	let mode = $state<SearchMode>('local');

	return {
		get query() { return query; },
		get results() { return results; },
		get isSearching() { return isSearching; },
		get mode() { return mode; },

		setQuery(q: string) { query = q; },
		setResults(r: GraphNode[]) { results = r; },
		setSearching(val: boolean) { isSearching = val; },
		setMode(m: SearchMode) { mode = m; },
		toggleMode() {
			const modes: SearchMode[] = ['local', 'keyword', 'natural'];
			mode = modes[(modes.indexOf(mode) + 1) % modes.length];
		},
		clear() {
			query = '';
			results = [];
		}
	};
}

export const searchStore = createSearchStore();
