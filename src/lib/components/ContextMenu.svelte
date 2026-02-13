<script lang="ts">
	import type { ContextMenuState } from '$lib/graph/cytoscape-events';
	import { getMenuLabel } from '$lib/graph/context-menu';

	interface Props {
		state: ContextMenuState;
		onaction: (action: string) => void;
		onclose: () => void;
	}

	let { state, onaction, onclose }: Props = $props();

	function handleClick(action: string) {
		onaction(action);
		onclose();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onclose();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if state.visible}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="fixed inset-0 z-40" onclick={onclose}></div>
	<ul
		class="menu bg-base-200 rounded-box shadow-xl z-50 absolute min-w-[160px] p-1"
		style="left: {state.x}px; top: {state.y}px;"
	>
		{#each state.items as item}
			<li>
				<button class="text-sm" onclick={() => handleClick(item.action)}>
					{getMenuLabel(item)}
				</button>
			</li>
		{/each}
	</ul>
{/if}
