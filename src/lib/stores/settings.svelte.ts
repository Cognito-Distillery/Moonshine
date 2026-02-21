import type { DateFormatId, TimeFormatId } from '$lib/utils/datetime';

export type SidebarPosition = 'left' | 'right' | 'top' | 'bottom';
export type ViewMode = 'card' | 'list';
export type ThemeId = 'monochrome-black' | 'monochrome-white' | 'warm-earth' | 'warm-earth-light' | 'peach' | 'forest' | 'oceanic';
export type ColorScheme = 'dark' | 'light';

export const themeRegistry: { id: ThemeId; colorScheme: ColorScheme }[] = [
	{ id: 'monochrome-black', colorScheme: 'dark' },
	{ id: 'monochrome-white', colorScheme: 'light' },
	{ id: 'warm-earth', colorScheme: 'dark' },
	{ id: 'warm-earth-light', colorScheme: 'light' },
	{ id: 'peach', colorScheme: 'light' },
	{ id: 'forest', colorScheme: 'dark' },
	{ id: 'oceanic', colorScheme: 'dark' }
];

export function getColorScheme(id: ThemeId): ColorScheme {
	return themeRegistry.find((t) => t.id === id)?.colorScheme ?? 'dark';
}

const STORAGE_KEY = 'moonshine-settings';

interface Settings {
	sidebarPosition: SidebarPosition;
	viewMode: ViewMode;
	dateFormat: DateFormatId;
	timeFormat: TimeFormatId;
	theme: ThemeId;
}

const DEFAULTS: Settings = {
	sidebarPosition: 'left',
	viewMode: 'list',
	dateFormat: 'medium',
	timeFormat: '24h',
	theme: 'monochrome-black'
};

function migrateTheme(theme: string): ThemeId {
	if (theme === 'dark' || theme === 'pitch') return 'monochrome-black';
	if (theme === 'light' || theme === 'moonlight') return 'monochrome-white';
	return theme as ThemeId;
}

function loadSettings(): Settings {
	if (typeof localStorage === 'undefined') return { ...DEFAULTS };
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw) {
			const parsed = { ...DEFAULTS, ...JSON.parse(raw) };
			parsed.theme = migrateTheme(parsed.theme);
			return parsed;
		}
	} catch {
		// ignore
	}
	return { ...DEFAULTS };
}

function saveSettings(settings: Settings) {
	if (typeof localStorage === 'undefined') return;
	localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
}

const initial = loadSettings();

let sidebarPosition = $state<SidebarPosition>(initial.sidebarPosition);
let viewMode = $state<ViewMode>(initial.viewMode);
let dateFormat = $state<DateFormatId>(initial.dateFormat);
let timeFormat = $state<TimeFormatId>(initial.timeFormat);
let theme = $state<ThemeId>(initial.theme);

function save() {
	saveSettings({ sidebarPosition, viewMode, dateFormat, timeFormat, theme });
}

export function getSidebarPosition() {
	return sidebarPosition;
}

export function setSidebarPosition(pos: SidebarPosition) {
	sidebarPosition = pos;
	save();
}

export function isHorizontal() {
	return sidebarPosition === 'top' || sidebarPosition === 'bottom';
}

export function getViewMode() {
	return viewMode;
}

export function setViewMode(mode: ViewMode) {
	viewMode = mode;
	save();
}

export function getDateFormat(): DateFormatId {
	return dateFormat;
}

export function setDateFormat(fmt: DateFormatId) {
	dateFormat = fmt;
	save();
}

export function getTimeFormat(): TimeFormatId {
	return timeFormat;
}

export function setTimeFormat(fmt: TimeFormatId) {
	timeFormat = fmt;
	save();
}

export function getTheme(): ThemeId {
	return theme;
}

export function setTheme(id: ThemeId) {
	theme = id;
	applyTheme();
	save();
}

export function applyTheme() {
	if (typeof document === 'undefined') return;
	document.documentElement.setAttribute('data-theme', theme);
}
