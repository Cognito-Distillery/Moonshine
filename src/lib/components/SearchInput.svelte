<script lang="ts">
	import { searchStore } from '$lib/stores/search.svelte';
	import type { SearchMode } from '$lib/stores/search.svelte';
	import { graphStore } from '$lib/stores/graph.svelte';
	import * as searchCmd from '$lib/commands/search';
	import { debounce } from '$lib/utils/debounce';
	import { matchKorean } from '$lib/utils/chosung';
	import { showToast } from '$lib/stores/toast.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/ko';

	interface Props {
		onfocus?: (nodeId: string) => void;
	}

	let { onfocus }: Props = $props();
	let showDropdown = $state(false);
	let showModeMenu = $state(false);

	const modeOptions: { value: SearchMode; label: string; tipKey: MessageKey }[] = [
		{ value: 'local', label: 'LK', tipKey: 'topbar.searchTipLocal' },
		{ value: 'keyword', label: 'K', tipKey: 'topbar.searchTipKeyword' },
		{ value: 'natural', label: 'N', tipKey: 'topbar.searchTipNatural' }
	];

	const doLocalSearch = debounce((q: string) => {
		if (!q.trim()) {
			searchStore.setResults([]);
			return;
		}
		const matched = graphStore.nodes.filter(
			(n) =>
				matchKorean(n.summary, q) ||
				matchKorean(n.context, q) ||
				matchKorean(n.memo, q)
		);
		searchStore.setResults(matched);
		graphStore.setSearchMatches(matched.map((n) => n.id));
		showDropdown = matched.length > 0;
	}, 200);

	async function doKeywordSearch(q: string) {
		if (!q.trim()) {
			searchStore.setResults([]);
			return;
		}
		searchStore.setSearching(true);
		try {
			const mashes = await searchCmd.searchKeyword(q);
			const nodes = mashes.map((m) => ({
				id: m.id,
				summary: m.summary,
				context: m.context,
				memo: m.memo,
				type: m.type,
				createdAt: m.createdAt,
				updatedAt: m.updatedAt
			}));
			searchStore.setResults(nodes);
			graphStore.setSearchMatches(nodes.map((n) => n.id));
			showDropdown = nodes.length > 0;
		} catch {
			searchStore.setResults([]);
			showToast(t('error.searchFailed'));
		} finally {
			searchStore.setSearching(false);
		}
	}

	async function doNaturalSearch(q: string) {
		if (!q.trim()) {
			searchStore.setResults([]);
			return;
		}
		searchStore.setSearching(true);
		try {
			const data = await searchCmd.searchSemantic(q);
			const nodes = data.nodes.map((n) => ({
				id: n.id,
				summary: n.summary,
				context: n.context,
				memo: n.memo,
				type: n.mashType,
				createdAt: n.createdAt,
				updatedAt: n.updatedAt
			}));
			searchStore.setResults(nodes);
			graphStore.mergeGraphData({
				nodes,
				edges: data.edges.map((e) => ({
					sourceId: e.sourceId,
					targetId: e.targetId,
					relationType: e.relationType as 'RELATED_TO' | 'SUPPORTS' | 'CONFLICTS_WITH',
					source: e.source as 'ai' | 'human',
					confidence: e.confidence
				}))
			});
			graphStore.setSearchMatches(nodes.map((n) => n.id));
			showDropdown = nodes.length > 0;
		} catch {
			searchStore.setResults([]);
			showToast(t('error.searchFailed'));
		} finally {
			searchStore.setSearching(false);
		}
	}

	function handleInput(e: Event) {
		const val = (e.target as HTMLInputElement).value;
		searchStore.setQuery(val);
		showDropdown = true;
		if (!val.trim()) {
			graphStore.clearSearchMatches();
		}
		if (searchStore.mode === 'local') {
			doLocalSearch(val);
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (searchStore.mode !== 'local' && e.key === 'Enter') {
			e.preventDefault();
			if (searchStore.mode === 'keyword') {
				doKeywordSearch(searchStore.query);
			} else {
				doNaturalSearch(searchStore.query);
			}
		}
	}

	function selectResult(nodeId: string) {
		graphStore.selectNode(nodeId);
		graphStore.setSearchMatches([nodeId]);
		showDropdown = false;
		searchStore.setQuery('');
		onfocus?.(nodeId);
	}

	function selectMode(mode: SearchMode) {
		searchStore.setMode(mode);
		searchStore.clear();
		graphStore.clearSearchMatches();
		showDropdown = false;
		showModeMenu = false;
	}

	let placeholder = $derived(
		searchStore.mode === 'local'
			? t('topbar.search')
			: searchStore.mode === 'keyword'
				? t('topbar.searchServer')
				: t('topbar.searchNatural')
	);

	let currentModeLabel = $derived(
		modeOptions.find((m) => m.value === searchStore.mode)!.label
	);
</script>

<div class="relative">
	<div class="flex items-center bg-black/60 backdrop-blur-sm border border-white/10 rounded-lg focus-within:border-primary/50 transition-colors">
		<!-- Mode dropdown trigger -->
		<div class="relative">
			<button
				class="shrink-0 px-2 py-1 text-[10px] font-mono font-bold text-primary hover:text-primary/80 transition-colors"
				onclick={() => (showModeMenu = !showModeMenu)}
				onblur={() => setTimeout(() => (showModeMenu = false), 150)}
			>
				{currentModeLabel} ▾
			</button>
			{#if showModeMenu}
				<ul class="absolute top-full left-0 mt-1 bg-black/90 backdrop-blur-md rounded-lg shadow-xl border border-white/[0.08] z-50 min-w-max p-1">
					{#each modeOptions as opt}
						<li>
							<button
								class="w-full text-left text-[11px] font-mono px-3 py-1.5 rounded transition-colors
									{searchStore.mode === opt.value ? 'text-primary bg-white/[0.08]' : 'text-base-content/60 hover:bg-white/[0.06]'}"
								onmousedown={() => selectMode(opt.value)}
								title={t(opt.tipKey)}
							>
								{opt.label}
							</button>
						</li>
					{/each}
				</ul>
			{/if}
		</div>
		<input
			type="text"
			{placeholder}
			class="bg-transparent border-none outline-none w-44 py-1 pr-2 text-xs placeholder:text-base-content/20"
			value={searchStore.query}
			oninput={handleInput}
			onkeydown={handleKeydown}
			onfocus={() => (showDropdown = searchStore.results.length > 0)}
			onblur={() => setTimeout(() => (showDropdown = false), 200)}
		/>
		{#if searchStore.isSearching}
			<span class="loading loading-spinner loading-xs text-primary mr-2"></span>
		{/if}
	</div>
	{#if showDropdown && searchStore.results.length > 0}
		<ul class="bg-black/80 backdrop-blur-md rounded-lg shadow-xl border border-white/[0.08] absolute top-full mt-1 w-full z-50 max-h-60 overflow-y-auto p-1">
			{#each searchStore.results as node}
				<li>
					<button
						class="w-full text-left text-xs px-2 py-1.5 rounded hover:bg-white/[0.08] flex items-center gap-1.5"
						onclick={() => selectResult(node.id)}
					>
						<span class="text-[10px] text-base-content/30">{node.type}</span>
						<span class="text-base-content/70">{node.summary}</span>
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>
