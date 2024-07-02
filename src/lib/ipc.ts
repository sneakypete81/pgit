import { type Event } from '@tauri-apps/api/event';
import { listen as eventListen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export type Commit = {
	id: string;
	message: string;
	author: Person;
	time: string;
	parents: string[];
	column: number;
};

export type Person = {
	name: string;
	email: string;
};

export type Payload = {
	branches: string[];
	commits: Commit[];
};

export function init() {
	invoke('init');
}

export function listen(callback: (payload: Payload) => void) {
	eventListen('status', (event: Event<Payload>) => callback(event.payload));
}
