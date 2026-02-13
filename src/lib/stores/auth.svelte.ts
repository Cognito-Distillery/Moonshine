import * as authCmd from '$lib/commands/auth';

function createAuthStore() {
	let passwordSet = $state(false);
	let authenticated = $state(false);
	let loading = $state(true);
	let error = $state('');

	return {
		get passwordSet() { return passwordSet; },
		get authenticated() { return authenticated; },
		get loading() { return loading; },
		get error() { return error; },

		async checkAuth() {
			loading = true;
			try {
				const status = await authCmd.checkAuth();
				passwordSet = status.passwordSet;
				authenticated = status.authenticated;
			} catch {
				authenticated = false;
				passwordSet = false;
			} finally {
				loading = false;
			}
		},

		async login(password: string) {
			error = '';
			try {
				const ok = await authCmd.login(password);
				if (ok) {
					authenticated = true;
				} else {
					error = 'login.wrongPassword';
				}
			} catch (e) {
				error = String(e);
			}
		},

		async logout() {
			try {
				await authCmd.logout();
			} catch {
				// clear local state regardless
			}
			authenticated = false;
		},

		clearError() {
			error = '';
		}
	};
}

export const authStore = createAuthStore();
