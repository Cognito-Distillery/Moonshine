<script lang="ts">
	import '../app.css';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import LoginScreen from '$lib/components/LoginScreen.svelte';
	import Toast from '$lib/components/Toast.svelte';
	import DistillTimer from '$lib/components/DistillTimer.svelte';
	import { getSidebarPosition, applyTheme } from '$lib/stores/settings.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	let { children } = $props();

	const isFloating = getCurrentWindow().label === 'floating-memo';

	const layoutClass = $derived(() => {
		const pos = getSidebarPosition();
		if (pos === 'top') return 'flex flex-col';
		if (pos === 'bottom') return 'flex flex-col-reverse';
		if (pos === 'right') return 'flex flex-row-reverse';
		return 'flex';
	});

	$effect(() => {
		applyTheme();
	});

	$effect(() => {
		if (!isFloating) {
			authStore.checkAuth();
		}
	});
</script>

{#if isFloating}
	{@render children()}
{:else if authStore.loading}
	<div class="flex h-screen items-center justify-center">
		<span class="loading loading-spinner loading-lg text-primary"></span>
	</div>
{:else if authStore.passwordSet && !authStore.authenticated}
	<LoginScreen />
{:else}
	<div class="{layoutClass()} h-screen overflow-hidden">
		<Sidebar />

		<main class="flex-1 overflow-y-auto py-10 px-12 relative">
			{@render children()}
			<div class="fixed bottom-2 right-3 z-10">
				<DistillTimer />
			</div>
		</main>
	</div>
{/if}

<Toast />
