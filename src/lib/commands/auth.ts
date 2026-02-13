import { invoke } from '@tauri-apps/api/core';

export interface AuthStatus {
	passwordSet: boolean;
	authenticated: boolean;
}

export function checkAuth(): Promise<AuthStatus> {
	return invoke<AuthStatus>('check_auth');
}

export function login(password: string): Promise<boolean> {
	return invoke<boolean>('login', { password });
}

export function setPassword(current: string | null, newPassword: string): Promise<void> {
	return invoke('set_password', { current, newPassword });
}

export function removePassword(current: string): Promise<void> {
	return invoke('remove_password', { current });
}

export function logout(): Promise<void> {
	return invoke('logout');
}
