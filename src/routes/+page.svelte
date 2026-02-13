<script lang="ts">
	import MashForm from '$lib/components/MashForm.svelte';
	import { t } from '$lib/i18n/index.svelte';
</script>

<div class="max-w-2xl mx-auto flex flex-col gap-8">
	<div>
		<h1 class="text-2xl font-semibold tracking-tight">{t('mashing.title')}</h1>
		<p class="text-sm text-base-content/40 mt-1">{t('mashing.subtitle')}</p>
	</div>
	<MashForm />
</div>
