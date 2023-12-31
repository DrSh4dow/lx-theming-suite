import { writable as internal, type Writable } from 'svelte/store';

declare type Updater<T> = (value: T) => T;
declare type StoreDict<T> = { [key: string]: Writable<T> };

/* eslint-disable @typescript-eslint/no-explicit-any */
interface Stores {
	local: StoreDict<any>;
	session: StoreDict<any>;
}

const stores: Stores = {
	local: {},
	session: {}
};

export interface Serializer<T> {
	parse(text: string): T;
	stringify(object: T): string;
}

export type StorageType = 'local' | 'session';

export interface Options<T> {
	serializer?: Serializer<T>;
	storage?: StorageType;
	syncTabs: boolean;
	onError?: (e: unknown) => void;
}

function getStorage(type: StorageType) {
	return type === 'local' ? localStorage : sessionStorage;
}

export function persisted<T>(key: string, initialValue: T, options?: Options<T>): Writable<T> {
	const serializer = options?.serializer ?? JSON;
	const storageType = options?.storage ?? 'local';
	const syncTabs = options?.syncTabs ?? true;
	const onError =
		options?.onError ??
		((e) =>
			console.error(`Error when writing value from persisted store "${key}" to ${storageType}`, e));
	const browser = typeof window !== 'undefined' && typeof document !== 'undefined';
	const storage = browser ? getStorage(storageType) : null;

	function updateStorage(key: string, value: T) {
		try {
			storage?.setItem(key, serializer.stringify(value));
		} catch (e) {
			onError(e);
		}
	}

	if (!stores[storageType][key]) {
		const store = internal(initialValue, (set) => {
			const json = storage?.getItem(key);

			if (json) {
				set(<T>serializer.parse(json));
			}

			if (browser && storageType == 'local' && syncTabs) {
				const handleStorage = (event: StorageEvent) => {
					if (event.key === key) set(event.newValue ? serializer.parse(event.newValue) : null);
				};

				window.addEventListener('storage', handleStorage);

				return () => window.removeEventListener('storage', handleStorage);
			}
		});

		const { subscribe, set } = store;

		stores[storageType][key] = {
			set(value: T) {
				set(value);
				updateStorage(key, value);
			},
			update(callback: Updater<T>) {
				return store.update((last) => {
					const value = callback(last);

					updateStorage(key, value);

					return value;
				});
			},
			subscribe
		};
	}

	return stores[storageType][key];
}
