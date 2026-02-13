import ko, { type MessageKey } from './ko';
import en from './en';

export type Locale = 'ko' | 'en';

const STORAGE_KEY = 'moonshine-locale';
const messagesMap = { ko, en } as const;

function loadLocale(): Locale {
	if (typeof localStorage === 'undefined') return 'ko';
	const saved = localStorage.getItem(STORAGE_KEY);
	if (saved === 'ko' || saved === 'en') return saved;
	return 'ko';
}

let locale = $state<Locale>(loadLocale());

export function getLocale(): Locale {
	return locale;
}

export function setLocale(l: Locale) {
	locale = l;
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(STORAGE_KEY, l);
	}
}

export function t(key: MessageKey): string {
	return messagesMap[locale][key];
}

export type { MessageKey };
