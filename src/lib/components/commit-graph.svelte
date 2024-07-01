<script lang="ts">
	import type { Commit } from './commit.svelte';
	import CommitCell from './commit.svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	import { listen, type Event } from '@tauri-apps/api/event';

	let commits: Commit[] = [];

	type Payload = {
		commits: string[];
	};

	listen(
		'branches',
		(event: Event<Payload>) =>
			(commits = event.payload.commits.map((message) => {
				return { message, author: 'me', date: new Date(), column: 0 };
			}))
	);
</script>

<ScrollArea class="h-32 h-full cursor-default select-none">
	{#each commits as commit}
		<CommitCell {commit} />
	{/each}
</ScrollArea>
