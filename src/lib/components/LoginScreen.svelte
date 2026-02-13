<script lang="ts">
	import { authStore } from '$lib/stores/auth.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let password = $state('');

	async function handleLogin(e: Event) {
		e.preventDefault();
		if (!password.trim()) return;
		await authStore.login(password.trim());
	}
</script>

<div class="flex h-screen items-center justify-center">
	<div class="w-full max-w-sm flex flex-col gap-6 px-6">
		<div class="text-center flex flex-col items-center">
			<svg width="96" height="96" viewBox="0 0 24 24" fill="none" class="mb-4">
				<rect x="7" y="3" width="10" height="2.5" rx="0.75" fill="currentColor" class="text-primary"/>
				<path d="M8 5.5H16V8C17.2 9 18 10.5 18 12.5V17.5C18 19.43 16.43 21 14.5 21H9.5C7.57 21 6 19.43 6 17.5V12.5C6 10.5 6.8 9 8 8V5.5Z" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linejoin="round" class="text-primary"/>
				<path d="M13 10A3 3 0 1 0 13 16A4 4 0 0 1 13 10Z" fill="currentColor" class="text-primary"/>
			</svg>
			<h1 class="text-2xl font-bold tracking-tight">Moonshine</h1>
			<p class="text-sm text-base-content/40 mt-1">{t('login.title')}</p>
		</div>

		<form class="flex flex-col gap-3" onsubmit={handleLogin}>
			<input
				type="password"
				class="input w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30"
				placeholder={t('login.password')}
				bind:value={password}
				required
			/>
			<button
				type="submit"
				class="btn btn-primary w-full"
				disabled={!password.trim()}
			>
				{t('login.enter')}
			</button>
		</form>

		{#if authStore.error}
			<p class="text-sm text-error text-center">
				{authStore.error.startsWith('login.') ? t(authStore.error as 'login.wrongPassword') : authStore.error}
			</p>
		{/if}
	</div>
</div>
