import { getLocale } from '$lib/i18n/index.svelte';

export type DateFormatId = 'short' | 'medium' | 'iso' | 'slash' | 'dot';
export type TimeFormatId = '24h' | '12h' | '24h-sec';

const localeTag: Record<string, string> = { ko: 'ko-KR', en: 'en-US' };

function getTag(): string {
	return localeTag[getLocale()] ?? 'ko-KR';
}

function pad2(n: number): string {
	return n < 10 ? '0' + n : String(n);
}

export function formatDate(ts: number, fmt: DateFormatId): string {
	const d = new Date(ts);
	const tag = getTag();

	switch (fmt) {
		case 'short':
			return d.toLocaleDateString(tag, { month: 'short', day: 'numeric' });
		case 'medium':
			return d.toLocaleDateString(tag, { year: 'numeric', month: 'short', day: 'numeric' });
		case 'iso':
			return `${d.getFullYear()}-${pad2(d.getMonth() + 1)}-${pad2(d.getDate())}`;
		case 'slash':
			return `${pad2(d.getMonth() + 1)}/${pad2(d.getDate())}/${d.getFullYear()}`;
		case 'dot':
			return `${d.getFullYear()}.${pad2(d.getMonth() + 1)}.${pad2(d.getDate())}`;
	}
}

export function formatTime(ts: number, fmt: TimeFormatId): string {
	const d = new Date(ts);
	const tag = getTag();

	switch (fmt) {
		case '24h':
			return d.toLocaleTimeString(tag, { hour: '2-digit', minute: '2-digit', hour12: false });
		case '12h':
			return d.toLocaleTimeString(tag, { hour: 'numeric', minute: '2-digit', hour12: true });
		case '24h-sec':
			return d.toLocaleTimeString(tag, { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false });
	}
}

export function formatDateTime(ts: number, dateFmt: DateFormatId, timeFmt: TimeFormatId): string {
	return `${formatDate(ts, dateFmt)} ${formatTime(ts, timeFmt)}`;
}
