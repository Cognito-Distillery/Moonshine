import { MashStatus, type Mash, type MashType } from '$lib/types';
import * as cmd from '$lib/commands/mashes';
import { showToast } from '$lib/stores/toast.svelte';

let mashes = $state<Mash[]>([]);
let searchQuery = $state('');
let loading = $state(false);

export function getMashes() {
	return mashes;
}

export function isLoading() {
	return loading;
}

export function getSearchQuery() {
	return searchQuery;
}

export function setSearchQuery(q: string) {
	searchQuery = q;
}

export async function loadMashes(status: string, query?: string) {
	loading = true;
	try {
		const q = query?.trim() || searchQuery.trim() || undefined;
		mashes = await cmd.getMashesByStatus(status, q);
	} catch (e) {
		showToast(String(e));
	} finally {
		loading = false;
	}
}

export async function setMashStatus(id: string, status: MashStatus) {
	try {
		await cmd.setMashStatus(id, status);
	} catch (e) {
		showToast(String(e));
		throw e;
	}
}

export async function addMash(data: {
	type: MashType;
	summary: string;
	context: string;
	memo: string;
}) {
	try {
		await cmd.addMash(data.type, data.summary, data.context, data.memo);
		await loadMashes(MashStatus.MASH_TUN);
	} catch (e) {
		showToast(String(e));
		throw e;
	}
}

export async function deleteMash(id: string) {
	try {
		await cmd.deleteMash(id);
	} catch (e) {
		showToast(String(e));
		throw e;
	}
}

export async function updateMash(
	id: string,
	data: Partial<Pick<Mash, 'type' | 'summary' | 'context' | 'memo'>>
) {
	try {
		await cmd.updateMash(id, data.type, data.summary, data.context, data.memo);
	} catch (e) {
		showToast(String(e));
		throw e;
	}
}
